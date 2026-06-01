# Blockworld Architecture

## 项目概述

Blockworld 是一个用 Rust 重写 Minecraft（Java Edition 1.16）的现代化开源实现。核心目标：

- **更好的性能**：利用 Rust 的多线程和零成本抽象
- **模块化设计**：可作为体素游戏引擎使用
- **WASM 模組 API**：模組可用 WebAssembly 编写
- **完全开源**：MIT 协议，无代码混淆
- **内容精简**：以 1.12.2 内容为基础，高版本内容通过 mod 提供

---

## 工作区结构（Cargo Workspace）

```
blockworld/
├── Cargo.toml                        # workspace root (resolver = "2")
├── blockworld-utils/                 # 共享工具库（无外部依赖的基础设施）
├── blockworld-server/                # 服务端游戏逻辑库
├── blockworld-client/                # 客户端应用（二进制，含 GPU 渲染）
└── blockworld-renderer/              # 独立渲染器（占位 stub，仅打印 Hello World）
```

### 依赖关系

```
blockworld-utils     (独立，不依赖其他 crate)
    ↑
blockworld-server    (依赖 blockworld-utils + glam + tokio)
    ↑
blockworld-client    (依赖 blockworld-utils + blockworld-server + wgpu + winit + egui)
```

---

## 一、blockworld-utils — 共享基础设施

**职责**：提供整个项目共用的基础类型和工具。

### 模块结构

```
blockworld-utils/src/
├── lib.rs               # 重导出 + 型別別名
├── constants.rs         # GAME_NAME = "blockworld"
├── registry.rs          # 泛型注册表 Registry<V: HasIdentifier>
└── resource/
    ├── mod.rs
    └── resource_location.rs  # Identifier（命名空间:路径）
```

### 核心类型

#### `Identifier`（resource_location.rs）
与 Minecraft 的 `Identifier` 一致。用单个 `String id` 存储 `namespace:path` 格式（如 `minecraft:stone`）。

```rust
pub struct Identifier { id: String }
```

支持 `Deref<Target=str>`，可直接当作 `&str` 使用。实现了 `From<&str>`、`Default`。

#### `Registry<V: HasIdentifier>`（registry.rs）
双向映射表，同时支持：
- `Identifier` → `V`（名称查询值）
- `u32` ↔ `Identifier`（数字 ID ↔ 名称，使用 BiMap）

用于方块注册表等场景。

#### 型別別名（lib.rs）
```rust
pub type AM<T> = Arc<Mutex<T>>;       // 线程安全共享
pub type RR<T> = Rc<RefCell<T>>;      // 单线程共享
pub type OAM<T> = Option<AM<T>>;
pub type ORR<T> = Option<RR<T>>;
```

---

## 二、blockworld-server — 服务端游戏逻辑

**职责**：定义方块、区块、世界存取接口、ECS 元件、网络包。

### 模块结构

```
blockworld-server/src/
├── lib.rs                   # Blockworld 主结构体
├── packet.rs                # Packet 枚举（网络包）
├── block/
│   ├── mod.rs               # BLOCK_REGISTRY 懒静态初始化
│   ├── block.rs             # Block 结构体 + Material 枚举
│   └── block_face_direction.rs  # 6 方向 bitflags
└── world/
    ├── mod.rs
    ├── chunk.rs             # SubChunk（16×16×16，YZX 编排）
    ├── chunk_access.rs      # WorldAccess trait（抽象接口）
    └── disk_chunk_access.rs # DiskChunkArray（HashMap 实现）
```

### 核心设计

#### 方块系统
```rust
pub struct Block {
    pub id: Identifier,  // 如 "minecraft:stone"
}

pub enum Material {
    Solid,  // 不透明固体
    Glass,  // 透明
    Air,    // 空气（默认）
}
```

`BLOCK_REGISTRY` 是一个懒静态全局注册表，目前注册了两个测试方块：`minecraft:air`（ID=0）、`minecraft:stone`（ID=1）。

#### SubChunk（chunk.rs）
最小存储单元，16×16×16 的方块立方体。使用 `Box<[u32; 4096]>` 存储（YZX 格式，与 Minecraft 区块格式一致）。

```
索引公式: index(x, y, z) = y * 16 * 16 + z * 16 + x
```

#### WorldAccess trait（chunk_access.rs）
抽象世界存取接口（类似 Minecraft 的 `IBlockReader`）：

```rust
pub trait WorldAccess {
    fn get_chunk(&self, pos: IVec3) -> &SubChunk;
    fn is_chunk_loaded(&self, pos: IVec3) -> bool;
    fn load_chunk(&mut self, pos: IVec3);
    fn unload_chunk(&mut self, pos: IVec3);
    fn is_air(&self, pos: IVec3) -> bool;
    fn get_block(&self, pos: IVec3) -> Identifier;
    fn set_block(&mut self, pos: IVec3, id: &Identifier);
    fn need_rerender(&self, pos: IVec3) -> bool;
    fn iter_loaded_chunks(&self) -> impl Iterator<Item = &SubChunk>;
    fn update(&mut self, packet: Packet);
}
```

#### DiskChunkArray（disk_chunk_access.rs）
`WorldAccess` trait 的当前实现。设计思路来自 Minecraft 的 `ClientChunkProvider.java`：

- 内部使用 `HashMap<IVec3, SubChunk>` 存储已载入区块
- 通过 `view_distance` 控制视野
- `need_rerender: Vec<IVec3>` 追踪需要重新生成网格的区块
- 含有一个临时地形生成器（正弦波，用于测试）
- `recenter()` 方法更新中心区块位置

#### ECS 元件（components/mod.rs）
使用 Bevy ECS：
- `HasView`：相机位置、朝向、FOV、移动速度等
- `Player`：玩家标记元件

#### 网络包（packet.rs）
目前只有三个变种：
- `BlockUpdate(IVec3, String)` — 方块更新
- `MoveTo(Vec3)` — 移动到位置
- `Pass` — 空包

#### Blockworld 主结构体（lib.rs）
```rust
pub struct Blockworld {
    pub chunks: DiskChunkArray,  // 区块存储（view_distance=8）
}
```

---

## 三、blockworld-client — 客户端应用

**职责**：窗口创建、输入处理、GPU 渲染、相机系统、区块网格生成。

### 模块结构

```
blockworld-client/
├── main.rs                     # 入口：pollster::block_on(run())
├── game/
│   ├── mod.rs
│   └── client.rs               # BlockworldClient（客户端镜像结构）
└── renderer/
    ├── mod.rs                  # 模组树
    ├── window_init.rs          # winit 事件循环 + App 入口
    ├── render_state.rs         # 中央渲染状态
    ├── world_renderer.rs       # WorldRenderer 编排器
    ├── camera.rs               # FPS 风格相机
    ├── input_manager.rs        # 键盘输入追踪
    ├── init_helpers.rs         # wgpu 初始化辅助函数
    ├── pipeline.rs             # RegularPipeline + WireframePipeline
    ├── texture.rs              # 纹理/深度纹理管理
    ├── vertex.rs               # TexturedVertex 型別
    ├── uniform.rs              # GPU uniform 缓冲抽象
    ├── atlas_image.rs          # 纹理图集构建
    ├── resource_manager.rs     # 懒静态 BLOCK_ATLAS
    ├── bytes_provider.rs       # 资源载入抽象（静态嵌入 / 文件系统）
    ├── shaders/
    │   ├── mod.rs              # WgslShader 加载器
    │   ├── default_shader.wgsl  # 主渲染着色器
    │   └── wireframe_shader.wgsl # 线框调试着色器
    └── meshing/
        ├── mod.rs
        ├── block_meshing.rs    # 方块面四边形生成
        └── meshing_manager.rs  # 区块网格构建与渲染
```

### 核心设计

#### 启动流程

```
main() → pollster::block_on(run())
    → EventLoop::new()
    → WindowApplication::default()
    → event_loop.run_app()
        → resumed()  → create_window() → RenderState::new(window)
        → window_event(RedrawRequested) → update() → render()
```

#### RenderState（render_state.rs）
渲染管线的中央状态对象：

```rust
pub struct RenderState {
    pub window: Arc<Window>,        // winit 窗口
    pub surface: Surface<'static>,   // wgpu 交换链表面
    pub device: Device,             // wgpu 设备
    pub queue: Queue,               // wgpu 命令队列
    pub config: SurfaceConfiguration,
    pub input_manager: InputManager,
    pub world_renderer: WorldRenderer,
    // ...
}
```

每帧流程：
1. `update()` — 计算 delta time、更新相机、更新 FPS 标题
2. `render()` — 获取 surface texture → 创建 encoder → 开始 render pass（天空蓝清除色）→ 绑定纹理和 uniform → 绘制 → 提交

#### WorldRenderer（world_renderer.rs）
顶层渲染编排器，负责初始化所有渲染资源：

| 资源 | 用途 |
|------|------|
| `diffuse_texture` | 方块纹理图集（BindableTexture） |
| `depth_texture` | 深度缓冲 |
| `matrix_uniform` | 相机 MVP 矩阵（Uniform buffer，binding=30） |
| `main_pipeline` | 正常渲染管线（填充三角形、背面剔除、alpha 混合） |
| `wireframe_pipeline` | 调试线框管线（无剔除） |
| `meshing_manager` | 区块网格管理器 |
| `camera` | FPS 相机 |

渲染时根据 `debug_mode`（F1 切换）选择管线。

#### 相机系统（camera.rs）
FPS 风格相机，使用右手坐标系（look_to_rh + perspective_rh）：

- 位置默认 `(0, 10, 5)`
- 鼠标移动更新 yaw/pitch
- WASD + Space/Shift 移动
- MVP 矩阵 = projection × view

#### 输入管理（input_manager.rs）
追踪当前按下的按键：W/A/S/D/Space/Shift → MovementRecord。窗口事件中处理键盘按下/释放。

#### 纹理图集系统（atlas_image.rs）
将多个 16×16 纹理文件拼接成一张 512×512 的纹理图集（Atlas）：

- 从 `assets/minecraft/textures/block/` 读取 PNG 文件
- 按网格排列（32 tiles/row × 32 rows）
- 提供 `query_uv(name)` 根据方块名称查询 UV 坐标

#### 方块网格生成（meshing/）

**block_meshing.rs**：
- 定义 8 个立方体顶点和 6 个面（X+/Y+/Z+/X-/Y-/Z-）
- `to_quad_mesh(face, center, uv_min, uv_max)` 为单个方块面生成 6 个带纹理坐标的顶点（两个三角形）

**meshing_manager.rs**：
- 维护 `Vec<RenderChunk>`，每个 RenderChunk 包含顶点缓冲和顶点数
- `update()` 遍历需要重新生成网格的已载入区块
- 对每个非空气方块，检查六个方向是否被遮挡（face culling）
- 为可见面生成带纹理的四边形
- `render()` 绑定顶点缓冲并绘制

---

## 四、数据流

### 方块从定义到渲染的完整路径

```
1. 定义
   Block { id: "minecraft:stone" }
        ↓
   BLOCK_REGISTRY.register(block)
        分配数字 ID（如 1），建立双向映射

2. 世界存储
   SubChunk::set_blockid(pos, "minecraft:stone")
        ↓
   BLOCK_REGISTRY.name_to_number_id("minecraft:stone") → 1
        ↓
   blocks[y*256 + z*16 + x] = 1   (YZX 格式)

3. 查询
   SubChunk::get_blockid(pos)
        ↓
   blocks[index] = 1
        ↓
   BLOCK_REGISTRY.number_id_to_name(1) → "minecraft:stone"

4. 网格生成（MeshingManager::update）
   遍历已载入区块 → 跳过空气方块
   对每个固体方块，检查 6 个方向：
     is_air(neighbor) → 生成该面的四边形
     !is_air(neighbor) → 跳过（face culling）
   查询纹理 UV：BLOCK_ATLAS.query_uv("minecraft:stone")
   生成 6 个 TexturedVertex → 上传到 GPU 顶点缓冲

5. GPU 渲染（WorldRenderer::render）
   set_pipeline(main_pipeline)
   set_bind_group(0, diffuse_texture)   // 纹理图集
   set_bind_group(1, matrix_uniform)    // MVP 矩阵
   set_vertex_buffer → draw()
```

### 顶点着色器数据流

```
TexturedVertex { position: [f32;3], uv: [f32;2] }
    ↓
Vertex Shader (default_shader.wgsl)
   @location(10) position: vec3<f32>
   @location(11) uv: vec2<f32>
    ↓ 乘以 MVP 矩阵
   @builtin(position) → rasterizer
    ↓
Fragment Shader
   采样 diffuse_texture @ uv → 输出颜色
```

---

## 五、设计模式与约定

### 1. 依赖最小化
刻意保持外部依赖精简。不使用 Bevy ECS，实体系统待后续按需实现。

### 2. Trait 抽象
- `WorldAccess`：抽象世界存取，允许不同后端实现（内存 HashMap、磁盘、网络同步）
- `HasIdentifier`：任何可被注册表的物件
- `BytesProvider`：抽象资源载入（静态嵌入、文件系统、未来 HTTP/资源包）
- `ToBytes` / `ToWgpuShader`：GPU 数据传送抽象

### 3. Minecraft 命名约定
代码刻意沿用 Minecraft 内部类名，以便 mod 作者更容易理解和移植：
- `DiskChunkArray` ≈ `ClientChunkProvider.java`
- `Identifier` ≡ Minecraft `Identifier`
- `SubChunk` ≈ `LevelChunkSection`
- YZX 区块格式 ≈ Minecraft 区块格式

### 4. 懒初始化
- `BLOCK_REGISTRY`：once_cell Lazy
- `BLOCK_ATLAS`：once_cell Lazy

---

## 六、键绑定

| 按键 | 功能 |
|------|------|
| WASD | 移动 |
| Space | 上升 |
| Shift | 下降 |
| 鼠标移动 | 旋转视角 |
| F1 | 切换线框渲染模式 |
| F2 | 切换光标锁定（测试用） |
| Esc | 退出 |

---

## 七、当前开发状态

### 已实现
- ✅ 工作区结构和 crate 依赖
- ✅ winit 窗口 + wgpu 渲染管线
- ✅ FPS 相机（WASD + 鼠标）
- ✅ 纹理图集系统（Atlas）
- ✅ 方块注册表（Registry）
- ✅ 区块存储系统（SubChunk + DiskChunkArray）
- ✅ 面剔除（face culling）
- ✅ 贪心网格（greedy meshing）
- ✅ 线框调试模式
- ✅ Bevy ECS 集成框架
- ✅ Minecraft 格式模型解析器
- ✅ wgpu 29 兼容

### 待实现（TODO）
- [ ] 抗锯齿（MSAA）
- [ ] 环境光遮蔽（Ambient Occlusion）
- [ ] Mipmap 纹理
- [ ] 自定义着色器系统
- [ ] 完整的 ECS 系统（Schedule 未接入）
- [ ] 命令行解析器
- [ ] 脚本语言支持
- [ ] GUI 系统（egui 集成）
- [ ] 存档序列化/反序列化
- [ ] 地图生成（目前仅有正弦波测试地形）
- [ ] 网络传输（tungstenite 上的实际网络代码）
- [ ] WASM 模組 API
- [ ] 纹理图集生成器（自动合并纹理）
- [ ] 资源包系统
