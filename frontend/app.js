"use strict";
// Define constants for canvas dimensions
const CANVAS_WIDTH = 800;
const CANVAS_HEIGHT = 600;
let canvas;
let ctx;
let wasmMemory;
let cutoffControl = true; // Control to determine which display to attach events of cutoff input
// Global animation loop control
let animationId = null;
function createAnimationImports() {
    return {
        start_animation_loop: () => {
            if (animationId !== null)
                return; // Already running
            function animationFrame() {
                window.step_animation();
                animationId = requestAnimationFrame(animationFrame);
            }
            animationId = requestAnimationFrame(animationFrame);
        },
        stop_animation_loop: () => {
            if (animationId !== null) {
                cancelAnimationFrame(animationId);
                animationId = null;
            }
        },
    };
}
// Helper function to decode WASM strings
function decodeWasmString(ptr, len) {
    const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
    return new TextDecoder("utf-8").decode(bytes);
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
        fill_text: (textPtr, textLen, x, y) => {
            const text = decodeWasmString(textPtr, textLen);
            ctx.fillText(text, x, y);
        },
        set_font: (fontPtr, fontLen) => {
            const font = decodeWasmString(fontPtr, fontLen);
            ctx.font = font;
        },
        set_text_align: (alignPtr, alignLen) => {
            const align = decodeWasmString(alignPtr, alignLen);
            ctx.textAlign = align;
        },
    };
}
function createConsoleImports() {
    return {
        log: (ptr, len) => {
            const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
            const msg = new TextDecoder("utf-8").decode(bytes);
            console.log("[WASM]", msg);
        },
        error: (ptr, len) => {
            const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
            const msg = new TextDecoder("utf-8").decode(bytes);
            console.error("[WASM]", msg);
        }
    };
}
function createBrowserImports() {
    return {
        alert: (ptr, len) => {
            const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
            const msg = new TextDecoder("utf-8").decode(bytes);
            window.alert(msg);
        },
        time_now: () => performance.now(),
    };
}
async function loadWasm() {
    try {
        const wasmModule = await WebAssembly.instantiateStreaming(fetch('./web_fourier.wasm'), {
            Animation: createAnimationImports(),
            Browser: createBrowserImports(),
            Canvas: createCanvasImports(),
            Console: createConsoleImports(),
        });
        const expo = wasmModule.instance.exports;
        wasmMemory = expo.memory;
        window.plot_step = expo.plot_step;
        // New self-contained Rust animation functions
        window.step_animation = expo.step_animation;
        window.play_pause_animation = expo.play_pause_animation;
        window.stop_animation = expo.stop_animation;
        window.increase_animation_speed = expo.increase_animation_speed;
        window.decrease_animation_speed = expo.decrease_animation_speed;
        console.log("WebAssembly loaded successfully!");
    }
    catch (error) {
        console.error("Failed to load WebAssembly:", error);
    }
}
// Initialize when page loads
document.addEventListener('DOMContentLoaded', async () => {
    // Set up the canvas
    canvas = document.getElementById('canvas');
    if (!canvas) {
        console.error("Canvas element not found");
        return;
    }
    ctx = canvas.getContext('2d'); // Use non-null assertion to ensure ctx is not null
    canvas.width = CANVAS_WIDTH;
    canvas.height = CANVAS_HEIGHT;
    // Load the WebAssembly module
    await loadWasm();
    // Add event listeners for buttons
    const stepFunctionButton = document.getElementById('plot-step');
    const playPauseButton = document.getElementById('play-pause');
    const stopButton = document.getElementById('stop');
    const backwardButton = document.getElementById('backward');
    const forwardButton = document.getElementById('forward');
    const cutoffInput = document.getElementById('cutoff');
    stepFunctionButton.addEventListener('click', () => {
        cutoffControl = true;
        window.plot_step(parseInt(cutoffInput.value, 10));
    });
    cutoffInput.addEventListener('change', () => {
        if (cutoffControl) {
            stepFunctionButton.click();
        }
        else {
            stopButton.click();
            playPauseButton.click();
        }
    });
    playPauseButton.addEventListener('click', () => {
        cutoffControl = false;
        window.play_pause_animation(parseInt(cutoffInput.value, 10));
    });
    stopButton.addEventListener('click', () => {
        window.stop_animation();
    });
    backwardButton.addEventListener('click', () => {
        window.decrease_animation_speed();
    });
    forwardButton.addEventListener('click', () => {
        window.increase_animation_speed();
    });
    // Initial plot
    stepFunctionButton.click();
});
