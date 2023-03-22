with import <nixpkgs> {};
  mkShell {
    packages = [
      cargo
      rustc
      rustfmt
      rust-analyzer-unwrapped
    ];

    RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  }
