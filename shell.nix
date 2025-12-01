{ pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball {
        url = "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
      }))
    ];
  }
}:

let
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" ];
#    targets    = [ "x86_64-unknown-linux-musl" ];
  };
in

pkgs.mkShell {
  nativeBuildInputs = [
    # full Rust toolchain (rustc, cargo, rustfmt, clippy, MUSL stdlibs, rust-src)
    rustToolchain

    # LSP/editor helpers
    pkgs.rust-analyzer
    pkgs.cargo-watch
    pkgs.cargo-expand

    # static linker + MUSL libc
    pkgs.zig

    # everything else…
    pkgs.nodejs
    pkgs.typescript
    pkgs.git
    pkgs.jq
    pkgs.awscli2
    pkgs.docker
    pkgs.docker-compose
    pkgs.duckdb
    pkgs.vim
    pkgs.htop
    pkgs.bat
    pkgs.tig
    pkgs.pkg-config
    pkgs.openssl
  ];

  shellHook = ''
    # 1) default target → MUSL
#    export CARGO_BUILD_TARGET=x86_64-unknown-linux-musl

    # 2) tell Cargo (and ring’s build script) to use zig as the C compiler
#    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="zig cc"
#    export CC_x86_64_unknown_linux_musl="zig cc"

    echo "Welcome to dsla (musl → lambda) shell – Zig is now your C compiler for ring!"
  '';
}
