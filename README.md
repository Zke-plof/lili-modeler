<div align="center">

<img width="1408" height="768" alt="image" src="https://github.com/user-attachments/assets/3f7480f1-00f7-4603-afea-8de125310377" />

# Lili Modeler

### Open-source 3D modeling for engineers

<br/>

![Version](https://img.shields.io/badge/version-0.1.0-blue?style=for-the-badge)
![Rust](https://img.shields.io/badge/rust-2021-orange?style=for-the-badge)
![Tauri](https://img.shields.io/badge/tauri-v2-green?style=for-the-badge)
![Three.js](https://img.shields.io/badge/three.js-0.168-black?style=for-the-badge)
![License](https://img.shields.io/badge/license-MIT-yellow?style=for-the-badge)

<br/>

**Lili Modeler** is a lightweight, cross-platform 3D modeling application<br/>
built for engineering students who need real tools without the bloat.

<br/>

[Getting Started](#getting-started) · [Features](#features) · [Screenshots](#screenshots) · [Keyboard Shortcuts](#keyboard-shortcuts) · [Contributing](#contributing)

</div>

---

<br/>

## Why Lili Modeler?

| | Blender | Fusion 360 | **Lili Modeler** |
|---|---------|-----------|------------------|
| Binary size | ~200MB | ~3GB | **~5MB** |
| Startup time | 5-15s | 10-30s | **<1s** |
| Price | Free | $$$ | **Free** |
| Full mesh editing | Yes | Limited | **Yes** |
| Import STL/GLTF | Yes | Yes | **Yes** |
| Sculpt mode | Yes | No | **Yes** |
| Cross-platform | Yes | Yes | **Yes** |

<br/>

## Features

### Modeling

- **12 Primitive Types** — Cube, Sphere, Cylinder, Torus, Plane, Cone, Monkey (Suzanne), Tetrahedron, Octahedron, Icosahedron, Dodecahedron, Torus Knot
- **Edit Mode** — Full vertex, edge, and face selection and manipulation
- **Boolean Operations** — Union, Difference, Intersect
- **Modifiers** — Bevel, Extrude, Inset, Loop Cut, Mirror, Array, Remesh, Skin, Lattice, Shrinkwrap, Bisect, Shear, Subdivision Surface
- **Shading** — Smooth and Flat per-object

### Sculpt Mode

- **6 Brushes** — Draw, Grab, Smooth, Inflate, Pinch, Flatten
- Configurable brush size and strength
- Real-time vertex deformation via raycasting
- Full undo support

### Import / Export

| Format | Import | Export |
|--------|--------|--------|
| OBJ | Yes | Yes |
| STL | Yes | Yes |
| GLTF/GLB | Yes | Yes |
| FBX | Yes | No |
| PLY | Yes | No |

### Viewport

- **4 Shading Modes** — Solid, Wireframe, Material Preview, Rendered
- **Interactive 3D Orientation Globe** — Drag to orbit, click axes to snap camera view
- Hover highlights (blue) and selection outlines (orange)
- Measurement tools — distance and angle
- 3D cursor with grid/vertex/edge/face snapping
- Camera presets — Front, Top, Right, Perspective with smooth animated transitions

### Scene Management

- **Outliner** — Object hierarchy with visibility toggles
- **Properties Panel** — Live position, rotation, scale editing
- **PBR Materials** — Color, metalness, roughness, opacity with presets
- **Snapping System** — Grid, vertex, edge, face, proportional editing

### Animation

- Timeline with frame-based playback and configurable FPS
- FCurve and keyframe interpolation system
- Dope Sheet overview
- Position, rotation, and scale constraints

### Physics

- Rigid Body dynamics
- Cloth simulation
- Soft Body deformation
- Fluid simulation
- Particle system
- Force fields (gravity, wind, turbulence)

<br/>

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                      Lili Modeler                            │
├────────────────────────────┬─────────────────────────────────┤
│   Frontend (TypeScript)    │      Backend (Rust)             │
│                            │                                 │
│   Three.js WebGL Renderer  │   BMesh Half-Edge Structure     │
│   TransformControls        │   Mesh Ops & Modifiers          │
│   Undo/Redo State Snapshots│   Sculpt Deformation Engine     │
│   Outliner & Properties    │   Import/Export Pipeline        │
│   Event System & Tools     │   Path Tracer with BVH          │
│   Timeline UI              │   Animation & Physics Engine     │
│                            │   Node Editor (150+ nodes)      │
├────────────────────────────┴─────────────────────────────────┤
│                    Tauri v2 IPC Bridge                        │
│         dialog · fs · clipboard · shell · window-state       │
└──────────────────────────────────────────────────────────────┘
```

<br/>

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Shell | [Tauri v2](https://v2.tauri.app/) — native window, file dialogs, ~5MB binary |
| Backend | [Rust](https://www.rust-lang.org/) — mesh ops, scene graph, physics, animation |
| Rendering | [Three.js](https://threejs.org/) — WebGL viewport, PBR materials, lighting |
| Language | [TypeScript](https://www.typescriptlang.org/) — type-safe frontend |
| Build | [Vite](https://vitejs.dev/) — instant HMR, optimized bundles |
| Mesh Core | BMesh half-edge — Blender-inspired topological data structure |

<br/>

## Getting Started

### Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| [Rust](https://rustup.rs/) | Latest stable | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| [Node.js](https://nodejs.org/) | v18+ | [Download](https://nodejs.org/) |
| [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/) | — | OS-specific dependencies |

### Quick Start

```bash
git clone https://github.com/Zke-plof/lili-modeler.git
cd lili-modeler
npm install
cargo tauri dev
```

### Production Build

```bash
cargo tauri build
```

Output binary: `src-tauri/target/release/lili-modeler`

<br/>

## Project Structure

```
lili-modeler/
├── src/                          Frontend
│   ├── main.ts                   Core viewport, tools, undo, events (~2900 lines)
│   └── styles.css                Blender-inspired dark theme
│
├── src-tauri/                    Backend
│   ├── src/
│   │   ├── main.rs               Tauri entry point
│   │   ├── engine/
│   │   │   ├── commands.rs       20+ IPC commands
│   │   │   ├── modifiers.rs      Modifier stack
│   │   │   ├── sculpt.rs         20 sculpt brushes
│   │   │   ├── animation.rs      FCurves, Keyframes, Constraints
│   │   │   ├── physics.rs        Rigid body, cloth, fluid, particles
│   │   │   ├── nodes.rs          Node editor (150+ types)
│   │   │   ├── renderer.rs       Path tracer with BVH
│   │   │   └── materials.rs      PBR materials, UV unwrap
│   │   ├── mesh/
│   │   │   ├── bmesh.rs          BMesh half-edge data structure
│   │   │   ├── mod.rs            Mesh operations
│   │   │   └── primitives.rs     12 primitive generators
│   │   ├── io/
│   │   │   ├── importers.rs      OBJ, STL, GLTF importer
│   │   │   └── exporters.rs      OBJ, STL, GLTF exporter
│   │   ├── scene/mod.rs          Scene graph, camera
│   │   └── render/               GPU renderer (WebGPU)
│   ├── capabilities/             Tauri v2 permissions
│   └── tauri.conf.json
│
├── index.html                    UI layout
├── package.json
├── vite.config.ts
└── README.md
```

<br/>

## Keyboard Shortcuts

<details>
<summary><strong>View all 30+ shortcuts</strong></summary>

| Shortcut | Action | Shortcut | Action |
|----------|--------|----------|--------|
| `G` | Move tool | `R` | Rotate tool |
| `S` | Scale tool | `E` | Extrude |
| `I` | Inset faces | `B` | Bevel |
| `K` | Knife cut | `L` | Loop cut |
| `Ctrl+Z` | Undo | `Ctrl+Y` | Redo |
| `Ctrl+C` | Copy | `Ctrl+V` | Paste |
| `Ctrl+D` | Duplicate | `Ctrl+A` | Select all |
| `Shift+A` | Deselect all | `X` | Delete |
| `Tab` | Toggle edit mode | `Z` | Cycle shading modes |
| `1` | Vertex select | `2` | Edge select |
| `3` | Face select | `M` | Measure |
| `Space` | Play/pause timeline | `Shift+D` | Duplicate |
| `Ctrl+B` | Bevel tool | `Ctrl+R` | Loop cut tool |
| `Numpad 1` | Front view | `Numpad 3` | Right view |
| `Numpad 7` | Top view | `Numpad 5` | Toggle perspective |

</details>

<br/>

## Contributing

Contributions are welcome!

1. **Fork** the repository
2. **Create** a feature branch → `git checkout -b feature/amazing-feature`
3. **Commit** → `git commit -m 'Add amazing feature'`
4. **Push** → `git push origin feature/amazing-feature`
5. **Open** a Pull Request

### Before submitting

- **Rust**: `cargo check` must pass with zero errors
- **TypeScript**: `npx tsc --noEmit` must pass
- Follow existing code patterns — check neighboring files first

<br/>

## Roadmap

- [ ] GPU-accelerated viewport rendering
- [ ] Node-based material editor
- [ ] UV editing tools
- [ ] Armature / skeleton system
- [ ] Particle system UI
- [ ] Plugin / extension system
- [ ] Collaborative editing
- [ ] Python scripting API

<br/>

## License

MIT — do whatever you want with it.

<br/>

---

<div align="center">

**Built for engineering students who deserve better tools.**

</div>
