let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-23.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShell {
	buildInputs = with pkgs; [
		cargo
    darwin.apple_sdk.frameworks.Security
    openssl_3
    protobuf
    pkg-config
    rustc
	];
}
