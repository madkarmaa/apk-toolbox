# APK toolbox

Compile and decompile Android applications with ease.

## About

**APK toolbox** is a CLI wrapper around essential Android reverse-engineering and build tools (like Apktool). It streamlines the compilation and decompilation process so you don't have to remember long, complicated commands to do the job yourself. 

It also provides full support for handling split APK formats, such as `.xapk` and `.apks`.

## Downloads

[![Windows x64](https://custom-icon-badges.demolab.com/badge/Windows-x64-357EC7?style=for-the-badge&logo=windows11&logoColor=white)](https://github.com/madkarmaa/apk-toolbox/releases/latest/download/apk-toolbox-windows-x64.exe) [![Android arm64-v8a](https://img.shields.io/badge/Android-arm64%20v8a-34A853?style=for-the-badge&logo=android&logoColor=white)](https://github.com/madkarmaa/apk-toolbox/releases/latest/download/apk-toolbox-android-arm64) [![Linux x64 (GCC)](https://img.shields.io/badge/Linux-x64%20GCC-F4BC00?style=for-the-badge&logo=linux&logoColor=white)](https://github.com/madkarmaa/apk-toolbox/releases/latest/download/apk-toolbox-linux-x64) [![Linux x64 (static)](https://img.shields.io/badge/Linux-x64%20static-F4BC00?style=for-the-badge&logo=linux&logoColor=white)](https://github.com/madkarmaa/apk-toolbox/releases/latest/download/apk-toolbox-linux-x64-musl) [![MacOS (Apple silicon)](https://img.shields.io/badge/MacOS-Apple%20silicon-000000?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/madkarmaa/apk-toolbox/releases/latest/download/apk-toolbox-macos-arm64)

## Requirements

1. Java JDK [> 11](https://apktool.org/docs/build#requirements)
2. [Apktool](https://github.com/iBotPeaches/Apktool/releases/latest)
3. [APKEditor](https://github.com/REAndroid/APKEditor/releases/latest)
4. Android SDK Build Tools - [no Android Studio?](#android-sdk-build-tools-without-android-studio)

### Android SDK Build Tools without Android Studio

1. Download the [Android SDK Command Line Tools](https://developer.android.com/studio#command-line-tools-only)
2. Unzip the downloaded file
3. Run

```shell
./cmdline-tools/bin/sdkmanager --sdk_root="." $(./cmdline-tools/bin/sdkmanager --sdk_root="." --list | grep "build-tools" | sort -t';' -k2 -Vr | head -1 | awk '{print $1}')
```

PowerShell:
```
./cmdline-tools/bin/sdkmanager --sdk_root="." (./cmdline-tools/bin/sdkmanager --sdk_root="." --list | Select-String "build-tools" | Sort-Object { [version](($_ -replace '.*build-tools;(\S+).*','$1') -replace '-rc\d+','') } -Descending | Select-Object -First 1 | ForEach-Object { ($_ -split '\|')[0].Trim() })
```

4. Accept the license agreements
5. The Android SDK Build Tools will be downloaded in `build-tools/<VERSION>`

## Usage

Use either the `help <cmd>` command or the `--help` (`-h`) flag. The program will also guide you with errors.

### Known issues

- On Windows CMD, configuring a directory ending with a `\` while wrapping it with double quotes `"` (such as `"C:\Some\Example\Path\"`) will result in a `Path not found` error.

## Build from source

Make sure you have the [Rust toolchain](https://rustup.rs) installed.
```shell
git clone https://github.com/madkarmaa/apk-toolbox
cd apk-toolbox
cargo build --release --locked
```

> [!NOTE]
> To build for Android (`aarch64-linux-android`), you must have [Docker](https://www.docker.com) running. Install and use [`cross`](https://github.com/cross-rs/cross) instead of `cargo`
> ```shell
> cargo install cross --git https://github.com/cross-rs/cross
> cross build --release --target aarch64-linux-android --locked
> ```

The compiled binary will be available in the `target/release/` directory (or `target/{target}/release` if cross-compiling).

## Contributing

Contributions are welcome! Feel free to [open an issue](https://github.com/madkarmaa/apk-toolbox/issues/new) or submit a pull request if you'd like to improve the tool or fix any bugs.

## License

This project is licensed under the [MIT License](./LICENSE).