# 🦞 及时claw助手

> USB 即插即用的 OpenClaw 便携版客户端

## 特性

- 🖥️ **Tauri v2** 原生桌面应用，轻量快速
- 💾 **USB 2.0 U盘** 即插即用，免安装免配置
- 🔧 **路径全自适应**，插入任意电脑自动识别盘符
- ⚡ **Vanilla JS + Vite** 前端，Rust 后端
- 📦 一键打包为 Setup.exe 安装包

## 快速开始

### 方式一：U盘即插即用
1. 将整个 `JiShiClaw` 文件夹复制到 U 盘根目录
2. 双击 `一键启动开发.bat`
3. 首次运行会自动安装依赖（需联网）
4. 等待窗口弹出即可使用

### 方式二：打包分发
1. 双击 `一键打包发布.bat`
2. 等待编译完成（首次约 5-10 分钟）
3. 在 `src-tauri/target/release/bundle/nsis/` 找到安装包
4. 分发 `及时claw助手_Setup.exe`

## 项目结构

```
JiShiClaw/
├── index.html              # 前端主页面
├── src/
│   └── main.js             # JS 入口
├── src-tauri/
│   ├── src/                # Rust 后端
│   │   ├── main.rs
│   │   └── lib.rs          # Tauri 命令
│   ├── Cargo.toml          # Rust 依赖
│   ├── tauri.conf.json     # Tauri 配置（窗口大小等）
│   ├── capabilities/       # 权限配置
│   └── icons/              # 应用图标
├── runtime/                # 内置便携运行时
│   ├── nodejs/             # Node.js 便携版
│   └── openclaw/           # OpenClaw 便携版
├── autorun.inf             # U盘自启动配置
├── claw.ico                # U盘图标
├── 一键启动开发.bat         # 开发模式启动
├── 一键打包发布.bat         # 构建安装包
├── package.json
├── vite.config.js
└── README.md
```

## 配置说明

### 窗口设置 (`src-tauri/tauri.conf.json`)
- 默认大小: **900×650**
- 最小尺寸: **600×500**
- 居中启动 ✅
- 可自由拉伸 ✅

### 自定义修改
所有配置已写死开箱即用。如需调整：
- 窗口大小 → 编辑 `tauri.conf.json` 的 `app.windows`
- 标题 → 编辑 `tauri.conf.json` 的 `productName`
- 图标 → 替换 `src-tauri/icons/` 下的 PNG/ICO 文件

## 技术栈

| 技术 | 版本 | 用這 |
|------|------|------|
| Tauri | v2.x | 桌面应用框架 |
| Vite | 6.x | 前端构建 |
| Rust | stable | 后端逻辑 |
| Vanilla JS | - | 前端交互 |

## 许可证

MIT License © 2026 及时claw助手
