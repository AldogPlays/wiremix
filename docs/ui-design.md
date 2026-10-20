# UI direction

Preserve Wiremix 0.10.0's PipeWire backend, keyboard and mouse actions, tab
order, mute/default-device operations, routing, profiles and peak meters.
Keep terminal colors inherited so the existing Alacritty palette applies.

References inspected: wlctl v0.1.10 (d080331), Bluetui v0.8.1 (c959c09).
Both use bordered sections with green focus indication and emphasized selection.
Use their visual language without copying their application-specific navigation.

The first pass adds a section frame around the current list, a compact selection
marker, highlighted selected cards, and consistent green menu frames. Existing
header/footer rows accommodate the frame; only horizontal list space shrinks.
Tabs remain at the bottom with their original click/keyboard actions.
Custom theme and character-set configuration remains supported.

Before desktop integration, review the visual result beside both references and
exercise playback/recording/output/input/configuration, volume, mute, routing,
default devices, profiles, dropdown scrolling, help, terminal resizing and mouse
hit targets against the real PipeWire session. Local automated checks cover
rendering geometry and existing regressions; they cannot establish live audio UX.

Nixpkgs is pinned to the same revision as the NixOS project. The upstream Rust
lockfile is retained. Run checks locally; no hosted CI or automation is wanted.
