let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-23.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShell {
	buildInputs = with pkgs; [
		cargo
        libiconv
        openssl_3.dev
        protobuf
        pkg-config
        rustc
	];
}
