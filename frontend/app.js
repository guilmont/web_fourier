"use strict";
// Define constants for canvas dimensions
const CANVAS_WIDTH = 800;
const CANVAS_HEIGHT = 400;
const STEP_CANVAS_ID = 1;
const ANIMATION_CANVAS_ID = 2;
let canvasRegistry = new Map();
let wasmMemory;
function createAnimationImports() {
    return {
        start_animation_loop: (canvasId) => {
            const canvasInfo = getCanvasInfo(canvasId);
            if (canvasInfo.animationId !== null)
                return; // Already running
            function animationFrame() {
                window.step_animation();
                canvasInfo.animationId = requestAnimationFrame(animationFrame);
            }
            canvasInfo.animationId = requestAnimationFrame(animationFrame);
        },
        stop_animation_loop: (canvasId) => {
            const canvasInfo = getCanvasInfo(canvasId);
            if (canvasInfo.animationId !== null) {
                cancelAnimationFrame(canvasInfo.animationId);
                canvasInfo.animationId = null;
            }
        },
    };
}
// Helper function to decode WASM strings
function decodeWasmString(ptr, len) {
    const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
    return new TextDecoder("utf-8").decode(bytes);
}
// Helper function to get canvas and context by integer ID
function getCanvasInfo(canvasId) {
    const canvasInfo = canvasRegistry.get(canvasId);
    if (!canvasInfo) {
        throw new Error(`Canvas with id ${canvasId} not found`);
    }
    return canvasInfo;
}
function createCanvasImports() {
    return {
        arc: (canvasId, x, y, radius, startAngle, endAngle) => {
            getCanvasInfo(canvasId).context.arc(x, y, radius, startAngle, endAngle);
        },
        begin_path: (canvasId) => {
            getCanvasInfo(canvasId).context.beginPath();
        },
        clear_rect: (canvasId, x, y, width, height) => {
            getCanvasInfo(canvasId).context.clearRect(x, y, width, height);
        },
        fill: (canvasId) => {
            getCanvasInfo(canvasId).context.fill();
        },
        fill_rect: (canvasId, x, y, width, height) => {
            getCanvasInfo(canvasId).context.fillRect(x, y, width, height);
        },
        height: (canvasId) => {
            return getCanvasInfo(canvasId).canvas.height;
        },
        line_to: (canvasId, x, y) => {
            getCanvasInfo(canvasId).context.lineTo(x, y);
        },
        move_to: (canvasId, x, y) => {
            getCanvasInfo(canvasId).context.moveTo(x, y);
        },
        set_fill_style_color: (canvasId, r, g, b, a) => {
            getCanvasInfo(canvasId).context.fillStyle = `rgba(${r}, ${g}, ${b}, ${a})`;
        },
        set_line_width: (canvasId, width) => {
            getCanvasInfo(canvasId).context.lineWidth = width;
        },
        set_stroke_style_color: (canvasId, r, g, b, a) => {
            getCanvasInfo(canvasId).context.strokeStyle = `rgba(${r}, ${g}, ${b}, ${a})`;
        },
        stroke: (canvasId) => {
            getCanvasInfo(canvasId).context.stroke();
        },
        stroke_rect: (canvasId, x, y, width, height) => {
            getCanvasInfo(canvasId).context.strokeRect(x, y, width, height);
        },
        width: (canvasId) => {
            return getCanvasInfo(canvasId).canvas.width;
        },
        fill_text: (canvasId, textPtr, textLen, x, y) => {
            const text = decodeWasmString(textPtr, textLen);
            getCanvasInfo(canvasId).context.fillText(text, x, y);
        },
        set_font: (canvasId, fontPtr, fontLen) => {
            const font = decodeWasmString(fontPtr, fontLen);
            getCanvasInfo(canvasId).context.font = font;
        },
        set_text_align: (canvasId, alignPtr, alignLen) => {
            const align = decodeWasmString(alignPtr, alignLen);
            getCanvasInfo(canvasId).context.textAlign = align;
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
// Function to register a canvas and return its integer ID
function registerCanvas(canvasName, canvasId, width, height) {
    const canvas = document.getElementById(canvasName);
    const context = canvas.getContext('2d');
    canvas.width = width;
    canvas.height = height;
    canvasRegistry.set(canvasId, { canvas, context, animationId: null });
}
// Initialize when page loads
document.addEventListener('DOMContentLoaded', async () => {
    // Set up the canvases
    registerCanvas('step-canvas', STEP_CANVAS_ID, CANVAS_WIDTH, CANVAS_HEIGHT);
    registerCanvas('animation-canvas', ANIMATION_CANVAS_ID, CANVAS_WIDTH, CANVAS_HEIGHT);
    // Load the WebAssembly module
    await loadWasm();
    // Add event listeners for buttons
    const playPauseButton = document.getElementById('play-pause');
    const stopButton = document.getElementById('stop');
    const backwardButton = document.getElementById('backward');
    const forwardButton = document.getElementById('forward');
    const stepCutoffInput = document.getElementById('step-cutoff');
    const animationCutoffInput = document.getElementById('animation-cutoff');
    stepCutoffInput.addEventListener('change', () => {
        window.plot_step(STEP_CANVAS_ID, parseInt(stepCutoffInput.value, 10));
    });
    animationCutoffInput.addEventListener('change', () => {
        stopButton.click();
        playPauseButton.click();
    });
    playPauseButton.addEventListener('click', () => {
        window.play_pause_animation(ANIMATION_CANVAS_ID, parseInt(animationCutoffInput.value, 10));
    });
    stopButton.addEventListener('click', () => { window.stop_animation(); });
    backwardButton.addEventListener('click', () => { window.decrease_animation_speed(); });
    forwardButton.addEventListener('click', () => { window.increase_animation_speed(); });
    // Initial plot
    stepCutoffInput.dispatchEvent(new Event('change'));
    playPauseButton.click();
    setTimeout(() => {
        playPauseButton.click();
    }, 10);
});
