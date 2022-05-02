import * as BABYLON from 'babylonjs';
import { BabylonFileLoaderConfiguration } from 'babylonjs';


const DONUT_CONTENT = 'Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undoubtable source. Lorem Ipsum comes from sections 1.10.32 and 1.10.33 of "de Finibus Bonorum et Malorum" (The Extremes of Good and Evil) by Cicero, written in 45 BC. This book is a treatise on the theory of ethics, very popular during the Renaissance. The first line of Lorem Ipsum, "Lorem ipsum dolor sit amet..", comes from a line in section 1.10.32.';


export default async function startAnimation() {

  // Lets create a babylonjs animation to the donut canvas
  const donutCanva: HTMLCanvasElement = document.getElementById("donutCanva") as HTMLCanvasElement;
  const engine = new BABYLON.Engine(donutCanva, true);

  function createScene(): BABYLON.Scene {
    const scene = new BABYLON.Scene(engine);
    const camera = new BABYLON.ArcRotateCamera('MainCamera', -Math.PI / 2, Math.PI / 4, 50, BABYLON.Vector3.Zero(), scene);

    //make the camera move with the mouse
    camera.attachControl(donutCanva, true);
    new BABYLON.HemisphericLight('light', new BABYLON.Vector3(0, 1, 0), scene);
    return scene
  }

  const scene = createScene();
  engine.runRenderLoop(() => {
    scene.render();
  })

  BABYLON.SceneLoader.Load("", "./moon.babylon", engine, (newScene) => {
    newScene.executeWhenReady(() => {
      newScene.activeCamera.attachControl(donutCanva);
      engine.runRenderLoop(() => {
        newScene.render();
      });
    });
  }, (progresss) => {
    console.log(progresss);
  })


  //This Function prevents the mouse scroll inside the canvas
  //This Function prevents the mouse scroll inside the canvas
  document.getElementById("donutCanva").onwheel = function(event) {
    event.preventDefault();
  };
}

