# UI direction

Match wlctl and Bluetui's one-page layout and bottom shortcut hints while
preserving Wiremix 0.10.0's PipeWire behavior, volume/mute operations, default
devices, routing, profiles and peak meters. Terminal colors remain inherited.
References: wlctl v0.1.10 (d080331), Bluetui v0.8.1 (c959c09).

Playback, Recording, Output Devices, Input Devices and Configuration are stacked
as bordered sections. The focused frame is emphasized; selection highlighting
belongs to that section. Empty sections remain visible. Space follows content,
with a complete control card given priority in populated sections; longer lists
scroll independently. Very small terminals can show partial cards and fewer
shortcut hints; the Help shortcut remains at the start of the footer.

Tab/Shift+Tab and L/H move focus; clicking or scrolling a section focuses it.
Existing volume, mute, default-device, route/profile and help keys remain.
Moving focus closes the previous section's open menu. Mouse actions are scoped
to their section before selecting devices or adjusting sliders.

The footer lists actual configured keys for its contextual controls, wrapping
onto up to three lines. It changes to selection/confirmation/cancel hints while
a menu is open. The full scrollable help remains available. Historical config
names (`tab`, `TabLeft`, `TabRight`, `SelectTab`) are retained for compatibility
and now refer to section focus rather than separate pages.

Rendering, scrolling and lazy peak capture share the same per-section rectangles.
All displayed sections contribute visible objects and their backend dependencies,
including partial cards, so inactive sections still reflect live changes.
Custom theme and character-set configuration remains supported.

Before desktop integration, review the visual result beside both references and
exercise volume/mute, routing, defaults, profiles, scrolling, help, resizing and
mouse targets against the real PipeWire session. Automated checks cover layout,
focus, key remapping, partial-card visibility and existing regressions; they do
not establish every device's audio behavior.

Nixpkgs is pinned to the same revision as the NixOS project. The upstream Rust
lockfile is retained. Run checks locally; no hosted CI or automation is wanted.
