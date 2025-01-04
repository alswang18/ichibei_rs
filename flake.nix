{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Basic build tools
            gcc
            pkg-config
            lld

            # Rust tools
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
            clang
            lld
            rustPlatform.rustLibSrc

            # OpenSSL with development files
            openssl

            # I like zsh so I use it. This and the hook below are optional.
            zsh
          ];

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";

          # For pkg-config to find openssl
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.openssl
          ];

          shellHook = ''
            # This variable is solely used for checking if the shell is running.
            export NIX_SHELL_NAME="rust-dev"
            echo "Starting rust-dev shell"
            export SHELL=${pkgs.zsh}/bin/zsh
            exec $SHELL
          '';
        };
      }
    );

  nixConfig = {
    # Allow dirty git tree during development
    allow-dirty = true;
  };
}
