"use strict";
let canvas;
let ctx;
let wasmMemory;
function setupCanvas() {
    canvas = document.getElementById('canvas');
    if (!canvas) {
        console.error('Canvas element not found');
        return;
    }
    ctx = canvas.getContext('2d');
    canvas.width = 800;
    canvas.height = 600;
}
function createCanvasImports() {
    return {
        arc: (x, y, radius, startAngle, endAngle) => { ctx.arc(x, y, radius, startAngle, endAngle); },
        begin_path: () => { ctx.beginPath(); },
        clear_rect: (x, y, width, height) => { ctx.clearRect(x, y, width, height); },
        fill: () => { ctx.fill(); },
        fill_rect: (x, y, width, height) => { ctx.fillRect(x, y, width, height); },
        height: () => { return canvas.height; },
        line_to: (x, y) => { ctx.lineTo(x, y); },
        move_to: (x, y) => { ctx.moveTo(x, y); },
        set_fill_style_color: (r, g, b, a) => { ctx.fillStyle = `rgba(${r}, ${g}, ${b}, ${a})`; },
        set_line_width: (width) => { ctx.lineWidth = width; },
        set_stroke_style_color: (r, g, b, a) => { ctx.strokeStyle = `rgba(${r}, ${g}, ${b}, ${a})`; },
        stroke: () => { ctx.stroke(); },
        stroke_rect: (x, y, width, height) => { ctx.strokeRect(x, y, width, height); },
        width: () => { return canvas.width; },
    };
}
function createconsoleImports() {
    return {
        log: (ptr, len) => {
            const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
            const msg = new TextDecoder("utf-8").decode(bytes);
            console.log("[WASM]", msg);
        }
    };
}
function createMathImports() {
    return {
        random: Math.random,
    };
}
async function loadWasm() {
    try {
        setupCanvas();
        const wasmModule = await WebAssembly.instantiateStreaming(fetch('./wasm_rust.wasm'), {
            Math: createMathImports(),
            Console: createconsoleImports(),
            Canvas: createCanvasImports(),
        });
        const expo = wasmModule.instance.exports;
        wasmMemory = expo.memory;
        window.draw_random_pattern = expo.draw_random_pattern;
        window.say_hi = expo.say_hi;
        window.foo = expo.foo;
        console.log("WebAssembly loaded successfully!");
    }
    catch (error) {
        console.error("Failed to load WebAssembly:", error);
    }
}
// Initialize when page loads
document.addEventListener('DOMContentLoaded', async () => {
    await loadWasm();
    window.draw_random_pattern();
});
