# Mostly from https://nixos.wiki/wiki/Rust with modifications to run Bevy
{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      llvmPackages_latest.llvm
      llvmPackages_latest.bintools
      zlib.out
      rustup

      pkg-config
      openssl_1_1
      udev
      alsa-lib

      llvmPackages_latest.lld
      python3
      glibc
      pango
      cairo
      atk
      gtk3
      libsoup
      webkitgtk
      xlibsWrapper
      xkeyboard_config

      vulkan-tools vulkan-headers vulkan-loader vulkan-validation-layers
    ];
    RUSTC_VERSION = pkgs.lib.readFile ./rust-toolchain;
    # https://github.com/rust-lang/rust-bindgen#environment-variables
    LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
    HISTFILE = toString ./.history;
    shellHook = ''
      export PATH=$PATH:~/.cargo/bin
      export PATH=$PATH:~/.rustup/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
      export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
        pkgs.lib.makeLibraryPath [
          pkgs.udev
          pkgs.alsaLib
          pkgs.vulkan-loader
        ]
      }"'';
    # Add libvmi precompiled library to rustc search path
    RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
      # pkgs.libvmi
      pkgs.xlibsWrapper
    ]);
    # Add libvmi, glibc, clang, glib headers to bindgen search path
    BINDGEN_EXTRA_CLANG_ARGS = 
    # Includes with normal include path
    (builtins.map (a: ''-I"${a}/include"'') [
      # pkgs.libvmi
      pkgs.glibc.dev 
      pkgs.xlibsWrapper
    ])
    # Includes with special directory paths
    ++ [
      ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
      ''-I"${pkgs.glib.dev}/include/glib-2.0"''
      ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
      ''-I${pkgs.xlibsWrapper.out}/lib/''
    ];

  }
