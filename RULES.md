# Forge rules

The Forge runs by itself. These rules decide everything it does, so that nobody has to make a call by hand, and
anyone can predict and check the result.

## 1. When it runs

- Every 12 hours (00:00 and 12:00 UTC), and whenever someone with write access starts the workflow by hand.
- Each run starts from scratch: upstream `main` as it is at that moment, and the pull requests as they are.
  Nothing from a previous run is kept except the list of what changed.

## 2. What gets into `forge/main`

A pull request is merged when **all** of these hold:

1. It is open and not a draft.
2. It merges with upstream `main` without conflicts.
3. It merges after the pull requests already taken in this run, without conflicts.
4. With it, `cargo check --workspace --all-targets` passes.
5. With it, the repository's asset policy (`cargo xtask policy`) passes: nothing from the game, no archives,
   no binaries the policy refuses.
6. It is not held (rule 4).

Pull requests are tried **oldest first** (by number). An older one wins a conflict with a newer one. That is a
fixed order, not a judgement of quality.

## 3. What the status page says

Every open pull request gets one result:

| Result | Meaning |
|---|---|
| in forge | merged (rule 2) |
| conflicts with main | fails rule 2.2: rebase onto `main` |
| conflicts with pull requests | fails rule 2.3: the pull requests named under *with* changed the same files first |
| breaks the build | fails rule 2.4 or 2.5: the first error is shown |
| held | rule 4 |
| draft | skipped until ready |

"Since the last run" lists which pull requests joined the branch and which left it.

## 4. Holds

A hold keeps out a pull request that builds but is known to break the title. Holds are in
[held.json](held.json) and follow these rules:

- Each hold gives a **reason**: what broke, on what hardware, and when.
- Each hold names the **commit** it was observed on. **When the author pushes a new commit, the hold lifts by
  itself**, and the pull request is tried again.
- A hold is never used for style, opinion, or a competing approach. Only for observed breakage.
- Problems the checks already catch (build, asset policy) need no hold: rule 2 keeps those out on its own,
  and they come back in as soon as they are fixed.

## 5. Tests

After merging, the Forge runs `cargo test --workspace --no-fail-fast` on the branch and lists failing tests on the
status page. A failing test does not keep a pull request out. GPU tests need a GPU, and the automatic runs have
none, so most GPU tests skip there.

## 6. What the Forge never does

- It never pushes to, comments on, labels, approves, or closes anything in the upstream repository.
  It only reads upstream.
- It never edits a pull request's code. Each one is merged as its author wrote it, one merge commit each.
- It never adds anything from the game, or anything the asset policy refuses.
- It never changes these rules by itself. Changes to the rules or holds are commits to this branch, with a reason.

## 7. Changing the rules

Open an issue or a pull request against the `forge-status` branch of this repository. Anyone can propose a change.
The change takes effect at the next run.
