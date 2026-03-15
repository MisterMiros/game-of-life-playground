import init , { LifeEngineWrapper } from '/rust/game-of-life-wasm/pkg/game_of_life_wasm.js';

const CELL_SIZE = 10;
const WORLD_COLS = 80;
const WORLD_ROWS = 60;
const GRID_COLOR = 'rgba(255, 255, 255, 0.07)';
const BACKGROUND_COLOR = '#04060f';
const CELL_COLOR = '#49ffa7';

const canvas = document.getElementById('game-of-life-canvas');
const ctx = canvas.getContext('2d', { alpha: false });
canvas.width = WORLD_COLS * CELL_SIZE;
canvas.height = WORLD_ROWS * CELL_SIZE;

const generationEl = document.getElementById('generation-count');
const aliveCountEl = document.getElementById('alive-count');
const nextButton = document.getElementById('next-generation');

await init();
const lifeEngine = new LifeEngineWrapper(WORLD_COLS, WORLD_ROWS);
lifeEngine.generate_random_square(5, 5, 50);

let generation = 0;
render();

nextButton.addEventListener('click', () => {
    lifeEngine.next();
    generation += 1;
    render();
});

document.addEventListener('keydown', (event) => {
    if (event.code === 'Space') {
        event.preventDefault();
        nextButton.click();
    }
});

function render() {
    const aliveCells = drawGrid();
    generationEl.textContent = generation.toString().padStart(3, '0');
    aliveCountEl.textContent = aliveCells.toString();
}

function drawGrid() {
    ctx.fillStyle = BACKGROUND_COLOR;
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
    ctx.lineWidth = 0.5;
    for (let x = 0; x <= WORLD_COLS; x += 1) {
        const posX = x * CELL_SIZE + 0.5;
        ctx.moveTo(posX, 0);
        ctx.lineTo(posX, canvas.height);
    }
    for (let y = 0; y <= WORLD_ROWS; y += 1) {
        const posY = y * CELL_SIZE + 0.5;
        ctx.moveTo(0, posY);
        ctx.lineTo(canvas.width, posY);
    }
    ctx.stroke();

    ctx.fillStyle = CELL_COLOR;
    let aliveCells = lifeEngine.get_alive_cells_count();
    lifeEngine.for_each_cell_do((x, y) => {
        ctx.fillRect(
            x * CELL_SIZE + 1,
            y * CELL_SIZE + 1,
            CELL_SIZE - 1.5,
            CELL_SIZE - 1.5
        );
    });
    return aliveCells;
}
