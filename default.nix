with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "lighthouse-v${version}";
  version = "0.1.2";
  src = ./.;
  buildInputs = [ openssl ];

  checkPhase = "";
  cargoSha256 = "sha256:1bk7kr6i5xh7b45caf93i096cbblajrli0nixx9m76m3ya7vnbp5";

  meta = with stdenv.lib; {
    description = "eth2 client in Rust";
    homepage = https://github.com/sigp/lighthouse;
    license = licenses.isc;
#    maintainers = [ maintainers.tailhook ];
    platforms = platforms.all;
  };
}
