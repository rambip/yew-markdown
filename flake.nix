{
    description = "yew library to convert markdown to html";
    inputs = {
        wasm-tooling.url = github:rambip/wasm-tooling;
        flake-utils.url = github:numtide/flake-utils;
        rust-overlay.url = github:oxalica/rust-overlay;
    };

    outputs = {self, nixpkgs, flake-utils, rust-overlay, wasm-tooling}: 
        with flake-utils.lib ;
        eachSystem ["x86_64-linux" "x86_64-darwin"] (system:
            let overlays = [rust-overlay.overlays.default];
                pkgs = import nixpkgs {inherit system overlays;};
                rust-tooling = pkgs.callPackage wasm-tooling.lib."${system}".rust {
                    cargo-toml = ./Cargo.toml;
                    rust-toolchain = pkgs.rust-bin.stable."1.69.0".minimal; 
                };
                build = example_name: rust-tooling.buildWithTrunk {
                 src=./.;
                 fixRelativeUrl = true;
                 relativeHtmlTarget = "examples/${example_name}/index.html";
            };
                examples = builtins.readDir ./examples;
                built_examples = builtins.mapAttrs (name: value: build name) examples;
                generate_copy_command = name : ''cp -r ${build name} $out/${name}'';
                copy_commands = builtins.map generate_copy_command (
                   builtins.attrNames (builtins.readDir ./examples)
                );
            in
            {
                packages = built_examples // {
                    default = 
                        nixpkgs.legacyPackages."${system}".stdenv.mkDerivation {
                         name = "markdown examples";
                         src = "/dev/null";
                         phases = [ "installPhase" ];
                         installPhase = ''
                         mkdir $out
                         ${builtins.concatStringsSep "\n" copy_commands}
                         '';
                     };
                };
                devShells.default = rust-tooling.devShell;
            }
        );
}
