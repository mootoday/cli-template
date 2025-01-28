{
  description = "Knope CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        version = "0.18.3"; # Replace with the latest version

        # Map Nix system to Knope's naming convention
        platformMap = {
          x86_64-linux = "unknown-linux-musl";
          x86_64-darwin = "x86-64-apple-darwin";
          aarch64-darwin = "aarch64-apple-darwin";
        };

        platform = platformMap.${system} or (throw "Unsupported system: ${system}");

      in
      {
        packages = rec {
          knope-cli = pkgs.stdenv.mkDerivation {
            pname = "knope";
            inherit version;

            src = pkgs.fetchurl {
              url = "https://github.com/knope-dev/knope/releases/download/knope%2Fv${version}/knope-${platform}.tgz";
              sha256 = null;
            };

            dontUnpack = true;

            installPhase = ''
              mkdir -p $out/bin
              tar -xzf $src -C $out/bin
              chmod +x $out/bin/knope-${platform}/knope
              mv $out/bin/knope-${platform}/knope $out/bin/knope
              rmdir $out/bin/knope-${platform}
            '';

            meta = with pkgs.lib; {
              description = "Knope CLI";
              homepage = "https://knope.tech/";
              license = licenses.mit;
              platforms = builtins.attrNames platformMap;
            };
          };
          default = knope-cli;
        };

        apps = rec {
          knope-cli = flake-utils.lib.mkApp {
            drv = self.packages.${system}.knope-cli;
          };
          default = knope-cli;
        };
      }
    );
}