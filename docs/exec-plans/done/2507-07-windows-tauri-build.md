# 2507-07 在 windows 上构建 tauri 版 Chnots 应用

- windows 使用 gnu 打包时报错
  在 `build` -> `try_build` 第一行下添加 `println!("cargo::rustc-link-arg=-Wl,--exclude-libs=ALL");`
  https://github.com/tauri-apps/tauri/issues/4794#issuecomment-2405076850
