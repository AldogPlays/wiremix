# Agent guide

This is a visual adaptation of Wiremix to match wlctl and Bluetui.
Read README.md and docs/ui-design.md before changing behavior.

- Preserve audio behavior, existing shortcuts, mouse interaction and configuration.
- Keep changes in rendering/styling unless a concrete defect requires otherwise.
- Preserve upstream history, attribution and licenses.
- Do not add GitHub Actions, CI, Dependabot or release automation.
- Keep builds reproducible with Nix and the committed lockfiles.
- Validate locally with nix build, rustfmt, and relevant rendering tests.
- Do not integrate into or activate the NixOS desktop until requested.

## Agent skills

### Issue tracker

Track issues and specs in GitHub Issues for `AldogPlays/wiremix`. Before tracker operations, read `docs/agents/issue-tracker.md`.

### Triage labels

Use the five default triage roles. Before classifying or labelling issues, read `docs/agents/triage-labels.md`.

### Domain docs

Use a single-context glossary and ADR layout. Before exploring the codebase, read `docs/agents/domain.md`.
