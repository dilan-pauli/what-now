{ pkgs, lib, config, inputs, ... }:

{
  # Latest stable Rust toolchain for development
  languages.rust = {
    enable = true;
    channel = "stable";
    rustflags = "";
  };

  # Nice-to-have tooling for Rust CLI development
  # (clippy, rustfmt, and rust-analyzer come bundled with the toolchain)
  packages = with pkgs; [
    cargo-nextest # fast test runner
    cargo-edit    # cargo add/rm/upgrade
  ];

  # Environment variables for Rust projects
  env = {
    RUST_BACKTRACE = 1;
    RUST_LOG = "debug";
  };

  # Verify the toolchain is ready
  enterTest = ''
    rustc --version
    cargo --version
    cargo nextest --version
  '';

  enterShell = ''
    echo "Ready to hack on whatnow (Rust ${config.languages.rust.version})"
  '';
}
