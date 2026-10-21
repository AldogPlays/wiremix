# Wiremix — Jeffrey's fork

A personal UI adaptation of [tsowell/wiremix](https://github.com/tsowell/wiremix),
the PipeWire terminal audio mixer. Based on upstream v0.10.0, with all five audio
sections visible together and Nix packaging.

The goal is to match wlctl and Bluetui's section frames, focus treatment and
selection styling while retaining Wiremix's audio behavior and controls.
See [the UI design notes](docs/ui-design.md).

## Build and run

Build locally with `nix build .#wiremix`; `nix flake check` builds the package
and runs its tests. Preview with `nix run . -- --tab output`.
`nix develop` supplies Cargo, rustfmt and the PipeWire development libraries.
Format Nix files with `nix fmt -- flake.nix package.nix`.
No GitHub Actions, CI, release automation or Dependabot is configured.
This fork is packaged for NixOS; the desktop's package selection is still managed
separately in the NixOS configuration repository.

All five audio sections appear together: Playback, Recording, Output Devices,
Input Devices and Configuration. Tab/Shift+Tab (or L/H) moves focus between
sections; clicking or scrolling a section focuses it. The bottom strip shows the
focused section's controls, including your configured shortcuts. Lists scroll
independently, and the focused frame identifies where keyboard commands apply.
`--tab` and existing `SelectTab` bindings select initial focus for compatibility.

## Using the fork on NixOS

The `wiremix` command and desktop entry keep their existing names. The package
adds no background service; it connects to the user's existing PipeWire session.
The x86-64 Nix package is build-tested locally. Other Linux outputs are exposed
but have not been validated on physical hardware.

For a reproducible system, add the fork as an input in your system flake:

```nix
inputs.wiremix = {
  url = "github:AldogPlays/wiremix/codex/ui-match";
  inputs.nixpkgs.follows = "nixpkgs";
};
```

Commit the resulting `flake.lock`; it pins the exact fork commit, so rebuilds
keep that revision until you explicitly update the input. Following the system's
nixpkgs keeps the fork on the same nixpkgs revision. Validate the result with your
pinned inputs; system-specific overlays are not automatically inherited by the
fork's flake package.

Pass `inputs` through `specialArgs` (or Home Manager's `extraSpecialArgs`), then
replace the stock `pkgs.wiremix` entry with:

```nix
inputs.wiremix.packages.${pkgs.stdenv.hostPlatform.system}.wiremix
```

Use that expression in either `environment.systemPackages` or `home.packages`,
replacing the old mixer package so there is only one `wiremix` on the user path.
The same `wiremix` launch commands, including `alacritty -e wiremix`, then open
the fork. Existing Waybar/keybinding launchers need no changes. Build and activate
the configuration through the system's normal rebuild process.

For a temporary test without changing the system package selection:

```bash
nix run github:AldogPlays/wiremix/codex/ui-match -- --tab output
```

For an explicit source revision, replace `codex/ui-match` with a full commit SHA.
The executable, desktop launcher, example configuration and upstream licenses are
installed in the package output. Existing `~/.config/wiremix/wiremix.toml` remains
user-managed and can override appearance and bindings; preview with
`--config /dev/null` if you want to inspect the fork's defaults in isolation.

## Controls and configuration

Press `?` for the configured keyboard shortcuts. Arrow keys or `hjkl` navigate
and adjust volume; `m` toggles mute, `d` sets the default source or sink, and
`Enter` or `c` opens routing and profile choices.

See [wiremix.toml](wiremix.toml) for settings and keybindings,
[the usage reference](docs/upstream-reference.md) for command-line options and
configuration examples, and [UI design notes](docs/ui-design.md) for the fork's layout and focus behavior.

## Upstream and license

This is an independent fork. Distribution packages and `cargo install wiremix`
install upstream Wiremix; use the Nix instructions above for this version.
Existing upstream tags are retained and do not identify releases of this UI fork.

Wiremix retains its upstream attribution and dual
[MIT](LICENSE-MIT) / [Apache 2.0](LICENSE-APACHE) licensing.
