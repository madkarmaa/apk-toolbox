# APK toolbox

## Usage

Use either the `help <cmd>` command or the `--help` (`-h`) flag. The program will also guide you with errors.

## Requirements

1. Java JDK [> 11](https://apktool.org/docs/build#requirements)
2. keytool (usually installed along Java)
3. [Apktool](https://github.com/iBotPeaches/Apktool/releases/latest)
4. [APKEditor](https://github.com/REAndroid/APKEditor/releases/latest)
5. zipalign (part of the Android SDK Build Tools)
6. apksigner (part of the Android SDK Build Tools)

### Android SDK Build Tools without Android Studio

1. Download the Command Line Tools [here](https://developer.android.com/studio#command-line-tools-only)
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
5. The Android SDK Build Tools will be downloaded in `./build-tools/<VERSION>`