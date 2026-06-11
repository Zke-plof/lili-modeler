# Lili Modeler

A lightweight 3D modeling desktop application built for engineering students. Powered by Tauri v2, Rust, and Three.js.

## Features

### Modeling
- **Primitive Shapes** — Cube, Sphere, Cylinder, Torus, Plane, Cone, Monkey, Tetrahedron, Octahedron, Icosahedron, Dodecahedron, Torus Knot
- **Edit Mode** — Vertex, Edge, and Face selection and manipulation
- **Boolean Operations** — Union, Difference, Intersect
- **Bevel Modifier** — Edge beveling with configurable segments
- **Extrude / Inset** — Face extrusion and inset operations
- **Loop Cut** — Subdivide geometry with loop cuts
- **Shade Smooth / Flat** — Per-object shading control

### Sculpt Mode
- **Brushes** — Draw, Grab, Smooth, Inflate, Pinch, Flatten
- **Configurable** — Brush size and strength sliders
- Real-time mesh deformation via raycasting

### Import / Export
- **Import** — OBJ, STL, GLTF/GLB, FBX, PLY
- **Export** — OBJ, STL, GLTF/GLB
- Native OS file dialogs via Tauri dialog plugin

### Viewport
- **Shading Modes** — Solid, Wireframe, Material Preview, Rendered
- **Orientation Gizmo** — Interactive 3D globe for camera navigation (drag to orbit, click axes to snap)
- **Hover & Selection Highlights** — Blue hover outline, orange selection outline
- **Measurement Tools** — Distance and angle measurement
- **3D Cursor** — Precise placement with snapping
- **Camera Views** — Front, Top, Right, Perspective with animated transitions

### Scene Management
- **Outliner** — Hierarchical object list
- **Properties Panel** — Position, rotation, scale with live updates
- **Material System** — PBR materials with metalness, roughness, opacity, and color presets
- **Snapping** — Grid, vertex, edge, face, and proportional editing

### Animation
- **Timeline** — Frame-based playback with configurable FPS
- **Keyframe System** — FCurves and keyframe interpolation
- **Constraints** — Position, rotation, scale constraints
- **Dope Sheet** — Keyframe overview and editing

### Physics
- Rigid Body, Cloth, Soft Body, Fluid, Particles, Force Fields

### Other
- **Undo / Redo** — Full state snapshot undo with 50-level stack (Ctrl+Z / Ctrl+Y)
- **Keyboard Shortcuts** — 30+ shortcuts for快速 workflow
- **Blender-inspired UI** — Dark theme with neutral grays and accent colors

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Runtime | [Tauri v2](https://v2.tauri.app/) |
| Backend | Rust (BMesh half-edge data structure, path tracer with BVH) |
| Frontend Rendering | [Three.js](https://threejs.org/) |
| UI Framework | Vanilla TypeScript + HTML/CSS |
| Build Tool | [Vite](https://vitejs.dev/) |

## Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v18+)
- [Tauri v2 Prerequisites](https://v2.tauri.app/start/prerequisites/)

### Install & Run

```bash
# Clone the repo
git clone https://github.com/your-username/lili-modeler.git
cd lili-modeler

# Install frontend dependencies
npm install

# Run in development mode
cargo tauri dev
```

### Build for Production

```bash
cargo tauri build
```

## Project Structure

```
lili-modeler/
├── src/                    # Frontend (TypeScript)
│   ├── main.ts            # Viewport, tools, undo/redo, event handling
│   └── styles.css         # Blender-inspired dark theme
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── main.rs        # Tauri entry point
│   │   ├── engine/        # Commands, modifiers, physics, animation, sculpting
│   │   ├── mesh/          # BMesh half-edge, primitives
│   │   ├── io/            # Import/export (OBJ, STL, GLTF)
│   │   ├── scene/         # Scene graph, camera
│   │   └── render/        # Path tracer with BVH
│   ├── capabilities/      # Tauri v2 permissions
│   └── tauri.conf.json
├── index.html
├── package.json
└── vite.config.ts
```

## License

MIT
