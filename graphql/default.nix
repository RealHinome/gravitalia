let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-23.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.rustPlatform.buildRustPackage rec {
    pname = "graphql";
    version = "0.1";
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;
    buildInputs = with pkgs; [
      darwin.apple_sdk.frameworks.Security
      openssl_3
      protobuf
      pkg-config
    ];
}
