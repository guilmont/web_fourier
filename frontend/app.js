"use strict";
let canvas;
let ctx;
let wasmMemory;
function animate(cutoff, i = 0) {
    if (i >= 400) {
        return;
    }
    window.animate_fourier(cutoff, i);
    setTimeout(() => {
        animate(cutoff, i + 1);
    }, 16);
}
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
        fill_text: (textPtr, textLen, x, y) => {
            const bytes = new Uint8Array(wasmMemory.buffer, textPtr, textLen);
            const text = new TextDecoder("utf-8").decode(bytes);
            ctx.fillText(text, x, y);
        },
        set_font: (fontPtr, fontLen) => {
            const bytes = new Uint8Array(wasmMemory.buffer, fontPtr, fontLen);
            const font = new TextDecoder("utf-8").decode(bytes);
            ctx.font = font;
        },
        set_text_align: (alignPtr, alignLen) => {
            const bytes = new Uint8Array(wasmMemory.buffer, alignPtr, alignLen);
            const align = new TextDecoder("utf-8").decode(bytes);
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
function createMathImports() {
    return {
        random: Math.random,
    };
}
function createBrowserImports() {
    return {
        alert: (ptr, len) => {
            const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
            const msg = new TextDecoder("utf-8").decode(bytes);
            window.alert(msg);
        }
    };
}
async function loadWasm() {
    try {
        setupCanvas();
        const wasmModule = await WebAssembly.instantiateStreaming(fetch('./web_fourier.wasm'), {
            Math: createMathImports(),
            Console: createConsoleImports(),
            Canvas: createCanvasImports(),
            Browser: createBrowserImports(),
        });
        const expo = wasmModule.instance.exports;
        wasmMemory = expo.memory;
        window.plot_step = expo.plot_step;
        window.plot_multiple_functions = expo.plot_multiple_functions;
        window.draw_random_pattern = expo.draw_random_pattern;
        window.animate_fourier = expo.animate_fourier;
        console.log("WebAssembly loaded successfully!");
    }
    catch (error) {
        console.error("Failed to load WebAssembly:", error);
    }
}
// Initialize when page loads
document.addEventListener('DOMContentLoaded', async () => {
    await loadWasm();
    // Add event listeners for buttons
    document.getElementById('random-pattern')?.addEventListener('click', () => { window.draw_random_pattern(); });
    document.getElementById('plot-multiple')?.addEventListener('click', () => { window.plot_multiple_functions(); });
    document.getElementById('plot-step')?.addEventListener('click', () => {
        const cutoffInput = document.getElementById('cutoff');
        const cutoff = parseInt(cutoffInput.value, 10);
        window.plot_step(cutoff);
    });
    document.getElementById('animate')?.addEventListener('click', () => {
        const cutoffInput = document.getElementById('cutoff');
        const cutoff = parseInt(cutoffInput.value, 10);
        animate(cutoff);
    });
    document.getElementById('cutoff')?.addEventListener('change', () => {
        document.getElementById('plot-step')?.click();
    });
    window.draw_random_pattern();
});
