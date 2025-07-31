"use strict";
// Define constants for canvas dimensions
const CANVAS_WIDTH = 800;
const CANVAS_HEIGHT = 400;
const SPECTRUM_CANVAS_HEIGHT = 200;
let WASM;
let CANVAS_REGISTRY = new Map();
// Helper function to decode WASM strings
function decodeWasmString(ptr, len) {
    const bytes = new Uint8Array(WASM.memory.buffer, ptr, len);
    return new TextDecoder("utf-8").decode(bytes);
}
// Helper function to get canvas and context by integer ID
function getCanvasInfo(canvasId) {
    const canvasInfo = CANVAS_REGISTRY.get(canvasId);
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
            getCanvasInfo(canvasId).context.font = decodeWasmString(fontPtr, fontLen);
        },
        set_text_align: (canvasId, alignPtr, alignLen) => {
            getCanvasInfo(canvasId).context.textAlign = decodeWasmString(alignPtr, alignLen);
        },
        measure_text_width: (canvasId, textPtr, textLen, fontPtr, fontLen) => {
            const text = decodeWasmString(textPtr, textLen);
            const font = decodeWasmString(fontPtr, fontLen);
            const ctx = getCanvasInfo(canvasId).context;
            ctx.save();
            ctx.font = font;
            const width = ctx.measureText(text).width;
            ctx.restore();
            return width;
        },
    };
}
function createConsoleImports() {
    return {
        log: (ptr, len) => { console.log("[WASM]", decodeWasmString(ptr, len)); },
        error: (ptr, len) => { console.error("[WASM]", decodeWasmString(ptr, len)); },
    };
}
function createBrowserImports() {
    return {
        alert: (ptr, len) => { window.alert(decodeWasmString(ptr, len)); },
        time_now: () => performance.now(),
    };
}
function createAnimationImports() {
    return {
        start_animation_loop: (canvasId) => {
            const canvasInfo = getCanvasInfo(canvasId);
            if (canvasInfo.animationId !== null)
                return; // Already running
            function animationFrame() {
                WASM.step_animation();
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
async function loadWasm() {
    try {
        const wasmModule = await WebAssembly.instantiateStreaming(fetch('./web_fourier.wasm'), {
            Animation: createAnimationImports(),
            Browser: createBrowserImports(),
            Canvas: createCanvasImports(),
            Console: createConsoleImports(),
        });
        WASM = wasmModule.instance.exports;
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
    CANVAS_REGISTRY.set(canvasId, { canvas, context, animationId: null });
    canvas.addEventListener('mousemove', (e) => {
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        WASM.canvas_mouse_move(canvasId, x, y);
    });
}
// Initialize when page loads
document.addEventListener('DOMContentLoaded', async () => {
    // Load the WebAssembly module
    await loadWasm();
    // Set up the canvases
    const EXAMPLE_CANVAS_ID = WASM.get_example_canvas_id();
    const SPECTRUM_CANVAS_ID = WASM.get_spectrum_canvas_id();
    const ANIMATION_CANVAS_ID = WASM.get_animation_canvas_id();
    registerCanvas('step-canvas', EXAMPLE_CANVAS_ID, CANVAS_WIDTH, CANVAS_HEIGHT);
    registerCanvas('animation-canvas', ANIMATION_CANVAS_ID, CANVAS_WIDTH, CANVAS_HEIGHT);
    registerCanvas('power-spectrum-canvas', SPECTRUM_CANVAS_ID, CANVAS_WIDTH, SPECTRUM_CANVAS_HEIGHT);
    // Add event listeners for buttons
    const playPauseButton = document.getElementById('play-pause');
    const stopButton = document.getElementById('stop');
    const backwardButton = document.getElementById('backward');
    const forwardButton = document.getElementById('forward');
    const exampleFreqMinInput = document.getElementById('example-freq-min');
    const exampleMaxFreqInput = document.getElementById('example-freq-max');
    const animationFreqMinInput = document.getElementById('animation-freq-min');
    const animationFreqMaxInput = document.getElementById('animation-freq-max');
    // Track current example
    let currentExample = 0; // 0=step, 1=sine, 2=square, 3=triangle
    function plotCurrentExample() {
        const kMin = parseInt(exampleFreqMinInput.value, 10);
        const kMax = parseInt(exampleMaxFreqInput.value, 10);
        WASM.plot_example(kMin, kMax, currentExample);
    }
    exampleMaxFreqInput.addEventListener('change', plotCurrentExample);
    exampleFreqMinInput.addEventListener('change', plotCurrentExample);
    // Example buttons
    document.querySelectorAll('.example-btn').forEach((btn, idx) => {
        btn.addEventListener('click', (e) => {
            currentExample = idx;
            // Highlight selected
            document.querySelectorAll('.example-btn').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            plotCurrentExample();
        });
    });
    function restartAnimation() {
        stopButton.click();
        playPauseButton.click();
    }
    animationFreqMaxInput.addEventListener('change', restartAnimation);
    animationFreqMinInput.addEventListener('change', restartAnimation);
    playPauseButton.addEventListener('click', () => {
        const kMin = parseInt(animationFreqMinInput.value, 10);
        const kMax = parseInt(animationFreqMaxInput.value, 10);
        WASM.play_pause_animation(ANIMATION_CANVAS_ID, kMin, kMax);
    });
    stopButton.addEventListener('click', () => { WASM.stop_animation(); });
    backwardButton.addEventListener('click', () => { WASM.decrease_animation_speed(); });
    forwardButton.addEventListener('click', () => { WASM.increase_animation_speed(); });
    // Initial plot and highlight
    document.querySelector('.example-btn[data-example="step"]')?.classList.add('active');
    plotCurrentExample();
    playPauseButton.click();
    setTimeout(() => {
        playPauseButton.click();
    }, 10);
});
