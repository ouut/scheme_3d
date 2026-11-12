# scheme_3d

**`scheme_3d`** 将 Rust 的 [Steel Scheme](https://github.com/mattwparas/steel) 虚拟机绑定到 [Macroquad](https://github.com/not-fl3/macroquad) 2D / 3D 渲染引擎，让你可以使用纯 Scheme 脚本实时编写、热重载交互式 2D 与 3D 图形程序、游戏原型与可编程可视化场景。

---

## 核心特性

- **宿主驱动异步主循环（方案 A）**：Rust 宿主管理窗口、OpenGL 上下文与 `next_frame().await` 帧循环，Scheme 脚本通过 `main.scm` 中的 `(init)`、`(on-reload)`、`(update dt)` 与 `(draw)` 回调驱动逻辑与渲染。
- **完备的第一阶段图形与交互绑定**：
  - **数学与向量**：`MqVec2`、`MqVec3` 向量运算、线性插值、角度/弧度转换，同时兼容原生对象与 Scheme 列表 `'(x y z)`。
  - **颜色与调色板**：`rgba`、`rgb`、`rgba8`、`rgb8`、`color-from-hex`、`color-with-alpha` 以及 25 种 Macroquad 标准颜色常量。
  - **2D 绘图**：线段、矩形、圆形、三角形、正多边形、椭圆、圆弧（实心与线框）。
  - **3D 几何体与场景**：立方体、球体（支持自定义细分）、圆柱/圆台、平面、3D 线段、地面参考网格。
  - **2D / 3D 相机系统**：支持透视/正交 3D 相机、2D 相机坐标转换，以及异常安全的 `(with-camera-3d cam ...)` / `(with-camera-2d cam ...)` 作用域宏。
  - **键鼠输入与文本 HUD**：122 个键盘按键符号与鼠标拖拽/滚轮/位置查询，配合屏幕空间文本渲染与尺寸测量。
- **多文件模块化 (`require` / `provide`)**：自动递归扫描脚本目录下的所有 `.scm` 文件，支持任意拆分子模块。
- **双缓冲热重载 + 跨重载状态持久化 (`PersistStore`)**：
  - 修改保存任意 `.scm` 文件后约 `80ms` 自动热重载；
  - 若脚本存在语法或运行时错误，**仅打印一次日志并在窗口顶部显示红色错误横幅**，保留上一个健康的 VM 实例继续运行，绝不刷屏或闪退；
  - 使用 `(state-get ...)` / `(state-set! ...)` 可在热重载之间无损保留相机角度、实体位置等状态。

---

## 快速开始

### 1. 系统依赖

编译 Macroquad 桌面端需要系统图形与窗口头文件：

- **Ubuntu / Debian Linux**:
  ```bash
  sudo apt-get update
  sudo apt-get install -y pkg-config libx11-dev libxi-dev libgl1-mesa-dev
  ```
- **macOS (Apple Silicon / Intel)**: 无需额外依赖（使用系统内置 Cocoa / Metal / OpenGL 框架）。
- **Windows (MSVC)**: 无需额外依赖。

### 2. 编译与运行示例

```bash
# 克隆仓库并构建
cargo build --release

# 运行默认 3D 交互场景示例 (examples/basic_3d)
cargo run --release -- examples/basic_3d

# 运行 2D 动画与热重载示例 (examples/hello_2d)
cargo run --release -- examples/hello_2d
```

你也可以直接从 [GitHub Releases](../../releases) 下载预编译好的 **Linux (`x86_64`)**、**Windows (`x86_64`)** 或 **macOS Apple Silicon (`aarch64` M 芯片)** 二进制压缩包，解压后直接执行：

```bash
./scheme_3d examples/basic_3d
```

---

## 命令行参数

```text
Usage: scheme_3d [OPTIONS] [SCRIPT_DIR]

Arguments:
  [SCRIPT_DIR]  Path to the directory containing `main.scm` [default: examples/basic_3d]

Options:
  -t, --title <TITLE>    Window title [default: "Scheme 3D"]
  -W, --width <WIDTH>    Initial window width in pixels [default: 1280]
  -H, --height <HEIGHT>  Initial window height in pixels [default: 720]
      --msaa <MSAA>      MSAA sample count (1, 2, 4, 8) [default: 4]
      --no-watch         Disable automatic `.scm` file watching and hot-reload
      --fullscreen       Start in fullscreen mode
      --high-dpi         Enable high-DPI display support
  -h, --help             Print help
  -V, --version          Print version
```

---

## 示例项目说明

### 1. `examples/basic_3d`（多文件 3D 轨道相机与几何场景）
展示多模块拆分（`main.scm` + `camera.scm` + `scene.scm`）、3D 轨道相机交互、几何体组合渲染与跨热重载状态保持：
- **鼠标左键拖拽 / `WASD` / 方向键**：旋转 3D 轨道相机视角
- **鼠标滚轮 / `Q` / `E`**：缩放相机距离
- **`Space`**：暂停 / 继续场景动画
- **`G`**：显示 / 隐藏地面参考网格
- **`R`**：重置相机与动画状态
- **实时热重载体验**：在程序运行时打开 `examples/basic_3d/scene.scm`，修改立方体颜色或旋转速度并保存，画面将在瞬间无损更新且保持当前相机视角！

### 2. `examples/hello_2d`（2D 几何图形与键鼠交互）
展示 2D 基础几何图形、鼠标跟随十字准星、键盘方向键控制与 2D HUD 文本测量。

---

## 文档导航

- 📘 **[完整 API 参考手册 (`docs/API_REFERENCE.md`)](docs/API_REFERENCE.md)**：涵盖所有生命周期函数、模块系统、`PersistStore`、向量数学、颜色、2D/3D 绘图、相机、输入与窗口 API 的详细签名与示例。
- 🏗️ **[多 Agent 协作架构设计文档 (`docs/MULTI_AGENT_PLAN.md`)](docs/MULTI_AGENT_PLAN.md)**：项目模块边界、类型契约与分阶段并行实现说明。

---

## 开发与测试

```bash
# 运行全部单元测试与无头 VM 集成测试
cargo test

# 运行 Clippy 静态检查
cargo clippy --all-targets -- -D warnings
```
