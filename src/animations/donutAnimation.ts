import * as BABYLON from 'babylonjs';

export default function startAnimation() {

  // Lets create a babylonjs animation to the donut canvas
  const donutCanva: HTMLCanvasElement = document.getElementById("donutCanva") as HTMLCanvasElement;
  const engine = new BABYLON.Engine(donutCanva, true);

  function createScene(): BABYLON.Scene {
    const scene = new BABYLON.Scene(engine);
    const camera = new BABYLON.FreeCamera('camera', new BABYLON.Vector3(0, 0, -5), scene);
    new BABYLON.HemisphericLight('light', new BABYLON.Vector3(0, 1, 0), scene);
    const donut = BABYLON.MeshBuilder.CreateBox('donut', { size: 1 }, scene);

    const wheelKeys = [];

    //At the animation key 0, the value of rotation.y is 0
    wheelKeys.push({
      frame: 0,
      value: 0
    });

    //At the animation key 30, (after 1 sec since animation fps = 30) the value of rotation.y is 2PI for a complete rotation
    wheelKeys.push({
      frame: 60,
      value: 2 * Math.PI
    });

    const animWheel = new BABYLON.Animation("wheelAnimation", "rotation.y", 30, BABYLON.Animation.ANIMATIONTYPE_FLOAT, BABYLON.Animation.ANIMATIONLOOPMODE_CYCLE);
    animWheel.setKeys(wheelKeys);
    donut.animations = [];
    donut.animations.push(animWheel);
    scene.beginAnimation(donut, 0, 30, true);


    return scene
  }

  const scene = createScene();

  engine.runRenderLoop(() => {
    scene.render();
  })
}

