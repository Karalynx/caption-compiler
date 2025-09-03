{}:

let
  sources = import ./npins;
  rustOverlay = sources.rustOverlay;
  pkgs = import sources.nixpkgs {
    overlays = [(import rustOverlay)];
  };
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      (rust-bin.stable.latest.default.override {
        extensions = ["rust-src"];
      })
      rust-analyzer
    ];
    RUST_BACKTRACE = 1;
  }
