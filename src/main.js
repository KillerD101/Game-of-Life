let cnv;

const scene_alpha = 255;

const BACKGROUND_COLOR = {
    r: 200, g: 200, b: 200
};

const ROWS = 50;
const COLS = 50;

let board = new Array(ROWS * COLS);

let scl_width;
let scl_height;

function get_board(i, j) {
    return board[i * COLS + j];
}

function set_board(i, j, val) {
    if(val != undefined) board[i * COLS + j] = val;
    else board[i * COLS + j] = !board[i * COLS + j];
}

function setup() {
    cnv = createCanvas(512, 512);
    windowResized();

    for(var i = 0; i < board.length; i++)
        board[i] = false;

    set_board(2, 3);
    set_board(2, 4);
    set_board(2, 5);
}

function draw() {
    background(
        BACKGROUND_COLOR.r, BACKGROUND_COLOR.g,
        BACKGROUND_COLOR.b, scene_alpha
    );

    fill(0, 0, 0, scene_alpha);
    noStroke();
    for(var i = 0; i < COLS; i++) for(var j = 0; j < ROWS; j++)
        if(get_board(j, i)) rect(
            i * scl_width, j * scl_height,
            scl_width, scl_height
        );
}

function windowResized() {
    cnv.position(
        (windowWidth - width) / 2, 0
    );

    scl_width = width / ROWS;
    scl_height = height / COLS;
}