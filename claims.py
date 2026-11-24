"""Claims: who works on what, read from the coordination issue.

Reads the comments of the coordination issue named in coordination.json (in the
upstream repository; read only) and the open work in work.json, and writes
CLAIMS.md (and claims.json): every task, who holds it, its pull request, and
the comments the rules do not accept.

Commands, one per line, anywhere in a comment (case does not matter):
    claiming: ID[, ID...]      claim tasks; renews your own claim
    claiming: <one line>       claim something not listed yet (gets a NEW-n ID)
    done: ID -> #123           your pull request for the task is open
    release: ID                give the task back
    finding: <one line>        something that needs doing (gets a FIND-n ID)

Rules (RULES.md, "Claims"): first valid claim wins; a claim without a pull
request expires after `expire_days`; an edited comment does not count (post a
new one); only the holder can mark done or release.

Usage: python -I claims.py [--dir <folder with coordination.json/work.json>]
Needs gh (GH_TOKEN for API limits). Standard library only.
"""
import argparse, datetime as dt, json, os, re, subprocess, sys

ap = argparse.ArgumentParser()
ap.add_argument("--dir", default=os.path.dirname(os.path.abspath(__file__)))
a = ap.parse_args()
cfg = json.load(open(os.path.join(a.dir, "coordination.json"), encoding="utf-8"))
work = json.load(open(os.path.join(a.dir, "work.json"), encoding="utf-8"))
REPO, ISSUE, EXPIRE = cfg["repo"], cfg["issue"], int(cfg.get("expire_days", 14))
now = dt.datetime.now(dt.timezone.utc)


def gh(*args):
    r = subprocess.run(["gh", *args], capture_output=True, text=True, encoding="utf-8", errors="replace")
    if r.returncode:
        sys.exit(f"gh {' '.join(args)}: {r.stderr.strip()}")
    return r.stdout


def when(s):
    return dt.datetime.fromisoformat(s.replace("Z", "+00:00"))


comments = [json.loads(line) for line in gh(
    "api", "--paginate", f"repos/{REPO}/issues/{ISSUE}/comments?per_page=100", "--jq", ".[]").splitlines() if line.strip()]
comments.sort(key=lambda c: c["id"])

prs = {p["number"]: p for p in json.loads(gh(
    "pr", "list", "-R", REPO, "--state", "all", "--limit", "3000",
    "--json", "number,title,body,author,state,url,mergedAt"))}

tasks = {t["id"].upper(): dict(t, id=t["id"].upper(), source="work.json") for t in work}
# Tasks also come from a table in the issue body: | `ID` | what | where | needs | ...
body = json.loads(gh("api", f"repos/{REPO}/issues/{ISSUE}"))["body"] or ""
for line in body.splitlines():
    cols = [c.strip() for c in line.strip().strip("|").split("|")]
    if len(cols) >= 2 and re.fullmatch(r"`[A-Za-z][A-Za-z0-9-]*-[A-Za-z0-9-]+`", cols[0]):
        tid = cols[0].strip("`").upper()
        tasks.setdefault(tid, {"id": tid, "what": cols[1], "where": cols[2] if len(cols) > 2 else "",
                               "needs": cols[3] if len(cols) > 3 else "", "source": "issue"})
claims = {}     # id -> holder dict(user, at, url)
waiting = {}    # id -> [user]
done = {}       # id -> pr number
problems = []
new_n = find_n = 0
ID = re.compile(r"^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+$", re.I)
CMD = re.compile(r"^\s*[-*]?\s*(claiming|done|release|finding)\s*:\s*(.+?)\s*$", re.I)

for c in comments:
    user = c["user"]["login"]
    at = when(c["created_at"])
    edited = when(c["updated_at"]) - at > dt.timedelta(seconds=60)
    link = c["html_url"]
    for line in c["body"].splitlines():
        m = CMD.match(line)
        if not m:
            continue
        cmd, arg = m.group(1).lower(), m.group(2)
        if edited and cmd in ("claiming", "done", "release"):
            problems.append(f"[comment]({link}) by @{user}: edited after posting, so its `{cmd}` does not count; post a new comment.")
            continue
        if cmd == "finding":
            find_n += 1
            tid = f"FIND-{find_n}"
            tasks[tid] = {"id": tid, "what": arg, "where": "", "needs": "", "source": f"[finding]({link}) by @{user}"}
            continue
        # Each comma-separated part starts with its ID; words after it explain.
        parts = [x.strip() for x in arg.split("->")[0].split(",") if x.strip()]
        ids = [x.split()[0].strip("`").upper() for x in parts]
        if cmd == "claiming":
            listed = [i for i in ids if i in tasks]
            if not listed and not any(ID.match(i) for i in ids):
                new_n += 1
                tid = f"NEW-{new_n}"
                tasks[tid] = {"id": tid, "what": arg, "where": "", "needs": "", "source": f"[claim]({link}) by @{user}"}
                listed = [tid]
            for i in [i for i in ids if ID.match(i) and i not in tasks]:
                problems.append(f"[comment]({link}) by @{user}: `{i}` is not a task ID; use `claiming: <one line>` for something new.")
            for tid in listed:
                h = claims.get(tid)
                if h and h["user"] != user and tid not in done:
                    if user not in [w["user"] for w in waiting.get(tid, [])]:
                        waiting.setdefault(tid, []).append({"user": user, "at": at, "url": link})
                elif tid not in done:
                    claims[tid] = {"user": user, "at": at, "url": link}
        elif cmd == "done":
            pm = re.search(r"#(\d+)", arg)
            for tid in ids[:1]:
                h = claims.get(tid)
                if not h or h["user"] != user:
                    problems.append(f"[comment]({link}) by @{user}: `done: {tid}` without holding the claim.")
                elif not pm:
                    problems.append(f"[comment]({link}) by @{user}: `done: {tid}` names no pull request (`-> #123`).")
                else:
                    done[tid] = int(pm.group(1))
        elif cmd == "release":
            for tid in ids:
                # Out of the queue too, when waiting behind another holder.
                if tid in waiting:
                    waiting[tid] = [w for w in waiting[tid] if w["user"] != user]
                h = claims.get(tid)
                if h and h["user"] == user:
                    del claims[tid]
                    done.pop(tid, None)
                    nxt = waiting.get(tid, [])
                    if nxt:
                        claims[tid] = dict(nxt.pop(0), at=at)

# Pull requests that name a task ID in their title or body count as `done`
# for the holder's own pull request.
for tid, h in claims.items():
    if tid in done:
        continue
    for n, p in prs.items():
        if p["author"]["login"] == h["user"] and p["state"] == "OPEN" and tid in (p["title"] + " " + (p["body"] or "")).upper():
            done[tid] = n
            break


# An expired claim passes to the next in line, from the day it expired.
for tid in list(claims):
    while tid not in done and (now - claims[tid]["at"]).days >= EXPIRE and waiting.get(tid):
        nxt = waiting[tid].pop(0)
        claims[tid] = dict(nxt, at=max(nxt["at"], claims[tid]["at"] + dt.timedelta(days=EXPIRE)))


def state(tid):
    h = claims.get(tid)
    if tid in done:
        p = prs.get(done[tid])
        if p and p["state"] == "MERGED":
            return "merged", f"#{done[tid]} merged"
        if p and p["state"] == "CLOSED":
            return "open", f"#{done[tid]} closed; free again"
        return "in review", f"[#{done[tid]}]({p['url'] if p else ''}) by @{h['user'] if h else '?'}"
    if h:
        age = (now - h["at"]).days
        if age >= EXPIRE:
            return "open", f"claim by @{h['user']} expired ({age} days, no pull request)"
        return "claimed", f"@{h['user']} since {h['at']:%Y-%m-%d} ([claim]({h['url']}))" + (
            f"; waiting: {', '.join('@' + w['user'] for w in waiting.get(tid, []))}" if waiting.get(tid) else "")
    return "open", ""


def cell(s):
    return str(s).replace("|", "\\|").replace("\n", " ")


rows = [(tid, t, *state(tid)) for tid, t in tasks.items()]
order = {"open": 0, "claimed": 1, "in review": 2, "merged": 3}
natural = lambda s: [int(x) if x.isdigit() else x for x in re.split(r"(\d+)", s)]
rows.sort(key=lambda r: (order[r[2]], natural(r[0])))
md = ["# Claims", "",
      f"Who works on what in VIctory, from the comments in [{REPO}#{ISSUE}](https://github.com/{REPO}/issues/{ISSUE}). "
      f"Updated {now:%Y-%m-%d %H:%M} UTC every 30 minutes. Rules: [RULES.md](RULES.md#claims).", "",
      f"**Before you write code:** pick an open task below and comment `claiming: <ID>` in the issue "
      f"(or `claiming: <one line>` for something not listed). When your pull request is open, comment "
      f"`done: <ID> -> #<PR>` or put the ID in the pull request. Seen something that needs doing? Comment "
      f"`finding: <one line>`.", "",
      "| ID | Status | What | Where | Needs | Holder / pull request |", "|---|---|---|---|---|---|"]
for tid, t, st, who in rows:
    md.append(f"| `{tid}` | {st} | {cell(t['what'])} | {cell(t.get('where', ''))} | {cell(t.get('needs', ''))} | {cell(who)} |")
if problems:
    md += ["", "## Not accepted", ""] + [f"- {p}" for p in problems]
md.append("")
path = os.path.join(a.dir, "CLAIMS.md")
stamp = re.compile(r"Updated [0-9-]+ [0-9:]+ UTC")
before = open(path, encoding="utf-8").read() if os.path.exists(path) else ""
if stamp.sub("", before) == stamp.sub("", "\n".join(md)):
    print("no change")
    sys.exit(0)
open(path, "w", encoding="utf-8", newline="\n").write("\n".join(md))
json.dump({"updated": f"{now:%Y-%m-%dT%H:%MZ}",
           "tasks": [{"id": tid, "status": st, "holder": claims.get(tid, {}).get("user"), "pr": done.get(tid)}
                     for tid, t, st, who in rows]},
          open(os.path.join(a.dir, "claims.json"), "w", encoding="utf-8", newline="\n"), indent=1)
print(f"{len(rows)} tasks, {len(claims)} claims, {len(problems)} not accepted")
