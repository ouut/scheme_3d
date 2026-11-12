# `scheme_3d` API 参考手册 (API Reference)

`scheme_3d` 将 Rust 的 [Steel Scheme](https://github.com/mattwparas/steel) 虚拟机绑定到 [Macroquad](https://github.com/not-fl3/macroquad) 2D/3D 图形、相机、键鼠输入、文本 HUD 与窗口接口，并提供毫秒级递归文件监听热重载与跨重载状态持久化机制。

---

## 目录

1. [生命周期与入口约定 (Lifecycle & Entry Contract)](#1-生命周期与入口约定)
2. [模块系统与多文件组织 (Module System)](#2-模块系统与多文件组织)
3. [命令行参数 (CLI Options)](#3-命令行参数)
4. [跨热重载状态持久化 (`PersistStore` API)](#4-跨热重载状态持久化-persiststore-api)
5. [数学与向量 API (Math & Vectors)](#5-数学与向量-api-math--vectors)
6. [颜色 API 与内置调色板 (Color API)](#6-颜色-api-与内置调色板-color-api)
7. [2D 基础几何图形 API (2D Shapes)](#7-2d-基础几何图形-api-2d-shapes)
8. [2D / 3D 相机与矩阵栈 API (Camera 2D & 3D)](#8-2d--3d-相机与矩阵栈-api-camera-2d--3d)
9. [3D 基础几何体与网格 API (3D Models)](#9-3d-基础几何体与网格-api-3d-models)
10. [键盘与鼠标输入 API (Keyboard & Mouse Input)](#10-键盘与鼠标输入-api-keyboard--mouse-input)
11. [文本与 HUD 渲染 API (Text & HUD)](#11-文本与-hud-渲染-api-text--hud)
12. [窗口、帧率与时间 API (Window & Time)](#12-窗口帧率与时间-api-window--time)

---

## 1. 生命周期与入口约定

`scheme_3d` 采用 **方案 A（宿主驱动主循环）**：Rust 宿主负责管理 Macroquad 异步主循环（`next_frame().await`）与文件监听，Scheme 脚本目录中必须包含 **`main.scm`** 作为固定入口文件。

在 `main.scm` 中可以定义以下**可选**顶层生命周期回调函数：

| 函数签名 | 调用时机 | 说明 |
| :--- | :--- | :--- |
| `(init)` | 进程首次成功加载 `main.scm` 后调用**一次** | 用于初始化默认游戏状态、配置相机或打印欢迎信息 |
| `(on-reload)` | 每次热重载重新编译成功后调用**一次** | 用于在修改代码后修正状态或打印重载日志 |
| `(update dt)` | 每帧渲染前调用，`dt` 为上一帧耗时（秒，浮点数） | 用于处理键鼠输入、物理模拟、相机控制与状态更新 |
| `(draw)` | 每帧在 `(update dt)` 之后调用 | 用于执行 `clear-background`、3D 场景绘制与 2D HUD 绘制 |

### 最小示例 (`main.scm`)

```scheme
(require-builtin macroquad)
(require "scheme3d/prelude")

(define (init)
  (state-set! 'angle 0.0))

(define (update dt)
  (state-set! 'angle (+ (state-get 'angle 0.0) dt)))

(define (draw)
  (clear-background BLACK)
  (draw-text "Hello scheme_3d!" 24 40 32 WHITE))
```

### 错误防刷屏与安全热重载机制

- **双缓冲热重载**：当你保存项目目录下的任意 `.scm` 文件时，宿主在约 `80ms` 防抖窗口后在后台构建全新的 Steel `Engine` 实例。
  - 若新代码存在语法错误或顶层异常，宿主**仅在终端打印一次错误并保留旧的正常 `Engine` 继续运行**，窗口右上角会显示红色错误横幅提示，绝不闪退。
- **帧回调防刷屏**：若 `(update dt)` 或 `(draw)` 在运行期抛出异常，同一错误签名只会向终端输出一次日志，并在屏幕顶部 HUD 展示错误详情；一旦保存修复后的 `.scm` 文件，横幅立即消失并恢复正常。

---

## 2. 模块系统与多文件组织

### 2.1 导入内置模块

在任何需要使用 Macroquad 绑定或预置宏的 `.scm` 文件顶部添加：

```scheme
;; 导入所有 Rust 原生 Macroquad 绑定函数与常量
(require-builtin macroquad)

;; 导入 Scheme 标准辅助宏（如 with-camera-3d / with-camera-2d）
(require "scheme3d/prelude")
```

### 2.2 多文件模块化 (`require` 与 `provide`)

项目目录（`<script_dir>`）下的所有 `.scm` 文件会在启动和每次热重载时被递归扫描并注册到内存模块解析器中。你可以使用相对路径导入子模块：

**`camera.scm`**：
```scheme
(require-builtin macroquad)
(provide make-orbit-cam)

(define (make-orbit-cam dist)
  (make-camera-3d (vec3 0.0 dist dist) (vec3 0.0 0.0 0.0) (vec3 0.0 1.0 0.0)))
```

**`main.scm`**：
```scheme
(require-builtin macroquad)
(require "camera.scm") ; 也可省略后缀写为 (require "camera")
```

---

## 3. 命令行参数

```bash
scheme_3d [OPTIONS] [SCRIPT_DIR]
```

| 参数 / 选项 | 默认值 | 说明 |
| :--- | :--- | :--- |
| `[SCRIPT_DIR]` | `examples/basic_3d` | 包含入口文件 `main.scm` 的脚本文件夹路径 |
| `-t, --title <TITLE>` | `"Scheme 3D"` | 窗口标题 |
| `-W, --width <WIDTH>` | `1280` | 初始窗口宽度（像素） |
| `-H, --height <HEIGHT>` | `720` | 初始窗口高度（像素） |
| `--msaa <SAMPLES>` | `4` | MSAA 多重采样抗锯齿采样数 |
| `--no-watch` | `false` | 禁用 `.scm` 文件监听热重载 |
| `--fullscreen` | `false` | 启动时全屏运行 |
| `--high-dpi` | `false` | 启用高分屏 High-DPI 支持 |

---

## 4. 跨热重载状态持久化 (`PersistStore` API)

由于热重载会重建全新的 Steel VM，普通的顶层 `(define x 0)` 会在热重载后重置。为了在热重载后保留玩家位置、相机角度、分数等状态，`scheme_3d` 提供了独立于 VM 堆的 Rust 侧键值存储 `PersistStore`。

- **键名类型 (`key`)**：接受符号（如 `'yaw`）或字符串（如 `"yaw"`）。
- **支持持久化的值类型 (`val`)**：
  - 基础标量：`bool` (`#t`/`#f`)、`int`、`float`、`string`、`symbol`
  - 原生图形类型：`MqVec2`、`MqVec3`、`MqColor`、`MqCamera2D`、`MqCamera3D`
  - 复合容器：任意嵌套的上述类型列表 `list` 或向量 `vector`

### 函数列表

#### `(state-set! key val) -> val`
将 `key` 对应的值存入持久化存储，并返回 `val` 本身。

```scheme
(state-set! 'cam-yaw 0.65)
(state-set! 'player-pos (vec3 1.0 2.0 3.0))
```

#### `(state-get key [default]) -> val`
读取 `key` 对应的值：
- 若传入了第 2 个参数 `default` 且 `key` 不存在，则**自动将 `default` 写入存储**并返回 `default`（非常适合在 `(init)` 或 `(update dt)` 中做惰性初始化）。
- 若未传 `default` 且 `key` 不存在，则返回 `#f`。

```scheme
;; 若 'cam-yaw 尚未设置，则初始化为 0.65 并返回 0.65
(define yaw (state-get 'cam-yaw 0.65))
```

#### `(state-try-get key) -> val | #f`
只读查询 `key` 对应的值；若不存在则返回 `#f`，不会写入默认值。

#### `(state-has? key) -> bool`
检查 `key` 是否已存在于持久化存储中，返回 `#t` 或 `#f`。

#### `(state-remove! key) -> bool`
从持久化存储中删除 `key`，若此前存在则返回 `#t`，否则返回 `#f`。

#### `(state-clear!) -> void`
清空持久化存储中的所有键值对（例如按 `R` 键重置场景时调用）。

---

## 5. 数学与向量 API (Math & Vectors)

所有接受 `Vec2` 或 `Vec3` 的函数同时支持：
1. 原生 `MqVec2` / `MqVec3` 对象（推荐，性能最佳）
2. 长度为 2 或 3 的 Scheme 列表 `'(x y)` / `'(x y z)` 或向量 `#(x y)` / `#(x y z)`

### 5.1 数学常量与标量辅助函数

| 常量 / 函数 | 签名与说明 |
| :--- | :--- |
| `PI` | 圆周率常量 `3.1415927` |
| `TAU` | `2 * PI` 常量 `6.2831855` |
| `(clamp x min max)` | 将数值 `x` 限制在 `[min, max]` 区间内 |
| `(lerp a b t)` | 标量线性插值：`a + (b - a) * t` |
| `(deg->rad deg)` | 角度转弧度 |
| `(rad->deg rad)` | 弧度转角度 |

### 5.2 二维向量 (`MqVec2`)

| 函数 | 签名与说明 |
| :--- | :--- |
| `(vec2 x y)` | 构造 `MqVec2` 对象 |
| `(vec2? v)` | 判断 `v` 是否为原生 `MqVec2` 对象 |
| `(vec2-x v)` / `(vec2-y v)` | 读取 `x` / `y` 分量 |
| `(vec2+ a b)` | 向量加法 `a + b` |
| `(vec2- a b)` | 向量减法 `a - b` |
| `(vec2* v s)` | 向量数乘 `v * s` |
| `(vec2-length v)` | 向量模长 |
| `(vec2-length-squared v)` | 向量模长的平方 |
| `(vec2-normalize v)` | 安全归一化（零向量返回 `(vec2 0.0 0.0)`，不产生 `NaN`） |
| `(vec2-dot a b)` | 向量点积 |
| `(vec2-lerp a b t)` | 二维向量线性插值 |
| `(vec2-distance a b)` | 两点间欧氏距离 |

### 5.3 三维向量 (`MqVec3`)

| 函数 | 签名与说明 |
| :--- | :--- |
| `(vec3 x y z)` | 构造 `MqVec3` 对象 |
| `(vec3? v)` | 判断 `v` 是否为原生 `MqVec3` 对象 |
| `(vec3-x v)` / `(vec3-y v)` / `(vec3-z v)` | 读取 `x` / `y` / `z` 分量 |
| `(vec3+ a b)` | 三维向量加法 `a + b` |
| `(vec3- a b)` | 三维向量减法 `a - b` |
| `(vec3* v s)` | 三维向量数乘 `v * s` |
| `(vec3-length v)` | 三维向量模长 |
| `(vec3-length-squared v)` | 三维向量模长的平方 |
| `(vec3-normalize v)` | 安全归一化（零向量返回 `(vec3 0.0 0.0 0.0)`） |
| `(vec3-dot a b)` | 三维向量点积 |
| `(vec3-cross a b)` | 三维向量叉积 |
| `(vec3-lerp a b t)` | 三维向量线性插值 |
| `(vec3-distance a b)` | 三维空间两点间欧氏距离 |

---

## 6. 颜色 API 与内置调色板 (Color API)

所有接受颜色参数的函数除了支持原生 `MqColor` 对象外，还支持直接传入 3 元素或 4 元素浮点数列表 `'(r g b)` / `'(r g b a)`（分量范围 `0.0 ~ 1.0`）。

### 6.1 颜色构造与转换函数

| 函数 | 签名与说明 |
| :--- | :--- |
| `(rgba r g b a)` | 使用归一化浮点数（`0.0 ~ 1.0`）构造颜色 |
| `(rgb r g b)` | 使用归一化浮点数（`0.0 ~ 1.0`）构造不透明颜色（`a = 1.0`） |
| `(rgba8 r g b a)` | 使用字节整数（`0 ~ 255`）构造颜色 |
| `(rgb8 r g b)` | 使用字节整数（`0 ~ 255`）构造不透明颜色（`a = 255`） |
| `(color-from-hex hex)` | 从 24 位十六进制整数（如 `#xFF8800`）构造不透明颜色 |
| `(color-with-alpha color alpha)` | 返回将 `color` 的透明度替换为 `alpha`（`0.0 ~ 1.0`）后的新颜色 |
| `(color? c)` | 判断 `c` 是否为原生 `MqColor` 对象 |
| `(color-r c)` / `(color-g c)` / `(color-b c)` / `(color-a c)` | 读取颜色的 `r` / `g` / `b` / `a` 浮点分量（`0.0 ~ 1.0`） |

### 6.2 内置 25 色常量

`LIGHTGRAY`、`GRAY`、`DARKGRAY`、`YELLOW`、`GOLD`、`ORANGE`、`PINK`、`RED`、`MAROON`、`GREEN`、`LIME`、`DARKGREEN`、`SKYBLUE`、`BLUE`、`DARKBLUE`、`PURPLE`、`VIOLET`、`DARKPURPLE`、`BEIGE`、`BROWN`、`DARKBROWN`、`WHITE`、`BLACK`、`BLANK`、`MAGENTA`。

---

## 7. 2D 基础几何图形 API (2D Shapes)

| 函数 | 参数与说明 |
| :--- | :--- |
| `(clear-background color)` | 使用指定颜色清空当前帧屏幕背景与深度缓冲 |
| `(draw-line x1 y1 x2 y2 thickness color)` | 绘制从 `(x1, y1)` 到 `(x2, y2)`、线宽为 `thickness` 的线段 |
| `(draw-rectangle x y w h color)` | 绘制左上角位于 `(x, y)`、宽高为 `(w, h)` 的实心矩形 |
| `(draw-rectangle-lines x y w h thickness color)` | 绘制矩形线框 |
| `(draw-circle x y radius color)` | 绘制圆心在 `(x, y)`、半径为 `radius` 的实心圆 |
| `(draw-circle-lines x y radius thickness color)` | 绘制圆形线框 |
| `(draw-triangle v1 v2 v3 color)` | 绘制顶点为 `v1, v2, v3`（`Vec2`）的实心三角形 |
| `(draw-triangle-lines v1 v2 v3 thickness color)` | 绘制三角形线框 |
| `(draw-poly x y sides radius rotation color)` | 绘制正多边形（`sides` 为边数，`rotation` 为角度制旋转角） |
| `(draw-poly-lines x y sides radius rotation thickness color)` | 绘制正多边形线框 |
| `(draw-ellipse x y w h rotation color)` | 绘制实心椭圆 |
| `(draw-ellipse-lines x y w h rotation thickness color)` | 绘制椭圆线框 |
| `(draw-arc x y sides radius rotation thickness arc color)` | 绘制圆弧（`rotation` 与 `arc` 均为角度制） |

---

## 8. 2D / 3D 相机与矩阵栈 API (Camera 2D & 3D)

### 8.1 3D 相机 (`MqCamera3D`)

| 函数 | 参数与说明 |
| :--- | :--- |
| `(camera-3d-default)` | 创建默认 3D 相机 |
| `(make-camera-3d position target [up] [fovy] [projection])` | 创建 3D 相机：<br>• `position` (`Vec3`)：相机位置<br>• `target` (`Vec3`)：观察目标点<br>• `up` (`Vec3`，可选，默认 `(vec3 0 1 0)`)：上方向向量<br>• `fovy` (可选，默认 `45.0` 度或弧度；若 `> PI` 自动按角度转换弧度)<br>• `projection` (可选，`'perspective` 或 `'orthographic`，默认 `'perspective`) |
| `(camera-3d? cam)` | 判断是否为 `MqCamera3D` 对象 |
| `(camera-3d-position cam)` | 获取相机位置 (`MqVec3`) |
| `(camera-3d-target cam)` | 获取相机观察目标点 (`MqVec3`) |
| `(camera-3d-up cam)` | 获取相机上方向 (`MqVec3`) |
| `(camera-3d-fovy cam)` | 获取垂直视场角（弧度） |
| `(camera-3d-projection cam)` | 获取投影模式符号（`'perspective` 或 `'orthographic`） |
| `(camera-3d-near cam)` / `(camera-3d-far cam)` | 获取近/远裁剪面距离 |
| `(camera-3d-set-position! cam pos)` | 原地修改相机位置并返回 `cam` |
| `(camera-3d-set-target! cam target)` | 原地修改相机目标点并返回 `cam` |
| `(camera-3d-set-up! cam up)` | 原地修改相机上方向并返回 `cam` |
| `(camera-3d-set-fovy! cam fovy)` | 原地修改相机 `fovy`（若 `> PI` 自动按角度转弧度）并返回 `cam` |
| `(camera-3d-set-aspect! cam aspect)` | 设置宽高比（传入 `#f` 恢复自动宽高比） |
| `(camera-3d-set-projection! cam proj)` | 设置投影模式（`'perspective` 或 `'orthographic`） |
| `(camera-3d-set-clip! cam near far)` | 设置近裁剪面 `near` 与远裁剪面 `far` |

### 8.2 2D 相机 (`MqCamera2D`)

| 函数 | 参数与说明 |
| :--- | :--- |
| `(camera-2d-default)` | 创建默认 2D 相机 |
| `(make-camera-2d target [zoom] [rotation] [offset])` | 创建 2D 相机（`target`、`zoom`、`offset` 为 `Vec2`，`rotation` 为角度制） |
| `(camera-2d? cam)` | 判断是否为 `MqCamera2D` 对象 |
| `(camera-2d-target cam)` / `(camera-2d-zoom cam)` / `(camera-2d-rotation cam)` / `(camera-2d-offset cam)` | 读取 2D 相机属性 |
| `(camera-2d-set-target! cam target)` / `(camera-2d-set-zoom! cam zoom)` / `(camera-2d-set-rotation! cam rot)` / `(camera-2d-set-offset! cam offset)` | 原地修改 2D 相机属性并返回 `cam` |
| `(camera-2d-world->screen cam pt)` | 将 2D 世界坐标 `pt` (`Vec2`) 转换为屏幕像素坐标 (`MqVec2`) |
| `(camera-2d-screen->world cam pt)` | 将屏幕像素坐标 `pt` (`Vec2`) 转换为 2D 世界坐标 (`MqVec2`) |

### 8.3 相机切换与 `scheme3d/prelude` 作用域宏

| 函数 / 宏 | 说明 |
| :--- | :--- |
| `(set-camera-3d cam)` | 激活指定 3D 相机 |
| `(set-camera-2d cam)` | 激活指定 2D 相机 |
| `(set-default-camera)` | 恢复默认 2D 屏幕像素坐标系相机（绘制 HUD 前必调，或使用下方 `with-camera-3d` 宏） |
| `(push-camera-state)` / `(pop-camera-state)` | 压入 / 弹出当前相机状态栈 |
| `(with-camera-3d cam body ...)` | **推荐使用**（需 `(require "scheme3d/prelude")`）：自动压栈、激活 `cam`、执行 `body ...`，并在退出或发生异常时自动恢复原相机状态 |
| `(with-camera-2d cam body ...)` | **推荐使用**（需 `(require "scheme3d/prelude")`）：自动在 `cam` 作用域内执行 2D 世界绘制并安全恢复 |

```scheme
(with-camera-3d cam
  (draw-grid 20 1.0 GRAY DARKGRAY)
  (draw-cube (vec3 0.0 1.0 0.0) (vec3 2.0 2.0 2.0) RED))
;; 此处已自动恢复默认 2D 屏幕相机，可直接绘制 HUD 文本
(draw-text "HUD Overlay" 20 30 24 WHITE)
```

---

## 9. 3D 基础几何体与网格 API (3D Models)

| 函数 | 参数与说明 |
| :--- | :--- |
| `(draw-cube position size color)` | 绘制中心在 `position` (`Vec3`)、三轴尺寸为 `size` (`Vec3`) 的实心立方体 |
| `(draw-cube-wires position size color)` | 绘制立方体线框 |
| `(draw-sphere center radius color)` | 绘制球心在 `center` (`Vec3`)、半径为 `radius` 的实心球体 |
| `(draw-sphere-wires center radius color)` | 绘制球体线框 |
| `(draw-sphere-ex center radius rings slices wireframe? color)` | 自定义经纬细分度（`rings`, `slices`）绘制实心 (`#f`) 或线框 (`#t`) 球体 |
| `(draw-cylinder position radius-top radius-bottom height [sides] color)` | 绘制实心圆柱/圆台（支持 5 参数省略 `sides`，或 6 参数指定 `sides` 边数） |
| `(draw-cylinder-wires position radius-top radius-bottom height [sides] color)` | 绘制圆柱/圆台线框（支持 5 参数省略 `sides`，或 6 参数指定 `sides` 边数） |
| `(draw-cylinder-ex position radius-top radius-bottom height sides wireframe? color)` | 完整参数绘制实心 (`#f`) 或线框 (`#t`) 圆柱/圆台 |
| `(draw-plane center size color)` | 在 XZ 平面绘制中心为 `center` (`Vec3`)、尺寸为 `size` (`Vec2`) 的实心平面 |
| `(draw-grid slices spacing axes-color other-color)` | 在 XZ 平面原点绘制参考网格（`axes-color` 为主轴颜色，`other-color` 为普通网格线颜色） |
| `(draw-line-3d start end color)` | 绘制从 `start` (`Vec3`) 到 `end` (`Vec3`) 的 3D 空间线段 |

---

## 10. 键盘与鼠标输入 API (Keyboard & Mouse Input)

键名与鼠标按键均使用 **Scheme 符号（Symbol）** 或字符串表示（大小写不敏感，支持连字符风格）。

### 10.1 键盘输入函数

| 函数 | 说明 |
| :--- | :--- |
| `(is-key-down? key)` | 指定按键当前帧是否处于按住状态 |
| `(is-key-pressed? key)` | 指定按键是否在当前帧刚刚按下（单次触发） |
| `(is-key-released? key)` | 指定按键是否在当前帧刚刚松开 |
| `(is-any-key-down?)` | 当前是否有任意受支持的键盘按键处于按下状态 |
| `(get-last-key-pressed)` | 返回当前帧最后按下的按键符号（如 `'space`、`'a`），若无则返回 `#f` |
| `(get-char-pressed)` | 返回当前帧输入的字符（1 字符长度的 `string`），若无则返回 `#f` |

#### 常用按键符号速查表

- **字母键**：`'a` ~ `'z`
- **顶部数字键**：`'0` ~ `'9`（或别名 `'key-0` ~ `'key-9`）
- **方向键**：`'right`、`'left`、`'down`、`'up`
- **常用控制键**：`'space`、`'enter`（别名 `'return`）、`'escape`（别名 `'esc`）、`'tab`、`'backspace`
- **修饰键**：`'left-shift`（别名 `'lshift`）、`'right-shift`（别名 `'rshift`）、`'left-control`（别名 `'lctrl`）、`'right-control`（别名 `'rctrl`）、`'left-alt`（别名 `'lalt`）、`'right-alt`（别名 `'ralt`）
- **功能键**：`'f1` ~ `'f25`
- **编辑与导航键**：`'insert`、`'delete`、`'home`、`'end`、`'page-up`、`'page-down`
- **小键盘键**：`'kp-0` ~ `'kp-9`、`'kp-decimal`、`'kp-divide`、`'kp-multiply`、`'kp-subtract`、`'kp-add`、`'kp-enter`、`'kp-equal`

### 10.2 鼠标输入函数

鼠标按键符号支持：
- 左键：`'left`（或 `'mouse-left`）
- 右键：`'right`（或 `'mouse-right`）
- 中键：`'middle`（或 `'mouse-middle`）

| 函数 | 说明 |
| :--- | :--- |
| `(is-mouse-button-down? btn)` | 指定鼠标按键当前是否按住 |
| `(is-mouse-button-pressed? btn)` | 指定鼠标按键是否在当前帧刚刚按下 |
| `(is-mouse-button-released? btn)` | 指定鼠标按键是否在当前帧刚刚松开 |
| `(mouse-position)` | 返回当前鼠标屏幕像素坐标 `MqVec2` |
| `(mouse-position-local)` | 返回归一化屏幕坐标 `MqVec2`（范围 `[-1.0, 1.0]`） |
| `(mouse-delta-position)` | 返回相邻两帧之间的鼠标位移差 `MqVec2` |
| `(mouse-wheel)` | 返回当前帧鼠标滚轮滚动量 `MqVec2`（垂直滚动读取 `(vec2-y (mouse-wheel))`） |
| `(set-cursor-grab grab?)` | 锁定/解锁鼠标光标在窗口内（适合第一人称相机） |
| `(show-mouse show?)` | 显示 (`#t`) 或隐藏 (`#f`) 鼠标系统光标 |

---

## 11. 文本与 HUD 渲染 API (Text & HUD)

> **注意**：在 `with-camera-3d` 块之外（或调用 `(set-default-camera)` 之后）调用以下文本函数，即可在屏幕像素坐标系下渲染清晰的 2D HUD 界面。

| 函数 | 参数与说明 |
| :--- | :--- |
| `(draw-text text x y font-size color)` | 在屏幕坐标 `(x, y)` 处绘制单行文本 |
| `(draw-multiline-text text x y font-size [line-distance] color)` | 绘制包含换行符 `\n` 的多行文本（支持 5 参数省略 `line-distance` 或 6 参数显式指定行距倍率） |
| `(measure-text text font-size)` | 测量文本渲染尺寸，返回 `(vec2 width height)`，可用于文本居中对齐计算 |

---

## 12. 窗口、帧率与时间 API (Window & Time)

| 函数 | 返回值与说明 |
| :--- | :--- |
| `(screen-width)` | 当前窗口渲染区域宽度（浮点数，像素） |
| `(screen-height)` | 当前窗口渲染区域高度（浮点数，像素） |
| `(screen-dpi-scale)` | 当前显示器 DPI 缩放倍率 |
| `(get-frame-time)` | 上一帧耗时 `dt`（秒） |
| `(get-fps)` | 当前瞬时帧率（整数） |
| `(get-time)` | 自程序启动以来的累计运行时间（秒，双精度浮点数） |
| `(request-new-screen-size width height)` | 请求调整窗口尺寸为 `width x height` |
| `(set-fullscreen fullscreen?)` | 切换窗口全屏状态（`#t` 或 `#f`） |
| `(is-quit-requested?)` | 检查用户是否请求关闭窗口 |
| `(draw-fps [x y])` | 在屏幕左上角 `(10, 22)` 或指定坐标 `(x, y)` 快速绘制当前 FPS |
