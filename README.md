<p align="center">
  <img src="src-tauri/icons/icon.ico" alt="Lili Modeler" width="100"/>
</p>

<h1 align="center">Lili Modeler</h1>

<p align="center">
  <strong>A lightweight, cross-platform 3D modeling application for engineering students.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue" alt="Version"/>
  <img src="https://img.shields.io/badge/rust-2021-orange" alt="Rust Edition"/>
  <img src="https://img.shields.io/badge/tauri-v2-green" alt="Tauri v2"/>
  <img src="https://img.shields.io/badge/three.js-0.168-black" alt="Three.js"/>
  <img src="https://img.shields.io/badge/license-MIT-yellow" alt="License"/>
</p>

<p align="center">
  Built with <a href="https://v2.tauri.app/">Tauri v2</a> + <a href="https://www.rust-lang.org/">Rust</a> + <a href="https://threejs.org/">Three.js</a>
</p>

---

## Overview

Lili Modeler is a desktop 3D modeling application that aims to bring Blender-level functionality in a lightweight, fast package tailored for engineering students. It combines a high-performance Rust backend with a Three.js-powered viewport, all wrapped in a native desktop shell via Tauri v2.

**Why?** Existing 3D tools are either too heavy (Blender at 200MB+), too simple (online viewers), or too expensive (Fusion 360). Lili Modeler fills the gap — fast startup, small binary size (~5MB), and the core features engineers actually need.

---

## Key Features

<table>
<tr>
<td width="50%">

### Modeling & Editing
- **12 Primitive Types** — Cube, Sphere, Cylinder, Torus, Plane, Cone, Monkey, Tetrahedron, Octahedron, Icosahedron, Dodecahedron, Torus Knot
- **Edit Mode** — Vertex, Edge, and Face selection with click and marquee
- **Boolean Operations** — Union, Difference, Intersect
- **Modifiers** — Bevel, Extrude, Inset, Loop Cut, Remesh, Skin, Lattice, Shrinkwrap, Bisect, Shear
- **Shading** — Smooth and Flat per-object

</td>
<td width="50%">

### Sculpt Mode
- **6 Brushes** — Draw, Grab, Smooth, Inflate, Pinch, Flatten
- Configurable brush **size** and **strength**
- Real-time vertex deformation via raycasting
- Full undo support on sculpt operations

</td>
</tr>
<tr>
<td>

### Import / Export
- **Import:** OBJ, STL, GLTF/GLB, FBX, PLY
- **Export:** OBJ, STL, GLTF/GLB
- Native OS file dialogs
- Automatic geometry reconstruction from backend mesh data

</td>
<td>

### Viewport
- **4 Shading Modes** — Solid, Wireframe, Material Preview, Rendered
- **Interactive Orientation Globe** — Drag to orbit, click axes to snap camera
- Hover highlights (blue) and selection outlines (orange)
- Measurement tools (distance + angle)
- 3D cursor with snapping

</td>
</tr>
<tr>
<td>

### Scene Management
- **Outliner** with object visibility toggles
- **Properties Panel** — Live position, rotation, scale editing
- **Material System** — PBR materials with presets, metalness, roughness, opacity
- **Snapping** — Grid, vertex, edge, face

</td>
<td>

### Animation & Physics
- **Timeline** with frame-based playback
- **Keyframe System** — FCurves, interpolation, Dope Sheet
- **Constraints** — Position, Rotation, Scale
- **Physics** — Rigid Body, Cloth, Soft Body, Fluid, Particles, Force Fields

</td>
</tr>
</table>

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Lili Modeler                      │
├──────────────────────┬──────────────────────────────┤
│   Frontend (TS)      │      Backend (Rust)          │
│                      │                              │
│   Three.js Renderer  │  BMesh Half-Edge Structure   │
│   TransformControls  │  Mesh Operations & Modifiers │
│   Undo/Redo (State)  │  Sculpt Deformation Engine   │
│   Outliner / Props   │  Import/Export Pipeline       │
│   Event System       │  Path Tracer (BVH)           │
│   Timeline UI        │  Animation & Physics Engine   │
│                      │  Node Editor (150+ nodes)     │
├──────────────────────┴──────────────────────────────┤
│                  Tauri v2 Bridge                     │
│          IPC Commands  │  Plugin System              │
├─────────────────────────────────────────────────────┤
│   Plugins: dialog, fs, clipboard, shell, window-state│
└─────────────────────────────────────────────────────┘
```

---

## Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Desktop Shell | [Tauri v2](https://v2.tauri.app/) | Native window, file dialogs, permissions |
| Backend | [Rust](https://www.rust-lang.org/) | Mesh operations, scene graph, import/export |
| Rendering | [Three.js](https://threejs.org/) | WebGL viewport, materials, lighting |
| Language | [TypeScript](https://www.typescriptlang.org/) | Type-safe frontend logic |
| Build | [Vite](https://vitejs.dev/) | Fast HMR and bundling |
| Mesh System | BMesh (half-edge) | Topological mesh data structure |

---

## Getting Started

### Prerequisites

| Requirement | Version | Install |
|------------|---------|---------|
| [Rust](https://rustup.rs/) | Latest stable | `rustup update` |
| [Node.js](https://nodejs.org/) | v18+ | [Download](https://nodejs.org/) |
| [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/) | — | See link for OS-specific deps |

### Installation

```bash
# Clone the repository
git clone https://github.com/Zke-plof/lili-modeler.git
cd lili-modeler

# Install frontend dependencies
npm install

# Start development server
cargo tauri dev
```

### Production Build

```bash
cargo tauri build
```

The output binary will be in `src-tauri/target/release/`.

---

## Project Structure

```
lili-modeler/
├── src/                          # Frontend
│   ├── main.ts                   # Core: viewport, tools, undo, events (~2900 lines)
│   └── styles.css                # Blender-inspired dark theme
├── src-tauri/                    # Backend
│   ├── src/
│   │   ├── main.rs               # Tauri entry point
│   │   ├── engine/
│   │   │   ├── commands.rs       # 20+ IPC commands (primitives, booleans, import...)
│   │   │   ├── modifiers.rs      # Modifier stack (bevel, mirror, array, subdivision)
│   │   │   ├── sculpt.rs         # 20 sculpt brushes
│   │   │   ├── animation.rs      # FCurves, Keyframes, Constraints, Dope Sheet
│   │   │   ├── physics.rs        # Rigid body, cloth, soft body, fluid, particles
│   │   │   ├── nodes.rs          # Node editor (150+ node types)
│   │   │   ├── renderer.rs       # Path tracer with BVH acceleration
│   │   │   └── materials.rs      # PBR materials, UV unwrap
│   │   ├── mesh/
│   │   │   ├── bmesh.rs          # BMesh half-edge data structure
│   │   │   ├── mod.rs            # Mesh operations (extrude, bevel, merge)
│   │   │   └── primitives.rs     # 12 primitive generators
│   │   ├── io/
│   │   │   ├── importers.rs      # OBJ, STL, GLTF/GLB import
│   │   │   └── exporters.rs      # OBJ, STL, GLTF/GLB export
│   │   ├── scene/mod.rs          # Scene graph, camera, SceneObject
│   │   └── render/               # GPU renderer (WebGPU/WGSL)
│   ├── capabilities/             # Tauri v2 permission grants
│   └── tauri.conf.json
├── index.html                    # UI layout (outliner, properties, toolbar, timeline)
├── package.json
├── vite.config.ts
└── README.md
```

---

## Keyboard Shortcuts

| Shortcut | Action | Shortcut | Action |
|----------|--------|----------|--------|
| `G` | Move tool | `R` | Rotate tool |
| `S` | Scale tool | `E` | Extrude |
| `I` | Inset faces | `B` | Bevel |
| `Ctrl+Z` | Undo | `Ctrl+Y` | Redo |
| `Ctrl+C` | Copy | `Ctrl+V` | Paste |
| `Ctrl+D` | Duplicate | `Ctrl+A` | Select all |
| `X` | Delete | `Tab` | Toggle edit mode |
| `1` | Vertex select | `2` | Edge select |
| `3` | Face select | `Z` | Cycle shading modes |
| `Numpad 1` | Front view | `Numpad 3` | Right view |
| `Numpad 7` | Top view | `Numpad 5` | Toggle perspective |
| `M` | Measure | `Space` | Play/pause timeline |

---

## Contributing

Contributions are welcome! Here's how to get started:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines

- **Rust code**: Run `cargo check` before committing (zero errors required, warnings are fine)
- **TypeScript code**: Run `npx tsc --noEmit` to verify types
- **Commit messages**: Use descriptive, imperative-style messages
- **Code style**: Follow existing patterns — check neighboring files before adding new ones

---

## Roadmap

- [ ] GPU-accelerated viewport rendering
- [ ] Full node-based material editor
- [ ] UV editing tools
- [ ] Armature / skeleton system
- [ ] Particle system UI
- [ ] Plugin/extension system
- [ ] Collaborative editing (real-time)
- [ ] Python scripting API

---

## License

Distributed under the **MIT License**. See `LICENSE` for details.

---

<p align="center">
  Built with passion for engineering students everywhere.
</p>
