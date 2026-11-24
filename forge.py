"""VIctory Forge: upstream main plus every open pull request that merges and builds.

Builds one branch anyone can test, and a status page saying, for each open
pull request, whether it is in that branch, which file it conflicts on (and
with which merged pull requests), or what it breaks.

Usage (from anywhere; needs git >= 2.38, gh, cargo):
    python -I forge.py --repo <VIctory worktree> --out <status folder>
        [--upstream Marvisak/VIctory] [--remote origin] [--branch forge/main]
        [--batch 20] [--held held.json] [--limit N]

The worktree is reset to the forge branch: use a worktree of its own.
Nothing from any game is read or written; only the repository's code.
"""
import argparse, datetime, json, os, subprocess, sys, time

ap = argparse.ArgumentParser()
ap.add_argument("--repo", required=True)
ap.add_argument("--out", required=True)
ap.add_argument("--upstream", default="Marvisak/VIctory")
ap.add_argument("--remote", default="origin")
ap.add_argument("--branch", default="forge/main")
ap.add_argument("--batch", type=int, default=20)
ap.add_argument("--held", default=os.path.join(os.path.dirname(os.path.abspath(__file__)), "held.json"))
ap.add_argument("--limit", type=int, default=0, help="only the first N pull requests (testing)")
a = ap.parse_args()
os.makedirs(a.out, exist_ok=True)
LOG = open(os.path.join(a.out, "forge.log"), "a", encoding="utf-8")


def log(*m):
    line = time.strftime("%H:%M:%S ") + " ".join(str(x) for x in m)
    print(line, flush=True)
    LOG.write(line + "\n")
    LOG.flush()


def git(*args, check=False):
    r = subprocess.run(["git", *args], cwd=a.repo, capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if check and r.returncode:
        raise RuntimeError(f"git {' '.join(args)}: {r.stderr.strip()}")
    return r


def cargo_check():
    r = subprocess.run(["cargo", "check", "--workspace", "--all-targets", "--message-format=short"],
                       cwd=a.repo, capture_output=True, text=True, encoding="utf-8",
                       errors="replace", timeout=3600)
    errs = [l.strip() for l in r.stderr.splitlines() if ": error" in l or l.startswith("error")]
    return r.returncode == 0, errs[:3]


held = {}
if os.path.exists(a.held):
    held = {int(k): v for k, v in json.load(open(a.held, encoding="utf-8")).items()}

# Open pull requests.
r = subprocess.run(["gh", "pr", "list", "-R", a.upstream, "--state", "open", "--limit", "3000",
                    "--json", "number,title,author,isDraft,headRefOid,updatedAt,url"],
                   capture_output=True, text=True, encoding="utf-8", errors="replace")
if r.returncode:
    sys.exit(r.stderr)
prs = sorted(json.loads(r.stdout), key=lambda p: p["number"])
if a.limit:
    prs = prs[: a.limit]
log(f"{len(prs)} open pull requests")

git("fetch", "-q", a.remote, "main", check=True)
base = git("rev-parse", f"{a.remote}/main", check=True).stdout.strip()
log("base", base)
for i in range(0, len(prs), 100):
    specs = [f"+refs/pull/{p['number']}/head:refs/forge/pr/{p['number']}" for p in prs[i:i + 100]]
    f = git("fetch", "-q", a.remote, *specs)
    if f.returncode:
        log("fetch error", f.stderr.strip()[-300:])

COAUTHOR = os.environ.get("FORGE_TRAILER", "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>")
status = {}  # number -> dict(result, detail)
files_of = {}  # number -> files the pull request changes against the base


def ref(n):
    return f"refs/forge/pr/{n}"


def changed(n):
    if n not in files_of:
        files_of[n] = set(git("diff", "--name-only", f"{base}...{ref(n)}").stdout.split())
    return files_of[n]


todo = []
for p in prs:
    n = p["number"]
    if git("rev-parse", "-q", "--verify", ref(n)).returncode:
        status[n] = {"result": "not fetched"}
    elif p["isDraft"]:
        status[n] = {"result": "draft"}
    elif n in held:
        status[n] = {"result": "held", "detail": held[n]}
    else:
        mt = git("merge-tree", "--write-tree", "--name-only", "--no-messages", base, ref(n))
        if mt.returncode == 1:
            status[n] = {"result": "conflicts with main", "files": mt.stdout.split()[1:6]}
        elif mt.returncode:
            status[n] = {"result": "error", "detail": mt.stderr.strip()[-200:]}
        else:
            todo.append(p)
log(f"{len(todo)} merge cleanly with main")

git("checkout", "-q", "-B", a.branch, base, check=True)
good = base
merged = []  # numbers in the branch, in order
pending = []


def merge(n, title):
    r = git("merge", "--no-ff", "-q", "-m", f"Forge: #{n} {title}", "-m", COAUTHOR, ref(n))
    if r.returncode == 0:
        return None
    files = git("diff", "--name-only", "--diff-filter=U").stdout.split()
    if git("rev-parse", "-q", "--verify", "MERGE_HEAD").returncode == 0:
        git("merge", "--abort")
    else:
        git("reset", "-q", "--hard", "HEAD")
    return files


def settle():
    global good, pending
    if not pending:
        return
    ok, errs = cargo_check()
    if ok:
        merged.extend(n for n, _ in pending)
        for n, _ in pending:
            status[n] = {"result": "in forge"}
        good = git("rev-parse", "HEAD", check=True).stdout.strip()
        log(f"batch of {len(pending)} builds; {len(merged)} in forge")
        pending = []
        return
    log(f"batch of {len(pending)} breaks the build; splitting it")
    batch, pending = pending, []
    git("reset", "-q", "--hard", good, check=True)
    bisect(batch, errs)


def bisect(batch, errs):
    """Adds the pull requests of `batch` that build, halving a failing part
    until the one breaking it is alone (`errs`: the whole batch's errors)."""
    global good
    if len(batch) == 1:
        n, t = batch[0]
        status[n] = {"result": "breaks the build", "detail": errs}
        log(f"#{n} breaks the build: {errs[:1]}")
        return
    half = len(batch) // 2
    for part in (batch[:half], batch[half:]):
        kept = []
        for n, t in part:
            if merge(n, t) is None:
                kept.append((n, t))
            else:
                status[n] = {"result": "conflicts with pull requests", "files": [], "with": []}
        if not kept:
            continue
        ok, part_errs = cargo_check()
        if ok:
            good = git("rev-parse", "HEAD", check=True).stdout.strip()
            merged.extend(n for n, _ in kept)
            for n, _ in kept:
                status[n] = {"result": "in forge"}
        else:
            git("reset", "-q", "--hard", good, check=True)
            bisect(kept, part_errs)


for p in todo:
    n, t = p["number"], p["title"]
    files = merge(n, t)
    if files is None:
        pending.append((n, t))
        if len(pending) >= a.batch:
            settle()
        continue
    # Conflicts here but not with main: with what is already in the branch.
    partners = [m for m in merged + [x for x, _ in pending] if changed(m) & set(files)]
    status[n] = {"result": "conflicts with pull requests", "files": files[:5], "with": partners[:8]}
settle()
git("reset", "-q", "--hard", good, check=True)
log(f"done: {len(merged)} pull requests in {a.branch} at {good}")

# Status page.
now = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
by = {}
for p in prs:
    s = status.get(p["number"], {"result": "?"})
    by.setdefault(s["result"], []).append((p, s))
order = ["in forge", "conflicts with pull requests", "conflicts with main", "breaks the build",
         "held", "draft", "not fetched", "error", "?"]
json.dump({"generated": now, "base": base, "head": good, "branch": a.branch,
           "status": {p["number"]: status.get(p["number"]) for p in prs}},
          open(os.path.join(a.out, "status.json"), "w", encoding="utf-8"), indent=1)


def cell(s):
    return str(s).replace("|", "\\|").replace("\n", " ")


md = [f"# VIctory Forge status", "",
      f"Generated {now}. Base: upstream `main` at `{base[:10]}`. Branch `{a.branch}` at `{good[:10]}`: "
      f"**{len(merged)} of {len(prs)} open pull requests** merged and building (`cargo check --workspace --all-targets`).", "",
      "| Result | Pull requests |", "|---|---:|"]
for k in order:
    if k in by:
        md.append(f"| {k} | {len(by[k])} |")
md.append("")
for k in order:
    if k not in by:
        continue
    md += [f"## {k} ({len(by[k])})", "", "| PR | Title | Author | Detail |", "|---|---|---|---|"]
    for p, s in by[k]:
        d = ""
        if s.get("files"):
            d = "files: " + ", ".join(f"`{f}`" for f in s["files"])
        if s.get("with"):
            d += "; with " + ", ".join(f"#{m}" for m in s["with"])
        if s.get("detail"):
            d += cell(s["detail"] if isinstance(s["detail"], str) else " / ".join(s["detail"]))
        md.append(f"| [#{p['number']}]({p['url']}) | {cell(p['title'])} | {p['author']['login']} | {cell(d)} |")
    md.append("")
open(os.path.join(a.out, "STATUS.md"), "w", encoding="utf-8").write("\n".join(md))
log("wrote", os.path.join(a.out, "STATUS.md"))
