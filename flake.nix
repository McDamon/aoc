{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      fenix,
      flake-utils,
      treefmt-nix,
      nixpkgs,
    }:
    let
      # Define `pkgs` for each system
      eachSystem = flake-utils.lib.eachDefaultSystem;

      # Eval the treefmt modules from ./treefmt.nix
      treefmtEval =
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
    in
    eachSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        toolchain = fenix.packages.${system}.complete.toolchain;

        # Define the Rust package
        rustPackage =
          (pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          }).buildRustPackage
            {
              inherit (cargoToml.package) name version;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
            };

        # Treefmt configuration
        treefmtConfig = treefmtEval system;
      in
      {
        # Default package
        packages.default = rustPackage;

        # Dev shell with Rust and treefmt
        devShells.default = pkgs.mkShell {
          inputsFrom = [ rustPackage ];

          buildInputs = with pkgs; [
            clippy
            just
            lldb_19
            python312Packages.datetime # for some reason we need this for lldb_19 formatters
            treefmtConfig.config.build.wrapper
          ];
        };

        # Formatter for `nix fmt`
        formatter = treefmtConfig.config.build.wrapper;

        # Checks for `nix flake check`
        checks.formatting = treefmtConfig.config.build.check self;
      }
    );
}
