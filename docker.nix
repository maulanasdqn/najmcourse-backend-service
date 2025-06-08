{pkgs, ...}: let
  baseImage = pkgs.ociTools.pullImage {
    imageName = "ubuntu";
    tag = "latest";
  };
in
  pkgs.dockerTools.buildImage {
    name = "najm-cms-api";

    fromImage = baseImage;

    copyToRoot = pkgs.buildEnv {
      name = "najm-cms-api";
      paths = [
        (pkgs.stdenv.mkDerivation {
          name = "najm-cms-api";
          src = ./src;

          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.openssl
            pkgs.pkg-config
          ];

          buildPhase = ''
            cargo build --release
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp target/release/najm-course-api $out/bin/
          '';
        })
      ];
    };

    config = {
      Cmd = ["/bin/najm-cms-api"];
      WorkingDir = "/bin";
    };
  }
