with import <nixpkgs> {};
mkShell {
  packages = [
    cargo
    rustc
    rust-analyzer
  ];
}
