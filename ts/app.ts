let canvas: HTMLCanvasElement;
let ctx: CanvasRenderingContext2D;
let wasmMemory: WebAssembly.Memory;

function setupCanvas(): void {
    canvas = document.getElementById('canvas') as HTMLCanvasElement;
    if (!canvas) {
        console.error('Canvas element not found');
        return;
    }

    ctx = canvas.getContext('2d')!;
    canvas.width = 800;
    canvas.height = 600;
}

function createCanvasImports() {
    return {
        arc:                    (x: number, y: number, radius: number, startAngle: number, endAngle: number) => { ctx.arc(x, y, radius, startAngle, endAngle); },
        begin_path:             () => { ctx.beginPath(); },
        clear_rect:             (x: number, y: number, width: number, height: number) => { ctx.clearRect(x, y, width, height); },
        fill:                   () => { ctx.fill(); },
        fill_rect:              (x: number, y: number, width: number, height: number) => { ctx.fillRect(x, y, width, height); },
        height:                 (): number => { return canvas.height; },
        line_to:                (x: number, y: number) => { ctx.lineTo(x, y); },
        move_to:                (x: number, y: number) => { ctx.moveTo(x, y); },
        set_fill_style_color:   (r: number, g: number, b: number, a: number) => { ctx.fillStyle = `rgba(${r}, ${g}, ${b}, ${a})`; },
        set_line_width:         (width: number) => { ctx.lineWidth = width; },
        set_stroke_style_color: (r: number, g: number, b: number, a: number) => { ctx.strokeStyle = `rgba(${r}, ${g}, ${b}, ${a})`; },
        stroke:                 () => { ctx.stroke(); },
        stroke_rect:            (x: number, y: number, width: number, height: number) => { ctx.strokeRect(x, y, width, height); },
        width:                  (): number => { return canvas.width; },

        fill_text: (textPtr: number, textLen: number, x: number, y: number) => {
            const bytes = new Uint8Array(wasmMemory.buffer, textPtr, textLen);
            const text = new TextDecoder("utf-8").decode(bytes);
            ctx.fillText(text, x, y);
        },
        set_font: (fontPtr: number, fontLen: number) => {
            const bytes = new Uint8Array(wasmMemory.buffer, fontPtr, fontLen);
            const font = new TextDecoder("utf-8").decode(bytes);
            ctx.font = font;
        },
        set_text_align: (alignPtr: number, alignLen: number) => {
            const bytes = new Uint8Array(wasmMemory.buffer, alignPtr, alignLen);
            const align = new TextDecoder("utf-8").decode(bytes);
            ctx.textAlign = align as CanvasTextAlign;
        },
    };
}

function createConsoleImports() {
    return {
        log: (ptr: number, len: number) => {
            const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
            const msg = new TextDecoder("utf-8").decode(bytes);
            console.log("[WASM]", msg);
        },
        error: (ptr: number, len: number) => {
            const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
            const msg = new TextDecoder("utf-8").decode(bytes);
            console.error("[WASM]", msg);
        }
    };
}

function createMathImports() {
    return {
        random: Math.random,
    }
}

function createBrowserImports() {
    return {
        alert: (ptr: number, len: number) => {
            const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
            const msg = new TextDecoder("utf-8").decode(bytes);
            window.alert(msg);
        }
    };
}

async function loadWasm(): Promise<void> {
    try {
        setupCanvas();

        const wasmModule = await WebAssembly.instantiateStreaming(
            fetch('./web_fourier.wasm'),
            {
                Math: createMathImports(),
                Console: createConsoleImports(),
                Canvas: createCanvasImports(),
                Browser: createBrowserImports(),
            }
        );

        const expo = wasmModule.instance.exports as WebAssembly.Exports;
        wasmMemory = expo.memory as WebAssembly.Memory;

        window.plot_step = expo.plot_step;
        window.plot_multiple_functions = expo.plot_multiple_functions;
        window.draw_random_pattern = expo.draw_random_pattern;

        console.log("WebAssembly loaded successfully!");

    } catch (error) {
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
        const cutoffInput = document.getElementById('cutoff') as HTMLInputElement;
        const cutoff = parseInt(cutoffInput.value, 10);
        window.plot_step(cutoff);
    });

    document.getElementById('cutoff')?.addEventListener('change', () => {
        document.getElementById('plot-step')?.click();
    });

    window.draw_random_pattern();
});
