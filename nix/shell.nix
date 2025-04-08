{pkgs}:
pkgs.mkShell {
  packages = with pkgs; [
    (fenix.complete.withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
      "rust-analyzer"
    ])
    lldb

    alejandra
    nil
  ];
}
