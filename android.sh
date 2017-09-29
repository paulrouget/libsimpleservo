#!/bin/sh

export OPENSSL_STATIC='TRUE'
export CMAKE_TOOLCHAIN_FILE='/Users/paul/git/servo/support/android/toolchain.cmake'
export CPPFLAGS='--sysroot /Users/paul/Desktop/android-ndk-r12b/platforms/android-21/arch-arm64'
export CXXFLAGS='--sysroot /Users/paul/Desktop/android-ndk-r12b/platforms/android-21/arch-arm64 -I/Users/paul/Desktop/android-ndk-r12b/sources/android/support/include -I/Users/paul/Desktop/android-ndk-r12b/sources/cxx-stl/llvm-libc++/libcxx/include -I/Users/paul/Desktop/android-ndk-r12b/sources/cxx-stl/llvm-libc++abi/libcxxabi/include'
export ANDROID_NDK='/Users/paul/Desktop/android-ndk-r12b'
export ANDROID_HOME='/Users/paul/Library/Android/sdk'
export PATH='/Users/paul/Desktop/android-ndk-r12b/toolchains/aarch64-linux-android-4.9/prebuilt/darwin-x86_64/bin:/Users/paul/git/servo/.servo/rust/2017-09-17-alt/rustc-nightly-x86_64-apple-darwin/rustc/bin:/Users/paul/git/servo/.servo/rust/2017-09-17-alt/rustc-nightly-x86_64-apple-darwin/bin:/Users/paul/git/servo/.servo/cargo/2017-09-17/cargo/bin:/Users/paul/git/servo/.servo/cargo/2017-09-17/bin:/Users/paul/git/servo/python/_virtualenv/bin:/Users/paul/.cargo/bin:/usr/local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:/opt/X11/bin:/Library/Frameworks/Mono.framework/Versions/Current/Commands:/Users/paul/.cargo/bin'
export ANDROID_PLATFORM='android-21'
export RUSTFLAGS=' -C target-feature=+neon -W unused-extern-crates'
export OPENSSL_VERSION='1.0.2k'
export ANDROID_SDK='/Users/paul/Library/Android/sdk'
export ANDROID_NDK_ROOT='/Users/paul/Desktop/android-ndk-r12b'
export OPENSSL_INCLUDE_DIR='/Users/paul/git/servo/target/aarch64-linux-android/release/native/openssl/openssl-1.0.2k/include'
export NDK_ANDROID_VERSION='21'
export NDK_HOME='/Users/paul/Desktop/android-ndk-r12b'
export CFLAGS='--sysroot /Users/paul/Desktop/android-ndk-r12b/platforms/android-21/arch-arm64 -I/Users/paul/Desktop/android-ndk-r12b/sources/android/support/include'
export CMAKE_ANDROID_ARCH_ABI='arm64-v8a'
export ANDROID_SYSROOT='/Users/paul/Desktop/android-ndk-r12b/platforms/android-21/arch-arm64'
export OPENSSL_LIB_DIR='/Users/paul/git/servo/target/aarch64-linux-android/release/native/openssl/openssl-1.0.2k'
export RUST_TARGET='aarch64-linux-android'

cargo build --release --target aarch64-linux-android

TARGET="./target/aarch64-linux-android/release/libsimpleservo.so"
$ANDROID_NDK_ROOT/toolchains/aarch64-linux-android-4.9/prebuilt/darwin-x86_64/aarch64-linux-android/bin/strip $TARGET
