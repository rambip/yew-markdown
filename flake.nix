{
    description = "yew library to convert markdown to html";
    inputs.wasm-tooling.url = github:rambip/wasm-tooling;

    outputs = {self, nixpkgs, wasm-tooling}: {
        defaultPackage.x86_64-linux =
             let pkgs = import nixpkgs {system = "x86_64-linux";};
                 tooling = pkgs.callPackage wasm-tooling {};
                 build = example_name : tooling.rust.buildWithTrunk {
                     # TODO: specify the right cargo.toml and cargo.lock
                     src=./.;
                     fixRelativeUrl = true;
                     relativeHtmlTarget = "examples/${example_name}/index.html";
                 };
                 generate_copy_command = name : ''cp -r ${build name} $out/${name}'';
                 copy_commands = builtins.map generate_copy_command (
                    builtins.attrNames (builtins.readDir ./examples)
                 );
             in
             pkgs.stdenv.mkDerivation {
                 name = "markdown examples";
                  src = "/dev/null";
                  phases = [ "installPhase" ];
                  installPhase = ''
                    mkdir $out
                    ${builtins.concatStringsSep "\n" copy_commands}
                  '';
             };
        devShell.x86_64-linux = 
            let pkgs = import nixpkgs {system = "x86_64-linux";};
                tooling = pkgs.callPackage wasm-tooling {};
                in 
                    tooling.rust.devShell {src = ./.;};
    };
}
