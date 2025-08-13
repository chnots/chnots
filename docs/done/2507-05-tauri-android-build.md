# 2507-05 使用 tauri 构建应用到 android

利用 andoroid-studio 安装 sdk

- Android SDK Platform
- Android SDK Platform-Tools
- NDK (Side by side)
- Android SDK Build-Tools
- Android SDK Command-line Tools

```shell
$ export ANDROID_HOME="$HOME/Android/Sdk"
$ export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"
$ export JAVA_HOME=/opt/android-studio/jbr
$ rustup target add aarch64-linux-android
```

## 2507-08 安装 apk 时报错：app not installed as package appears to be invalid

https://github.com/tauri-apps/tauri/discussions/9872 提到因为 apk 没有签名。

- key.properties 例子
  ```
  password=<pass>
  keyPassword=<pass>
  storePassword=<pass>
  keyAlias=upload
  storeFile=<path/to/.jks>
  ```
