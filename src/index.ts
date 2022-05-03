// @ts-ignore: Typescript don't know about Cargo.toml import run but it does
import { run } from '../Cargo.toml';
import startMoonAnimation from './animations/moonAnimation';

//This is the main function that will load the wasm data into the browser
// Main Function
run();

// Animations //
startMoonAnimation();
