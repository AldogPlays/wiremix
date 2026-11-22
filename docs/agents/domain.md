# Domain docs

This is a single-context repository.

## Before exploring the codebase

Read the root `GLOSSARY.md` and any ADRs in `docs/adr/` relevant to the area being changed. If either is absent, proceed silently; create domain documentation only when terms or decisions are actually resolved.

## Vocabulary and decisions

Use the glossary's terms in issue titles, proposals, hypotheses and test names. If a concept is missing, reconsider whether the project uses it; record a real terminology gap for domain modeling.

If a proposal contradicts an existing ADR, identify the conflict explicitly and explain why the decision should be reconsidered.

## Layout

- `GLOSSARY.md`: repository-wide domain vocabulary.
- `docs/adr/`: architecture decision records, created as decisions are recorded.
