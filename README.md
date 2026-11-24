# VIctory Forge

VIctory has hundreds of open pull requests, and most of them can only be tried one at a time against `main`.
Many touch the same files, some fix the same problem twice, and nobody can test them together.

The Forge does that for everyone:

- **`forge/main`**: upstream `main` plus every open pull request that merges cleanly and still builds, one merge
  commit per pull request (`Forge: #N title`), oldest first. Check it out and test the combined state.
- **[STATUS.md](STATUS.md)**: what happened to each open pull request.
- **[forge.py](forge.py)**: the tool that builds both. Run it yourself; it needs only git, `gh` and cargo.

The Forge is not a fork of the project and does not replace review. Pull requests still go to
[Marvisak/VIctory](https://github.com/Marvisak/VIctory); the Forge only shows how they combine.

## What the status means

| Result | Meaning | What the author can do |
|---|---|---|
| in forge | Merged into `forge/main`, and the workspace builds with it. | Nothing. |
| conflicts with main | Does not merge with current `main` on its own. | Rebase onto `main`. |
| conflicts with pull requests | Merges with `main`, but not after the pull requests listed in *with*. These changed the same files first. | Look at those pull requests: often one of them already does the same, or the two need to agree. |
| breaks the build | Merges, but `cargo check --workspace --all-targets` or the repository's asset policy (`cargo xtask policy`) fails with it (first error shown). Often the pull request is older than an API change on `main`. | Rebase and build; remove files the policy refuses. |
| held | Left out on purpose: known to break the title in testing, or adding files the asset policy refuses (reason given in [held.json](held.json)). | See the reason. |
| draft | Skipped until it is ready for review. | |

Pull requests are merged in number order, so an older pull request wins a conflict. Being listed under "conflicts
with pull requests" is not a judgement; it shows who else is working in the same place.

## How it is built

1. List the open pull requests and fetch every head.
2. Check each one against `main` with `git merge-tree` (no checkout).
3. Merge the ones that merge cleanly into a fresh branch from `main`, 25 at a time, and run
   `cargo check --workspace --all-targets` after each batch. If a batch fails, its pull requests are redone one
   at a time and the ones that break the build are left out.
4. Write `STATUS.md` and `status.json`.

Only `cargo check` gates a merge: the GPU tests need a GPU, and running them per pull request would take days.
The test results of the final branch are reported at the top of `STATUS.md` when they were run.

## Run it yourself

```
git worktree add --detach ../VIctory-forge origin/main
python -I forge.py --repo ../VIctory-forge --out <folder outside the repo>
```

Use a worktree of its own: the tool resets it. With `--limit 25` it tries only the first 25 pull requests.

## Game files

None. The Forge reads and writes only the repository's code and the pull requests' code. Nothing from any game
goes into the branch, the status page or the tool. Measurements made on a title (for example with
`victory translate` on your own copy) belong in the pull requests themselves, as counts only.

*Made with AI assistance (Claude Code), run by KevinPratap.*
