import * as BABYLON from 'babylonjs';
import { AbstractMesh, Light } from 'babylonjs';

export default async function startAnimation() {
  BABYLON.SceneLoader.ShowLoadingScreen = false;
  const moonCanva: HTMLCanvasElement = document.getElementById('moonCanvas') as HTMLCanvasElement;
  const engine = new BABYLON.Engine(moonCanva, true);
  const cameraPosition: BABYLON.Vector3 = new BABYLON.Vector3(0, 0, -300);
  const sunPosition: BABYLON.Vector3 = new BABYLON.Vector3(-10, -20, 9);
  BABYLON.SceneLoader.Load(
    'https://raw.githubusercontent.com/Nojipiz/Nojipiz.github.io/dev/src/animations/',
    'moon.babylon',
    engine,
    (mainScene) => {
      mainScene.executeWhenReady(() => {
        const moonMeshes: AbstractMesh[] = mainScene.meshes;
        const sunLight: Light = setLightInScene(mainScene);
        clearSceneBackground(mainScene);
        setCameraInScene(mainScene, moonMeshes[0].position, sunLight);
        setRotationAnimationForMoon(mainScene, moonMeshes);
        engine.runRenderLoop(() => {
          mainScene.render();
        });
      });
    },
    (): void => {}
  );

  const setCameraInScene: Function = (
    scene: BABYLON.Scene,
    moonPosition: BABYLON.Vector3,
    light: BABYLON.DirectionalLight
  ): void => {
    scene.activeCamera = new BABYLON.ArcRotateCamera('MainCamera', 0, 0, 0, moonPosition, scene);
    scene.activeCamera.attachControl(moonCanva);
    scene.activeCamera.position = cameraPosition;
    scene.activeCamera.inputs.removeByType('ArcRotateCameraMouseWheelInput');
    light.parent = scene.activeCamera;
  };

  const setLightInScene: Function = (scene: BABYLON.Scene): BABYLON.DirectionalLight => {
    const sunLight = new BABYLON.DirectionalLight('SunLight', sunPosition, scene);
    scene.addLight(sunLight);
    return sunLight;
  };

  const clearSceneBackground: Function = (scene: BABYLON.Scene): void => {
    scene.lights = [];
    scene.clearColor = new BABYLON.Color4(0, 0, 0, 0);
  };

  const setRotationAnimationForMoon: Function = (scene: BABYLON.Scene, moonMeshes: AbstractMesh[]): void => {
    const moonAnimationYAxis = new BABYLON.Animation(
      'MoonAnimation',
      'rotation.y',
      30,
      BABYLON.Animation.ANIMATIONTYPE_FLOAT,
      BABYLON.Animation.ANIMATIONLOOPMODE_RELATIVE
    );
    const moonAnimationXAxis = new BABYLON.Animation(
      'MoonAnimation',
      'rotation.x',
      30,
      BABYLON.Animation.ANIMATIONTYPE_FLOAT,
      BABYLON.Animation.ANIMATIONLOOPMODE_RELATIVE
    );
    const yRotationKeys = [
      { frame: 0, value: 0 },
      { frame: 30, value: 0.35 },
    ];
    const xRotationKeys = [
      { frame: 0, value: 0 },
      { frame: 30, value: 0.12 },
    ];
    moonAnimationYAxis.setKeys(yRotationKeys);
    moonAnimationXAxis.setKeys(xRotationKeys);
    moonMeshes.forEach((mesh) => {
      mesh.animations = [moonAnimationYAxis, moonAnimationXAxis];
      scene.beginAnimation(mesh, 0, 30, true);
    });
  };
}
