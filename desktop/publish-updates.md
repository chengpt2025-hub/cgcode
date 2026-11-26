# cgcode 桌面版自动更新发布说明

客户端只会到下面这个地址检查更新(构建时由 `desktop/patch-update.mjs` 写死)：

```
https://chengpt2025-hub.github.io/cgcode/updates/feeds/win-x64/stable.yml
```

## 一次性准备
1. 仓库里已有 `gh-pages` 分支(只放更新清单和说明页)。
2. 仓库 Settings → Pages → Build and deployment：Source 选 “Deploy from a branch”，Branch 选 `gh-pages`、目录 `/ (root)`，保存。

## 每次发布新版本
1. 在 `desktop/` 的版本号基础上构建出新安装包(构建产物 `cgcode-windows-x64` 里有 `cgcode-<版本>-win-x64-unsigned.exe`、`.exe.blockmap`，以及由 `desktop/make-update-feed.mjs` 自动生成的 `stable.yml`)。
2. 新建 GitHub Release(标签如 `desktop-v<版本>`)，上传安装包和 `.blockmap`。
3. 把 `stable.yml` 里的 `url` 和 `path` 改成该 Release 的下载地址(或文件名并保持同目录)，提交到 `gh-pages` 分支的 `updates/feeds/win-x64/stable.yml`。
4. 已安装的客户端会在下次检查时发现新版本并提示更新。

## 注意
- 版本号必须比已发布的版本大，客户端才会更新。
- 安装包未签名时 Windows 可能弹安全提示；购买代码签名证书后可消除。
