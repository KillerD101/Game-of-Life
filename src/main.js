let cnv;

let scene_alpha = 255;

let BACKGROUND_COLOR = {
    r: 50, g: 50, b: 50
};

function resize_canvas() {
    cnv.position(
        (windowWidth - width) / 2, 0
    );
}

function setup() {
    cnv = createCanvas(512, 512);
    resize_canvas();
}

function draw() {
    background(
        BACKGROUND_COLOR.r, BACKGROUND_COLOR.g,
        BACKGROUND_COLOR.b, scene_alpha
    );
}

function windowResized() {
    resize_canvas();
}