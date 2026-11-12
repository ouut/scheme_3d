# `scheme_3d` 多 Agent 分阶段并行协作实现方案

本文档定义了将 **Rust Steel (`steel-core`)** 绑定到 **Macroquad** 绘图/3D 接口、并实现目录多文件模块化（`require` / `provide`）与热重载的**多 Agent 分工、接口契约、并行阶段与验收门禁**。

---

## 1. 总体技术约定（所有 Agent 必须遵守的基线）

### 1.1 已确认架构决策
- **项目形态**：Library + CLI Binary（核心绑定库 `scheme_3d` + 宿主启动器 `scheme_3d <script_dir>`）。
- **主循环模型（方案 A）**：Rust 拥有 Macroquad `async fn main()` 主循环与每帧 `next_frame().await`；Scheme 负责生命周期回调。
- **入口与回调约定**：
  - 指定脚本目录下的 `main.scm` 为入口文件。
  - 支持约定顶层回调函数（均为可选，未定义则跳过）：
    - `(init)`：首次加载（或需要显式初始化时）执行一次。
    - `(update dt)`：每帧更新逻辑，传入帧间隔秒数 `dt` (`f32 -> f64`)。
    - `(draw)`：每帧渲染逻辑。
  - 若需跨文件组织代码，脚本目录自动加入 Steel 模块搜索路径，支持 `(require "...")` 与 `(provide ...)`。
- **模块暴露方式**：
  - 将 Macroquad 接口注册为 Steel 内置模块（如 `(require-builtin macroquad)`），确保被 `require` 的子模块（如 `scene.scm`、`camera.scm`）也能显式导入绘图函数。
  - 同时在入口执行前自动为 `main.scm` 注入常用 prelude（或默认引入 `macroquad` 与 `with-camera-3d` 等辅助函数），兼顾零样板快速编写与多文件严格模块化。
- **热重载与错误兜底**：
  - 监听脚本目录内所有 `.scm` 文件变更（带约 80ms 防抖），触发后在**候选 `Engine`** 中重新编译加载整个项目。
  - **双缓冲切换**：新 `Engine` 加载成功才替换当前运行中的 `Engine`；若编译或运行出错，**打印错误日志并保留旧 `Engine` 继续渲染**。
  - **跨重载持久化存储**：提供 `(state-get key default)` / `(state-set! key val)` / `(state-has? key)`，底层使用**独立于 Steel VM 堆的 Rust 值枚举 `PersistValue`** 存储，避免跨 `Engine` 持有 `Gc<SteelVal>` 导致内存/GC 崩溃。
- **目标平台与音频**：
  - Phase 0 ~ Phase 2 优先跑通桌面端（Linux / macOS / Windows），同时用 `#[cfg(not(target_arch = "wasm32"))]` 隔离文件系统、CLI 参数与文件监听。
  - `macroquad` 默认关闭 `audio`（避免 Linux 下强依赖 `libasound2-dev`），音频作为可选 feature 留待 Phase 3 接入。
  - Phase 3 专门解决 `wasm32` 下的静态/异步脚本打包与 `steel-core`（`wasm-bindgen` / `js-sys`）和 `macroquad`（`mq_js_bundle.js`）的胶水兼容。

### 1.2 代码目录规划（防冲突文件归属）

```text
scheme_3d/
├── Cargo.toml
├── docs/
│   └── MULTI_AGENT_PLAN.md          # 本文档
├── scheme/
│   └── prelude.scm                  # Scheme 侧辅助库（如 with-camera-3d / with-camera-2d）
├── src/
│   ├── lib.rs                       # 对外库导出 [Agent 0 建立骨架, Agent 5 收口]
│   ├── main.rs                      # CLI 宿主入口与 Macroquad 帧循环 [Agent 5]
│   ├── types.rs                     # 共享自定义类型 & 数值转换契约 [Agent 0]
│   ├── module.rs                    # 内置模块总装配 [Agent 0 骨架, Agent 5 接线]
│   ├── bindings/
│   │   ├── mod.rs                   # 绑定子模块声明 [Agent 0]
│   │   ├── math.rs                  # Vec2 / Vec3 / 数学辅助 [Agent 1]
│   │   ├── color.rs                 # Color 构造与颜色常量 [Agent 1]
│   │   ├── shapes2d.rs              # 2D 图形绘制 [Agent 1]
│   │   ├── camera.rs                # Camera2D / Camera3D / 相机切换 [Agent 2]
│   │   ├── models3d.rs              # 3D 几何体绘制 [Agent 2]
│   │   ├── input.rs                 # 键盘 / 鼠标输入与符号映射 [Agent 3]
│   │   ├── text.rs                  # 文本 / HUD 绘制与测量 [Agent 3]
│   │   └── window.rs                # 窗口尺寸 / 时间 / FPS / 光标控制 [Agent 3]
│   └── runtime/
│       ├── mod.rs                   # 运行时对外接口与 Trait 契约 [Agent 0]
│       ├── persist.rs               # 跨重载 PersistValue 存储 [Agent 4]
│       ├── loader.rs                # 目录脚本解析与虚拟源抽象 [Agent 4]
│       ├── watcher.rs               # 桌面端文件监听与防抖 [Agent 4]
│       └── engine.rs                # 双缓冲 SteelEngine 包装与回调调度 [Agent 4]
├── examples/
│   ├── hello_2d/
│   │   └── main.scm                 # 2D 基础示例 [Agent 6]
│   └── basic_3d/
│       ├── main.scm                 # 3D 多文件入口 [Agent 6]
│       ├── camera.scm               # 相机控制模块 (provide/require) [Agent 6]
│       └── scene.scm                # 3D 场景模块 (provide/require) [Agent 6]
└── tests/
    └── integration_vm.rs            # 无头集成测试：绑定/模块/热重载/持久化 [Agent 7]
```

> **核心并行规则**：除 Phase 0 冻结共享契约、Phase 2 集成接线外，Phase 1 的所有 Agent **严格只在各自负责的文件内编码**，绝不交叉修改他人文件。

---

## 2. 分阶段协作拓扑（DAG 依赖图）

```text
[Phase 0: 架构与契约底座]
        Agent 0 (Architect)
        产出 Cargo.toml + types.rs + module.rs 骨架 + runtime/mod.rs 契约
                 │
    ┌────────────┼────────────┬────────────┐
    ▼            ▼            ▼            ▼
[Phase 1: 核心绑定与运行时四路并行]
Agent 1      Agent 2      Agent 3      Agent 4
Math/Color   Camera/3D    Input/Text   VM Runtime
& Shapes2D   & Prelude    & Window     & HotReload
    └────────────┼────────────┴────────────┘
                 │
                 ▼
[Phase 2: 宿主集成、示例与质量验收]
   ┌─────────────┴─────────────┐
   ▼                           ▼
Agent 5                     Agent 6
CLI 宿主与总装               多文件 2D/3D 示例与文档
   └─────────────┬─────────────┘
                 ▼
              Agent 7
          无头集成测试与 Clippy 验收
                 │
                 ▼
[Phase 3: 跨平台 WASM 与扩展能力（按需并行）]
   ┌─────────────┴─────────────┐
   ▼                           ▼
Agent 8                     Agent 9
WASM 打包与 JS Shim          Audio / 纹理 / Mesh / Shader
```

---

## 3. 各阶段详细任务书与接口契约

### Phase 0：基座与接口契约（串联，前置必须完成）

#### Agent 0：Architecture & Contract Lead
- **独占修改范围**：`Cargo.toml`、`src/lib.rs`、`src/types.rs`、`src/module.rs`、`src/bindings/mod.rs`、`src/runtime/mod.rs`
- **关键任务**：
  1. 初始化 Cargo 工程，确认依赖与特性：
     - `steel-core = "0.8"`、`steel-derive = "0.8"`（桌面端默认关闭或按需开启特性，确保 `wasm32` 不引入 Cranelift JIT / dylibs）。
     - `macroquad = { version = "0.4", default-features = false }`（关闭默认音频，避免 Linux 缺少 `libasound2-dev` 阻塞构建；预留 `audio = ["macroquad/audio"]`）。
     - `notify`（可选/仅限 `#[cfg(not(target_arch = "wasm32"))]`，或轻量 mtime 轮询）。
  2. 在 `src/types.rs` 中定义跨模块共享的包装类型与转换工具：
     - `pub fn steel_to_f32(val: &SteelVal) -> Result<f32, SteelErr>`：同时兼容 `IntV(isize)` 与 `NumV(f64)`（**关键体验要求**：允许 Scheme 传整数 `0` 或浮点 `0.0`）。
     - `pub struct MqVec2(pub Vec2)`、`pub struct MqVec3(pub Vec3)`、`pub struct MqColor(pub Color)`、`pub struct MqCamera2D(...)`、`pub struct MqCamera3D(...)`（**关键坑点**：Macroquad 的 `Camera3D` 因包含 `RenderTarget` 不实现 `Clone`，因此 `MqCamera3D` 必须保存可 `Clone` 的参数字段，在调用 `set_camera` 时按需构造 `macroquad::camera::Camera3D`）。
     - 可选辅助转换：`steel_to_vec2` / `steel_to_vec3` / `steel_to_color` 除接受自定义类型外，也兼容 2/3/4 元素 Scheme `list` 或 `vector`。
  3. 在 `src/module.rs` 与 `src/bindings/mod.rs` 约定子模块注册签名：
     ```rust
     pub fn register(module: &mut BuiltInModule)
     ```
  4. 在 `src/runtime/mod.rs` 冻结宿主运行时 Trait/Struct 契约（`ScriptHost`、`ReloadStatus`、`FrameErrorState`）。
- **Phase 0 验收门禁**：
  - `cargo check` 在当前机器通过（验证 `steel-core` 与 `macroquad` 版本兼容、API 路径无误）。

---

### Phase 1：核心绑定与运行时（4 个 Agent 完全并行）

#### Agent 1：Math, Color & 2D Shapes 绑定工程师
- **输入契约**：`src/types.rs`、`src/bindings/mod.rs`
- **独占修改范围**：`src/bindings/math.rs`、`src/bindings/color.rs`、`src/bindings/shapes2d.rs`
- **核心 API 清单**：
  - `math.rs`：
    - `vec2`, `vec2-x`, `vec2-y`, `vec2+`, `vec2-`, `vec2*`, `vec2-length`, `vec2-normalize`, `vec2-dot`, `vec2-lerp`
    - `vec3`, `vec3-x`, `vec3-y`, `vec3-z`, `vec3+`, `vec3-`, `vec3*`, `vec3-length`, `vec3-normalize`, `vec3-dot`, `vec3-cross`, `vec3-lerp`, `vec3-distance`
  - `color.rs`：
    - `(rgba r g b a)`、`(rgb r g b)`、`(rgba8 r g b a)`、`color-with-alpha`
    - 常用常量：`WHITE`, `BLACK`, `RED`, `GREEN`, `BLUE`, `YELLOW`, `ORANGE`, `PURPLE`, `PINK`, `GRAY`, `LIGHTGRAY`, `DARKGRAY`, `SKYBLUE`, `GOLD`
  - `shapes2d.rs`：
    - `(clear-background color)`
    - `(draw-line x1 y1 x2 y2 thickness color)`
    - `(draw-rectangle x y w h color)`、`(draw-rectangle-lines x y w h thickness color)`
    - `(draw-circle x y r color)`、`(draw-circle-lines x y r thickness color)`
    - `(draw-triangle v1 v2 v3 color)`、`(draw-triangle-lines v1 v2 v3 thickness color)`
    - `(draw-poly x y sides radius rotation color)`、`(draw-poly-lines x y sides radius rotation thickness color)`
    - `(draw-ellipse x y w h rotation color)`、`(draw-arc x y sides radius rotation thickness arc color)`
- **单元测试要求**：不依赖 Macroquad 渲染上下文（注：Macroquad 的绘制/相机/时间函数依赖主线程全局 `CONTEXT`，无头 `cargo test` 中不能直接调用 `draw_*`）。单测重点覆盖 `vec2`/`vec3` 运算、`IntV`/`NumV` 混合传参、颜色构造和列表隐式转换。

#### Agent 2：Camera & 3D Primitives 绑定工程师
- **输入契约**：`src/types.rs`、`src/bindings/mod.rs`
- **独占修改范围**：`src/bindings/camera.rs`、`src/bindings/models3d.rs`、`scheme/prelude.scm`
- **核心 API 清单**：
  - `camera.rs`：
    - `(make-camera-3d pos target up fovy)` / `(camera-3d-default)` 与属性修改器（`camera-3d-set-position!`, `camera-3d-set-target!`, `camera-3d-set-up!`, `camera-3d-set-fovy!`, `camera-3d-set-projection!` `'perspective`/`'orthographic`, `camera-3d-set-clip!` `near` `far`）
    - `(make-camera-2d target zoom rotation offset)`
    - `(set-camera-3d cam)`、`(set-camera-2d cam)`、`(set-default-camera)`、`(push-camera-state)`、`(pop-camera-state)`
  - `models3d.rs`：
    - `(draw-cube pos size color)`、`(draw-cube-wires pos size color)`
    - `(draw-sphere center radius color)`、`(draw-sphere-wires center radius color)`、`(draw-sphere-ex center radius rings slices wires? color)`
    - `(draw-cylinder pos radius-top radius-bottom height slices color)`、`(draw-cylinder-wires ...)`
    - `(draw-plane center size color)`
    - `(draw-grid slices spacing axes-color other-color)`
    - `(draw-line-3d start end color)`
  - `scheme/prelude.scm`：
    - 高阶相机封装 `(with-camera-3d cam thunk)`、`(with-camera-2d cam thunk)`（内部通过 `push-camera-state` / `pop-camera-state` 或 `set-default-camera` 保证 3D 绘图后自动恢复 2D HUD 相机状态）。
    - **注意**：需在 `with-camera-3d` 中注意异常时尽量恢复状态，避免出错导致整帧相机混乱。
- **单元测试要求**：测试 `MqCamera3D` 参数转换、投影模式枚举校验、以及 `prelude.scm` 语法解析正确性（不触发需要 `CONTEXT` 的 `set_camera` / `draw_*` 调用）。

#### Agent 3：Input, Text & Window 绑定工程师
- **输入契约**：`src/types.rs`、`src/bindings/mod.rs`
- **独占修改范围**：`src/bindings/input.rs`、`src/bindings/text.rs`、`src/bindings/window.rs`
- **核心 API 清单**：
  - `input.rs`：
    - 按键符号映射表（`&str`/Symbol -> `macroquad::input::KeyCode`）：覆盖字母 `'a`..`'z`、数字 `'key0`..`'key9`、方向键 `'up`/`'down`/`'left`/`'right`、控制键 `'space`/`'enter`/`'escape`/`'tab`/`'left-shift`/`'right-shift`/`'left-control`/`'left-alt`、功能键 `'f1`..`'f12` 等。
    - `(is-key-down? key)`、`(is-key-pressed? key)`、`(is-key-released? key)`、`(get-last-key-pressed)`、`(get-char-pressed)`
    - 鼠标按钮映射：`'left` / `'right` / `'middle`
    - `(is-mouse-button-down? btn)`、`(is-mouse-button-pressed? btn)`、`(is-mouse-button-released? btn)`
    - `(mouse-position)`（返回 `(vec2 x y)` 与全项目 `vec2` 类型保持一致）、`(mouse-position-local)`、`(mouse-delta-position)`、`(mouse-wheel)`
    - `(set-cursor-grab grab?)`、`(show-mouse show?)`
  - `text.rs`：
    - `(draw-text text x y font-size color)`
    - `(draw-multiline-text text x y font-size line-factor color)`
    - `(measure-text text font-size)`（返回 `(list width height offset-y)`）
  - `window.rs`：
    - `(screen-width)`、`(screen-height)`、`(get-frame-time)`、`(get-fps)`、`(get-time)`
    - `(request-new-screen-size w h)`、`(is-quit-requested?)`
- **单元测试要求**：对全部支持的 `KeyCode` 与 `MouseButton` 符号做纯函数映射单测，保证大小写/连字符风格一致且无遗漏（不在无头单测中直接读取 Macroquad `CONTEXT`）。

#### Agent 4：Steel VM 运行时、目录模块与热重载工程师
- **输入契约**：`src/types.rs`、`src/runtime/mod.rs`、`src/module.rs` 注入接口
- **独占修改范围**：`src/runtime/persist.rs`、`src/runtime/loader.rs`、`src/runtime/watcher.rs`、`src/runtime/engine.rs`
- **核心任务**：
  1. **跨重载持久化 (`persist.rs`)**：
     - 定义 `PersistValue`（`Void`, `Bool(bool)`, `Int(isize)`, `Float(f64)`, `String(String)`, `Symbol(String)`, `List(Vec<PersistValue>)`, `Map(...)`, `Vec2([f32; 2])`, `Vec3([f32; 3])`, `Color([f32; 4])`, `Camera3D(...)`）。
     - 线程安全/共享句柄 `PersistStore(Arc<Mutex<HashMap<String, PersistValue>>>)`，注册 `state-get`、`state-set!`、`state-has?`、`state-remove!`、`state-clear!` 给 Steel VM。
  2. **目录解析 (`loader.rs`)**：
     - 桌面端接收目录路径 `PathBuf`，校验 `<dir>/main.scm` 存在。
     - 调用 `engine.add_search_directory(...)` 使目录下相对/模块化 `(require ...)` & `(provide ...)` 正常解析。
     - 预留 `#[cfg(target_arch = "wasm32")]` 的内存/虚拟源加载接口骨架（供 Phase 3 Agent 8 填充）。
  3. **文件监听 (`watcher.rs`)**：
     - 仅在 `#[cfg(not(target_arch = "wasm32"))]` 启用。
     - 递归监控脚本目录内 `.scm` 文件变更，做 ~80ms 防抖合并（避免编辑器原子保存触发多次重载）。
  4. **双缓冲执行器 (`engine.rs`)**：
     - `try_build_engine(dir, persist_store, register_fn)`：创建新 `Engine`，注册内置 Macroquad 模块 + 持久化模块 + `prelude.scm`，运行 `main.scm`。
     - 首次启动若定义 `(init)` 则执行；热重载时可选保留不重跑 `(init)` 或通过 `(on-reload)` / `(state-has?)` 由脚本控制。
     - 每帧调度 `call_update(dt)` 和 `call_draw()`：检测 `update`/`draw` 是否存在，不存在则安静跳过；**若某帧运行报错，打印一次错误日志并标记当前帧错误态（避免 60fps 刷屏），等待下一次文件保存热重载自动恢复**。
     - 热重载失败时：**丢弃失败的候选 `Engine`，打印 `SteelErr` 诊断信息，保留旧 `Engine` 继续工作**。
- **单元测试要求**：使用临时目录编写无头 VM 测试，验证：
  - `main.scm` 通过 `(require "sub.scm")` 引入 `(provide ...)` 导出的函数；
  - 修改 `sub.scm` 后重建 `Engine` 新逻辑生效；
  - 故意写入语法错误后重载失败但旧 `Engine` 仍可正常调用 `update`/`draw`；
  - 跨重载后 `state-get` 仍能读回重载前 `state-set!` 的数值与 `Vec3`/`Camera3D`。

---

### Phase 2：宿主总装、示例与自动化验收

#### Agent 5：CLI 宿主与总装工程师
- **前置依赖**：Phase 1 全部完成
- **独占修改范围**：`src/main.rs`、`src/lib.rs`、`src/module.rs`
- **核心任务**：
  1. 在 `src/module.rs` 串联 Agent 1/2/3 注册的所有函数到 `BuiltInModule::new("macroquad")`（并处理内置注入与 prelude 加载顺序）。
  2. 在 `src/main.rs` 解析命令行参数：
     - `scheme_3d <dir>`（默认当前目录或示例目录）
     - 可选参数：`--title`、`--width`、`--height`、`--msaa`（默认 `4` 对 3D 更友好）、`--no-watch`
  3. 先完成 CLI 参数解析（保证 `--help` / 参数校验错误在弹窗前退出），再通过 `macroquad::Window::from_config(conf, async move { ... })`（或 `#[macroquad::main(window_conf)]`）启动窗口并将动态窗口配置传入 `Conf`。
  4. 在主循环内串联（严格保持在 Macroquad 启动的主线程上调用，满足 `thread_assert::same_thread()`）：`watcher.poll_changed()` -> `host.try_reload()` -> `host.update(get_frame_time())` -> `host.draw()` -> `next_frame().await`。

#### Agent 6：Scheme 示例工程与使用文档工程师
- **前置依赖**：Phase 1 API 定稿（可与 Agent 5 尾声并行）
- **独占修改范围**：`examples/hello_2d/*`、`examples/basic_3d/*`、`README.md`
- **核心任务**：
  1. `examples/hello_2d/main.scm`：最简 2D 动画与鼠标交互示例。
  2. `examples/basic_3d/`：演示多文件组织与热重载工作流：
     - `camera.scm`：封装 WASD + 鼠标右键/锁定视角的 3D 自由相机（`provide` 更新与获取相机接口）。
     - `scene.scm`：绘制 3D 网格地面、旋转立方体、球体、圆柱与 3D 线段。
     - `main.scm`：组合 `camera.scm` 与 `scene.scm`，使用 `state-get`/`state-set!` 保存相机位置以支持热重载不断点，并在 `with-camera-3d` 后绘制 2D FPS/操作提示 HUD。
  3. `README.md`：包含构建说明、Linux 系统依赖说明、CLI 用法、热重载工作流、以及第一阶段 Scheme API 速查表。

#### Agent 7：QA & 自动化验收工程师
- **前置依赖**：Agent 5、Agent 6 完成
- **独占修改范围**：`tests/integration_vm.rs`
- **核心任务**：
  1. 运行全量 `cargo test`、`cargo clippy --all-targets -- -D warnings`。
  2. 在无头环境下通过 `Engine` 执行 `examples/hello_2d/main.scm` 与 `examples/basic_3d/main.scm` 的静态检查/加载路径测试（模拟调用注册符号检查 `update`/`draw` 定义完整性，确认不存在未绑定符号或跨模块 `require` 失败）。
  3. 检查 `#[cfg(target_arch = "wasm32")]` 隔离边界是否干净（不让 `notify` / `std::fs` 泄漏到共享核心模块）。

---

### Phase 3：跨平台 WASM 与可选扩展（Phase 2 验收通过后开启）

#### Agent 8：WASM 构建与虚拟模块工程师
- **核心任务**：
  1. 在 `src/runtime/loader.rs` 的 `wasm32` 分支实现脚本加载：
     - 方案选型（二选一或双支持）：编译期嵌入目录（`include_dir`）或启动期 `macroquad::file::load_string` 拉取清单，并将多文件源码挂载为 Steel 可 `require` 的模块源。
  2. 解决 `steel-core`（`getrandom`/`js-sys` -> `wasm-bindgen`）与 `macroquad`（`mq_js_bundle.js`）的 WASM 桥接：
     - 编写 `web/index.html`、`scripts/build_wasm.sh` 与 `wasm-bindgen` + `mq_js_bundle.js` 兼容胶水。
  3. 实测 `cargo build --target wasm32-unknown-unknown` 并在本地静态服务器验证 3D 示例在浏览器渲染与交互。

#### Agent 9：Audio 与进阶 3D 能力工程师（按需并行）
- **核心任务**：
  1. 在 `--features audio` 下绑定 `macroquad::audio`（`load-sound` 异步预加载队列、`play-sound`、`stop-sound`、`set-sound-volume`）。
  2. 扩展 3D 高阶渲染：`Texture2D` 加载与贴图立方体/球体、自定义 `Mesh`（顶点/索引/法线/UV）、`Material` 自定义 GLSL Shader（简单漫反射光照示例）、以及可选 `.obj` 模型加载。

---

## 4. 协作流程与每阶段验收检查单

1. **启动 Phase 0 前**：确保先由 Agent 0 跑通 `steel-core 0.8` + `macroquad 0.4` 的最小编译，固定 `src/types.rs` 和 `src/runtime/mod.rs`。
2. **Phase 1 并行期**：Agent 1~4 独立提交各自模块及单元测试；每日/每轮汇合跑一次模块编译验证。
3. **Phase 2 集成期**：Agent 5 统一接线，Agent 6 提供真实 Scheme 场景压测 API 易用性（发现不顺手的函数名或参数顺序及时微调），Agent 7 输出最终测试报告。
4. **交付标准**：
   - `cargo test` 全绿，`cargo clippy` 无严重警告；
   - 运行 `cargo run -- examples/basic_3d` 能弹出 3D 窗口、流畅控制相机、显示 2D HUD；
   - 修改 `examples/basic_3d/scene.scm` 保存后窗口实时刷新且相机位置不丢失；
   - 故意在 `.scm` 中引入语法错误保存后，终端打印错误日志但窗口不崩溃，改回正确后自动恢复。
