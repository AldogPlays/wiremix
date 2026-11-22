# Issue tracker: GitHub

Issues and specs for this fork live in `AldogPlays/wiremix` on GitHub. Use the `gh` CLI with explicit `--repo AldogPlays/wiremix` for issue operations; the upstream remote is a separate project.

## Operations

- Publish to the tracker: `gh issue create --repo AldogPlays/wiremix --title "..." --body-file <path>`. Write multiline bodies to a file and pass it with `--body-file`.
- Fetch a ticket: `gh issue view <number> --repo AldogPlays/wiremix --json number,title,body,labels,comments`.
- List tickets: `gh issue list --repo AldogPlays/wiremix --state open --json number,title,body,labels`, with appropriate filters.
- Comment: `gh issue comment <number> --repo AldogPlays/wiremix --body-file <path>`.
- Apply or remove labels: `gh issue edit <number> --repo AldogPlays/wiremix --add-label "..."` or `--remove-label "..."`. Read `triage-labels.md` for role names.
- Close: post the resolution as a comment, then `gh issue close <number> --repo AldogPlays/wiremix`. Post comments separately even when the ticket is already closed.

Before the first issue creation in a session, verify the target with `gh repo view AldogPlays/wiremix --json nameWithOwner`.

## Related work

Use native GitHub sub-issues and blocking dependencies when available. Otherwise, put `Part of #<parent>` and `Blocked by: #<number>` lines at the top of child issues, and list children in the parent's task list. A ticket is unblocked when all its blockers are closed.

For wayfinding, use one map issue containing Notes, Decisions-so-far and Fog, with a child issue for each research, prototype, grilling or implementation task. Pick the first open, unblocked, unassigned child in map order. Claim it by assigning yourself before working; resolve it with an answer comment and closure, then append a gist and link to the map's Decisions-so-far.

## Pull requests as a triage surface

**PRs as a request surface: no.**
