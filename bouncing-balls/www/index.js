import * as wasm from "bouncing-balls";

const WIDTH = window.innerWidth;
const HEIGHT = window.innerHeight;

let canvas = document.getElementById('canvas');
canvas.width = WIDTH;
canvas.height = HEIGHT;

wasm.draw();
