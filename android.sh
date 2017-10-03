#!/bin/sh

rustup target add aarch64-linux-android

export ANDROID_SDK="/Users/paul/Library/Android/sdk/"
export ANDROID_NDK="/Users/paul/Desktop/android-ndk-r12b/"

export PATH="$ANDROID_NDK/toolchains/aarch64-linux-android-4.9/prebuilt/darwin-x86_64/bin:$PATH"

export ANDROID_HOME="$ANDROID_SDK"
export ANDROID_NDK_ROOT="$ANDROID_NDK"
export NDK_ANDROID_VERSION="21"
export NDK_HOME="$ANDROID_NDK"
export ANDROID_SYSROOT="$ANDROID_NDK/platforms/android-21/arch-arm64"
export ANDROID_PLATFORM="android-21"

export CMAKE_TOOLCHAIN_FILE="/Users/paul/git/servo/support/android/toolchain.cmake"
export CMAKE_ANDROID_ARCH_ABI="arm64-v8a"

export CPPFLAGS="--sysroot $ANDROID_NDK/platforms/android-21/arch-arm64"
export CXXFLAGS="--sysroot $ANDROID_NDK/platforms/android-21/arch-arm64 -I$ANDROID_NDK/sources/android/support/include -I$ANDROID_NDK/sources/cxx-stl/llvm-libc++/libcxx/include -I$ANDROID_NDK/sources/cxx-stl/llvm-libc++abi/libcxxabi/include"
export CFLAGS="--sysroot $ANDROID_NDK/platforms/android-21/arch-arm64 -I$ANDROID_NDK/sources/android/support/include"

export RUSTFLAGS=" -C target-feature=+neon -W unused-extern-crates"
export RUST_TARGET="aarch64-linux-android"

export OPENSSL_VERSION="1.0.2k"
export OPENSSL_STATIC="TRUE"
export OPENSSL_INCLUDE_DIR="/Users/paul/git/servo/target/aarch64-linux-android/release/native/openssl/openssl-1.0.2k/include"
export OPENSSL_LIB_DIR="/Users/paul/git/servo/target/aarch64-linux-android/release/native/openssl/openssl-1.0.2k"


cargo build --release --target aarch64-linux-android

TARGET="./target/aarch64-linux-android/release/libsimpleservo.so"
$ANDROID_NDK_ROOT/toolchains/aarch64-linux-android-4.9/prebuilt/darwin-x86_64/aarch64-linux-android/bin/strip $TARGET
