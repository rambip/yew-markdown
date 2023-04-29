{
    description = "yew library to convert markdown to html";
    inputs.wasm-tooling.url = github:rambip/wasm-tooling;

    outputs = {self, nixpkgs, wasm-tooling}: {
        devShell.x86_64-linux = 
            let pkgs = import nixpkgs {system = "x86_64-linux";};
                tooling = pkgs.callPackage wasm-tooling {};
                in 
                    tooling.rust.devShell {src = ./.;};
    };
}
