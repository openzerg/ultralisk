{ pkgs, lib, config, inputs, ... }:

let
  ultralisk-app = pkgs.rustPlatform.buildRustPackage {
    pname = "ultralisk";
    version = "0.1.0";
    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;
    nativeBuildInputs = [ pkgs.pkg-config pkgs.protobuf ];
    buildInputs = [ pkgs.openssl pkgs.sqlite ];
  };
in
{
  languages.rust = {
    enable = true;
    channel = "nightly";
  };

  packages = with pkgs; [
    buf
    protobuf
    sqlite
    pkg-config
    openssl
    bubblewrap
    cacert
    inputs.nix2container.packages.${pkgs.system}.skopeo-nix2container
  ];

  processes = {
    ultralisk.exec = "./target/release/ultralisk serve --host 0.0.0.0 --port 15317";
  };

  containers.ultralisk = {
    name = "ultralisk";
    copyToRoot = [ 
      ultralisk-app 
      pkgs.cacert
      pkgs.sqlite
    ];
    startupCommand = "${ultralisk-app}/bin/ultralisk serve --host 0.0.0.0 --port 15317";
  };

  tasks = {
    "ci:build" = {
      exec = "cargo build --release";
    };
    "ci:test" = {
      exec = "cargo test";
    };
    "ci:check" = {
      exec = "cargo check";
    };
    "ci:proto" = {
      exec = "cd proto && buf generate";
    };
    "container:copy-to-podman" = {
      exec = ''
        IMAGE=$(devenv container build ultralisk 2>&1 | tail -1)
        skopeo copy nix:$IMAGE containers-storage:ultralisk:latest
        echo "Container copied to podman: ultralisk:latest"
      '';
    };
  };

  enterShell = ''
    echo "Ultralisk Development Environment"
    echo "Commands: cargo run -- serve | cargo test | cargo build --release"
    echo ""
    echo "Container commands:"
    echo "  devenv container build ultralisk                    - Build container"
    echo "  skopeo copy nix:<image.json> containers-storage:ultralisk:latest"
  '';
}