import './styles.css';
import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { TransformControls } from 'three/addons/controls/TransformControls.js';

// ═══════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════

interface SceneObjectData {
  id: string;
  name: string;
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  vertex_count: number;
  face_count: number;
  visible: boolean;
}

interface MeshInfo {
  vertex_count: number;
  edge_count: number;
  face_count: number;
  triangle_count: number;
  volume: number;
  surface_area: number;
}

interface GizmoVertex { position: THREE.Vector3; index: number; }
interface GizmoEdge { v0: number; v1: number; }
interface GizmoFace { indices: number[]; center: THREE.Vector3; }

type SelectMode = 'Object' | 'Vertex' | 'Edge' | 'Face';
type ToolMode = 'cursor' | 'move' | 'rotate' | 'scale' | 'extrude' | 'inset' | 'bevel' | 'loopcut' | 'measure' | 'knife' | 'spin' | 'screw' | 'lattice' | 'skin' | 'remesh' | 'convex' | 'bisect' | 'shear' | 'shrinkwrap';

// ═══════════════════════════════════════════════════════════════════
// App State
// ═══════════════════════════════════════════════════════════════════

const state = {
  mode: 'object' as 'object' | 'edit' | 'sculpt' | 'weight_paint' | 'texture_paint' | 'vertex_paint',
  selectMode: 'Object' as SelectMode,
  tool: 'cursor' as ToolMode,
  selectedObjects: new Set<string>(),
  selectedVertices: new Set<number>(),
  selectedEdges: new Set<number>(),
  selectedFaces: new Set<number>(),
  activeObject: null as string | null,
  objects: new Map<string, SceneObjectData>(),
  threeObjects: new Map<string, THREE.Object3D>(),
  editMeshes: new Map<string, THREE.BufferGeometry>(),
  threeObjectsInverse: new Map<THREE.Object3D, string>(),
  measurePoints: [] as THREE.Vector3[],
  measureMode: 'distance' as 'distance' | 'angle' | 'area',
  historyIndex: -1,
  history: [] as string[],
  clipboard: null as THREE.BufferGeometry | null,
  pivotPoint: 'median' as 'median' | 'individual' | 'center' | 'cursor' | 'active',
  orientation: 'global' as 'global' | 'local' | 'normal',
  snapping: { enabled: false, type: 'increment' as string, size: 0.1 },
  proportional: { enabled: false, radius: 1.0, falloff: 'smooth' as string },
  overlay: { wireframe: false, faceOrientation: false, normals: false, statistics: false },
  cursor3d: new THREE.Vector3(),
  lastMousePos: new THREE.Vector2(),
  isDragging: false,
  dragStartPos: new THREE.Vector3(),
  frameCount: 0,
  lastFpsTime: performance.now(),
  lastTickTime: performance.now(),
  fps: 0,
  sculpt: { brush: 'draw' as string, size: 0.5, strength: 0.5 },
  timeline: { playing: false, frame: 1, startFrame: 1, endFrame: 250, fps: 24, keyframes: new Map<string, number[]>() },
  physics: { enabled: false, type: 'active' as 'active' | 'passive', mass: 1.0, friction: 0.5, bounciness: 0.3, shape: 'box' as string },
  contextMenuVisible: false,
  hoverObject: null as string | null,
  viewportShading: 'solid' as 'solid' | 'wireframe' | 'material' | 'rendered',
  lastSculptPoint: null as THREE.Vector3 | null,
  isSculpting: false,
};

// ═══════════════════════════════════════════════════════════════════
// Three.js Setup
// ═══════════════════════════════════════════════════════════════════

const canvas = document.getElementById('viewport') as HTMLCanvasElement;
const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: false, powerPreference: 'high-performance' });
renderer.setPixelRatio(window.devicePixelRatio);
renderer.setClearColor(0x1a1a2e);
renderer.shadowMap.enabled = true;
renderer.shadowMap.type = THREE.PCFSoftShadowMap;
renderer.toneMapping = THREE.ACESFilmicToneMapping;
renderer.toneMappingExposure = 1.2;

const scene = new THREE.Scene();
scene.background = new THREE.Color(0x1a1a2e);
scene.fog = new THREE.Fog(0x1a1a2e, 50, 200);

const camera = new THREE.PerspectiveCamera(45, 1, 0.01, 2000);
camera.position.set(5, 4, 6);
camera.lookAt(0, 0, 0);

const controls = new OrbitControls(camera, canvas);
controls.enableDamping = true;
controls.dampingFactor = 0.08;
controls.screenSpacePanning = true;
controls.minDistance = 0.05;
controls.maxDistance = 1000;
controls.rotateSpeed = 0.8;
controls.panSpeed = 0.6;
controls.zoomSpeed = 1.2;
controls.target.set(0, 0, 0);

function updatePropertiesTransform() {
  if (transformControls.object) {
    const obj3d = transformControls.object;
    const id = state.threeObjectsInverse.get(obj3d);
    if (!id) return;
    const obj = state.objects.get(id);
    if (!obj) return;
    obj.position = [obj3d.position.x, obj3d.position.y, obj3d.position.z];
    obj.rotation = [
      obj3d.quaternion.x,
      obj3d.quaternion.y,
      obj3d.quaternion.z,
      obj3d.quaternion.w,
    ];
    obj.scale = [obj3d.scale.x, obj3d.scale.y, obj3d.scale.z];
    updateProperties();
  }
}

const transformControls = new TransformControls(camera, canvas);
transformControls.setMode('translate');
transformControls.setSize(0.75);
transformControls.addEventListener('dragging-changed', (event) => {
  controls.enabled = !event.value;
  if (event.value) {
    pushUndo();
    state.isDragging = true;
    state.dragStartPos.copy(transformControls.object?.position || new THREE.Vector3());
  } else {
    state.isDragging = false;
    if (transformControls.object) {
      const obj = state.threeObjectsInverse.get(transformControls.object);
      if (obj) syncTransformToBackend(obj);
    }
  }
});
transformControls.addEventListener('objectChange', () => {
  updateGizmoOverlay();
  updatePropertiesTransform();
});
scene.add(transformControls);

// ═══════════════════════════════════════════════════════════════════
// Reverse lookup map
// ═══════════════════════════════════════════════════════════════════

const threeObjectsInverse = state.threeObjectsInverse;

// Lighting setup - Blender-like HDRI approximation
const ambientLight = new THREE.AmbientLight(0x404060, 0.4);
scene.add(ambientLight);

const keyLight = new THREE.DirectionalLight(0xfff4e0, 1.8);
keyLight.position.set(5, 8, 6);
keyLight.castShadow = true;
keyLight.shadow.mapSize.set(4096, 4096);
keyLight.shadow.camera.near = 0.1;
keyLight.shadow.camera.far = 100;
keyLight.shadow.camera.left = -10;
keyLight.shadow.camera.right = 10;
keyLight.shadow.camera.top = 10;
keyLight.shadow.camera.bottom = -10;
keyLight.shadow.bias = -0.0002;
scene.add(keyLight);

const fillLight = new THREE.DirectionalLight(0xc0d0ff, 0.6);
fillLight.position.set(-4, 3, -3);
scene.add(fillLight);

const rimLight = new THREE.DirectionalLight(0xffffff, 0.4);
rimLight.position.set(0, 5, -8);
scene.add(rimLight);

const hemisphereLight = new THREE.HemisphereLight(0x87ceeb, 0x362d2d, 0.3);
scene.add(hemisphereLight);

// Grid
const gridHelper = new THREE.GridHelper(50, 50, 0x555577, 0x333355);
gridHelper.material.opacity = 0.6;
gridHelper.material.transparent = true;
scene.add(gridHelper);

// Sub-grid (finer)
const subGrid = new THREE.GridHelper(50, 500, 0x2a2a44, 0x222240);
subGrid.material.opacity = 0.3;
subGrid.material.transparent = true;
subGrid.position.y = 0.001;
scene.add(subGrid);

// Axis lines
const axisLength = 50;
const axisGroup = new THREE.Group();
const xAxisGeo = new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(-axisLength, 0, 0), new THREE.Vector3(axisLength, 0, 0)]);
const yAxisGeo = new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(0, -axisLength, 0), new THREE.Vector3(0, axisLength, 0)]);
const zAxisGeo = new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(0, 0, -axisLength), new THREE.Vector3(0, 0, axisLength)]);
axisGroup.add(new THREE.Line(xAxisGeo, new THREE.LineBasicMaterial({ color: 0x883333, transparent: true, opacity: 0.3 })));
axisGroup.add(new THREE.Line(yAxisGeo, new THREE.LineBasicMaterial({ color: 0x338833, transparent: true, opacity: 0.3 })));
axisGroup.add(new THREE.Line(zAxisGeo, new THREE.LineBasicMaterial({ color: 0x333388, transparent: true, opacity: 0.3 })));
scene.add(axisGroup);

// 3D Cursor
const cursorGroup = new THREE.Group();
const cursorRing = new THREE.RingGeometry(0.06, 0.08, 32);
const cursorDot = new THREE.CircleGeometry(0.03, 16);
const cursorRingMat = new THREE.MeshBasicMaterial({ color: 0xff4444, side: THREE.DoubleSide, transparent: true, opacity: 0.8 });
const cursorDotMat = new THREE.MeshBasicMaterial({ color: 0xff4444, side: THREE.DoubleSide });
const cursorRingMesh = new THREE.Mesh(cursorRing, cursorRingMat);
const cursorDotMesh = new THREE.Mesh(cursorDot, cursorDotMat);
cursorGroup.add(cursorRingMesh);
cursorGroup.add(cursorDotMesh);
cursorGroup.rotation.x = -Math.PI / 2;
cursorGroup.position.y = 0.01;
scene.add(cursorGroup);

// ═══════════════════════════════════════════════════════════════════
// Orientation Gizmo (interactive 3D globe)
// ═══════════════════════════════════════════════════════════════════

const gizmoCanvas = document.getElementById('orientation-gizmo') as HTMLCanvasElement;
const gizmoSize = 130;
gizmoCanvas.width = gizmoSize * window.devicePixelRatio;
gizmoCanvas.height = gizmoSize * window.devicePixelRatio;
const gizmoRenderer = new THREE.WebGLRenderer({ canvas: gizmoCanvas, antialias: true, alpha: true });
gizmoRenderer.setPixelRatio(window.devicePixelRatio);
gizmoRenderer.setSize(gizmoSize, gizmoSize);

const gizmoScene = new THREE.Scene();
const gizmoCamera = new THREE.PerspectiveCamera(50, 1, 0.1, 100);
gizmoCamera.position.set(0, 0, 3.2);

const gizmoGroup = new THREE.Group();
gizmoScene.add(gizmoGroup);

const gizmoAxisLength = 1.0;
const gizmoLabels: { sprite: THREE.Sprite; axis: THREE.Vector3; color: string; label: string }[] = [];

function createGizmoAxis(dir: THREE.Vector3, color: number, label: string, labelColor: string) {
  const geo = new THREE.BufferGeometry().setFromPoints([
    new THREE.Vector3(0, 0, 0),
    dir.clone().multiplyScalar(gizmoAxisLength)
  ]);
  const mat = new THREE.LineBasicMaterial({ color, linewidth: 2 });
  const line = new THREE.Line(geo, mat);
  gizmoGroup.add(line);

  const tipGeo = new THREE.SphereGeometry(0.06, 12, 12);
  const tipMat = new THREE.MeshBasicMaterial({ color });
  const tip = new THREE.Mesh(tipGeo, tipMat);
  tip.position.copy(dir.clone().multiplyScalar(gizmoAxisLength));
  gizmoGroup.add(tip);

  const canvas2d = document.createElement('canvas');
  canvas2d.width = 64;
  canvas2d.height = 64;
  const ctx = canvas2d.getContext('2d')!;
  ctx.fillStyle = labelColor;
  ctx.font = 'bold 44px Inter, sans-serif';
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.fillText(label, 32, 32);
  const texture = new THREE.CanvasTexture(canvas2d);
  const spriteMat = new THREE.SpriteMaterial({ map: texture, transparent: true, depthTest: false });
  const sprite = new THREE.Sprite(spriteMat);
  sprite.scale.set(0.3, 0.3, 1);
  sprite.position.copy(dir.clone().multiplyScalar(gizmoAxisLength + 0.32));
  gizmoGroup.add(sprite);

  gizmoLabels.push({ sprite, axis: dir.clone(), color: labelColor, label });
}

createGizmoAxis(new THREE.Vector3(1, 0, 0), 0xff4444, 'X', '#ff4444');
createGizmoAxis(new THREE.Vector3(0, 1, 0), 0x44ff44, 'Y', '#44ff44');
createGizmoAxis(new THREE.Vector3(0, 0, 1), 0x4488ff, 'Z', '#4488ff');

// Wireframe sphere behind axes
const sphereGeo = new THREE.SphereGeometry(0.85, 20, 20);
const sphereWire = new THREE.WireframeGeometry(sphereGeo);
const sphereLines = new THREE.LineSegments(sphereWire, new THREE.LineBasicMaterial({ color: 0x444444, transparent: true, opacity: 0.25 }));
gizmoGroup.add(sphereLines);

// Center sphere
const centerGeo = new THREE.SphereGeometry(0.08, 16, 16);
const centerMat = new THREE.MeshBasicMaterial({ color: 0x888888 });
const centerSphere = new THREE.Mesh(centerGeo, centerMat);
gizmoGroup.add(centerSphere);

// Hover highlight
const hoverSphere = new THREE.Mesh(
  new THREE.SphereGeometry(0.12, 16, 16),
  new THREE.MeshBasicMaterial({ color: 0xffffff, transparent: true, opacity: 0 })
);
gizmoGroup.add(hoverSphere);

function updateOrientationGizmo() {
  const mainDir = new THREE.Vector3();
  camera.getWorldDirection(mainDir);
  gizmoCamera.position.copy(mainDir.clone().negate().multiplyScalar(3.2));
  gizmoCamera.lookAt(0, 0, 0);

  gizmoLabels.forEach(({ sprite, axis }) => {
    const worldPos = axis.clone();
    const camDir = gizmoCamera.position.clone().normalize();
    const dot = worldPos.dot(camDir);
    sprite.material.opacity = dot < 0 ? 1.0 : 0.25;
  });

  gizmoRenderer.render(gizmoScene, gizmoCamera);
}

// --- Click: snap camera to axis ---
gizmoCanvas.addEventListener('click', (e) => {
  if (gizmoWasDragged) { gizmoWasDragged = false; return; }
  const rect = gizmoCanvas.getBoundingClientRect();
  const cx = (e.clientX - rect.left) / rect.width * 2 - 1;
  const cy = -((e.clientY - rect.top) / rect.height) * 2 + 1;

  const hitRadius = 0.35;
  const axisHits: { axis: string; dist: number }[] = [];

  gizmoLabels.forEach(({ sprite, axis }) => {
    const screenPos = axis.clone().project(gizmoCamera);
    const dx = screenPos.x - cx;
    const dy = screenPos.y - cy;
    const dist = Math.sqrt(dx * dx + dy * dy);
    if (dist < hitRadius) {
      const axisName = axis.x > 0.5 ? 'x' : axis.y > 0.5 ? 'y' : 'z';
      axisHits.push({ axis: axisName, dist });
    }
  });

  if (axisHits.length > 0) {
    axisHits.sort((a, b) => a.dist - b.dist);
    const hit = axisHits[0];
    const dist = camera.position.distanceTo(controls.target);
    const targetPos = controls.target.clone();

    let camPos: THREE.Vector3;
    switch (hit.axis) {
      case 'x': camPos = targetPos.clone().add(new THREE.Vector3(dist, 0, 0)); break;
      case 'y': camPos = targetPos.clone().add(new THREE.Vector3(0, dist, 0)); break;
      case 'z': camPos = targetPos.clone().add(new THREE.Vector3(0, 0, dist)); break;
      default: return;
    }
    animateCamera(camPos, targetPos);
  }
});

// --- Drag: rotate camera orbit around target ---
let gizmoDragging = false;
let gizmoWasDragged = false;
let gizmoLastMouse = { x: 0, y: 0 };

gizmoCanvas.addEventListener('mousedown', (e) => {
  gizmoDragging = true;
  gizmoWasDragged = false;
  gizmoLastMouse = { x: e.clientX, y: e.clientY };
  e.preventDefault();
});

window.addEventListener('mousemove', (e) => {
  if (!gizmoDragging) return;
  const dx = e.clientX - gizmoLastMouse.x;
  const dy = e.clientY - gizmoLastMouse.y;
  gizmoLastMouse = { x: e.clientX, y: e.clientY };

  if (Math.abs(dx) > 1 || Math.abs(dy) > 1) gizmoWasDragged = true;

  const spherical = new THREE.Spherical();
  spherical.setFromVector3(camera.position.clone().sub(controls.target));
  spherical.theta -= dx * 0.01;
  spherical.phi -= dy * 0.01;
  spherical.phi = Math.max(0.05, Math.min(Math.PI - 0.05, spherical.phi));

  const newPos = new THREE.Vector3().setFromSpherical(spherical).add(controls.target);
  camera.position.copy(newPos);
  camera.lookAt(controls.target);
  controls.update();
  updateOrientationGizmo();
});

window.addEventListener('mouseup', () => {
  gizmoDragging = false;
});

// ═══════════════════════════════════════════════════════════════════
// Materials
// ═══════════════════════════════════════════════════════════════════

const materialPresets: Record<string, THREE.MeshStandardMaterial> = {
  default: new THREE.MeshStandardMaterial({ color: 0x888899, metalness: 0.1, roughness: 0.6 }),
  selected: new THREE.MeshStandardMaterial({ color: 0x00d2ff, metalness: 0.2, roughness: 0.4, emissive: 0x003344, emissiveIntensity: 0.3 }),
  edit: new THREE.MeshStandardMaterial({ color: 0x7777aa, metalness: 0.1, roughness: 0.6, transparent: true, opacity: 0.55, side: THREE.DoubleSide }),
  editSelected: new THREE.MeshStandardMaterial({ color: 0xff6b35, metalness: 0.2, roughness: 0.4, transparent: true, opacity: 0.7, side: THREE.DoubleSide }),
  wireframe: new THREE.MeshStandardMaterial({ color: 0x888899, wireframe: true }),
  metal: new THREE.MeshStandardMaterial({ color: 0xcccccc, metalness: 0.9, roughness: 0.15 }),
  plastic: new THREE.MeshStandardMaterial({ color: 0x4488cc, metalness: 0.0, roughness: 0.4 }),
  glass: new THREE.MeshStandardMaterial({ color: 0xeeeeff, metalness: 0.0, roughness: 0.0, transparent: true, opacity: 0.15 }),
  rubber: new THREE.MeshStandardMaterial({ color: 0x222222, metalness: 0.0, roughness: 0.95 }),
  emissive: new THREE.MeshStandardMaterial({ color: 0x444444, emissive: 0xff6600, emissiveIntensity: 2.0 }),
  wood: new THREE.MeshStandardMaterial({ color: 0x8B6914, roughness: 0.8, metalness: 0.0 }),
  marble: new THREE.MeshStandardMaterial({ color: 0xf0f0f0, roughness: 0.3, metalness: 0.0 }),
  stone: new THREE.MeshStandardMaterial({ color: 0x808080, roughness: 0.9, metalness: 0.0 }),
  ceramic: new THREE.MeshStandardMaterial({ color: 0xe8e0d0, roughness: 0.2, metalness: 0.0 }),
  gold: new THREE.MeshStandardMaterial({ color: 0xffd700, roughness: 0.15, metalness: 0.95 }),
  copper: new THREE.MeshStandardMaterial({ color: 0xb87333, roughness: 0.2, metalness: 0.9 }),
  aluminium: new THREE.MeshStandardMaterial({ color: 0xd0d0d0, roughness: 0.2, metalness: 0.85 }),
  carbon: new THREE.MeshStandardMaterial({ color: 0x222222, roughness: 0.5, metalness: 0.3 }),
};

const vertexMaterial = new THREE.PointsMaterial({ color: 0xff6b35, size: 0.06, sizeAttenuation: true, depthTest: true });
const edgeMaterial = new THREE.LineBasicMaterial({ color: 0xffaa00, linewidth: 1.5 });
const faceCenterMaterial = new THREE.PointsMaterial({ color: 0x44aaff, size: 0.04, sizeAttenuation: true });

// ═══════════════════════════════════════════════════════════════════
// Edit Mode Overlay Objects
// ═══════════════════════════════════════════════════════════════════

const editOverlay = new THREE.Group();
scene.add(editOverlay);

// ═══════════════════════════════════════════════════════════════════
// Measurement overlay
// ═══════════════════════════════════════════════════════════════════

const measureOverlay = new THREE.Group();
scene.add(measureOverlay);

const measurePointsObj = new THREE.Group();
measureOverlay.add(measurePointsObj);

const measureLinesObj = new THREE.Group();
measureOverlay.add(measureLinesObj);

const measureLabelsObj = new THREE.Group();
measureOverlay.add(measureLabelsObj);

// ═══════════════════════════════════════════════════════════════════
// Proportional editing falloff visualization
// ═══════════════════════════════════════════════════════════════════

const proportionalRing = new THREE.RingGeometry(1, 1.02, 64);
const proportionalMat = new THREE.MeshBasicMaterial({ color: 0xffffff, side: THREE.DoubleSide, transparent: true, opacity: 0.3 });
const proportionalMesh = new THREE.Mesh(proportionalRing, proportionalMat);
proportionalMesh.visible = false;
proportionalMesh.rotation.x = -Math.PI / 2;
scene.add(proportionalMesh);

// ═══════════════════════════════════════════════════════════════════
// Command wrappers
// ═══════════════════════════════════════════════════════════════════

async function createPrimitive(type: string, name: string, segments = 32, size = 1.0): Promise<string> {
  return await invoke('create_primitive', {
    args: { name, primitive_type: type, segments, size }
  });
}

async function deleteObject(id: string): Promise<void> {
  await invoke('delete_object', { objectId: id });
}

async function transformObject(id: string, pos: [number, number, number], rot: [number, number, number, number], scale: [number, number, number]): Promise<void> {
  await invoke('transform_object', {
    args: { objectId: id, position: pos, rotation: rot, scale }
  });
}

async function getSceneData(): Promise<{ objects: SceneObjectData[] }> {
  return await invoke('get_scene_data');
}

async function getMeshInfo(id: string): Promise<MeshInfo> {
  return await invoke('get_mesh_info', { objectId: id });
}

async function getMeshData(objectId: string): Promise<{ id: string, name: string, positions: number[], indices: number[], normals: number[], vertex_count: number, face_count: number }> {
  return await invoke('get_mesh_data', { objectId });
}

async function extrudeSelection(distance: number, faceIds: number[]): Promise<void> {
  await invoke('extrude_selection', { args: { distance, face_ids: faceIds } });
}

async function insetFaces(thickness: number, faceIds: number[]): Promise<void> {
  await invoke('inset_faces', { args: { thickness, face_ids: faceIds } });
}

async function bevelEdges(segments: number, edgeIds: number[]): Promise<void> {
  await invoke('bevel_edges', { args: { segments, edge_ids: edgeIds } });
}

async function measureDistance(a: [number, number, number], b: [number, number, number]): Promise<number> {
  return await invoke('measure_distance', { args: { point_a: a, point_b: b } });
}

async function measureAngle(a: [[number, number, number], [number, number, number], [number, number, number]]): Promise<number> {
  return await invoke('measure_angle', { args: [{ point_a: a[0], point_b: a[1] }, { point_a: a[1], point_b: a[2] }] });
}

async function importMesh(filePath: string): Promise<string[]> {
  return await invoke('import_mesh', { args: { file_path: filePath, options: { scale: 1.0, flip_normals: false, flip_uv: false, center_origin: true } } });
}

async function exportMesh(filePath: string, objectIds: string[]): Promise<void> {
  await invoke('export_mesh', { args: { file_path: filePath, object_ids: objectIds, options: { scale: 1.0, flip_normals: false, flip_uv: false, include_normals: true, include_uv: true, binary: false } } });
}

async function openFileDialog() {
  const selected = await open({
    multiple: false,
    filters: [{
      name: '3D Models',
      extensions: ['obj', 'stl', 'gltf', 'glb', 'fbx', 'ply']
    }]
  });
  if (Array.isArray(selected)) return selected[0] ?? null;
  return selected;
}

async function saveFileDialog(defaultName: string = 'model') {
  const filePath = await save({
    defaultPath: defaultName,
    filters: [{
      name: '3D Models',
      extensions: ['obj', 'stl', 'gltf', 'glb']
    }]
  });
  return filePath;
}

// ═══════════════════════════════════════════════════════════════════
// Sync Three.js <-> Backend
// ═══════════════════════════════════════════════════════════════════

async function syncTransformToBackend(id: string) {
  const threeObj = state.threeObjects.get(id);
  if (!threeObj) return;

  const objData = state.objects.get(id);
  if (!objData) return;

  objData.position = [threeObj.position.x, threeObj.position.y, threeObj.position.z];
  objData.rotation = [threeObj.quaternion.x, threeObj.quaternion.y, threeObj.quaternion.z, threeObj.quaternion.w];
  objData.scale = [threeObj.scale.x, threeObj.scale.y, threeObj.scale.z];

  try {
    await transformObject(id, objData.position, objData.rotation, objData.scale);
  } catch (e) { console.error('Sync error:', e); }
}

async function refreshSceneData() {
  try {
    const data = await getSceneData();
    data.objects.forEach(obj => {
      state.objects.set(obj.id, obj);
    });
    updateOutliner();
  } catch (e) { console.error('Refresh error:', e); }
}

// ═══════════════════════════════════════════════════════════════════
// Edit Mode - Vertex/Edge/Face Overlay
// ═══════════════════════════════════════════════════════════════════

interface EditMeshData {
  vertices: GizmoVertex[];
  edges: GizmoEdge[];
  faces: GizmoFace[];
}

function extractEditData(id: string): EditMeshData | null {
  const threeObj = state.threeObjects.get(id);
  if (!threeObj || !(threeObj instanceof THREE.Mesh)) return null;

  const geo = threeObj.geometry;
  const posAttr = geo.getAttribute('position') as THREE.BufferAttribute;
  const indexAttr = geo.index;

  const vertices: GizmoVertex[] = [];
  const posArray = posAttr.array as Float32Array;
  const uniquePositions = new Map<string, number>();

  for (let i = 0; i < posAttr.count; i++) {
    const x = posArray[i * 3];
    const y = posArray[i * 3 + 1];
    const z = posArray[i * 3 + 2];
    const key = `${x.toFixed(6)},${y.toFixed(6)},${z.toFixed(6)}`;

    if (!uniquePositions.has(key)) {
      uniquePositions.set(key, vertices.length);
      vertices.push({
        position: new THREE.Vector3(x, y, z).applyMatrix4(threeObj.matrixWorld),
        index: uniquePositions.size - 1,
      });
    }
  }

  const edges: GizmoEdge[] = [];
  const faces: GizmoFace[] = [];

  if (indexAttr) {
    const indices = indexAttr.array as Uint16Array | Uint32Array;
    const edgeSet = new Set<string>();

    for (let i = 0; i < indices.length; i += 3) {
      const a = indices[i], b = indices[i + 1], c = indices[i + 2];

      const ka = `${posArray[a * 3].toFixed(6)},${posArray[a * 3 + 1].toFixed(6)},${posArray[a * 3 + 2].toFixed(6)}`;
      const kb = `${posArray[b * 3].toFixed(6)},${posArray[b * 3 + 1].toFixed(6)},${posArray[b * 3 + 2].toFixed(6)}`;
      const kc = `${posArray[c * 3].toFixed(6)},${posArray[c * 3 + 1].toFixed(6)},${posArray[c * 3 + 2].toFixed(6)}`;

      const ia = uniquePositions.get(ka)!;
      const ib = uniquePositions.get(kb)!;
      const ic = uniquePositions.get(kc)!;

      const addEdge = (a: number, b: number) => {
        const key = a < b ? `${a}-${b}` : `${b}-${a}`;
        if (!edgeSet.has(key)) {
          edgeSet.add(key);
          edges.push({ v0: a, v1: b });
        }
      };
      addEdge(ia, ib);
      addEdge(ib, ic);
      addEdge(ia, ic);

      const center = new THREE.Vector3();
      center.add(vertices[ia].position);
      center.add(vertices[ib].position);
      center.add(vertices[ic].position);
      center.divideScalar(3);

      faces.push({ indices: [ia, ib, ic], center });
    }
  }

  return { vertices, edges, faces };
}

function enterEditMode(id: string) {
  editOverlay.clear();
  const editData = extractEditData(id);
  if (!editData) return;

  const threeObj = state.threeObjects.get(id);
  if (threeObj instanceof THREE.Mesh) {
    threeObj.material = materialPresets.edit;
  }

  // Draw vertices
  const vertsGeo = new THREE.BufferGeometry();
  const vertsPos = new Float32Array(editData.vertices.length * 3);
  editData.vertices.forEach((v, i) => {
    vertsPos[i * 3] = v.position.x;
    vertsPos[i * 3 + 1] = v.position.y;
    vertsPos[i * 3 + 2] = v.position.z;
  });
  vertsGeo.setAttribute('position', new THREE.BufferAttribute(vertsPos, 3));
  const vertsPoints = new THREE.Points(vertsGeo, vertexMaterial.clone());
  (vertsPoints as any).userData = { type: 'vertices', data: editData };
  editOverlay.add(vertsPoints);

  // Draw edges
  const edgesGeo = new THREE.BufferGeometry();
  const edgesPos = new Float32Array(editData.edges.length * 6);
  editData.edges.forEach((e, i) => {
    const v0 = editData.vertices[e.v0].position;
    const v1 = editData.vertices[e.v1].position;
    edgesPos[i * 6] = v0.x; edgesPos[i * 6 + 1] = v0.y; edgesPos[i * 6 + 2] = v0.z;
    edgesPos[i * 6 + 3] = v1.x; edgesPos[i * 6 + 4] = v1.y; edgesPos[i * 6 + 5] = v1.z;
  });
  edgesGeo.setAttribute('position', new THREE.BufferAttribute(edgesPos, 3));
  const edgesLines = new THREE.LineSegments(edgesGeo, edgeMaterial.clone());
  (edgesLines as any).userData = { type: 'edges', data: editData };
  editOverlay.add(edgesLines);

  // Draw face centers
  const faceGeo = new THREE.BufferGeometry();
  const facePos = new Float32Array(editData.faces.length * 3);
  editData.faces.forEach((f, i) => {
    facePos[i * 3] = f.center.x;
    facePos[i * 3 + 1] = f.center.y;
    facePos[i * 3 + 2] = f.center.z;
  });
  faceGeo.setAttribute('position', new THREE.BufferAttribute(facePos, 3));
  const facePoints = new THREE.Points(faceGeo, faceCenterMaterial.clone());
  (facePoints as any).userData = { type: 'faces', data: editData };
  editOverlay.add(facePoints);

  setStatus(`Edit Mode: ${editData.vertices.length} verts, ${editData.edges.length} edges, ${editData.faces.length} faces`);
}

function exitEditMode() {
  editOverlay.clear();
  state.selectedVertices.clear();
  state.selectedEdges.clear();
  state.selectedFaces.clear();

  state.selectedObjects.forEach(id => {
    const threeObj = state.threeObjects.get(id);
    if (threeObj instanceof THREE.Mesh) {
      threeObj.material = materialPresets.selected;
    }
  });
}

function highlightSelectedEditComponents() {
  editOverlay.children.forEach(child => {
    if (child instanceof THREE.Points) {
      const mat = child.material as THREE.PointsMaterial;
      const userData = (child as any).userData;
      if (userData?.type === 'vertices') {
        const colors = new Float32Array(userData.data.vertices.length * 3);
        userData.data.vertices.forEach((v: GizmoVertex, i: number) => {
          if (state.selectedVertices.has(v.index)) {
            colors[i * 3] = 1.0; colors[i * 3 + 1] = 0.42; colors[i * 3 + 2] = 0.21;
          } else {
            colors[i * 3] = 1.0; colors[i * 3 + 1] = 0.42; colors[i * 3 + 2] = 0.21;
          }
        });
      }
    } else if (child instanceof THREE.LineSegments) {
      const userData = (child as any).userData;
      if (userData?.type === 'edges') {
        const colors = new Float32Array(userData.data.edges.length * 6);
        userData.data.edges.forEach((e: GizmoEdge, i: number) => {
          const selected = state.selectedEdges.has(i);
          const c = selected ? [1.0, 0.42, 0.21] : [1.0, 0.67, 0.0];
          colors[i * 6] = c[0]; colors[i * 6 + 1] = c[1]; colors[i * 6 + 2] = c[2];
          colors[i * 6 + 3] = c[0]; colors[i * 6 + 4] = c[1]; colors[i * 6 + 5] = c[2];
        });
        child.geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3));
        child.material = new THREE.LineBasicMaterial({ vertexColors: true });
      }
    }
  });
}

// ═══════════════════════════════════════════════════════════════════
// Edit mode click selection
// ═══════════════════════════════════════════════════════════════════

function editModeClickSelect(e: MouseEvent) {
  if (state.mode !== 'edit') return;
  if (state.selectedObjects.size !== 1) return;

  const id = Array.from(state.selectedObjects)[0];
  const editData = extractEditData(id);
  if (!editData) return;

  const rect = canvas.getBoundingClientRect();
  const mouse = new THREE.Vector2(
    ((e.clientX - rect.left) / rect.width) * 2 - 1,
    -((e.clientY - rect.top) / rect.height) * 2 + 1
  );

  const raycaster = new THREE.Raycaster();
  raycaster.setFromCamera(mouse, camera);
  raycaster.params.Points.threshold = 0.1;

  if (state.selectMode === 'Vertex' && editOverlay.children[0] instanceof THREE.Points) {
    const intersects = raycaster.intersectObject(editOverlay.children[0]);
    if (intersects.length > 0) {
      const idx = intersects[0].index!;
      const vertIdx = editData.vertices[idx].index;
      if (e.ctrlKey) {
        if (state.selectedVertices.has(vertIdx)) state.selectedVertices.delete(vertIdx);
        else state.selectedVertices.add(vertIdx);
      } else {
        state.selectedVertices.clear();
        state.selectedVertices.add(vertIdx);
      }
      highlightSelectedEditComponents();
      updateSelectionInfo();
    }
  } else if (state.selectMode === 'Edge' && editOverlay.children[1] instanceof THREE.LineSegments) {
    const lineSegments = editOverlay.children[1] as THREE.LineSegments;
    const intersects = raycaster.intersectObject(lineSegments);
    if (intersects.length > 0) {
      const faceIdx = Math.floor(intersects[0].faceIndex!);
      if (e.ctrlKey) {
        if (state.selectedEdges.has(faceIdx)) state.selectedEdges.delete(faceIdx);
        else state.selectedEdges.add(faceIdx);
      } else {
        state.selectedEdges.clear();
        state.selectedEdges.add(faceIdx);
      }
      highlightSelectedEditComponents();
      updateSelectionInfo();
    }
  } else if (state.selectMode === 'Face' && editOverlay.children[2] instanceof THREE.Points) {
    const intersects = raycaster.intersectObject(editOverlay.children[2]);
    if (intersects.length > 0) {
      const idx = intersects[0].index!;
      if (e.ctrlKey) {
        if (state.selectedFaces.has(idx)) state.selectedFaces.delete(idx);
        else state.selectedFaces.add(idx);
      } else {
        state.selectedFaces.clear();
        state.selectedFaces.add(idx);
      }
      highlightSelectedEditComponents();
      updateSelectionInfo();
    }
  }
}

// ═══════════════════════════════════════════════════════════════════
// Measurement tools
// ═══════════════════════════════════════════════════════════════════

function addMeasurePoint(point: THREE.Vector3) {
  state.measurePoints.push(point);

  const dotGeo = new THREE.SphereGeometry(0.04, 8, 8);
  const dotMat = new THREE.MeshBasicMaterial({ color: 0xffaa00 });
  const dot = new THREE.Mesh(dotGeo, dotMat);
  dot.position.copy(point);
  measurePointsObj.add(dot);

  if (state.measurePoints.length === 2) {
    const a = state.measurePoints[0];
    const b = state.measurePoints[1];

    const lineGeo = new THREE.BufferGeometry().setFromPoints([a, b]);
    const lineMat = new THREE.LineDashedMaterial({ color: 0xffaa00, dashSize: 0.1, gapSize: 0.05 });
    const line = new THREE.Line(lineGeo, lineMat);
    line.computeLineDistances();
    measureLinesObj.add(line);

    const mid = a.clone().add(b).multiplyScalar(0.5);

    const sprite = makeTextSprite(a.distanceTo(b).toFixed(4));
    sprite.position.copy(mid);
    sprite.position.y += 0.1;
    measureLabelsObj.add(sprite);

    setStatus(`Distance: ${a.distanceTo(b).toFixed(4)} units`);
    state.measurePoints = [];
  } else if (state.measurePoints.length === 3) {
    const a = state.measurePoints[0];
    const vertex = state.measurePoints[1];
    const b = state.measurePoints[2];

    const dirA = a.clone().sub(vertex).normalize();
    const dirB = b.clone().sub(vertex).normalize();
    const angle = dirA.angleTo(dirB) * (180 / Math.PI);

    const sprite = makeTextSprite(`${angle.toFixed(1)}°`);
    sprite.position.copy(vertex).add(new THREE.Vector3(0, 0.2, 0));
    measureLabelsObj.add(sprite);

    setStatus(`Angle: ${angle.toFixed(1)}°`);
    state.measurePoints = [];
  }
}

function makeTextSprite(text: string): THREE.Sprite {
  const canvas2d = document.createElement('canvas');
  const ctx = canvas2d.getContext('2d')!;
  canvas2d.width = 256;
  canvas2d.height = 64;
  ctx.fillStyle = 'rgba(0,0,0,0.7)';
  ctx.fillRect(0, 0, 256, 64);
  ctx.fillStyle = '#ffcc00';
  ctx.font = 'bold 28px Inter, sans-serif';
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.fillText(text, 128, 32);

  const texture = new THREE.CanvasTexture(canvas2d);
  const mat = new THREE.SpriteMaterial({ map: texture, transparent: true });
  const sprite = new THREE.Sprite(mat);
  sprite.scale.set(0.5, 0.125, 1);
  return sprite;
}

function clearMeasurements() {
  measurePointsObj.clear();
  measureLinesObj.clear();
  measureLabelsObj.clear();
  state.measurePoints = [];
}

// ═══════════════════════════════════════════════════════════════════
// Gizmo overlay for edit mode
// ═══════════════════════════════════════════════════════════════════

function updateGizmoOverlay() {
  if (state.mode !== 'object') return;
  if (state.selectedObjects.size === 1) {
    const id = Array.from(state.selectedObjects)[0];
    const threeObj = state.threeObjects.get(id);
    if (threeObj) {
      transformControls.attach(threeObj);
    }
  } else {
    transformControls.detach();
  }
}

// ═══════════════════════════════════════════════════════════════════
// UI Updates
// ═══════════════════════════════════════════════════════════════════

function updateOutliner() {
  const list = document.getElementById('outliner-list')!;
  const searchInput = document.getElementById('outliner-search') as HTMLInputElement | null;
  const searchTerm = searchInput?.value?.toLowerCase() || '';
  list.innerHTML = '';
  state.objects.forEach((obj, id) => {
    if (searchTerm && !obj.name.toLowerCase().includes(searchTerm)) return;
    const item = document.createElement('div');
    item.className = `outliner-item${state.selectedObjects.has(id) ? ' selected' : ''}${!obj.visible ? ' hidden-item' : ''}`;
    item.innerHTML = `
      <button class="outliner-visibility" data-id="${id}" title="Toggle visibility">${obj.visible ? '&#128065;' : '&#128064;'}</button>
      <div class="outliner-icon" style="background:${obj.visible ? '#666' : '#333'}"></div>
      <span>${obj.name}</span>
      <span class="outliner-count">${obj.vertex_count}v / ${obj.face_count}f</span>
    `;
    item.onclick = (e) => {
      const target = e.target as HTMLElement;
      if (target.classList.contains('outliner-visibility')) {
        e.stopPropagation();
        const threeObj = state.threeObjects.get(id);
        if (threeObj) {
          threeObj.visible = !threeObj.visible;
          obj.visible = threeObj.visible;
          updateOutliner();
        }
        return;
      }
      selectObject(id, e.ctrlKey);
    };
    list.appendChild(item);
  });
}

function updateSelectionInfo() {
  const selInfo = document.getElementById('selection-info')!;
  if (state.mode === 'edit') {
    const total = state.selectedVertices.size + state.selectedEdges.size + state.selectedFaces.size;
    selInfo.textContent = total > 0
      ? `${state.selectedVertices.size}V / ${state.selectedEdges.size}E / ${state.selectedFaces.size}F selected`
      : 'Edit mode - Click to select';
  } else {
    const count = state.selectedObjects.size;
    selInfo.textContent = count === 0 ? 'No selection' :
      count === 1 ? `Selected: ${state.objects.get(Array.from(state.selectedObjects)[0])?.name}` :
      `${count} objects selected`;
  }
}

async function updateProperties() {
  const selCount = state.selectedObjects.size;
  const selInfo = document.getElementById('selection-info')!;

  if (selCount === 0) {
    selInfo.textContent = 'No selection';
    ['info-verts', 'info-edges', 'info-faces', 'info-volume', 'info-area'].forEach(id => {
      document.getElementById(id)!.textContent = '0';
    });
    transformControls.detach();
    return;
  }

  if (selCount === 1) {
    const id = Array.from(state.selectedObjects)[0];
    const obj = state.objects.get(id);
    if (!obj) return;

    selInfo.textContent = `Selected: ${obj.name}`;
    (document.getElementById('pos-x') as HTMLInputElement)!.value = obj.position[0].toFixed(3);
    (document.getElementById('pos-y') as HTMLInputElement)!.value = obj.position[1].toFixed(3);
    (document.getElementById('pos-z') as HTMLInputElement)!.value = obj.position[2].toFixed(3);
    (document.getElementById('scale-x') as HTMLInputElement)!.value = obj.scale[0].toFixed(3);
    (document.getElementById('scale-y') as HTMLInputElement)!.value = obj.scale[1].toFixed(3);
    (document.getElementById('scale-z') as HTMLInputElement)!.value = obj.scale[2].toFixed(3);

    try {
      const info = await getMeshInfo(id);
      document.getElementById('info-verts')!.textContent = info.vertex_count.toString();
      document.getElementById('info-edges')!.textContent = info.edge_count.toString();
      document.getElementById('info-faces')!.textContent = info.face_count.toString();
      document.getElementById('info-volume')!.textContent = info.volume.toFixed(4);
      document.getElementById('info-area')!.textContent = info.surface_area.toFixed(4);
    } catch (e) { console.error('Mesh info error:', e); }

    updateGizmoOverlay();
  } else {
    selInfo.textContent = `${selCount} objects selected`;
    transformControls.detach();
  }
}

function selectObject(id: string, additive = false) {
  if (state.mode === 'edit') {
    exitEditMode();
  }

  if (!additive) {
    state.selectedObjects.forEach(selId => {
      const obj = state.threeObjects.get(selId);
      if (obj instanceof THREE.Mesh) {
        obj.material = materialPresets.default.clone();
      }
    });
    state.selectedObjects.clear();
  }

  if (state.selectedObjects.has(id)) {
    state.selectedObjects.delete(id);
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) obj.material = materialPresets.default.clone();
  } else {
    state.selectedObjects.add(id);
    state.activeObject = id;
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) obj.material = materialPresets.selected.clone();
  }

  updateOutliner();
  updateProperties();
  updateSelectionInfo();
  updateGizmoOverlay();
  updateSelectionOutlines();
}

function deselectAll() {
  state.selectedObjects.forEach(id => {
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) obj.material = materialPresets.default.clone();
  });
  state.selectedObjects.clear();
  state.selectedVertices.clear();
  state.selectedEdges.clear();
  state.selectedFaces.clear();
  state.activeObject = null;
  transformControls.detach();
  editOverlay.clear();
  updateOutliner();
  updateProperties();
  updateSelectionInfo();
  updateSelectionOutlines();
}

function selectAll() {
  state.objects.forEach((_, id) => {
    state.selectedObjects.add(id);
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) obj.material = materialPresets.selected.clone();
  });
  updateOutliner();
  updateProperties();
}

// ═══════════════════════════════════════════════════════════════════
// Add Primitive
// ═══════════════════════════════════════════════════════════════════

async function addPrimitive(type: string) {
  pushUndo();
  const name = `${type.charAt(0).toUpperCase() + type.slice(1)}_${Date.now() % 10000}`;
  try {
    const id = await createPrimitive(type, name);
    const threeGeo = createThreeGeometry(type);
    const mat = materialPresets.default.clone();
    const threeObj = new THREE.Mesh(threeGeo, mat);
    threeObj.castShadow = true;
    threeObj.receiveShadow = true;

    if (state.snapping.enabled) {
      threeObj.position.copy(state.cursor3d);
    }

    scene.add(threeObj);
    state.threeObjects.set(id, threeObj);
    state.threeObjectsInverse.set(threeObj, id);
    state.objects.set(id, {
      id, name,
      position: [threeObj.position.x, threeObj.position.y, threeObj.position.z],
      rotation: [0, 0, 0, 1],
      scale: [1, 1, 1],
      vertex_count: 0,
      face_count: 0,
      visible: true,
    });
    selectObject(id);
    setStatus(`Added ${name}`);
  } catch (e) {
    setStatus(`Error: ${e}`);
  }
}

function createThreeGeometry(type: string): THREE.BufferGeometry {
  switch (type) {
    case 'cube': return new THREE.BoxGeometry(1, 1, 1, 2, 2, 2);
    case 'sphere': return new THREE.SphereGeometry(0.5, 48, 48);
    case 'cylinder': return new THREE.CylinderGeometry(0.5, 0.5, 1, 48);
    case 'torus': return new THREE.TorusGeometry(0.5, 0.15, 24, 64);
    case 'plane': return new THREE.PlaneGeometry(2, 2, 10, 10);
    case 'cone': return new THREE.ConeGeometry(0.5, 1, 48);
    case 'monkey': return new THREE.IcosahedronGeometry(0.6, 2);
    case 'torusknot': return new THREE.TorusKnotGeometry(0.4, 0.12, 128, 32);
    case 'dodecahedron': return new THREE.DodecahedronGeometry(0.5, 0);
    case 'icosahedron': return new THREE.IcosahedronGeometry(0.5, 0);
    case 'octahedron': return new THREE.OctahedronGeometry(0.5, 0);
    case 'tetrahedron': return new THREE.TetrahedronGeometry(0.5, 0);
    default: return new THREE.BoxGeometry(1, 1, 1);
  }
}

// ═══════════════════════════════════════════════════════════════════
// Keyboard Shortcuts
// ═══════════════════════════════════════════════════════════════════

document.addEventListener('keydown', async (e) => {
  if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

  const key = e.key.toLowerCase();
  const ctrl = e.ctrlKey || e.metaKey;
  const shift = e.shiftKey;
  const alt = e.altKey;

  if (ctrl) {
    switch (key) {
      case 'z': e.preventDefault(); await undoAction(); return;
      case 'y': e.preventDefault(); await redoAction(); return;
      case 'c': e.preventDefault(); await copySelection(); return;
      case 'v': e.preventDefault(); await pasteClipboard(); return;
      case 'x': e.preventDefault(); await cutSelection(); return;
      case 'a': e.preventDefault(); selectAll(); return;
      case 'j': e.preventDefault(); await joinSelected(); return;
      case 'b': setTool('bevel'); return;
      case 'r': e.preventDefault(); setTool('loopcut'); return;
      case 'i': e.preventDefault(); invertSelection(); return;
    }
    if (shift) {
      switch (key) {
        case 'd': e.preventDefault(); await duplicateObject(); return;
        case ' ': e.preventDefault(); toggleTimelinePlay(); return;
      }
    }
  }

  if (shift && !ctrl) {
    switch (key) {
      case 'a': e.preventDefault(); deselectAll(); return;
      case 'g': selectGrouped(); return;
      case 's': setTool('scale'); return;
    }
  }

  if (alt && !ctrl && !shift) {
    switch (key) {
      case 's': e.preventDefault(); shrinkFatten(); return;
    }
  }

  switch (key) {
    case 'q': setTool('cursor'); break;
    case 'g': setTool('move'); break;
    case 'r': setTool('rotate'); break;
    case 's': setTool('scale'); break;
    case 'e': setTool('extrude'); break;
    case 'i': setTool('inset'); break;
    case 'j': setTool('loopcut'); break;
    case 'k': setTool('knife'); break;
    case 'x':
    case 'delete':
      await deleteSelected();
      break;
    case 'tab':
      e.preventDefault();
      toggleMode();
      break;
    case '1': setSelectMode('Vertex'); break;
    case '2': setSelectMode('Edge'); break;
    case '3': setSelectMode('Face'); break;
    case '0': setSelectMode('Object'); break;
    case 'a': deselectAll(); break;
    case 'f': focusSelected(); break;
    case 'h': toggleVisibility(); break;
    case 'm': setTool('measure'); break;
    case 'n': toggleRightPanel(); break;
    case 'z': cyclePivotPoint(); break;
    case 'escape':
      if (state.mode === 'edit') { exitEditMode(); setMode('object'); }
      else deselectAll();
      break;
    case 'numpad1': setCameraView('front'); break;
    case 'numpad3': setCameraView('right'); break;
    case 'numpad7': setCameraView('top'); break;
    case 'numpad0': setCameraView('camera'); break;
    case 'numpad5': togglePerspective(); break;
    case 'numpadperiod': focusSelected(); break;
    case 'arrowup': e.preventDefault(); stepFrame(1); break;
    case 'arrowdown': e.preventDefault(); stepFrame(-1); break;
  }
});

// ═══════════════════════════════════════════════════════════════════
// Tool & Mode Management
// ═══════════════════════════════════════════════════════════════════

function setTool(tool: ToolMode) {
  state.tool = tool;
  document.querySelectorAll('.tool-btn').forEach(btn => btn.classList.remove('active'));
  const btn = document.getElementById(`tool-${tool}`);
  if (btn) btn.classList.add('active');

  switch (tool) {
    case 'move': transformControls.setMode('translate'); break;
    case 'rotate': transformControls.setMode('rotate'); break;
    case 'scale': transformControls.setMode('scale'); break;
    default: transformControls.detach(); break;
  }

  if (tool !== 'measure') clearMeasurements();
  if (state.selectedObjects.size === 1 && (tool === 'move' || tool === 'rotate' || tool === 'scale')) {
    const id = Array.from(state.selectedObjects)[0];
    const threeObj = state.threeObjects.get(id);
    if (threeObj) transformControls.attach(threeObj);
  }

  setStatus(`Tool: ${tool.charAt(0).toUpperCase() + tool.slice(1)}`);
}

function setSelectMode(mode: SelectMode) {
  state.selectMode = mode;
  document.querySelectorAll('.mode-btn').forEach(btn => {
    btn.classList.toggle('active', (btn as HTMLElement).dataset.mode === (mode === 'Object' ? 'object' : 'edit'));
  });
  if (mode !== 'Object' && state.mode !== 'edit') {
    setMode('edit');
  }
  setStatus(`Select Mode: ${mode}`);
}

function setMode(mode: typeof state.mode) {
  if (state.mode === 'edit' && mode !== 'edit') exitEditMode();
  state.mode = mode;

  document.querySelectorAll('.mode-btn').forEach(btn => {
    btn.classList.toggle('active', (btn as HTMLElement).dataset.mode === mode);
  });

  // Show/hide sculpt panel
  const sculptPanel = document.getElementById('sculpt-panel');
  if (sculptPanel) {
    sculptPanel.classList.toggle('hidden', mode !== 'sculpt');
  }

  if (mode === 'edit' && state.selectedObjects.size === 1) {
    enterEditMode(Array.from(state.selectedObjects)[0]);
  }

  if (mode === 'object') {
    transformControls.setSpace(state.orientation === 'local' ? 'local' : 'world');
  }

  setStatus(`Mode: ${mode.charAt(0).toUpperCase() + mode.slice(1)}`);
}

function toggleMode() {
  if (state.mode === 'object') {
    setMode('edit');
    if (state.selectMode === 'Object') state.selectMode = 'Vertex';
  } else {
    setMode('object');
  }
}

async function deleteSelected() {
  if (state.mode === 'edit') {
    if (state.selectedVertices.size > 0 || state.selectedEdges.size > 0 || state.selectedFaces.size > 0) {
      setStatus('Deleted selected components');
      state.selectedVertices.clear();
      state.selectedEdges.clear();
      state.selectedFaces.clear();
      if (state.selectedObjects.size === 1) {
        enterEditMode(Array.from(state.selectedObjects)[0]);
      }
      return;
    }
  }

  pushUndo();
  for (const id of state.selectedObjects) {
    const obj = state.threeObjects.get(id);
    if (obj) {
      scene.remove(obj);
      threeObjectsInverse.delete(obj);
      // Clean up outlines
      const hoverOutline = hoverOutlines.get(id);
      if (hoverOutline) { scene.remove(hoverOutline); hoverOutlines.delete(id); }
      const selOutline = selectionOutlines.get(id);
      if (selOutline) { scene.remove(selOutline); selectionOutlines.delete(id); }
    }
    state.threeObjects.delete(id);
    state.objects.delete(id);
    try { await deleteObject(id); } catch (e) { console.error(e); }
  }
  state.selectedObjects.clear();
  transformControls.detach();
  updateOutliner();
  updateProperties();
  setStatus('Deleted');
}

function focusSelected() {
  if (state.selectedObjects.size === 1) {
    const id = Array.from(state.selectedObjects)[0];
    const obj = state.threeObjects.get(id);
    if (obj) {
      const box = new THREE.Box3().setFromObject(obj);
      const center = box.getCenter(new THREE.Vector3());
      const size = box.getSize(new THREE.Vector3());
      const maxDim = Math.max(size.x, size.y, size.z);
      const dist = maxDim * 2;
      controls.target.copy(center);
      camera.position.copy(center).add(new THREE.Vector3(dist * 0.7, dist * 0.5, dist * 0.7));
    }
  }
}

function toggleVisibility() {
  state.selectedObjects.forEach(id => {
    const obj = state.threeObjects.get(id);
    if (obj) {
      obj.visible = !obj.visible;
      const objData = state.objects.get(id);
      if (objData) objData.visible = obj.visible;
      setStatus(obj.visible ? 'Visible' : 'Hidden');
    }
  });
  updateOutliner();
}

function cyclePivotPoint() {
  const points = ['median', 'individual', 'center', 'cursor', 'active'] as const;
  const idx = points.indexOf(state.pivotPoint);
  state.pivotPoint = points[(idx + 1) % points.length];
  setStatus(`Pivot: ${state.pivotPoint}`);
}

function toggleRightPanel() {
  const panel = document.getElementById('right-panel')!;
  panel.style.display = panel.style.display === 'none' ? '' : 'none';
}

// ═══════════════════════════════════════════════════════════════════
// Camera Views
// ═══════════════════════════════════════════════════════════════════

function setCameraView(view: string) {
  const target = controls.target.clone();
  const dist = camera.position.distanceTo(target);
  let pos: THREE.Vector3;

  switch (view) {
    case 'front': pos = target.clone().add(new THREE.Vector3(0, 0, dist)); break;
    case 'back': pos = target.clone().add(new THREE.Vector3(0, 0, -dist)); break;
    case 'right': pos = target.clone().add(new THREE.Vector3(dist, 0, 0)); break;
    case 'left': pos = target.clone().add(new THREE.Vector3(-dist, 0, 0)); break;
    case 'top': pos = target.clone().add(new THREE.Vector3(0, dist, 0)); break;
    case 'bottom': pos = target.clone().add(new THREE.Vector3(0, -dist, 0)); break;
    case 'camera': pos = new THREE.Vector3(0, 1.6, 3); controls.target.set(0, 0, 0); break;
    default: return;
  }

  animateCamera(pos, target);
  setStatus(`View: ${view}`);
}

function animateCamera(targetPos: THREE.Vector3, targetLookAt: THREE.Vector3) {
  const startPos = camera.position.clone();
  const startTarget = controls.target.clone();
  const duration = 300;
  const startTime = performance.now();

  function step() {
    const elapsed = performance.now() - startTime;
    const t = Math.min(elapsed / duration, 1);
    const ease = t * (2 - t);

    camera.position.lerpVectors(startPos, targetPos, ease);
    controls.target.lerpVectors(startTarget, targetLookAt, ease);
    controls.update();

    if (t < 1) requestAnimationFrame(step);
  }
  step();
}

function togglePerspective() {
  camera.fov = camera.fov === 45 ? 1 : 45;
  camera.updateProjectionMatrix();
  setStatus(camera.fov === 1 ? 'Orthographic' : 'Perspective');
}

// ═══════════════════════════════════════════════════════════════════
// Undo/Redo System
// ═══════════════════════════════════════════════════════════════════

interface UndoSnapshot {
  objects: Map<string, SceneObjectData>;
  selectedObjects: string[];
  cameraPos: [number, number, number];
  cameraTarget: [number, number, number];
}

const undoStack: UndoSnapshot[] = [];
const redoStack: UndoSnapshot[] = [];
const MAX_UNDO = 50;

function captureSnapshot(): UndoSnapshot {
  const objectsCopy = new Map<string, SceneObjectData>();
  state.objects.forEach((obj, id) => {
    objectsCopy.set(id, { ...obj, position: [...obj.position], rotation: [...obj.rotation], scale: [...obj.scale] });
  });
  return {
    objects: objectsCopy,
    selectedObjects: Array.from(state.selectedObjects),
    cameraPos: [camera.position.x, camera.position.y, camera.position.z],
    cameraTarget: [controls.target.x, controls.target.y, controls.target.z],
  };
}

function pushUndo() {
  const snapshot = captureSnapshot();
  undoStack.push(snapshot);
  if (undoStack.length > MAX_UNDO) undoStack.shift();
  redoStack.length = 0;
}

async function undoAction() {
  if (undoStack.length === 0) { setStatus('Nothing to undo'); return; }
  const currentSnapshot = captureSnapshot();
  redoStack.push(currentSnapshot);
  const snapshot = undoStack.pop()!;
  await restoreSnapshot(snapshot);
  setStatus(`Undo (${undoStack.length} remaining)`);
}

async function redoAction() {
  if (redoStack.length === 0) { setStatus('Nothing to redo'); return; }
  const currentSnapshot = captureSnapshot();
  undoStack.push(currentSnapshot);
  const snapshot = redoStack.pop()!;
  await restoreSnapshot(snapshot);
  setStatus(`Redo (${redoStack.length} remaining)`);
}

async function restoreSnapshot(snapshot: UndoSnapshot) {
  // Remove current objects from scene
  state.threeObjects.forEach((obj, id) => {
    if (!snapshot.objects.has(id)) {
      scene.remove(obj);
      threeObjectsInverse.delete(obj);
    }
  });

  state.objects.clear();
  state.threeObjects.clear();
  state.selectedObjects.clear();
  hoverOutlines.forEach((o) => scene.remove(o));
  hoverOutlines.clear();
  selectionOutlines.forEach((o) => scene.remove(o));
  selectionOutlines.clear();

  // Restore objects
  for (const [id, objData] of snapshot.objects) {
    state.objects.set(id, objData);
    const cachedGeo = importedGeometryCache.get(id);
    let threeGeo: THREE.BufferGeometry;
    if (cachedGeo && cachedGeo.positions.length > 0) {
      threeGeo = new THREE.BufferGeometry();
      threeGeo.setAttribute('position', new THREE.Float32BufferAttribute(cachedGeo.positions, 3));
      threeGeo.setIndex(new THREE.BufferAttribute(new Uint32Array(cachedGeo.indices), 1));
      threeGeo.computeVertexNormals();
    } else {
      const typeName = objData.name.toLowerCase().replace(/_\d+$/, '');
      threeGeo = createThreeGeometry(typeName);
    }
    const mat = materialPresets.default.clone();
    mat.side = THREE.DoubleSide;
    const threeObj = new THREE.Mesh(threeGeo, mat);
    threeObj.position.set(objData.position[0], objData.position[1], objData.position[2]);
    // rotation stored as quaternion [x,y,z,w]
    threeObj.quaternion.set(objData.rotation[0], objData.rotation[1], objData.rotation[2], objData.rotation[3]);
    threeObj.scale.set(objData.scale[0], objData.scale[1], objData.scale[2]);
    threeObj.castShadow = true;
    threeObj.receiveShadow = true;
    scene.add(threeObj);
    state.threeObjects.set(id, threeObj);
    state.threeObjectsInverse.set(threeObj, id);
  }

  // Restore selection
  for (const id of snapshot.selectedObjects) {
    if (state.objects.has(id)) {
      state.selectedObjects.add(id);
      const obj = state.threeObjects.get(id);
      if (obj instanceof THREE.Mesh) obj.material = materialPresets.selected.clone();
    }
  }

  // Restore camera
  camera.position.set(snapshot.cameraPos[0], snapshot.cameraPos[1], snapshot.cameraPos[2]);
  controls.target.set(snapshot.cameraTarget[0], snapshot.cameraTarget[1], snapshot.cameraTarget[2]);
  controls.update();

  updateOutliner();
  updateProperties();
  updateGizmoOverlay();
  updateSelectionOutlines();
}

async function copySelection() {
  if (state.selectedObjects.size === 1) {
    const id = Array.from(state.selectedObjects)[0];
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) {
      state.clipboard = obj.geometry.clone();
      setStatus('Copied');
    }
  }
}

async function cutSelection() {
  await copySelection();
  await deleteSelected();
}

async function pasteClipboard() {
  if (!state.clipboard) return;
  const name = `Pasted_${Date.now() % 10000}`;
  try {
    const id = await createPrimitive('cube', name);
    const mat = materialPresets.default.clone();
    const threeObj = new THREE.Mesh(state.clipboard.clone(), mat);
    scene.add(threeObj);
    state.threeObjects.set(id, threeObj);
    state.threeObjectsInverse.set(threeObj, id);
    state.objects.set(id, {
      id, name,
      position: [0, 0, 0],
      rotation: [0, 0, 0, 1],
      scale: [1, 1, 1],
      vertex_count: 0,
      face_count: 0,
      visible: true,
    });
    selectObject(id);
    setStatus('Pasted');
  } catch (e) { console.error(e); }
}

async function joinSelected() {
  if (state.selectedObjects.size < 2) return;
  const ids = Array.from(state.selectedObjects);
  setStatus(`Joining ${ids.length} objects...`);
}

// ═══════════════════════════════════════════════════════════════════
// Status
// ═══════════════════════════════════════════════════════════════════

function setStatus(msg: string) {
  document.getElementById('status-text')!.textContent = msg;
}

// ═══════════════════════════════════════════════════════════════════
// UI Event Bindings
// ═══════════════════════════════════════════════════════════════════

document.getElementById('btn-add')!.addEventListener('click', () => {
  document.getElementById('add-menu')!.classList.toggle('hidden');
});

document.querySelectorAll('.add-item').forEach(btn => {
  btn.addEventListener('click', (e) => {
    const type = (e.target as HTMLElement).dataset.type!;
    addPrimitive(type);
    document.getElementById('add-menu')!.classList.add('hidden');
  });
});

document.getElementById('btn-file')!.addEventListener('click', () => {
  document.getElementById('file-menu')!.classList.toggle('hidden');
});

document.getElementById('file-import')?.addEventListener('click', async () => {
  try {
    const filePath = await openFileDialog();
    if (!filePath) { document.getElementById('file-menu')?.classList.add('hidden'); return; }
    pushUndo();
    const ids = await importMesh(filePath);
    for (const id of ids) {
      const meshData = await getMeshData(id);
      const data = await getSceneData();
      const objData = data.objects.find(o => o.id === id);
      if (!objData) continue;

      const threeGeo = new THREE.BufferGeometry();
      if (meshData.positions.length > 0 && meshData.indices.length > 0) {
        importedGeometryCache.set(id, { positions: meshData.positions, indices: meshData.indices, normals: meshData.normals });
        threeGeo.setAttribute('position', new THREE.Float32BufferAttribute(meshData.positions, 3));
        threeGeo.setIndex(new THREE.BufferAttribute(new Uint32Array(meshData.indices), 1));
        threeGeo.computeVertexNormals();
      } else {
        const box = new THREE.BoxGeometry(1, 1, 1);
        threeGeo.copy(box);
        box.dispose();
      }

      const mat = materialPresets.default.clone();
      const threeObj = new THREE.Mesh(threeGeo, mat);
      threeObj.castShadow = true;
      threeObj.receiveShadow = true;
      scene.add(threeObj);
      state.threeObjects.set(id, threeObj);
      state.threeObjectsInverse.set(threeObj, id);
      state.objects.set(id, objData);
      selectObject(id, true);
    }
    updateOutliner();
    setStatus(`Imported ${ids.length} object(s)`);
  } catch (e) {
    console.error('Import error:', e);
    setStatus(`Import error: ${e}`);
  }
  document.getElementById('file-menu')?.classList.add('hidden');
});

document.getElementById('file-export-obj')?.addEventListener('click', async () => {
  if (state.selectedObjects.size === 0) { setStatus('Select objects to export'); return; }
  try {
    const filePath = await save({ defaultPath: 'export.obj', filters: [{ name: 'OBJ', extensions: ['obj'] }] });
    if (filePath) {
      const ids = Array.from(state.selectedObjects);
      await exportMesh(filePath, ids);
      setStatus(`Exported to ${filePath}`);
    }
  } catch (e) {
    setStatus(`Export error: ${e}`);
  }
  document.getElementById('file-menu')?.classList.add('hidden');
});

document.getElementById('file-export-stl')?.addEventListener('click', async () => {
  if (state.selectedObjects.size === 0) { setStatus('Select objects to export'); return; }
  try {
    const filePath = await save({ defaultPath: 'export.stl', filters: [{ name: 'STL', extensions: ['stl'] }] });
    if (filePath) {
      const ids = Array.from(state.selectedObjects);
      await exportMesh(filePath, ids);
      setStatus(`Exported to ${filePath}`);
    }
  } catch (e) {
    setStatus(`Export error: ${e}`);
  }
  document.getElementById('file-menu')?.classList.add('hidden');
});

document.getElementById('file-export-gltf')?.addEventListener('click', async () => {
  if (state.selectedObjects.size === 0) { setStatus('Select objects to export'); return; }
  try {
    const filePath = await save({ defaultPath: 'export.gltf', filters: [{ name: 'GLTF', extensions: ['gltf', 'glb'] }] });
    if (filePath) {
      const ids = Array.from(state.selectedObjects);
      await exportMesh(filePath, ids);
      setStatus(`Exported to ${filePath}`);
    }
  } catch (e) {
    setStatus(`Export error: ${e}`);
  }
  document.getElementById('file-menu')?.classList.add('hidden');
});

document.querySelectorAll('.mode-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    const mode = (btn as HTMLElement).dataset.mode as typeof state.mode;
    setMode(mode);
  });
});

document.querySelectorAll('.tool-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    const tool = btn.id.replace('tool-', '') as ToolMode;
    setTool(tool);
  });
});

document.querySelectorAll('.panel-collapse-btn').forEach(btn => {
  btn.addEventListener('click', (e) => {
    const panel = (e.target as HTMLElement).closest('.panel');
    const body = panel?.querySelector('.panel-body') as HTMLElement | null;
    if (body) {
      const hidden = body.style.display === 'none';
      body.style.display = hidden ? '' : 'none';
      (e.target as HTMLElement).textContent = hidden ? '−' : '+';
    }
  });
});

document.querySelectorAll('.vec3-input input').forEach(input => {
  input.addEventListener('change', async () => {
    if (state.selectedObjects.size !== 1) return;
    pushUndo();
    const id = Array.from(state.selectedObjects)[0];
    const obj = state.objects.get(id);
    if (!obj) return;

    obj.position = [
      parseFloat((document.getElementById('pos-x') as HTMLInputElement).value) || 0,
      parseFloat((document.getElementById('pos-y') as HTMLInputElement).value) || 0,
      parseFloat((document.getElementById('pos-z') as HTMLInputElement).value) || 0,
    ];
    obj.scale = [
      parseFloat((document.getElementById('scale-x') as HTMLInputElement).value) || 1,
      parseFloat((document.getElementById('scale-y') as HTMLInputElement).value) || 1,
      parseFloat((document.getElementById('scale-z') as HTMLInputElement).value) || 1,
    ];

    const threeObj = state.threeObjects.get(id);
    if (threeObj) {
      threeObj.position.set(...obj.position);
      threeObj.scale.set(...obj.scale);
    }
    try { await transformObject(id, obj.position, obj.rotation, obj.scale); } catch (e) { console.error(e); }
  });
});

// Viewport click selection
canvas.addEventListener('mousedown', (e) => {
  if (state.mode === 'edit') {
    editModeClickSelect(e);
    return;
  }

  if (state.tool === 'measure') {
    const rect = canvas.getBoundingClientRect();
    const mouse = new THREE.Vector2(
      ((e.clientX - rect.left) / rect.width) * 2 - 1,
      -((e.clientY - rect.top) / rect.height) * 2 + 1
    );
    const raycaster = new THREE.Raycaster();
    raycaster.setFromCamera(mouse, camera);
    const intersects = raycaster.intersectObjects(scene.children, true);
    if (intersects.length > 0) {
      addMeasurePoint(intersects[0].point.clone());
    }
    return;
  }

  if (state.tool === 'cursor') {
    const rect = canvas.getBoundingClientRect();
    const mouse = new THREE.Vector2(
      ((e.clientX - rect.left) / rect.width) * 2 - 1,
      -((e.clientY - rect.top) / rect.height) * 2 + 1
    );
    const raycaster = new THREE.Raycaster();
    raycaster.setFromCamera(mouse, camera);

    const meshObjects = Array.from(state.threeObjects.values()).filter(o => o instanceof THREE.Mesh && o.visible);
    const intersects = raycaster.intersectObjects(meshObjects);

    if (intersects.length > 0) {
      const hit = intersects[0].object;
      const id = threeObjectsInverse.get(hit);
      if (id) selectObject(id, e.ctrlKey);
    } else {
      deselectAll();
    }
  }
});

canvas.addEventListener('dblclick', (e) => {
  if (state.mode === 'object' && state.selectedObjects.size === 1) {
    const id = Array.from(state.selectedObjects)[0];
    setMode('edit');
  }
});

document.addEventListener('click', (e) => {
  const target = e.target as HTMLElement;
  if (!target.closest('#add-menu') && !target.closest('#btn-add')) {
    document.getElementById('add-menu')?.classList.add('hidden');
  }
  if (!target.closest('#file-menu') && !target.closest('#btn-file')) {
    document.getElementById('file-menu')?.classList.add('hidden');
  }
});

// ═══════════════════════════════════════════════════════════════════
// Resize
// ═══════════════════════════════════════════════════════════════════

function onResize() {
  const container = document.getElementById('viewport-container')!;
  const w = container.clientWidth;
  const h = container.clientHeight;
  renderer.setSize(w, h);
  camera.aspect = w / h;
  camera.updateProjectionMatrix();
}
window.addEventListener('resize', onResize);

// ═══════════════════════════════════════════════════════════════════
// Animation Loop
// ═══════════════════════════════════════════════════════════════════

const hoverOutlines = new Map<string, THREE.Mesh>();
const selectionOutlines = new Map<string, THREE.Mesh>();

function animate() {
  requestAnimationFrame(animate);
  controls.update();

  if (state.proportional.enabled && transformControls.object) {
    proportionalMesh.visible = true;
    proportionalMesh.position.copy(transformControls.object.position);
    proportionalMesh.scale.setScalar(state.proportional.radius);
  } else {
    proportionalMesh.visible = false;
  }

  // Timeline playback
  const now = performance.now();
  if (state.timeline.playing) {
    const frameDuration = 1000 / state.timeline.fps;
    if (now - state.lastTickTime >= frameDuration) {
      state.timeline.frame++;
      if (state.timeline.frame > state.timeline.endFrame) {
        state.timeline.frame = state.timeline.startFrame;
      }
      const frameInput = document.getElementById('timeline-frame') as HTMLInputElement | null;
      if (frameInput) frameInput.value = state.timeline.frame.toString();
      renderTimeline();
      state.lastTickTime = now;
    }
  }

  renderer.render(scene, camera);
  updateOrientationGizmo();

  // Sync outlines with object transforms
  selectionOutlines.forEach((outline, id) => {
    const threeObj = state.threeObjects.get(id);
    if (threeObj && threeObj instanceof THREE.Mesh) {
      const posAttr = threeObj.geometry.getAttribute('position') as THREE.BufferAttribute;
      if (!posAttr || posAttr.count > 5000) {
        scene.remove(outline);
        selectionOutlines.delete(id);
        return;
      }
      outline.position.copy(threeObj.position);
      outline.quaternion.copy(threeObj.quaternion);
      outline.scale.copy(threeObj.scale).multiplyScalar(1.05);
    }
  });
  hoverOutlines.forEach((outline, id) => {
    const threeObj = state.threeObjects.get(id);
    if (threeObj && threeObj instanceof THREE.Mesh) {
      const posAttr = threeObj.geometry.getAttribute('position') as THREE.BufferAttribute;
      if (!posAttr || posAttr.count > 5000) {
        scene.remove(outline);
        hoverOutlines.delete(id);
        return;
      }
      outline.position.copy(threeObj.position);
      outline.quaternion.copy(threeObj.quaternion);
      outline.scale.copy(threeObj.scale).multiplyScalar(1.05);
    }
  });

  state.frameCount++;
  if (now - state.lastFpsTime >= 1000) {
    document.getElementById('fps-counter')!.textContent = `${state.frameCount} FPS`;
    state.frameCount = 0;
    state.lastFpsTime = now;
  }
}

// ═══════════════════════════════════════════════════════════════════
// Init
// ═══════════════════════════════════════════════════════════════════

onResize();
setTool('cursor');
animate();
updateOutliner();

// ═══════════════════════════════════════════════════════════════════
// Snap & Proportional Toggle Handlers
// ═══════════════════════════════════════════════════════════════════

document.getElementById('snap-toggle')!.addEventListener('click', (e) => {
  state.snapping.enabled = !state.snapping.enabled;
  (e.target as HTMLElement).classList.toggle('active', state.snapping.enabled);
  setStatus(`Snap: ${state.snapping.enabled ? 'ON' : 'OFF'}`);
});

document.querySelectorAll('.snap-btn').forEach(btn => {
  btn.addEventListener('click', (e) => {
    document.querySelectorAll('.snap-btn').forEach(b => b.classList.remove('active'));
    (e.target as HTMLElement).classList.add('active');
    state.snapping.type = (e.target as HTMLElement).dataset.snap!;
    setStatus(`Snap Type: ${state.snapping.type}`);
  });
});

document.getElementById('snap-size')!.addEventListener('change', (e) => {
  state.snapping.size = parseFloat((e.target as HTMLInputElement).value) || 0.1;
});

document.getElementById('prop-toggle')!.addEventListener('click', (e) => {
  state.proportional.enabled = !state.proportional.enabled;
  (e.target as HTMLElement).classList.toggle('active', state.proportional.enabled);
  proportionalMesh.visible = state.proportional.enabled;
  setStatus(`Proportional: ${state.proportional.enabled ? 'ON' : 'OFF'}`);
});

document.getElementById('prop-radius')!.addEventListener('change', (e) => {
  state.proportional.radius = parseFloat((e.target as HTMLInputElement).value) || 1.0;
});

document.getElementById('prop-falloff')!.addEventListener('change', (e) => {
  state.proportional.falloff = (e.target as HTMLSelectElement).value;
});

// ═══════════════════════════════════════════════════════════════════
// Material Handlers
// ═══════════════════════════════════════════════════════════════════

document.getElementById('mat-preset')!.addEventListener('change', (e) => {
  const preset = (e.target as HTMLSelectElement).value;
  pushUndo();
  state.selectedObjects.forEach(id => {
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh && materialPresets[preset]) {
      obj.material = materialPresets[preset].clone();
      if (state.selectedObjects.has(id)) {
        (obj.material as THREE.MeshStandardMaterial).emissive.setHex(0x003344);
        (obj.material as THREE.MeshStandardMaterial).emissiveIntensity = 0.3;
      }
    }
  });
});

document.getElementById('mat-color')!.addEventListener('input', (e) => {
  const color = (e.target as HTMLInputElement).value;
  state.selectedObjects.forEach(id => {
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) {
      (obj.material as THREE.MeshStandardMaterial).color.set(color);
    }
  });
});

document.getElementById('mat-metallic')!.addEventListener('input', (e) => {
  const val = parseFloat((e.target as HTMLInputElement).value);
  state.selectedObjects.forEach(id => {
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) {
      (obj.material as THREE.MeshStandardMaterial).metalness = val;
    }
  });
});

document.getElementById('mat-roughness')!.addEventListener('input', (e) => {
  const val = parseFloat((e.target as HTMLInputElement).value);
  state.selectedObjects.forEach(id => {
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) {
      (obj.material as THREE.MeshStandardMaterial).roughness = val;
    }
  });
});

document.getElementById('mat-opacity')!.addEventListener('input', (e) => {
  const val = parseFloat((e.target as HTMLInputElement).value);
  state.selectedObjects.forEach(id => {
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) {
      const mat = obj.material as THREE.MeshStandardMaterial;
      mat.transparent = val < 1.0;
      mat.opacity = val;
    }
  });
});

// ═══════════════════════════════════════════════════════════════════
// Modifier Button Handlers
// ═══════════════════════════════════════════════════════════════════

document.getElementById('btn-array')?.addEventListener('click', () => {
  if (state.selectedObjects.size !== 1) return;
  const id = Array.from(state.selectedObjects)[0];
  const threeObj = state.threeObjects.get(id);
  if (!(threeObj instanceof THREE.Mesh)) return;
  const geo = threeObj.geometry.clone();
  const mat = threeObj.material;
  const group = new THREE.Group();
  for (let i = 0; i < 4; i++) {
    const clone = new THREE.Mesh(geo.clone(), mat);
    clone.position.x = i * 1.2;
    group.add(clone);
  }
  scene.add(group);
  setStatus('Array modifier applied (visual)');
});

document.getElementById('btn-mirror')?.addEventListener('click', () => {
  if (state.selectedObjects.size !== 1) return;
  const id = Array.from(state.selectedObjects)[0];
  const threeObj = state.threeObjects.get(id);
  if (!(threeObj instanceof THREE.Mesh)) return;
  const clone = threeObj.clone();
  clone.scale.x = -1;
  scene.add(clone);
  setStatus('Mirror applied (visual)');
});

document.getElementById('btn-subsurf')?.addEventListener('click', () => {
  if (state.selectedObjects.size !== 1) return;
  setStatus('Subdivision Surface (backend)');
});

document.getElementById('btn-solidify')?.addEventListener('click', () => {
  setStatus('Solidify modifier (backend)');
});

document.getElementById('btn-triangulate')?.addEventListener('click', () => {
  state.selectedObjects.forEach(id => {
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) {
      const geo = obj.geometry;
      const posAttr = geo.getAttribute('position') as THREE.BufferAttribute;
      const indexArr = geo.index ? Array.from(geo.index.array) as number[] : [];
      const newGeo = new THREE.BufferGeometry();
      newGeo.setAttribute('position', posAttr);
      if (indexArr.length > 0) {
        const triangles: number[] = [];
        for (let i = 0; i < indexArr.length; i += 3) {
          triangles.push(indexArr[i], indexArr[i + 1], indexArr[i + 2]);
        }
        newGeo.setIndex(triangles);
      }
      obj.geometry = newGeo;
    }
  });
  setStatus('Triangulated');
});

// ═══════════════════════════════════════════════════════════════════
// Window state save/restore placeholder
// ═══════════════════════════════════════════════════════════════════

window.addEventListener('beforeunload', () => {
  try {
    localStorage.setItem('lili_camera', JSON.stringify({
      pos: camera.position.toArray(),
      target: controls.target.toArray(),
      fov: camera.fov,
    }));
  } catch (e) {}
});

try {
  const saved = localStorage.getItem('lili_camera');
  if (saved) {
    const data = JSON.parse(saved);
    const pos: number[] = data.pos;
    const target: number[] = data.target;
    camera.position.set(pos[0], pos[1], pos[2]);
    controls.target.set(target[0], target[1], target[2]);
    camera.fov = (data.fov as number) || 45;
    camera.updateProjectionMatrix();
  }
} catch (e) {}

// ═══════════════════════════════════════════════════════════════════
// Boolean Operations
// ═══════════════════════════════════════════════════════════════════

async function booleanOp(operation: 'union' | 'difference' | 'intersect') {
  if (state.selectedObjects.size !== 2) { setStatus('Select 2 objects for boolean'); return; }
  pushUndo();
  const ids = Array.from(state.selectedObjects);
  try {
    await invoke('boolean_operation', { args: { target_id: ids[0], operator_id: ids[1], operation } });
    const operatorObj = state.threeObjects.get(ids[1]);
    if (operatorObj) { scene.remove(operatorObj); threeObjectsInverse.delete(operatorObj); }
    state.threeObjects.delete(ids[1]); state.objects.delete(ids[1]);
    selectObject(ids[0]);
    setStatus(`Boolean ${operation} applied`);
  } catch(e) { setStatus(`Boolean error: ${e}`); }
}

document.getElementById('btn-boolean-union')?.addEventListener('click', () => booleanOp('union'));
document.getElementById('btn-boolean-diff')?.addEventListener('click', () => booleanOp('difference'));
document.getElementById('btn-boolean-intersect')?.addEventListener('click', () => booleanOp('intersect'));

document.getElementById('btn-boolean-apply')?.addEventListener('click', () => {
  const op = (document.getElementById('bool-op') as HTMLSelectElement)?.value || 'union';
  booleanOp(op as 'union' | 'difference' | 'intersect');
});

document.getElementById('btn-bevel-mod')?.addEventListener('click', () => {
  if (state.selectedObjects.size !== 1) return;
  pushUndo();
  const id = Array.from(state.selectedObjects)[0];
  const threeObj = state.threeObjects.get(id);
  if (!(threeObj instanceof THREE.Mesh)) return;
  const geo = threeObj.geometry;
  const posAttr = geo.getAttribute('position') as THREE.BufferAttribute;
  const indexAttr = geo.index;
  if (!indexAttr) return;
  const positions = posAttr.array as Float32Array;
  const indices = Array.from(indexAttr.array as Uint16Array | Uint32Array);
  const bevelAmount = 0.05;
  const newPositions: number[] = [];
  const newIndices: number[] = [];
  const edgeMidpoints = new Map<string, { pos: [number, number, number]; v0: number; v1: number }>();
  for (let i = 0; i < indices.length; i += 3) {
    const a = indices[i], b = indices[i + 1], c = indices[i + 2];
    const edges: [number, number][] = [[a, b], [b, c], [c, a]];
    for (const [v0, v1] of edges) {
      const key = Math.min(v0, v1) + '-' + Math.max(v0, v1);
      if (!edgeMidpoints.has(key)) {
        const mx = (positions[v0 * 3] + positions[v1 * 3]) / 2;
        const my = (positions[v0 * 3 + 1] + positions[v1 * 3 + 1]) / 2;
        const mz = (positions[v0 * 3 + 2] + positions[v1 * 3 + 2]) / 2;
        edgeMidpoints.set(key, { pos: [mx, my, mz], v0, v1 });
      }
    }
  }
  const baseIdx = posAttr.count;
  let addedVerts = 0;
  edgeMidpoints.forEach((data) => {
    const nx = data.pos[0] * bevelAmount;
    const ny = data.pos[1] * bevelAmount;
    const nz = data.pos[2] * bevelAmount;
    newPositions.push(
      positions[data.v0 * 3] + nx, positions[data.v0 * 3 + 1] + ny, positions[data.v0 * 3 + 2] + nz,
      positions[data.v1 * 3] + nx, positions[data.v1 * 3 + 1] + ny, positions[data.v1 * 3 + 2] + nz
    );
    newIndices.push(baseIdx + addedVerts, baseIdx + addedVerts + 1);
    addedVerts += 2;
  });
  const allPositions = new Float32Array(posAttr.count * 3 + newPositions.length);
  allPositions.set(positions);
  for (let i = 0; i < newPositions.length; i++) allPositions[posAttr.count * 3 + i] = newPositions[i];
  const allIndices = [...indices, ...newIndices];
  const newGeo = new THREE.BufferGeometry();
  newGeo.setAttribute('position', new THREE.BufferAttribute(allPositions, 3));
  newGeo.setIndex(allIndices);
  newGeo.computeVertexNormals();
  threeObj.geometry = newGeo;
  setStatus('Bevel modifier applied');
});

// ═══════════════════════════════════════════════════════════════════
// Duplicate Object
// ═══════════════════════════════════════════════════════════════════

async function duplicateObject() {
  if (state.selectedObjects.size !== 1) return;
  pushUndo();
  const id = Array.from(state.selectedObjects)[0];
  const obj = state.objects.get(id);
  const threeObj = state.threeObjects.get(id);
  if (!obj || !threeObj) return;

  const name = `${obj.name}_copy`;
  try {
    const newId = await createPrimitive('cube', name);
    const newThreeObj = (threeObj as THREE.Mesh).clone();
    newThreeObj.position.x += 1;
    scene.add(newThreeObj);
    state.threeObjects.set(newId, newThreeObj);
    state.threeObjectsInverse.set(newThreeObj, newId);
    state.objects.set(newId, { ...obj, id: newId, name, position: [obj.position[0]+1, obj.position[1], obj.position[2]] });
    selectObject(newId);
    setStatus(`Duplicated ${obj.name}`);
  } catch(e) { console.error(e); }
}

// ═══════════════════════════════════════════════════════════════════
// Shrink/Fatten (Scale along normals - uniform for now)
// ═══════════════════════════════════════════════════════════════════

function shrinkFatten() {
  state.selectedObjects.forEach(id => {
    const threeObj = state.threeObjects.get(id);
    if (threeObj) {
      const factor = 1.1;
      threeObj.scale.multiplyScalar(factor);
      const objData = state.objects.get(id);
      if (objData) {
        objData.scale = [threeObj.scale.x, threeObj.scale.y, threeObj.scale.z];
      }
    }
  });
  setStatus('Shrink/Fatten applied');
}

// ═══════════════════════════════════════════════════════════════════
// Select Grouped
// ═══════════════════════════════════════════════════════════════════

function selectGrouped() {
  if (state.selectedObjects.size === 0) return;
  const selectedId = Array.from(state.selectedObjects)[0];
  const selectedObj = state.objects.get(selectedId);
  if (!selectedObj) return;
  const baseName = selectedObj.name.replace(/_\d+$/, '');
  state.objects.forEach((obj, id) => {
    if (obj.name.startsWith(baseName)) {
      state.selectedObjects.add(id);
      const threeObj = state.threeObjects.get(id);
      if (threeObj instanceof THREE.Mesh) threeObj.material = materialPresets.selected.clone();
    }
  });
  updateOutliner();
  updateProperties();
  updateSelectionInfo();
  setStatus(`Selected grouped: ${baseName}*`);
}

// ═══════════════════════════════════════════════════════════════════
// Invert Selection
// ═══════════════════════════════════════════════════════════════════

function invertSelection() {
  const newSelection = new Set<string>();
  state.objects.forEach((_, id) => {
    if (state.selectedObjects.has(id)) {
      const threeObj = state.threeObjects.get(id);
      if (threeObj instanceof THREE.Mesh) threeObj.material = materialPresets.default.clone();
    } else {
      newSelection.add(id);
      const threeObj = state.threeObjects.get(id);
      if (threeObj instanceof THREE.Mesh) threeObj.material = materialPresets.selected.clone();
    }
  });
  state.selectedObjects = newSelection;
  updateOutliner();
  updateProperties();
  updateSelectionInfo();
  setStatus(`Inverted selection: ${state.selectedObjects.size} selected`);
}

// ═══════════════════════════════════════════════════════════════════
// Timeline Functions
// ═══════════════════════════════════════════════════════════════════

function playTimeline() {
  state.timeline.playing = true;
  const playBtn = document.getElementById('timeline-play');
  if (playBtn) playBtn.textContent = '⏸';
  setStatus('Timeline playing');
}

function stopTimeline() {
  state.timeline.playing = false;
  const playBtn = document.getElementById('timeline-play');
  if (playBtn) playBtn.textContent = '▶';
  setStatus('Timeline stopped');
}

function toggleTimelinePlay() {
  if (state.timeline.playing) stopTimeline();
  else playTimeline();
}

function setFrame(n: number) {
  state.timeline.frame = Math.max(state.timeline.startFrame, Math.min(state.timeline.endFrame, n));
  const frameInput = document.getElementById('timeline-frame') as HTMLInputElement | null;
  if (frameInput) frameInput.value = state.timeline.frame.toString();
  renderTimeline();
}

function stepFrame(delta: number) {
  setFrame(state.timeline.frame + delta);
}

function addKeyframe() {
  if (state.selectedObjects.size === 0) return;
  state.selectedObjects.forEach(id => {
    const frames = state.timeline.keyframes.get(id) || [];
    if (!frames.includes(state.timeline.frame)) {
      frames.push(state.timeline.frame);
      frames.sort((a, b) => a - b);
      state.timeline.keyframes.set(id, frames);
    }
  });
  renderTimeline();
  setStatus(`Keyframe added at frame ${state.timeline.frame}`);
}

function removeKeyframe() {
  state.selectedObjects.forEach(id => {
    const frames = state.timeline.keyframes.get(id);
    if (frames) {
      const idx = frames.indexOf(state.timeline.frame);
      if (idx !== -1) frames.splice(idx, 1);
    }
  });
  renderTimeline();
  setStatus(`Keyframe removed at frame ${state.timeline.frame}`);
}

// ═══════════════════════════════════════════════════════════════════
// Timeline Render
// ═══════════════════════════════════════════════════════════════════

function renderTimeline() {
  const container = document.getElementById('timeline-track');
  if (!container) return;
  container.innerHTML = '';
  const { startFrame, endFrame, frame, keyframes } = state.timeline;
  const totalFrames = endFrame - startFrame + 1;
  const pxPerFrame = 4;

  container.style.width = `${totalFrames * pxPerFrame}px`;

  for (let f = startFrame; f <= endFrame; f++) {
    const tick = document.createElement('div');
    tick.className = 'timeline-tick';
    tick.style.left = `${(f - startFrame) * pxPerFrame}px`;
    if (f === frame) tick.classList.add('current');
    if (f % 10 === 0) tick.classList.add('major');
    container.appendChild(tick);
  }

  keyframes.forEach((frames, _id) => {
    frames.forEach(f => {
      if (f < startFrame || f > endFrame) return;
      const diamond = document.createElement('div');
      diamond.className = 'timeline-keyframe';
      diamond.style.left = `${(f - startFrame) * pxPerFrame - 4}px`;
      diamond.title = `Keyframe at ${f}`;
      container.appendChild(diamond);
    });
  });

  const playhead = document.getElementById('timeline-playhead');
  if (playhead) {
    playhead.style.left = `${(frame - startFrame) * pxPerFrame}px`;
  }
}

// ═══════════════════════════════════════════════════════════════════
// Viewport Hover Highlighting & Selection Outline
// ═══════════════════════════════════════════════════════════════════

const hoverOutlineMat = new THREE.MeshBasicMaterial({
  color: 0x4a9eff,
  transparent: true,
  opacity: 0.3,
  side: THREE.BackSide,
  depthTest: true,
});

const selectionOutlineMat = new THREE.MeshBasicMaterial({
  color: 0xff6b35,
  transparent: true,
  opacity: 0.5,
  side: THREE.BackSide,
  depthTest: true,
});

const importedGeometryCache = new Map<string, { positions: number[], indices: number[], normals: number[] }>();

function createOutlineMesh(id: string, threeObj: THREE.Object3D, material: THREE.Material): THREE.Mesh | null {
  if (!(threeObj instanceof THREE.Mesh)) return null;
  const geo = threeObj.geometry;
  const posAttr = geo.getAttribute('position') as THREE.BufferAttribute;
  if (!posAttr || posAttr.count > 5000) return null;
  const outlineGeo = geo.clone();
  const outline = new THREE.Mesh(outlineGeo, material.clone());
  outline.position.copy(threeObj.position);
  outline.quaternion.copy(threeObj.quaternion);
  outline.scale.copy(threeObj.scale).multiplyScalar(1.05);
  outline.renderOrder = -1;
  return outline;
}

function updateHoverOutline(id: string | null) {
  // Remove old hover outline
  if (state.hoverObject && state.hoverObject !== id) {
    const oldOutline = hoverOutlines.get(state.hoverObject);
    if (oldOutline) {
      scene.remove(oldOutline);
      hoverOutlines.delete(state.hoverObject);
    }
  }

  state.hoverObject = id;

  // Add new hover outline
  if (id && !state.selectedObjects.has(id)) {
    const threeObj = state.threeObjects.get(id);
    if (threeObj && !hoverOutlines.has(id)) {
      const outline = createOutlineMesh(id, threeObj, hoverOutlineMat);
      if (outline) {
        scene.add(outline);
        hoverOutlines.set(id, outline);
      }
    }
  }
}

function updateSelectionOutlines() {
  // Remove outlines for deselected objects
  selectionOutlines.forEach((outline, id) => {
    if (!state.selectedObjects.has(id)) {
      scene.remove(outline);
      selectionOutlines.delete(id);
    }
  });

  // Add outlines for selected objects
  state.selectedObjects.forEach(id => {
    if (!selectionOutlines.has(id)) {
      const threeObj = state.threeObjects.get(id);
      if (threeObj) {
        const outline = createOutlineMesh(id, threeObj, selectionOutlineMat);
        if (outline) {
          scene.add(outline);
          selectionOutlines.set(id, outline);
        }
      }
    }
  });
}

canvas.addEventListener('mousemove', (e) => {
  if (state.mode !== 'object') return;
  if (state.tool !== 'cursor') return;

  const rect = canvas.getBoundingClientRect();
  const mouse = new THREE.Vector2(
    ((e.clientX - rect.left) / rect.width) * 2 - 1,
    -((e.clientY - rect.top) / rect.height) * 2 + 1
  );
  const raycaster = new THREE.Raycaster();
  raycaster.setFromCamera(mouse, camera);
  const meshObjects = Array.from(state.threeObjects.values()).filter(o => o instanceof THREE.Mesh && o.visible);
  const intersects = raycaster.intersectObjects(meshObjects);

  if (intersects.length > 0) {
    const hit = intersects[0].object;
    const id = threeObjectsInverse.get(hit);
    updateHoverOutline(id || null);
    canvas.style.cursor = id ? 'pointer' : 'default';
  } else {
    updateHoverOutline(null);
    canvas.style.cursor = 'default';
  }
});

// ═══════════════════════════════════════════════════════════════════
// Sculpt Brush Visualization
// ═══════════════════════════════════════════════════════════════════

const sculptBrushRing = new THREE.RingGeometry(1, 1.02, 64);
const sculptBrushMat = new THREE.MeshBasicMaterial({ color: 0xff6b35, side: THREE.DoubleSide, transparent: true, opacity: 0.5 });
const sculptBrushMesh = new THREE.Mesh(sculptBrushRing, sculptBrushMat);
sculptBrushMesh.visible = false;
sculptBrushMesh.rotation.x = -Math.PI / 2;
scene.add(sculptBrushMesh);

function updateSculptBrush(e: MouseEvent) {
  if (state.mode !== 'sculpt') { sculptBrushMesh.visible = false; return; }
  const rect = canvas.getBoundingClientRect();
  const mouse = new THREE.Vector2(
    ((e.clientX - rect.left) / rect.width) * 2 - 1,
    -((e.clientY - rect.top) / rect.height) * 2 + 1
  );
  const raycaster = new THREE.Raycaster();
  raycaster.setFromCamera(mouse, camera);
  const meshObjects = Array.from(state.threeObjects.values()).filter(o => o instanceof THREE.Mesh && o.visible);
  const intersects = raycaster.intersectObjects(meshObjects);
  if (intersects.length > 0) {
    sculptBrushMesh.visible = true;
    sculptBrushMesh.position.copy(intersects[0].point);
    sculptBrushMesh.position.y += 0.01;
    sculptBrushMesh.scale.setScalar(state.sculpt.size);
  } else {
    sculptBrushMesh.visible = false;
  }
}

// ═══════════════════════════════════════════════════════════════════
// Context Menu
// ═══════════════════════════════════════════════════════════════════

canvas.addEventListener('contextmenu', (e) => {
  e.preventDefault();
  const menu = document.getElementById('context-menu');
  if (!menu) return;
  menu.style.left = `${e.clientX}px`;
  menu.style.top = `${e.clientY}px`;
  menu.classList.remove('hidden');
  state.contextMenuVisible = true;
});

document.addEventListener('click', (e) => {
  const menu = document.getElementById('context-menu');
  if (menu && state.contextMenuVisible && !(e.target as HTMLElement).closest('#context-menu')) {
    menu.classList.add('hidden');
    state.contextMenuVisible = false;
  }
});

document.getElementById('ctx-duplicate')?.addEventListener('click', () => {
  duplicateObject();
  document.getElementById('context-menu')?.classList.add('hidden');
});

document.getElementById('ctx-delete')?.addEventListener('click', () => {
  deleteSelected();
  document.getElementById('context-menu')?.classList.add('hidden');
});

document.getElementById('ctx-shade-smooth')?.addEventListener('click', () => {
  pushUndo();
  state.selectedObjects.forEach(id => {
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) {
      (obj.material as THREE.MeshStandardMaterial).flatShading = false;
      obj.geometry.computeVertexNormals();
    }
  });
  document.getElementById('context-menu')?.classList.add('hidden');
});

document.getElementById('ctx-shade-flat')?.addEventListener('click', () => {
  pushUndo();
  state.selectedObjects.forEach(id => {
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) {
      (obj.material as THREE.MeshStandardMaterial).flatShading = true;
    }
  });
  document.getElementById('context-menu')?.classList.add('hidden');
});

// ═══════════════════════════════════════════════════════════════════
// Viewport Shading Modes
// ═══════════════════════════════════════════════════════════════════

function setViewportShading(mode: typeof state.viewportShading) {
  state.viewportShading = mode;

  // Update UI buttons
  document.querySelectorAll('.shading-btn').forEach(btn => {
    (btn as HTMLElement).classList.toggle('active', (btn as HTMLElement).dataset.shading === mode);
  });

  // Apply shading to all objects
  state.threeObjects.forEach((obj, id) => {
    if (!(obj instanceof THREE.Mesh)) return;

    const isSelected = state.selectedObjects.has(id);

    switch (mode) {
      case 'solid':
        if (isSelected) {
          obj.material = materialPresets.selected.clone();
        } else {
          obj.material = materialPresets.default.clone();
        }
        (obj.material as THREE.MeshStandardMaterial).wireframe = false;
        break;
      case 'wireframe':
        if (isSelected) {
          obj.material = materialPresets.selected.clone();
        } else {
          obj.material = materialPresets.default.clone();
        }
        (obj.material as THREE.MeshStandardMaterial).wireframe = true;
        break;
      case 'material':
        if (isSelected) {
          obj.material = materialPresets.selected.clone();
        } else {
          obj.material = materialPresets.default.clone();
        }
        (obj.material as THREE.MeshStandardMaterial).wireframe = false;
        break;
      case 'rendered':
        if (isSelected) {
          obj.material = materialPresets.selected.clone();
        } else {
          obj.material = materialPresets.default.clone();
        }
        (obj.material as THREE.MeshStandardMaterial).wireframe = false;
        break;
    }
  });

  // Update lighting based on mode
  switch (mode) {
    case 'solid':
      ambientLight.intensity = 0.4;
      keyLight.intensity = 1.8;
      fillLight.intensity = 0.6;
      rimLight.intensity = 0.4;
      hemisphereLight.intensity = 0.3;
      scene.background = new THREE.Color(0x1a1a2e);
      scene.fog = new THREE.Fog(0x1a1a2e, 50, 200);
      break;
    case 'wireframe':
      ambientLight.intensity = 0.8;
      keyLight.intensity = 0.5;
      fillLight.intensity = 0.3;
      rimLight.intensity = 0.2;
      hemisphereLight.intensity = 0.1;
      scene.background = new THREE.Color(0x111122);
      scene.fog = new THREE.Fog(0x111122, 80, 300);
      break;
    case 'material':
      ambientLight.intensity = 0.6;
      keyLight.intensity = 2.0;
      fillLight.intensity = 0.8;
      rimLight.intensity = 0.5;
      hemisphereLight.intensity = 0.4;
      scene.background = new THREE.Color(0x1a1a2e);
      scene.fog = new THREE.Fog(0x1a1a2e, 50, 200);
      break;
    case 'rendered':
      ambientLight.intensity = 0.5;
      keyLight.intensity = 2.5;
      fillLight.intensity = 0.7;
      rimLight.intensity = 0.6;
      hemisphereLight.intensity = 0.5;
      scene.background = new THREE.Color(0x1a1a2e);
      scene.fog = new THREE.Fog(0x1a1a2e, 50, 200);
      break;
  }

  setStatus(`Viewport shading: ${mode.charAt(0).toUpperCase() + mode.slice(1)}`);
}

// Shading mode buttons
document.getElementById('shading-solid')?.addEventListener('click', () => setViewportShading('solid'));
document.getElementById('shading-wireframe')?.addEventListener('click', () => setViewportShading('wireframe'));
document.getElementById('shading-material')?.addEventListener('click', () => setViewportShading('material'));
document.getElementById('shading-rendered')?.addEventListener('click', () => setViewportShading('rendered'));

// ═══════════════════════════════════════════════════════════════════
// Viewport Overlay Toggles
// ═══════════════════════════════════════════════════════════════════

document.getElementById('overlay-wireframe')?.addEventListener('click', (e) => {
  state.overlay.wireframe = !state.overlay.wireframe;
  (e.target as HTMLElement).classList.toggle('active', state.overlay.wireframe);
  state.threeObjects.forEach(obj => {
    if (obj instanceof THREE.Mesh) {
      (obj.material as THREE.MeshStandardMaterial).wireframe = state.overlay.wireframe;
    }
  });
  setStatus(`Wireframe: ${state.overlay.wireframe ? 'ON' : 'OFF'}`);
});

document.getElementById('overlay-normals')?.addEventListener('click', (e) => {
  state.overlay.normals = !state.overlay.normals;
  (e.target as HTMLElement).classList.toggle('active', state.overlay.normals);
  setStatus(`Normals display: ${state.overlay.normals ? 'ON' : 'OFF'}`);
});

document.getElementById('overlay-face-orientation')?.addEventListener('click', (e) => {
  state.overlay.faceOrientation = !state.overlay.faceOrientation;
  (e.target as HTMLElement).classList.toggle('active', state.overlay.faceOrientation);
  setStatus(`Face orientation: ${state.overlay.faceOrientation ? 'ON' : 'OFF'}`);
});

document.getElementById('overlay-statistics')?.addEventListener('click', (e) => {
  state.overlay.statistics = !state.overlay.statistics;
  (e.target as HTMLElement).classList.toggle('active', state.overlay.statistics);
  const statsEl = document.getElementById('viewport-statistics');
  if (statsEl) statsEl.style.display = state.overlay.statistics ? 'block' : 'none';
  if (state.overlay.statistics) {
    let totalVerts = 0, totalFaces = 0;
    state.objects.forEach(obj => { totalVerts += obj.vertex_count; totalFaces += obj.face_count; });
    statsEl!.textContent = `Objects: ${state.objects.size} | Verts: ${totalVerts} | Faces: ${totalFaces}`;
  }
  setStatus(`Statistics overlay: ${state.overlay.statistics ? 'ON' : 'OFF'}`);
});

// ═══════════════════════════════════════════════════════════════════
// Sculpt Mode Brush Handlers
// ═══════════════════════════════════════════════════════════════════

document.querySelectorAll('.sculpt-brush-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    document.querySelectorAll('.sculpt-brush-btn').forEach(b => b.classList.remove('active'));
    (btn as HTMLElement).classList.add('active');
    state.sculpt.brush = (btn as HTMLElement).dataset.brush || 'draw';
    setStatus(`Sculpt brush: ${state.sculpt.brush}`);
  });
});

document.getElementById('sculpt-size')?.addEventListener('input', (e) => {
  state.sculpt.size = parseFloat((e.target as HTMLInputElement).value) || 0.5;
});

document.getElementById('sculpt-strength')?.addEventListener('input', (e) => {
  state.sculpt.strength = parseFloat((e.target as HTMLInputElement).value) || 0.5;
});

// ═══════════════════════════════════════════════════════════════════
// Timeline UI Handlers
// ═══════════════════════════════════════════════════════════════════

document.getElementById('timeline-play')?.addEventListener('click', toggleTimelinePlay);
document.getElementById('timeline-stop')?.addEventListener('click', stopTimeline);
document.getElementById('timeline-rewind')?.addEventListener('click', () => setFrame(state.timeline.startFrame));
document.getElementById('timeline-add-keyframe')?.addEventListener('click', addKeyframe);
document.getElementById('timeline-remove-keyframe')?.addEventListener('click', removeKeyframe);
document.getElementById('timeline-frame')?.addEventListener('change', (e) => {
  setFrame(parseInt((e.target as HTMLInputElement).value) || 1);
});
document.getElementById('timeline-next')?.addEventListener('click', () => stepFrame(1));
document.getElementById('timeline-prev')?.addEventListener('click', () => stepFrame(-1));
document.getElementById('timeline-end')?.addEventListener('click', () => setFrame(state.timeline.endFrame));

// ═══════════════════════════════════════════════════════════════════
// Physics Panel Handlers
// ═══════════════════════════════════════════════════════════════════

document.getElementById('physics-toggle')?.addEventListener('click', (e) => {
  state.physics.enabled = !state.physics.enabled;
  (e.target as HTMLElement).classList.toggle('active', state.physics.enabled);
  setStatus(`Physics: ${state.physics.enabled ? 'ON' : 'OFF'}`);
});

document.getElementById('physics-type')?.addEventListener('change', (e) => {
  state.physics.type = (e.target as HTMLSelectElement).value as 'active' | 'passive';
});

document.getElementById('physics-mass')?.addEventListener('input', (e) => {
  state.physics.mass = parseFloat((e.target as HTMLInputElement).value) || 1.0;
});

document.getElementById('physics-friction')?.addEventListener('input', (e) => {
  state.physics.friction = parseFloat((e.target as HTMLInputElement).value) || 0.5;
});

document.getElementById('physics-bounciness')?.addEventListener('input', (e) => {
  state.physics.bounciness = parseFloat((e.target as HTMLInputElement).value) || 0.3;
});

document.getElementById('physics-shape')?.addEventListener('change', (e) => {
  state.physics.shape = (e.target as HTMLSelectElement).value;
});

// ═══════════════════════════════════════════════════════════════════
// Outliner Search Handler
// ═══════════════════════════════════════════════════════════════════

document.getElementById('outliner-search')?.addEventListener('input', () => {
  updateOutliner();
});

// ═══════════════════════════════════════════════════════════════════
// Sculpt mode mouse move
// ═══════════════════════════════════════════════════════════════════

canvas.addEventListener('mousemove', (e) => {
  if (state.mode === 'sculpt') {
    updateSculptBrush(e);
    if (state.isSculpting) {
      applySculptStroke(e);
    }
  }
});

canvas.addEventListener('mousedown', (e) => {
  if (state.mode === 'sculpt' && e.button === 0) {
    state.isSculpting = true;
    state.lastSculptPoint = null;
  }
});

canvas.addEventListener('mouseup', (e) => {
  if (state.mode === 'sculpt') {
    state.isSculpting = false;
    state.lastSculptPoint = null;
  }
});

function applySculptStroke(e: MouseEvent) {
  const rect = canvas.getBoundingClientRect();
  const mouse = new THREE.Vector2(
    ((e.clientX - rect.left) / rect.width) * 2 - 1,
    -((e.clientY - rect.top) / rect.height) * 2 + 1
  );
  const raycaster = new THREE.Raycaster();
  raycaster.setFromCamera(mouse, camera);
  const meshObjects = Array.from(state.threeObjects.values()).filter(o => o instanceof THREE.Mesh && o.visible);
  const intersects = raycaster.intersectObjects(meshObjects);

  if (intersects.length === 0) return;

  const hit = intersects[0];
  const hitPoint = hit.point;
  const hitObject = hit.object as THREE.Mesh;

  if (!(hitObject instanceof THREE.Mesh)) return;

  const id = threeObjectsInverse.get(hitObject);
  if (!id) return;

  const geo = hitObject.geometry;
  const posAttr = geo.getAttribute('position') as THREE.BufferAttribute;
  const positions = posAttr.array as Float32Array;
  const normalAttr = geo.getAttribute('normal') as THREE.BufferAttribute;
  const normals = normalAttr ? normalAttr.array as Float32Array : null;

  const brushSize = state.sculpt.size;
  const brushStrength = state.sculpt.strength;

  // Get hit normal
  const hitNormal = hit.face?.normal
    ? hit.face.normal.clone().applyMatrix4(hitObject.matrixWorld).normalize()
    : new THREE.Vector3(0, 1, 0);

  let modified = false;

  for (let i = 0; i < posAttr.count; i++) {
    const vx = positions[i * 3];
    const vy = positions[i * 3 + 1];
    const vz = positions[i * 3 + 2];

    const worldPos = new THREE.Vector3(vx, vy, vz).applyMatrix4(hitObject.matrixWorld);
    const dist = worldPos.distanceTo(hitPoint);

    if (dist < brushSize) {
      const falloff = 1.0 - (dist / brushSize);
      const factor = falloff * falloff * brushStrength * 0.02;

      let dx = 0, dy = 0, dz = 0;

      switch (state.sculpt.brush) {
        case 'draw':
          dx = hitNormal.x * factor;
          dy = hitNormal.y * factor;
          dz = hitNormal.z * factor;
          break;
        case 'grab':
          if (state.lastSculptPoint) {
            const delta = hitPoint.clone().sub(state.lastSculptPoint);
            dx = delta.x * falloff * brushStrength;
            dy = delta.y * falloff * brushStrength;
            dz = delta.z * falloff * brushStrength;
          }
          break;
        case 'smooth':
          if (normals) {
            const nx = normals[i * 3], ny = normals[i * 3 + 1], nz = normals[i * 3 + 2];
            dx = -nx * factor * 0.5;
            dy = -ny * factor * 0.5;
            dz = -nz * factor * 0.5;
          }
          break;
        case 'inflate':
          dx = hitNormal.x * factor;
          dy = hitNormal.y * factor;
          dz = hitNormal.z * factor;
          break;
        case 'pinch':
          const toCenter = hitPoint.clone().sub(worldPos).normalize();
          dx = toCenter.x * factor * 0.3;
          dy = toCenter.y * factor * 0.3;
          dz = toCenter.z * factor * 0.3;
          break;
        case 'flatten':
          const projected = hitPoint.clone().sub(worldPos);
          const dot = projected.dot(hitNormal);
          dx = -hitNormal.x * dot * falloff * brushStrength * 0.5;
          dy = -hitNormal.y * dot * falloff * brushStrength * 0.5;
          dz = -hitNormal.z * dot * falloff * brushStrength * 0.5;
          break;
        default:
          dx = hitNormal.x * factor;
          dy = hitNormal.y * factor;
          dz = hitNormal.z * factor;
      }

      positions[i * 3] += dx;
      positions[i * 3 + 1] += dy;
      positions[i * 3 + 2] += dz;
      modified = true;
    }
  }

  if (modified) {
    posAttr.needsUpdate = true;
    geo.computeVertexNormals();
    if (normalAttr) normalAttr.needsUpdate = true;
  }

  state.lastSculptPoint = hitPoint.clone();
}

setStatus('Lili Modeler v0.3.0 - Ready');

// ═══════════════════════════════════════════════
// COMMAND PALETTE (Ctrl+Space)
// ═══════════════════════════════════════════════
const commandPalette = document.getElementById('command-palette')!;
const commandSearch = document.getElementById('command-search') as HTMLInputElement;
const commandList = document.getElementById('command-list')!;
let cmdActiveIndex = 0;

interface CommandDef {
  label: string;
  icon: string;
  shortcut?: string;
  group: string;
  action: () => void;
}

function shadeSmooth() {
  pushUndo();
  state.selectedObjects.forEach(id => {
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) {
      (obj.material as THREE.MeshStandardMaterial).flatShading = false;
      obj.geometry.computeVertexNormals();
    }
  });
}

function shadeFlat() {
  pushUndo();
  state.selectedObjects.forEach(id => {
    const obj = state.threeObjects.get(id);
    if (obj instanceof THREE.Mesh) {
      (obj.material as THREE.MeshStandardMaterial).flatShading = true;
    }
  });
}

async function addModifier(type: string) {
  if (state.selectedObjects.size === 0) return;
  pushUndo();
  for (const id of state.selectedObjects) {
    try { await invoke('add_modifier', { objectId: id, modifierType: type }); } catch { /* fallback */ }
  }
  setStatus(`Added ${type} modifier`);
}

const commands: CommandDef[] = [
  { label: 'Add Cube', icon: '+', shortcut: 'Shift+A', group: 'Add', action: () => addPrimitive('cube') },
  { label: 'Add Sphere', icon: '+', group: 'Add', action: () => addPrimitive('sphere') },
  { label: 'Add Cylinder', icon: '+', group: 'Add', action: () => addPrimitive('cylinder') },
  { label: 'Add Torus', icon: '+', group: 'Add', action: () => addPrimitive('torus') },
  { label: 'Add Cone', icon: '+', group: 'Add', action: () => addPrimitive('cone') },
  { label: 'Add Plane', icon: '+', group: 'Add', action: () => addPrimitive('plane') },
  { label: 'Add Grid', icon: '+', group: 'Add', action: () => addPrimitive('grid') },
  { label: 'Add Ico Sphere', icon: '+', group: 'Add', action: () => addPrimitive('ico-sphere') },
  { label: 'Add UV Sphere', icon: '+', group: 'Add', action: () => addPrimitive('uv-sphere') },
  { label: 'Delete Selected', icon: '\u2715', shortcut: 'X', group: 'Edit', action: deleteSelected },
  { label: 'Duplicate', icon: '\u2398', shortcut: 'Ctrl+D', group: 'Edit', action: duplicateObject },
  { label: 'Undo', icon: '\u21B6', shortcut: 'Ctrl+Z', group: 'Edit', action: undoAction },
  { label: 'Redo', icon: '\u21B7', shortcut: 'Ctrl+Y', group: 'Edit', action: redoAction },
  { label: 'Toggle Edit Mode', icon: '\u25A3', shortcut: 'Tab', group: 'Mode', action: toggleMode },
  { label: 'Sculpt Mode', icon: '\u270E', group: 'Mode', action: () => setMode('sculpt') },
  { label: 'Object Mode', icon: '\u25CB', group: 'Mode', action: () => setMode('object') },
  { label: 'Shade Smooth', icon: '\u25D4', group: 'Object', action: shadeSmooth },
  { label: 'Shade Flat', icon: '\u25CB', group: 'Object', action: shadeFlat },
  { label: 'Boolean Union', icon: '\u222A', group: 'Boolean', action: () => booleanOp('union') },
  { label: 'Boolean Difference', icon: '\u2216', group: 'Boolean', action: () => booleanOp('difference') },
  { label: 'Boolean Intersect', icon: '\u2229', group: 'Boolean', action: () => booleanOp('intersect') },
  { label: 'Import OBJ', icon: '\u21E7', group: 'File', action: () => document.getElementById('file-import')?.click() },
  { label: 'Import STL', icon: '\u21E7', group: 'File', action: () => document.getElementById('file-import')?.click() },
  { label: 'Import GLTF', icon: '\u21E7', group: 'File', action: () => document.getElementById('file-import')?.click() },
  { label: 'Export OBJ', icon: '\u21E9', group: 'File', action: () => document.getElementById('file-export-obj')?.click() },
  { label: 'Export STL', icon: '\u21E9', group: 'File', action: () => document.getElementById('file-export-stl')?.click() },
  { label: 'Export GLTF', icon: '\u21E9', group: 'File', action: () => document.getElementById('file-export-gltf')?.click() },
  { label: 'Wireframe', icon: '\u25A1', group: 'Viewport', action: () => setViewportShading('wireframe') },
  { label: 'Solid', icon: '\u25A0', group: 'Viewport', action: () => setViewportShading('solid') },
  { label: 'Material Preview', icon: '\u25C6', group: 'Viewport', action: () => setViewportShading('material') },
  { label: 'Rendered', icon: '\u2600', group: 'Viewport', action: () => setViewportShading('rendered') },
  { label: 'Measure Tool', icon: '\u2571', shortcut: 'M', group: 'Tool', action: () => setTool('measure') },
  { label: 'Knife Tool', icon: '\u2E22', shortcut: 'K', group: 'Tool', action: () => setTool('knife') },
  { label: 'Play Animation', icon: '\u25B6', shortcut: 'Space', group: 'Animation', action: () => startPlayback() },
  { label: 'Stop Animation', icon: '\u25A0', shortcut: 'Esc', group: 'Animation', action: stopTimeline },
  { label: 'Add Array Modifier', icon: '\u2261', group: 'Modifier', action: () => addModifier('array') },
  { label: 'Add Mirror Modifier', icon: '\u257E', group: 'Modifier', action: () => addModifier('mirror') },
  { label: 'Add Subdivision', icon: '\u2573', group: 'Modifier', action: () => addModifier('subdivision') },
];

function renderCommandPalette(filter: string = '') {
  const lower = filter.toLowerCase();
  const filtered = commands.filter(c => c.label.toLowerCase().includes(lower));
  cmdActiveIndex = 0;

  let html = '';
  let lastGroup = '';
  filtered.forEach((cmd, i) => {
    if (cmd.group !== lastGroup) {
      html += `<div class="cmd-group-label">${cmd.group}</div>`;
      lastGroup = cmd.group;
    }
    html += `<div class="cmd-item${i === 0 ? ' active' : ''}" data-cmd="${i}">
      <span class="cmd-icon">${cmd.icon}</span>
      <span>${cmd.label}</span>
      ${cmd.shortcut ? `<span class="cmd-shortcut">${cmd.shortcut}</span>` : ''}
    </div>`;
  });
  commandList.innerHTML = html || '<div style="padding:12px;color:var(--text-dim);text-align:center;">No commands found</div>';

  commandList.querySelectorAll('.cmd-item').forEach(el => {
    el.addEventListener('click', () => {
      const idx = parseInt(el.getAttribute('data-cmd')!);
      filtered[idx]?.action();
      hideCommandPalette();
    });
  });
}

function showCommandPalette() {
  commandPalette.classList.remove('hidden');
  commandSearch.value = '';
  renderCommandPalette();
  commandSearch.focus();
}

function hideCommandPalette() {
  commandPalette.classList.add('hidden');
  commandSearch.blur();
}

commandSearch?.addEventListener('input', () => renderCommandPalette(commandSearch.value));

commandSearch?.addEventListener('keydown', (e: KeyboardEvent) => {
  const items = commandList.querySelectorAll('.cmd-item');
  if (e.key === 'Escape') { hideCommandPalette(); return; }
  if (e.key === 'ArrowDown') {
    e.preventDefault();
    cmdActiveIndex = Math.min(cmdActiveIndex + 1, items.length - 1);
    items.forEach((el, i) => el.classList.toggle('active', i === cmdActiveIndex));
  } else if (e.key === 'ArrowUp') {
    e.preventDefault();
    cmdActiveIndex = Math.max(cmdActiveIndex - 1, 0);
    items.forEach((el, i) => el.classList.toggle('active', i === cmdActiveIndex));
  } else if (e.key === 'Enter') {
    e.preventDefault();
    items[cmdActiveIndex]?.dispatchEvent(new Event('click'));
  }
});

document.addEventListener('keydown', (e: KeyboardEvent) => {
  if (e.ctrlKey && e.code === 'Space') {
    e.preventDefault();
    if (commandPalette.classList.contains('hidden')) showCommandPalette();
    else hideCommandPalette();
  }
});

// ═══════════════════════════════════════════════
// QUICK-ADD MENU (topbar + button)
// ═══════════════════════════════════════════════
const quickAddMenu = document.getElementById('quick-add-menu')!;
const addMenuBtn = document.getElementById('add-menu-btn') as HTMLButtonElement;

function toggleQuickAddMenu() {
  if (quickAddMenu.classList.contains('hidden')) {
    const rect = addMenuBtn.getBoundingClientRect();
    quickAddMenu.style.left = rect.left + 'px';
    quickAddMenu.style.top = (rect.bottom + 4) + 'px';
    quickAddMenu.style.transform = 'none';
    quickAddMenu.classList.remove('hidden');
  } else {
    quickAddMenu.classList.add('hidden');
  }
}

addMenuBtn?.addEventListener('click', (e) => {
  e.stopPropagation();
  toggleQuickAddMenu();
});

quickAddMenu.querySelectorAll('.dropdown-item').forEach(btn => {
  btn.addEventListener('click', () => {
    const type = btn.getAttribute('data-add');
    if (type) addPrimitive(type);
    quickAddMenu.classList.add('hidden');
  });
});

document.addEventListener('click', () => quickAddMenu.classList.add('hidden'));

// Search button opens command palette
document.getElementById('search-btn')?.addEventListener('click', (e) => {
  e.stopPropagation();
  showCommandPalette();
});

// ═══════════════════════════════════════════════
// STATUS BAR
// ═══════════════════════════════════════════════
const statusMode = document.getElementById('status-mode');
const statusObjects = document.getElementById('status-objects');
const statusVerts = document.getElementById('status-verts');
const statusCoords = document.getElementById('status-coords');

function updateStatusBar() {
  if (statusMode) statusMode.textContent = state.mode === 'object' ? 'Object Mode' : state.mode === 'edit' ? `Edit Mode (${state.selectMode})` : 'Sculpt Mode';
  if (statusObjects) statusObjects.textContent = `${state.objects.size} object${state.objects.size !== 1 ? 's' : ''}`;
  let totalVerts = 0;
  state.objects.forEach((_o, id) => { const m = state.threeObjects.get(id); if (m && (m as any).geometry) totalVerts += ((m as any).geometry as THREE.BufferGeometry).attributes.position.count; });
  if (statusVerts) statusVerts.textContent = `${totalVerts} verts`;
}
updateStatusBar();
setInterval(updateStatusBar, 500);

// Mouse coordinates in viewport
renderer.domElement.addEventListener('mousemove', (e: MouseEvent) => {
  if (!statusCoords) return;
  const rect = renderer.domElement.getBoundingClientRect();
  const ndcX = ((e.clientX - rect.left) / rect.width) * 2 - 1;
  const ndcY = -((e.clientY - rect.top) / rect.height) * 2 + 1;
  const rc = new THREE.Raycaster();
  rc.setFromCamera(new THREE.Vector2(ndcX, ndcY), camera);
  const plane = new THREE.Plane(new THREE.Vector3(0, 0, 1), 0);
  const target = new THREE.Vector3();
  rc.ray.intersectPlane(plane, target);
  if (target) statusCoords.textContent = `${target.x.toFixed(3)}, ${target.y.toFixed(3)}, ${target.z.toFixed(3)}`;
});

function startPlayback() {
  if (state.timeline.playing) return;
  state.timeline.playing = true;
}
