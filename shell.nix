with import <nixpkgs>
{
  overlays = [
    (import (builtins.fetchTarball {
      url = "https://github.com/oxalica/rust-overlay/archive/refs/heads/stable.zip";
    }))
  ];
};
let
  rust-dev = (rust-bin.fromRustupToolchainFile (if builtins.pathExists ./rust-toolchain then ./rust-toolchain else ./rust-toolchain.toml));
  bon-derive-crate = (builtins.fromTOML (builtins.readFile ./bon-macros/Cargo.toml)).package;
  rust-minimal = rust-bin.stable."${bon-derive-crate.rust-version}".default;
  format =
    writeShellApplication {
      name = "format";
      runtimeInputs = [ taplo rust-dev ];
      text = ''
        cargo fmt
        taplo fmt
      '';
    };

  ci = writeShellApplication {
    name = "ci";
    runtimeInputs = [ rust-minimal ];
    text = ''
      cargo check
    '';
  };

in
mkShell
{
  nativeBuildInputs = [
    rust-dev
    format
    taplo
    ci
  ];
}
