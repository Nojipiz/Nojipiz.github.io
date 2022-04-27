// @ts-ignore: Typescript don't know about Cargo.toml import run but it does
import { run } from '../Cargo.toml'
import * as BABYLON from 'babylonjs'
run()

console.log("Hi mom");


// Lets create a babylonjs animation to the donut canvas
const donutCanva: HTMLCanvasElement = document.getElementById("donutCanva") as HTMLCanvasElement;
const engine = new BABYLON.Engine(donutCanva, true);

function createScene(): BABYLON.Scene {
  const scene = new BABYLON.Scene(engine);
  const camera = new BABYLON.FreeCamera('camera', new BABYLON.Vector3(0, 0, -5), scene);
  camera.attachControl(donutCanva, true);
  const light = new BABYLON.HemisphericLight('light', new BABYLON.Vector3(0, 1, 0), scene);
  const box = BABYLON.MeshBuilder.CreateBox('box', { size: 1 }, scene);
  return scene
}

const scene = createScene();

engine.runRenderLoop(() => {
  scene.render();
})

