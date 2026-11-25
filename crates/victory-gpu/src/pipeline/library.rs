//! Compiled host pipelines kept on disk, and the title pipelines worth
//! starting early (decision 0007, "Pipeline cache").
//!
//! 1. Every host pipeline goes into the host's pipeline library, named by
//!    a hash of all that its host description holds (the translated
//!    shaders' code, which changes when a translation does, our root
//!    signature, the graphics state). The library is saved under the cache
//!    folder; the host refuses a library of another adapter or driver, and
//!    an entry whose description differs, so neither is ever used.
//! 2. The title pipelines a run drew or dispatched with (made in time or
//!    not) are listed by a key known when the title creates them (their
//!    shader containers and graphics state); the next run hands those to
//!    the workers at creation, during loading. A full list keeps the
//!    latest run's first.
//!
//! 3. The pre-build (`victory translate --cache --pipelines`) lists the
//!    title pipelines it has put in the library, by the same key, so a
//!    second pre-build skips them without translating or loading anything.
//!    The list holds only for the library bytes it was saved with and for
//!    what names the pipelines (the translator, this module and `host`).
//!
//! The library is free-threaded but for loads of one pipeline on several
//! threads (`ID3D12Device1::CreatePipelineLibrary`, "Thread Safety"), so
//! pipelines of different names load at the same time. A load is not always
//! quick: the library of an AMD driver holds about 140 bytes per pipeline,
//! and what its own cache lacks is compiled inside the load, for seconds.
//! Loaded one at a time, 650 pipelines kept the eight workers to one core
//! for minutes, while each title frame could wait out its 500 ms for
//! them (`make`). Stores and serialising keep the lock: the size asked for
//! must still be right when the bytes are written.

use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Condvar, Mutex, OnceLock};

use windows::Win32::Graphics::Direct3D12::*;
use windows::core::{HSTRING, Interface, PCWSTR};
use windows_sys::Win32::Storage::FileSystem::{
    FILE_SHARE_READ, MOVEFILE_WRITE_THROUGH, MoveFileExW,
};
use windows_sys::Win32::System::Memory::{
    CreateFileMappingW, FILE_MAP_READ, MEMORY_MAPPED_VIEW_ADDRESS, MapViewOfFile,
    PAGE_READONLY, UnmapViewOfFile,
};

static DIR: OnceLock<PathBuf> = OnceLock::new();

/// Keeps the pipeline library and the early list in `dir` from now on.
pub fn use_pipeline_cache(dir: PathBuf) {
    let _ = DIR.set(dir);
}

/// Host pipelines created, loaded from the library, and title pipelines
/// started at creation, since the start.
pub(crate) static MADE: AtomicU64 = AtomicU64::new(0);
pub(crate) static LOADED: AtomicU64 = AtomicU64::new(0);
pub(crate) static EARLY: AtomicU64 = AtomicU64::new(0);

struct Library {
    library: ID3D12PipelineLibrary1,
    /// The blob the library was made from: it must outlive the library.
    _blob: Option<MappedBlob>,
    /// Pipelines stored since the last save.
    added: u64,
    /// Title pipelines the pre-build put in the library or found there, by
    /// key, and whether that list changed since the last save.
    prebuilt: HashSet<String>,
    prebuilt_changed: bool,
}

// SAFETY: D3D12 objects are free-threaded (a pipeline library with the
// exception `NAMES` takes care of). The read-only view and its handles
// can be moved between threads.
unsafe impl Send for Library {}

/// A read-only view, with the handles that keep its file and section alive.
struct MappedBlob {
    view: MEMORY_MAPPED_VIEW_ADDRESS,
    len: usize,
    _mapping: OwnedHandle,
    _file: File,
}

impl MappedBlob {
    /// Opens an existing blob without sharing write or delete access.
    fn open(path: &Path) -> io::Result<Option<Self>> {
        let file = match OpenOptions::new().read(true).share_mode(FILE_SHARE_READ).open(path) {
            Ok(file) => file,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e),
        };
        let len = file.metadata()?.len();
        if len == 0 {
            return Ok(None);
        }
        if len > isize::MAX as u64 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "pipeline library too large"));
        }
        // SAFETY: a live read-only file handle. Zero maps the current file size.
        let mapping = unsafe {
            let (handle, none) = (file.as_raw_handle(), std::ptr::null());
            CreateFileMappingW(handle, none, PAGE_READONLY, 0, 0, std::ptr::null())
        };
        if mapping.is_null() {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: the new mapping handle is owned by this call.
        let mapping = unsafe { OwnedHandle::from_raw_handle(mapping) };
        // SAFETY: a live read-only section, with a view of the whole section.
        let view = unsafe { MapViewOfFile(mapping.as_raw_handle(), FILE_MAP_READ, 0, 0, 0) };
        if view.Value.is_null() {
            return Err(io::Error::last_os_error());
        }
        Ok(Some(Self { view, len: len as usize, _mapping: mapping, _file: file }))
    }

    fn bytes(&self) -> &[u8] {
        // SAFETY: the view covers this length and the file cannot be written
        // or truncated while its read-only handle is held.
        unsafe { std::slice::from_raw_parts(self.view.Value.cast(), self.len) }
    }
}

impl Drop for MappedBlob {
    fn drop(&mut self) {
        // SAFETY: this view is owned by us and no slice can still borrow it.
        unsafe { UnmapViewOfFile(self.view) };
    }
}

/// Names one thread at a time holds.
struct Names {
    held: Mutex<Option<HashSet<String>>>,
    released: Condvar,
}

/// A name held until this is dropped.
struct Held<'a> {
    names: &'a Names,
    name: &'a str,
}

impl Names {
    const fn new() -> Names {
        Names {
            held: Mutex::new(None),
            released: Condvar::new(),
        }
    }

    /// Holds `name`, once no other thread does.
    fn hold<'a>(&'a self, name: &'a str) -> Held<'a> {
        let mut held = self.held.lock().unwrap_or_else(|e| e.into_inner());
        while held.as_ref().is_some_and(|h| h.contains(name)) {
            held = self.released.wait(held).unwrap_or_else(|e| e.into_inner());
        }
        held.get_or_insert_default().insert(name.to_owned());
        Held { names: self, name }
    }
}

impl Drop for Held<'_> {
    fn drop(&mut self) {
        let mut held = self.names.held.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(h) = held.as_mut() {
            h.remove(self.name);
        }
        drop(held);
        self.names.released.notify_all();
    }
}

/// Pipelines being loaded, or made and stored.
static NAMES: Names = Names::new();

const LIBRARY_FILE: &str = "host-pipelines.bin";
const EARLY_FILE: &str = "early-pipelines.txt";
const PREBUILT_FILE: &str = "prebuilt-pipelines.txt";
/// Title pipelines listed for an early start, at most.
const EARLY_MAX: usize = 8192;

fn library() -> Option<&'static Mutex<Library>> {
    static LIBRARY: OnceLock<Option<Mutex<Library>>> = OnceLock::new();
    LIBRARY
        .get_or_init(|| {
            let device = crate::host::host()?.device.cast::<ID3D12Device1>().ok()?;
            let read = |file| DIR.get().and_then(|d| std::fs::read(d.join(file)).ok());
            let blob = DIR.get().and_then(|d| load_blob(d).ok().flatten());
            let tag = tag(blob.as_ref().map_or(&[], MappedBlob::bytes));
            let mut library = open(&device, blob);
            // The pre-build's list, if it was saved with these bytes.
            if let (Some(l), Some(list)) = (library.as_mut(), read(PREBUILT_FILE))
                && l._blob.is_some()
            {
                l.prebuilt = listed(&list, &tag);
            }
            if library.is_none() {
                tracing::warn!("no host pipeline library: pipelines are not kept");
            }
            library.map(Mutex::new)
        })
        .as_ref()
}

/// The library in `blob`, or an empty one if the host rejects the blob (an
/// adapter or driver change, damage).
fn open(device: &ID3D12Device1, blob: Option<MappedBlob>) -> Option<Library> {
    let make = |bytes: &[u8]| -> windows::core::Result<ID3D12PipelineLibrary1> {
        // SAFETY: the bytes outlive the library (kept beside it).
        unsafe {
            device.CreatePipelineLibrary::<ID3D12PipelineLibrary>(bytes)?.cast()
        }
    };
    if let Some(blob) = blob {
        let started = std::time::Instant::now();
        match make(blob.bytes()) {
            Ok(library) => {
                let ms = started.elapsed().as_millis() as u64;
                tracing::info!(bytes = blob.len, ms, "host pipeline library loaded");
                return Some(Library {
                    library,
                    _blob: Some(blob),
                    added: 0,
                    prebuilt: HashSet::new(),
                    prebuilt_changed: false,
                });
            }
            Err(e) => tracing::info!(error = %e, "host pipeline library rejected; starting an empty one"),
        }
    } else {
        tracing::info!("no host pipeline library yet; starting an empty one");
    }
    let library = make(&[]).ok()?;
    Some(Library {
        library,
        _blob: None,
        added: 0,
        prebuilt: HashSet::new(),
        prebuilt_changed: false,
    })
}

/// What the pre-build's list holds for: the library's bytes, and what
/// names its pipelines (the translation, as the shader cache keys it; the
/// host descriptions).
fn tag(blob: &[u8]) -> String {
    let limit = victory_shader::emit::loop_limit().to_le_bytes();
    let views = [victory_shader::emit::heap_views(), victory_shader::emit::title_views()].map(u32::to_le_bytes).concat();
    let translator = victory_shader::SOURCE_HASH.as_bytes();
    let mut h = blake3::Hasher::new();
    for part in [
        blob,
        translator,
        include_bytes!("../shader_cache.rs"),
        &limit,
        &views,
        include_bytes!("host.rs"),
        include_bytes!("library.rs"),
    ] {
        h.update(&(part.len() as u64).to_le_bytes());
        h.update(part);
    }
    h.finalize().to_hex().to_string()
}

/// The pre-build's list as saved: the tag it holds for, then its keys.
fn list(tag: &str, keys: &HashSet<String>) -> String {
    let keys = keys.iter().map(String::as_str);
    std::iter::once(tag).chain(keys).collect::<Vec<_>>().join("\n")
}

/// The keys of a saved list, if it holds for `tag` (else none).
fn listed(list: &[u8], tag: &str) -> HashSet<String> {
    let list = String::from_utf8_lossy(list);
    let mut lines = list.lines();
    match lines.next() == Some(tag) {
        true => lines.map(str::to_owned).collect(),
        false => HashSet::new(),
    }
}

/// Whether the pre-build has the title pipeline `key` in the library.
pub(crate) fn prebuilt(key: &str) -> bool {
    library().is_some_and(|l| l.lock().unwrap_or_else(|e| e.into_inner()).prebuilt.contains(key))
}

/// The pre-build has the title pipeline `key` in the library now.
pub(crate) fn note_prebuilt(key: String) {
    if let Some(l) = library() {
        let mut l = l.lock().unwrap_or_else(|e| e.into_inner());
        l.prebuilt_changed |= l.prebuilt.insert(key);
    }
}

/// A pipeline's name in the library: the hash of what its host
/// description holds.
pub(crate) fn name(kind: &str, root: &[u8], code: &[&[u8]], state: &str) -> String {
    let mut h = blake3::Hasher::new();
    for part in [kind.as_bytes(), root].into_iter().chain(code.iter().copied()).chain([state.as_bytes()]) {
        h.update(&(part.len() as u64).to_le_bytes());
        h.update(part);
    }
    h.finalize().to_hex()[..32].to_owned()
}

/// The pipeline named `name`: loaded from the library by `load` if it is
/// there with the same description, else made by `create` and stored. A
/// name asked for by several threads is loaded or made by one at a time
/// (the others then find it in the library); different names overlap.
pub(crate) fn pipeline(
    name: &str,
    load: impl FnOnce(&ID3D12PipelineLibrary1, PCWSTR) -> windows::core::Result<ID3D12PipelineState>,
    create: impl FnOnce() -> windows::core::Result<ID3D12PipelineState>,
) -> windows::core::Result<ID3D12PipelineState> {
    let wide = HSTRING::from(name);
    #[cfg(feature = "diagnostics")]
    LAST.set(Some((name.to_owned(), true)));
    let Some(library) = library() else {
        MADE.fetch_add(1, Ordering::Relaxed);
        return create();
    };
    let _held = NAMES.hold(name);
    // The interface alone under the lock: loads take up to seconds. Its
    // blob lives as long as the library, which is never dropped.
    let loads = library
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .library
        .clone();
    if let Ok(pso) = load(&loads, PCWSTR(wide.as_ptr())) {
        LOADED.fetch_add(1, Ordering::Relaxed);
        return Ok(pso);
    }
    // Compiled outside the lock: compiles take up to seconds.
    #[cfg(feature = "diagnostics")]
    LAST.set(Some((name.to_owned(), false)));
    let pso = create()?;
    MADE.fetch_add(1, Ordering::Relaxed);
    let mut l = library.lock().unwrap_or_else(|e| e.into_inner());
    // SAFETY: a live pipeline; a name already stored (a description that
    // differs) is refused, which leaves the library as it was.
    if unsafe { l.library.StorePipeline(PCWSTR(wide.as_ptr()), &pso) }.is_ok() {
        l.added += 1;
    }
    Ok(pso)
}

#[cfg(feature = "diagnostics")]
thread_local! {
    /// This thread's last pipeline: its name, and whether it was loaded
    /// (else made), for the pipeline log (diagnostics).
    pub(crate) static LAST: std::cell::Cell<Option<(String, bool)>> =
        const { std::cell::Cell::new(None) };
}

/// Writes the library and the early list if pipelines were added since the
/// last save and `SAVE_EVERY` has passed (or `now`).
pub(crate) fn save(now: bool) {
    static LAST: Mutex<Option<std::time::Instant>> = Mutex::new(None);
    const SAVE_EVERY: std::time::Duration = std::time::Duration::from_secs(20);
    let (Some(dir), Some(library)) = (DIR.get(), library()) else { return };
    // Held through publication, so that saves publish in order. A periodic
    // save gives way to one under way (the workers call this after every
    // pipeline and would wait for its writes); the shutdown save waits.
    let mut last = if now {
        LAST.lock().unwrap_or_else(|e| e.into_inner())
    } else {
        match LAST.try_lock() {
            Ok(last) => last,
            Err(std::sync::TryLockError::Poisoned(e)) => e.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => return,
        }
    };
    if !now && last.is_some_and(|t| t.elapsed() < SAVE_EVERY) {
        return;
    }
    *last = Some(std::time::Instant::now());
    let (blob, prebuilt) = {
        let mut l = library.lock().unwrap_or_else(|e| e.into_inner());
        if l.added == 0 && !l.prebuilt_changed && !USED_CHANGED.swap(false, Ordering::Relaxed) {
            return;
        }
        // SAFETY: a buffer of the size the library asks for.
        let bytes = unsafe {
            let mut bytes = vec![0u8; l.library.GetSerializedSize()];
            l.library.Serialize(&mut bytes).map(|()| bytes)
        };
        match bytes {
            Ok(bytes) => {
                l.added = 0;
                l.prebuilt_changed = false;
                // Kept beside the bytes it holds for (also when a run adds
                // pipelines).
                let prebuilt = (!l.prebuilt.is_empty()).then(|| list(&tag(&bytes), &l.prebuilt));
                (bytes, prebuilt)
            }
            Err(e) => {
                tracing::warn!(error = %e, "host pipeline library not serialised");
                return;
            }
        }
    };
    let used = USED.lock().unwrap_or_else(|e| e.into_inner()).as_ref().map(Used::list).unwrap_or_default();
    let written = std::fs::create_dir_all(dir)
        .and_then(|()| save_blob(dir, &blob))
        .and_then(|()| write_atomically(&dir.join(EARLY_FILE), used.join("\n").as_bytes()))
        .and_then(|()| match &prebuilt {
            Some(list) => write_atomically(&dir.join(PREBUILT_FILE), list.as_bytes()),
            None => Ok(()),
        });
    drop(last);
    let (created, distinct) = created();
    let (shaders_cached, shaders_translated) = crate::shader_cache::counts();
    match written {
        Ok(()) => tracing::info!(
            bytes = blob.len(),
            early = used.len(),
            made = MADE.load(Ordering::Relaxed),
            loaded = LOADED.load(Ordering::Relaxed),
            started_early = EARLY.load(Ordering::Relaxed),
            created,
            distinct,
            shaders_cached,
            shaders_translated,
            "host pipeline library saved"
        ),
        Err(e) => tracing::warn!(error = %e, "host pipeline library not saved"),
    }
}

/// Complete blobs in generation order, with the original filename first.
/// Names compare as the file system does, without case.
fn generations(dir: &Path) -> io::Result<Vec<(u64, PathBuf)>> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        let name = name.to_ascii_lowercase();
        let generation = if name == LIBRARY_FILE {
            0
        } else if let Some(suffix) =
            name.strip_prefix(LIBRARY_FILE).and_then(|s| s.strip_prefix('.'))
        {
            let Ok(generation) = suffix.parse::<u64>() else { continue };
            if generation == 0 || suffix != generation.to_string() {
                continue;
            }
            generation
        } else {
            continue;
        };
        files.push((generation, entry.path()));
    }
    files.sort_unstable_by_key(|(generation, _)| *generation);
    Ok(files)
}

/// Removes older generations, leaving files another run still maps alone.
fn remove_older(files: &[(u64, PathBuf)], generation: u64) {
    for (_, path) in files.iter().take_while(|(n, _)| *n < generation) {
        let _ = std::fs::remove_file(path);
    }
}

/// Opens the newest blob that opens, newest by when it was written (so a
/// build that writes only the original filename, as earlier ones did, is
/// not outvoted by an older numbered file), then by generation. A file that
/// does not open (empty, gone, or held by another process for the moment)
/// leaves the next older one to try. The older ones are removed where they
/// can be, and so are staging files of processes that ended during a save.
fn load_blob(dir: &Path) -> io::Result<Option<MappedBlob>> {
    remove_stale_staging(dir);
    let mut files = generations(dir)?;
    let written = |path: &Path| std::fs::metadata(path).and_then(|m| m.modified()).ok();
    files.sort_by_cached_key(|(generation, path)| (written(path), *generation));
    for (i, (_, path)) in files.iter().enumerate().rev() {
        match MappedBlob::open(path) {
            Ok(Some(blob)) => {
                // The older ones only: a newer one that did not open now
                // (held for the moment, or dated before its time by a clock
                // set back) is left for the next start.
                for (_, older) in &files[..i] {
                    let _ = std::fs::remove_file(older);
                }
                return Ok(Some(blob));
            }
            Ok(None) => {}
            Err(e) => {
                let path = path.display();
                tracing::info!(error = %e, %path, "a host pipeline library file does not open");
            }
        }
    }
    Ok(None)
}

/// The staging files `save_blob` writes are named for the process.
const STAGING: &str = ".tmp-";

/// Removes staging files of processes that are not running any more (a
/// save cut short by the end of the process): each is as big as the
/// library. A running process's are left alone, ours included.
fn remove_stale_staging(dir: &Path) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let prefix = format!("{LIBRARY_FILE}{STAGING}");
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        let Some(rest) = name.strip_prefix(&prefix) else { continue };
        let Some(pid) = rest.split('-').next().and_then(|p| p.parse::<u32>().ok()) else {
            continue;
        };
        if !running(pid) {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}

/// Whether process `pid` is running (ours is).
fn running(pid: u32) -> bool {
    use windows_sys::Win32::Foundation::{CloseHandle, STILL_ACTIVE};
    use windows_sys::Win32::System::Threading::{
        GetExitCodeProcess, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
    };
    if pid == std::process::id() {
        return true;
    }
    // SAFETY: a plain query; the handle is closed below.
    let process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
    if process.is_null() {
        // Only "no such process" is safe to clean up after: any other
        // failure (access denied, among others) counts as running.
        const ERROR_INVALID_PARAMETER: i32 = 87;
        return io::Error::last_os_error().raw_os_error() != Some(ERROR_INVALID_PARAMETER);
    }
    let mut code = 0;
    // SAFETY: a live process handle and a valid out pointer.
    let asked = unsafe { GetExitCodeProcess(process, &mut code) } != 0;
    let alive = asked && code == STILL_ACTIVE as u32;
    // SAFETY: the handle opened above.
    unsafe { CloseHandle(process) };
    alive
}

/// Publishes a new generation so a save never replaces a mapped file.
fn save_blob(dir: &Path, bytes: &[u8]) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    static NEXT: AtomicU64 = AtomicU64::new(0);

    const TRIES: usize = 128;
    let dir = dir.canonicalize()?;
    let mut staged = None;
    for _ in 0..TRIES {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let tmp = dir.join(format!("{LIBRARY_FILE}{STAGING}{}-{n}", std::process::id()));
        // Not shared: no one deletes it while it is written.
        match OpenOptions::new().write(true).create_new(true).share_mode(0).open(&tmp) {
            Ok(file) => {
                staged = Some((tmp, file));
                break;
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
    let (tmp, mut file) = staged.ok_or_else(|| io::Error::other("no free staging name"))?;
    let ready = file.write_all(bytes).and_then(|()| file.sync_all());
    drop(file);
    let res = ready.and_then(|()| {
        let source: Vec<_> = tmp.as_os_str().encode_wide().chain([0]).collect();
        for _ in 0..TRIES {
            let files = generations(&dir)?;
            let newest = files.last().map_or(0, |(n, _)| *n);
            let generation = newest
                .checked_add(1)
                .ok_or_else(|| io::Error::other("pipeline library generation exhausted"))?;
            let target = dir.join(format!("{LIBRARY_FILE}.{generation}"));
            let destination: Vec<_> = target.as_os_str().encode_wide().chain([0]).collect();
            // SAFETY: both paths are live, NUL-terminated strings. The move
            // stays in this directory and cannot replace an existing file.
            let moved = unsafe {
                MoveFileExW(source.as_ptr(), destination.as_ptr(), MOVEFILE_WRITE_THROUGH)
            };
            if moved != 0 {
                remove_older(&files, generation);
                return Ok(());
            }
            let e = io::Error::last_os_error();
            if e.kind() != io::ErrorKind::AlreadyExists {
                return Err(e);
            }
        }
        Err(io::Error::other("no free pipeline library generation"))
    });
    if res.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    res
}

/// Writes `bytes` to a temporary file beside `path`, then renames it over
/// `path`: a reader never sees a part. Serialised so concurrent worker and
/// shutdown saves do not race on the same staging file or target rename.
fn write_atomically(path: &Path, bytes: &[u8]) -> io::Result<()> {
    static WRITING: Mutex<()> = Mutex::new(());
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let _guard = WRITING.lock().unwrap_or_else(|e| e.into_inner());
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(format!(".tmp-{}-{n}", std::process::id()));
    let tmp = path.with_file_name(name);
    let res = std::fs::write(&tmp, bytes).and_then(|()| std::fs::rename(&tmp, path));
    if res.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    res
}

/// Title pipelines created, and their distinct keys.
static CREATED: Mutex<(u64, Option<HashSet<String>>)> = Mutex::new((0, None));

/// The title created a pipeline with `key`.
pub(crate) fn note_created(key: &str) {
    let mut created = CREATED.lock().unwrap_or_else(|e| e.into_inner());
    created.0 += 1;
    created.1.get_or_insert_with(HashSet::new).insert(key.to_owned());
}

/// Title pipelines created so far, and how many distinct ones.
fn created() -> (u64, usize) {
    let created = CREATED.lock().unwrap_or_else(|e| e.into_inner());
    (created.0, created.1.as_ref().map_or(0, HashSet::len))
}

/// Title pipeline keys drawn or dispatched with: in earlier runs (the
/// list as loaded), and in this one.
#[derive(Default)]
struct Used {
    before: HashSet<String>,
    now: HashSet<String>,
}

impl Used {
    /// The list to keep: this run's keys, then earlier runs' others, at
    /// most `EARLY_MAX` (the oldest go once it is full).
    fn list(&self) -> Vec<String> {
        let earlier = self.before.iter().filter(|k| !self.now.contains(*k));
        self.now.iter().chain(earlier).take(EARLY_MAX).cloned().collect()
    }
}

static USED: Mutex<Option<Used>> = Mutex::new(None);
static USED_CHANGED: AtomicBool = AtomicBool::new(false);

fn used() -> std::sync::MutexGuard<'static, Option<Used>> {
    let mut used = USED.lock().unwrap_or_else(|e| e.into_inner());
    if used.is_none() {
        let listed = DIR
            .get()
            .and_then(|d| std::fs::read_to_string(d.join(EARLY_FILE)).ok())
            .unwrap_or_default();
        let before: HashSet<String> = listed.lines().filter(|l| !l.is_empty()).map(str::to_owned).collect();
        tracing::info!(pipelines = before.len(), "title pipelines to start early");
        *used = Some(Used { before, now: HashSet::new() });
    }
    used
}

/// Reads the early list now if not yet, so a save keeps it (a pre-build
/// asks for no title pipeline).
pub(crate) fn keep_early_list() {
    drop(used());
}

/// Whether the title pipeline with `key` was drawn with in an earlier run.
pub(crate) fn wanted_early(key: &str) -> bool {
    used().as_ref().is_some_and(|u| u.before.contains(key))
}

/// The title pipeline with `key` was drawn or dispatched with.
pub(crate) fn note_used(key: &str) {
    if used().get_or_insert_with(Used::default).now.insert(key.to_owned()) {
        USED_CHANGED.store(true, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapped_blob_matches_file() {
        let dir = tempfile::tempdir_in(std::env::temp_dir()).unwrap();
        let path = dir.path().join(LIBRARY_FILE);
        let bytes = b"pipeline library bytes";
        std::fs::write(&path, bytes).unwrap();
        let blob = MappedBlob::open(&path).unwrap().unwrap();
        assert_eq!(blob.bytes(), bytes);
        assert_eq!(tag(blob.bytes()), tag(bytes));
    }

    #[test]
    fn mapped_blob_survives_saves() {
        let dir = tempfile::tempdir_in(std::env::temp_dir()).unwrap();
        let path = dir.path().join(LIBRARY_FILE);
        std::fs::write(&path, b"old bytes").unwrap();
        let old = load_blob(dir.path()).unwrap().unwrap();
        assert!(OpenOptions::new().write(true).open(&path).is_err());
        assert!(OpenOptions::new().write(true).truncate(true).open(&path).is_err());
        let rename = write_atomically(&path, b"replacement");
        println!("rename over mapped file: {rename:?}");
        assert!(rename.is_err());
        assert_eq!(old.bytes(), b"old bytes");

        save_blob(dir.path(), b"new bytes").unwrap();
        let loaded = load_blob(dir.path()).unwrap().unwrap();
        assert_eq!(loaded.bytes(), b"new bytes");
        assert_eq!(old.bytes(), b"old bytes");

        save_blob(dir.path(), b"newest bytes").unwrap();
        // Staging files of a save cut short: one named for a process ID no
        // process has, one of ours.
        let gone = (1_000_000u32..).step_by(4).find(|&p| !running(p)).unwrap();
        let stale = dir.path().join(format!("{LIBRARY_FILE}{STAGING}{gone}-0"));
        let ours = dir.path().join(format!("{LIBRARY_FILE}{STAGING}{}-999", std::process::id()));
        std::fs::write(&stale, b"partial").unwrap();
        std::fs::write(&ours, b"partial").unwrap();
        drop(old);
        drop(loaded);
        let newest = load_blob(dir.path()).unwrap().unwrap();
        assert_eq!(newest.bytes(), b"newest bytes");
        assert_eq!(generations(dir.path()).unwrap().len(), 1);
        assert!(!path.exists());
        assert!(!stale.exists(), "a dead process's staging file is removed");
        assert!(ours.exists(), "a running process's staging file is left");
    }

    /// The newest file that opens is taken: an empty newest generation
    /// leaves the one before; an original filename written after the
    /// numbered ones (by a build that writes only that name) wins.
    #[test]
    fn mapped_blob_takes_the_newest_file_that_opens() {
        let dir = tempfile::tempdir().unwrap();
        save_blob(dir.path(), b"first").unwrap();
        // Mapped, the first generation outlives the second's save.
        let held = load_blob(dir.path()).unwrap().unwrap();
        save_blob(dir.path(), b"second").unwrap();
        drop(held);
        assert_eq!(generations(dir.path()).unwrap().len(), 2);
        let newest = generations(dir.path()).unwrap().last().unwrap().1.clone();
        std::fs::write(&newest, b"").unwrap();
        let blob = load_blob(dir.path()).unwrap().unwrap();
        assert_eq!(blob.bytes(), b"first", "the generation before an empty one");
        drop(blob);
        // Written later than any numbered one, under the original name (the
        // times are set: a file system may keep them to the second).
        let numbered = generations(dir.path()).unwrap();
        let earlier = std::time::SystemTime::now() - std::time::Duration::from_secs(60);
        for (_, path) in &numbered {
            File::options().write(true).open(path).unwrap().set_modified(earlier).unwrap();
        }
        std::fs::write(dir.path().join(LIBRARY_FILE), b"from another build").unwrap();
        let blob = load_blob(dir.path()).unwrap().unwrap();
        assert_eq!(blob.bytes(), b"from another build");
    }

    #[test]
    fn mapped_blob_skips_missing_and_empty_files() {
        let dir = tempfile::tempdir_in(std::env::temp_dir()).unwrap();
        let path = dir.path().join(LIBRARY_FILE);
        assert!(MappedBlob::open(&path).unwrap().is_none());
        assert!(load_blob(dir.path()).unwrap().is_none());
        File::create(&path).unwrap();
        assert!(MappedBlob::open(&path).unwrap().is_none());
        assert!(load_blob(dir.path()).unwrap().is_none());
    }

    #[test]
    #[ignore = "synthetic private-memory measurement"]
    fn mapped_blob_private_memory() {
        use windows_sys::Win32::System::ProcessStatus::{
            K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS_EX,
        };
        use windows_sys::Win32::System::Threading::GetCurrentProcess;

        fn private_bytes() -> usize {
            let mut counters = PROCESS_MEMORY_COUNTERS_EX {
                cb: std::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32,
                ..Default::default()
            };
            // SAFETY: the extended structure and its size are passed together.
            let ok = unsafe {
                K32GetProcessMemoryInfo(
                    GetCurrentProcess(),
                    (&mut counters as *mut PROCESS_MEMORY_COUNTERS_EX).cast(),
                    counters.cb,
                )
            };
            assert_ne!(ok, 0);
            counters.PrivateUsage
        }

        const MIB: usize = 1024 * 1024;
        let dir = tempfile::tempdir_in(std::env::temp_dir()).unwrap();
        let path = dir.path().join(LIBRARY_FILE);
        let mut file = File::create(&path).unwrap();
        let chunk = vec![0xa5; MIB];
        for _ in 0..256 {
            file.write_all(&chunk).unwrap();
        }
        drop(file);
        drop(chunk);

        let before_read = private_bytes();
        let read = std::fs::read(&path).unwrap();
        let hash = blake3::hash(&read);
        let after_read = private_bytes();
        drop(read);

        let before_map = private_bytes();
        let map = MappedBlob::open(&path).unwrap().unwrap();
        assert_eq!(blake3::hash(map.bytes()), hash);
        let after_map = private_bytes();
        println!(
            "synthetic 256 MiB blob, process private MiB: read {:.2} -> {:.2}, map {:.2} -> {:.2}",
            before_read as f64 / MIB as f64,
            after_read as f64 / MIB as f64,
            before_map as f64 / MIB as f64,
            after_map as f64 / MIB as f64,
        );
    }

    #[test]
    fn names_change_with_what_goes_into_the_pipeline() {
        let a = name("graphics", b"root", &[b"vs", b"ps"], "state");
        assert_eq!(a, name("graphics", b"root", &[b"vs", b"ps"], "state"));
        assert_ne!(a, name("graphics", b"root", &[b"vs", b"ps2"], "state"));
        assert_ne!(a, name("graphics", b"root", &[b"vs", b"ps"], "state2"));
        assert_ne!(a, name("graphics", b"root2", &[b"vs", b"ps"], "state"));
        assert_ne!(a, name("mesh", b"root", &[b"vs", b"ps"], "state"));
        // Parts do not run into each other.
        assert_ne!(a, name("graphics", b"root", &[b"vsp", b"s"], "state"));
    }

    /// This run's keys stay in the list; earlier runs' fill it up to the cap.
    #[test]
    fn the_early_list_keeps_this_run_first() {
        let keys = |r: std::ops::Range<usize>| r.map(|i| format!("{i:032x}")).collect::<HashSet<_>>();
        let used = Used { before: keys(0..EARLY_MAX), now: keys(EARLY_MAX - 10..EARLY_MAX + 10) };
        let list = used.list();
        assert_eq!(list.len(), EARLY_MAX);
        assert!(used.now.iter().all(|k| list.contains(k)));
        assert_eq!(list.iter().collect::<HashSet<_>>().len(), EARLY_MAX, "no key twice");
    }

    /// The pre-build's list holds only for the library bytes it was
    /// saved with.
    #[test]
    fn the_prebuilt_list_holds_for_its_library_only() {
        let keys: HashSet<String> = ["a".repeat(32), "b".repeat(32)].into();
        let saved = list(&tag(b"library"), &keys);
        assert_eq!(listed(saved.as_bytes(), &tag(b"library")), keys);
        assert!(listed(saved.as_bytes(), &tag(b"library2")).is_empty());
        assert!(listed(b"", &tag(b"library")).is_empty());
    }

    #[test]
    fn different_names_are_held_at_the_same_time() {
        static NAMES: Names = Names::new();
        // Each thread holds its name until every thread holds its own.
        let all = std::sync::Barrier::new(8);
        std::thread::scope(|s| {
            for i in 0..8 {
                let all = &all;
                s.spawn(move || {
                    let name = format!("pipeline {i}");
                    let _held = NAMES.hold(&name);
                    all.wait();
                });
            }
        });
        assert!(NAMES.held.lock().unwrap().as_ref().unwrap().is_empty());
    }

    #[test]
    fn one_name_is_held_by_one_thread_at_a_time() {
        static NAMES: Names = Names::new();
        static INSIDE: AtomicU64 = AtomicU64::new(0);
        std::thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    for _ in 0..500 {
                        let _held = NAMES.hold("pipeline");
                        assert_eq!(INSIDE.fetch_add(1, Ordering::SeqCst), 0);
                        std::hint::spin_loop();
                        INSIDE.fetch_sub(1, Ordering::SeqCst);
                    }
                });
            }
        });
        // A holder that panics lets go of it.
        let panicked = std::thread::spawn(|| {
            let _held = NAMES.hold("pipeline");
            panic!("on purpose");
        });
        assert!(panicked.join().is_err());
        drop(NAMES.hold("pipeline"));
    }

    /// A library's pipelines load on many threads at once, each with its
    /// own description (a load with another's is refused), while others are
    /// stored and the library is serialised.
    #[test]
    fn pipelines_load_on_many_threads_at_once() {
        use windows::Win32::Graphics::Direct3D::Fxc::D3DCompile;
        use windows::core::s;
        const N: usize = 48;
        let Some(device) = crate::host::host().and_then(|h| h.device.cast::<ID3D12Device1>().ok())
        else {
            return; // no host GPU here
        };
        let shader = |i: usize| {
            let source = format!(
                "#define RS \"UAV(u0)\"\nRWByteAddressBuffer b : register(u0);\n\
                 [RootSignature(RS)] [numthreads(1, 1, 1)] void main() {{ b.Store(0, b.Load(4) * {i} + {i}); }}"
            );
            let mut blob = None;
            // SAFETY: the source outlives the call; the blob is read within it.
            unsafe {
                D3DCompile(
                    source.as_ptr().cast(),
                    source.len(),
                    None,
                    None,
                    None,
                    s!("main"),
                    s!("cs_5_1"),
                    0,
                    0,
                    &mut blob,
                    None,
                )
                .ok()?;
                let blob = blob?;
                Some(
                    std::slice::from_raw_parts(
                        blob.GetBufferPointer().cast::<u8>(),
                        blob.GetBufferSize(),
                    )
                    .to_vec(),
                )
            }
        };
        let Some(codes) = (0..2 * N).map(shader).collect::<Option<Vec<_>>>() else {
            return; // no shader compiler here
        };
        // SAFETY: the code holds its root signature.
        let root: ID3D12RootSignature =
            unsafe { device.CreateRootSignature(0, &codes[0]) }.unwrap();
        struct Shared(ID3D12Device1, ID3D12RootSignature, ID3D12PipelineLibrary1);
        // SAFETY: D3D12 objects are free-threaded.
        unsafe impl Sync for Shared {}
        impl Shared {
            fn desc(&self, code: &[u8]) -> D3D12_COMPUTE_PIPELINE_STATE_DESC {
                D3D12_COMPUTE_PIPELINE_STATE_DESC {
                    // SAFETY: a borrowed pointer the description never releases.
                    pRootSignature: unsafe { std::mem::transmute_copy(&self.1) },
                    CS: D3D12_SHADER_BYTECODE {
                        pShaderBytecode: code.as_ptr().cast(),
                        BytecodeLength: code.len(),
                    },
                    ..Default::default()
                }
            }
            fn store(&self, i: usize, code: &[u8]) {
                let name = HSTRING::from(format!("p{i}"));
                // SAFETY: the description points at live data.
                unsafe {
                    let pso: ID3D12PipelineState =
                        self.0.CreateComputePipelineState(&self.desc(code)).unwrap();
                    self.2.StorePipeline(&name, &pso).unwrap();
                }
            }
            fn load(&self, i: usize, code: &[u8]) -> windows::core::Result<ID3D12PipelineState> {
                let name = HSTRING::from(format!("p{i}"));
                // SAFETY: as above.
                unsafe { self.2.LoadComputePipeline(&name, &self.desc(code)) }
            }
            fn serialized(&self) -> Vec<u8> {
                // SAFETY: a buffer of the size asked for; nothing is stored
                // meanwhile.
                unsafe {
                    let mut blob = vec![0; self.2.GetSerializedSize()];
                    self.2.Serialize(&mut blob).unwrap();
                    blob
                }
            }
        }
        let first = open(&device, None).expect("an empty library");
        let made = Shared(device.clone(), root.clone(), first.library.clone());
        (0..N).for_each(|i| made.store(i, &codes[i]));
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(LIBRARY_FILE);
        std::fs::write(&path, made.serialized()).unwrap();
        let again = open(&device, MappedBlob::open(&path).unwrap()).expect("the library again");
        let shared = Shared(device, root, again.library.clone());
        let stores = Mutex::new(());
        std::thread::scope(|s| {
            for t in 0..8 {
                let (shared, codes, stores) = (&shared, &codes, &stores);
                s.spawn(move || {
                    for i in (t..N).step_by(8) {
                        shared.load(i, &codes[i]).expect("a stored pipeline");
                        assert!(
                            shared.load(i, &codes[(i + 1) % N]).is_err(),
                            "another description"
                        );
                        assert!(shared.load(N + i, &codes[N + i]).is_err(), "not stored yet");
                        let _one = stores.lock().unwrap();
                        shared.store(N + i, &codes[N + i]);
                        shared.serialized();
                    }
                });
            }
        });
        for (i, code) in codes.iter().enumerate() {
            shared.load(i, code).expect("every pipeline");
        }
    }

    #[test]
    fn a_rejected_blob_gives_an_empty_library() {
        let Some(device) = crate::host::host().and_then(|h| h.device.cast::<ID3D12Device1>().ok()) else {
            return; // no host GPU here
        };
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(LIBRARY_FILE);
        std::fs::write(&path, b"not a pipeline library".repeat(16)).unwrap();
        let library = open(&device, MappedBlob::open(&path).unwrap()).expect("an empty library");
        // SAFETY: a plain query on a live library.
        assert!(unsafe { library.library.GetSerializedSize() } > 0);
        assert!(library._blob.is_none());
    }

    #[test]
    fn concurrent_write_atomically_never_fails_or_truncates() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("host-pipelines.bin");
        let threads: Vec<_> = (0..4)
            .map(|t| {
                let target = target.clone();
                std::thread::spawn(move || {
                    let payload = vec![b'a' + t as u8; 4096];
                    for _ in 0..30 {
                        write_atomically(&target, &payload).unwrap();
                    }
                })
            })
            .collect();
        for t in threads {
            t.join().unwrap();
        }
        let final_bytes = std::fs::read(&target).unwrap();
        assert_eq!(final_bytes.len(), 4096);
        assert!(final_bytes.iter().all(|&b| b == final_bytes[0]));
    }
}
