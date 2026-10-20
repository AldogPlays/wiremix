{
  rustPlatform,
  lib,
  pkg-config,
  pipewire,
}:
let
  fs = lib.fileset;
  cargoPackage = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = cargoPackage.name;
  version = cargoPackage.version;

  src = fs.toSource {
    root = ./.;
    fileset = fs.unions [
      (fs.fileFilter (file: builtins.any file.hasExt [ "rs" ]) ./src)
      ./wiremix.desktop
      ./wiremix.toml
      ./Cargo.lock
      ./Cargo.toml
      ./LICENSE-MIT
      ./LICENSE-APACHE
    ];
  };

  nativeBuildInputs = [
    pkg-config
    rustPlatform.bindgenHook
  ];
  buildInputs = [ pipewire ];

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    description = "PipeWire TUI mixer with a single-page interface matching wlctl and Bluetui";
    homepage = "https://github.com/AldogPlays/wiremix";
    mainProgram = "wiremix";
    license = with lib.licenses; [
      mit
      asl20
    ];
    platforms = lib.platforms.linux;
  };

  # Vendor default configuration for reference or for wrapping
  # without having to commit the file to a git repository.
  postInstall = ''
    mkdir -p $out/share
    install -Dm644 ${./wiremix.toml} $out/share/wiremix.toml
    install -Dm644 ${./wiremix.desktop} $out/share/applications/wiremix.desktop
    install -Dm644 LICENSE-MIT $out/share/licenses/wiremix/LICENSE-MIT
    install -Dm644 LICENSE-APACHE $out/share/licenses/wiremix/LICENSE-APACHE
  '';
}
