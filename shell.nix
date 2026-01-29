with import <nixpkgs> {}; {
  qpidEnv = stdenv.mkDerivation {
    name = "build-environment-bis";
    buildInputs = [
        rustup
        rust-analyzer
        sdl3
    ];
  };
}
