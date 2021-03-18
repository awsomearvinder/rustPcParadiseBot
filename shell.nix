{pkgs? import <nixpkgs> {}}:
  pkgs.mkShell {
    name = "valheim_manager";
    nativeBuildInputs = [
      pkgs.pkg-config
    ];
    buildInputs = [
      pkgs.openssl
    ];
    src = ./.;
  }
