// @ts-ignore: Typescript don't know about Cargo.toml import run but it does
import { run } from '../Cargo.toml';
import startDonutAnimation from './animations/donutAnimation';

//This is the main function that will load the wasm data into the browser
// Main Function
run();

// Animations //
startDonutAnimation();
