import * as THREE from 'https://cdn.jsdelivr.net/npm/three@0.179.1/build/three.module.js';
import { OrbitControls } from 'https://cdn.jsdelivr.net/npm/three@0.179.1/examples/jsm/controls/OrbitControls.js';

const canvas = document.querySelector('#scene');
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(50, 1, 0.1, 1000); camera.position.set(1.8, 1.4, 2.4);
const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
const controls = new OrbitControls(camera, renderer.domElement); controls.enableDamping = true;
scene.add(new THREE.HemisphereLight(0xffffff, 0x444444, 2));
const base = new THREE.Mesh(new THREE.BoxGeometry(2, .2, .7), new THREE.MeshStandardMaterial()); scene.add(base);
const carriage = new THREE.Mesh(new THREE.BoxGeometry(.35, .25, .55), new THREE.MeshStandardMaterial()); carriage.position.y = .22; scene.add(carriage);
let state = { position: 0, forward: false, reverse: false };
const ws = new WebSocket(`ws://${location.host}`);
ws.onmessage = event => { const m = JSON.parse(event.data); if (m.type === 'plc-state') { state = m; document.querySelector('#state').textContent = JSON.stringify(m, null, 2); document.querySelector('#status').textContent = m.connected ? 'PLC connected' : 'Simulation mode'; carriage.position.x = (m.position / 100 - .5) * 1.5; } };
const send = message => ws.readyState === WebSocket.OPEN && ws.send(JSON.stringify(message));
document.querySelector('#forward').onclick = () => send({ type: 'force-input', name: 'home', value: false });
document.querySelector('#reverse').onclick = () => send({ type: 'force-input', name: 'end', value: false });
document.querySelector('#position').oninput = e => send({ type: 'set-position', value: Number(e.target.value) });
function frame() { renderer.setSize(canvas.clientWidth || 800, canvas.clientHeight || 600, false); camera.aspect = (canvas.clientWidth || 800)/(canvas.clientHeight || 600); camera.updateProjectionMatrix(); controls.update(); renderer.render(scene,camera); requestAnimationFrame(frame); } frame();
