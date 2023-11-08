{
    description = "a yew markdown component";

    inputs = {
        flake-utils.url = "github:numtide/flake-utils";
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs = {
                flake-utils.follows = "flake-utils";
            };
        };
        crane = {
          url = "github:ipetkov/crane";
          inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = { self, rust-overlay, nixpkgs, flake-utils, crane }: 
        flake-utils.lib.eachDefaultSystem (system:
        let 
            pkgs = import nixpkgs {
                inherit system;
                overlays = [ (import rust-overlay) ];
            };
            inherit (pkgs) lib;

            rustToolchain = pkgs.rust-bin.selectLatestNightlyWith(
                toolchain: toolchain.default.override 
                {
                    # Set the build targets supported by the toolchain,
                    # wasm32-unknown-unknown is required for trunk.
                    targets = [ "wasm32-unknown-unknown" ];
                }
            );
            craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

            # discard the directories inside examples to build the library less often
            libSrc = lib.cleanSourceWith {
                src = ./.; 
                filter = path: type:
                    (builtins.match ".*examples/.*" path == null)
                    && (craneLib.filterCargoSources path type)
                    ;
            };

            fullSrc = lib.cleanSourceWith {
                src = ./.;
                filter = path: type:
                    (lib.hasSuffix "\.html" path) || (lib.hasSuffix "\.css" path)
                    || (craneLib.filterCargoSources path type)
                    ;
            };

            CARGO_BUILD_TARGET = "wasm32-unknown-unknown";

            # Build *just* the cargo dependencies, so we can reuse
            # all of that work (e.g. via cachix) when running in CI
            cargoArtifacts = craneLib.buildDepsOnly {
                inherit CARGO_BUILD_TARGET;
                src = libSrc;
                doCheck = false;
            };

            buildExample = name: craneLib.buildTrunkPackage {
                inherit CARGO_BUILD_TARGET cargoArtifacts;
                src = fullSrc;
                pname = "yew-markdown-${name}";
                trunkIndexPath = "examples/${name}/index.html";
                cargoExtraArgs = "--package=./examples/${name}";
                # RELATIVE URLS are a MESS 
                # https://github.com/thedodd/trunk/pull/470
                trunkExtraBuildArgs = "--public-url=/yew-markdown/${name}";

                nativeBuildInputs = [
                    (pkgs.wasm-bindgen-cli.override {
                        version = "0.2.87";
                        hash = "sha256-0u9bl+FkXEK2b54n7/l9JOCtKo+pb42GF9E1EnAUQa0=";
                        cargoHash = "sha256-AsZBtE2qHJqQtuCt/wCAgOoxYMfvDh8IzBPAOkYSYko=";
                    })
                ];
            };
            example_names = builtins.attrNames(builtins.readDir ./examples);
            attr_examples = builtins.map 
                (name: {inherit name; path=buildExample name; value=buildExample name;}) 
                example_names;

            examples = builtins.listToAttrs attr_examples;
            in
            {
                checks = {};
                packages = examples // {
                    default = pkgs.linkFarm "yew-markdown examples" attr_examples;
                };

                devShells.default = pkgs.mkShell {
                    buildInputs = with pkgs; [
                        rustToolchain
                        binaryen
                        openssl 
                        pkg-config
                        trunk
                        rust-analyzer
                    ];
                };
            }
    );
}

