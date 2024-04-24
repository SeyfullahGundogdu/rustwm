{pkgs ? import <nixpkgs> {}}: let
  overrides = builtins.fromTOML (builtins.readFile ./rust-toolchain.toml);
in
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      clang
      # Replace llvmPackages with llvmPackages_X, where X is the latest LLVM version (at the time of writing, 16)
      llvmPackages.bintools
      rustup
      pkg-config
      pkgs.wayland.dev
      pkgs.libseat.dev
      pkgs.libinput.dev
      pkgs.libxkbcommon.dev
      pkgs.libGL
    ];
    RUSTC_VERSION = overrides.toolchain.channel;
    # https://github.com/rust-lang/rust-bindgen#environment-variables
    LIBCLANG_PATH = pkgs.lib.makeLibraryPath [pkgs.llvmPackages_latest.libclang.lib];
    shellHook = ''
      export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
      export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
    '';
    # Add precompiled library to rustc search path
    RUSTFLAGS = builtins.map (a: ''-L ${a}/lib'') [
      # add libraries here (e.g. pkgs.libvmi)
    ];
    LD_LIBRARY_PATH = "${pkgs.libxkbcommon}/lib/:${pkgs.libinput.dev}/lib:${pkgs.libGL}/lib";
  }
