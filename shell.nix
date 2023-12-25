with import <nixpkgs> {};
let src = fetchFromGitHub {
      owner = "mozilla";
      repo = "nixpkgs-mozilla";
      rev = "6eabade97bc28d707a8b9d82ad13ef143836736e";
      sha256 = "1ElPLD8eFfnuIk0G52HGGpRtQZ4QPCjChRlEOfkZ5ro=";
   };
in
with import "${src.out}/rust-overlay.nix" pkgs pkgs;
stdenv.mkDerivation {
  name = "rust-env";
  buildInputs = [
    # Note: to use use stable, just replace `nightly` with `stable`
    latest.rustChannels.stable.rust
    # Add some extra dependencies from `pkgs`
    pkg-config openssl
    pkgs.bacon
    pkgs.cmake
    pkgs.iconv
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
