{
  description = "Leptos Motion - A comprehensive animation library for Rust and Leptos";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
        
        # Node.js and pnpm
        nodejs = pkgs.nodejs_20;
        pnpm = pkgs.nodePackages.pnpm;
        
        # Development tools
        devTools = with pkgs; [
          # Build tools
          gnumake
          gcc
          pkg-config
          
          # Rust tools
          rustToolchain
          cargo-watch
          cargo-edit
          cargo-audit
          cargo-tarpaulin
          
          # Web development
          trunk
          wasm-pack
          
          # Testing and quality
          nodejs
          pnpm
          
          # System tools
          git
          jq
          ripgrep
          fd
          bat
          
          # Development utilities
          direnv
          nix-direnv
        ];
        
        # Environment variables
        envVars = {
          RUST_BACKTRACE = "1";
          RUST_LOG = "info";
          CARGO_INCREMENTAL = "1";
          RUSTFLAGS = "-C target-cpu=native";
        };
        
      in {
        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = devTools;
          
          shellHook = ''
            export RUST_BACKTRACE="1"
            export RUST_LOG="info"
            export CARGO_INCREMENTAL="1"
            export RUSTFLAGS="-C target-cpu=native"
            
            echo "🚀 Welcome to Leptos Motion development environment!"
            echo ""
            echo "Available tools:"
            echo "  • Rust: $(rustc --version)"
            echo "  • Cargo: $(cargo --version)"
            echo "  • Node.js: $(node --version)"
            echo "  • pnpm: $(pnpm --version)"
            echo "  • Trunk: $(trunk --version)"
            echo "  • Make: $(make --version | head -n1)"
            echo ""
            echo "Quick start:"
            echo "  • pnpm install          # Install Node.js dependencies"
            echo "  • make test             # Run all tests"
            echo "  • make dev              # Start development server"
            echo "  • make build            # Build all crates"
            echo "  • make clean            # Clean build artifacts"
            echo ""
            echo "Happy coding! 🦀"
          '';
        };
        
        # Build outputs
        packages = {
          # Build all Rust crates
          default = pkgs.stdenv.mkDerivation {
            name = "leptos-motion";
            src = ./.;
            
            buildInputs = with pkgs; [ rustToolchain trunk ];
            
            buildPhase = ''
              cargo build --release
              cd examples/showcase && trunk build
            '';
            
            installPhase = ''
              mkdir -p $out/bin
              cp -r target/release/* $out/bin/ || true
              cp -r examples/showcase/dist $out/
            '';
            
            meta = {
              description = "A comprehensive animation library for Rust and Leptos";
              homepage = "https://github.com/cloud-shuttle/leptos-motion";
              license = pkgs.lib.licenses.mit;
              platforms = pkgs.lib.platforms.all;
            };
          };
          
          # Development package
          dev = pkgs.stdenv.mkDerivation {
            name = "leptos-motion-dev";
            src = ./.;
            
            buildInputs = devTools;
            
            buildPhase = ''
              echo "Development package - no build required"
            '';
            
            installPhase = ''
              mkdir -p $out
              echo "Development environment ready" > $out/README
            '';
          };
        };
        
        # Apps
        apps = {
          # Development server
          dev = {
            type = "app";
            program = toString (pkgs.writeShellScript "dev" ''
              cd examples/showcase
              trunk serve --open
            '');
          };
          
          # Test runner
          test = {
            type = "app";
            program = toString (pkgs.writeShellScript "test" ''
              cargo test
              pnpm test:e2e
            '');
          };
          
          # Build
          build = {
            type = "app";
            program = toString (pkgs.writeShellScript "build" ''
              cargo build --release
              cd examples/showcase && trunk build
            '');
          };
        };
        
        # Checks
        checks = {
          # Rust tests
          rust-tests = pkgs.stdenv.mkDerivation {
            name = "rust-tests";
            src = ./.;
            buildInputs = [ rustToolchain ];
            buildPhase = "cargo test";
            installPhase = "mkdir -p $out";
          };
          
          # Format check
          rust-fmt = pkgs.stdenv.mkDerivation {
            name = "rust-fmt";
            src = ./.;
            buildInputs = [ rustToolchain ];
            buildPhase = "cargo fmt --check";
            installPhase = "mkdir -p $out";
          };
          
          # Clippy check
          rust-clippy = pkgs.stdenv.mkDerivation {
            name = "rust-clippy";
            src = ./.;
            buildInputs = [ rustToolchain ];
            buildPhase = "cargo clippy -- -D warnings";
            installPhase = "mkdir -p $out";
          };
        };
      }
    );
}
