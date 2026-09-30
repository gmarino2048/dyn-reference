{
  description = "dyn-reference dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        # Single source of truth: channel/components/targets come from
        # rust-toolchain.toml so rustup and Nix users match.
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [ toolchain ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];

          shellHook = ''
            # Keep the shell's PATH ahead of rustup's ~/.cargo/bin shims so the
            # Nix toolchain wins even if the user's rc prepends ~/.cargo/bin.
            export PATH="${toolchain}/bin:$PATH"
          '';
        };
      });
}
