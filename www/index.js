import { Universe, Cell } from "wasm-gameoflife";
import { memory } from "wasm-gameoflife/gameoflife_bg";

const CELL_SIZE = 16;
const GRID_COLOR = "#201010";
const DEAD_COLOR = "#402010";
const ALIVE_COLOR = "#fc427d";

const width = 80;
const height = 40;
var frametime = 60;
var renderHandle;
var universe = Universe.new(height, width);

const canvas = document.getElementById("gol-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    universe.tick();
    drawGrid();
    drawCells();
    renderHandle = setTimeout(() => requestAnimationFrame(renderLoop), frametime);
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1)
    }
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = universe.get_index(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
    ctx.stroke();
};

document.getElementById("frametime").addEventListener("change", (e) => {
    frametime = e.target.value;
});
document.getElementById("play").addEventListener("click", () => {
    requestAnimationFrame(renderLoop);
});
document.getElementById("pause").addEventListener("click", () => {
    clearTimeout(renderHandle);
});

var mousepressed = false;
function activateCell(e) {
    const rect = canvas.getBoundingClientRect()
    const y = e.clientY - rect.top;
    const x = e.clientX - rect.left;
    const row = Math.floor(y * height / rect.height);
    const col = Math.floor(x * width / rect.width);
    universe.activate_cell(row, col);
    drawCells();
};
canvas.addEventListener("mousedown", (e) => {
    mousepressed = true;
    activateCell(e);
});
canvas.addEventListener("mouseup", () => mousepressed = false);
canvas.addEventListener("mousemove", (e) => { if (mousepressed) { activateCell(e); } });

document.getElementById("reset").addEventListener("click", () => {
    universe = Universe.new(height, width);
    clearTimeout(renderHandle);
    drawGrid();
    drawCells();
});

document.getElementById("infinite").addEventListener("change", (e) => {
    universe.set_infinite(e.target.checked);
});

drawGrid();
drawCells();