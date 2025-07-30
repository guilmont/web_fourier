// Define constants for canvas dimensions
const CANVAS_WIDTH = 800;
const CANVAS_HEIGHT = 600;

let canvas: HTMLCanvasElement;
let ctx: CanvasRenderingContext2D;
let wasmMemory: WebAssembly.Memory;

let cutoffControl = true; // Control to determine which display to attach events of cutoff input

// Global animation loop control
let animationId: number | null = null;

function createAnimationImports() {
    return {
        start_animation_loop:  () => {
            if (animationId !== null) return; // Already running
            function animationFrame() {
                window.step_animation();
                animationId = requestAnimationFrame(animationFrame);
            }
            animationId = requestAnimationFrame(animationFrame);
        },
        stop_animation_loop:   () => {
            if (animationId !== null) {
                cancelAnimationFrame(animationId);
                animationId = null;
            }
        },
    };
}

// Helper function to decode WASM strings
function decodeWasmString(ptr: number, len: number): string {
    const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
    return new TextDecoder("utf-8").decode(bytes);
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
            const text = decodeWasmString(textPtr, textLen);
            ctx.fillText(text, x, y);
        },
        set_font: (fontPtr: number, fontLen: number) => {
            const font = decodeWasmString(fontPtr, fontLen);
            ctx.font = font;
        },
        set_text_align: (alignPtr: number, alignLen: number) => {
            const align = decodeWasmString(alignPtr, alignLen);
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

function createBrowserImports() {
    return {
        alert: (ptr: number, len: number) => {
            const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
            const msg = new TextDecoder("utf-8").decode(bytes);
            window.alert(msg);
        },
        time_now: (): number => performance.now(),
    };
}

async function loadWasm(): Promise<void> {
    try {
        const wasmModule = await WebAssembly.instantiateStreaming(
            fetch('./web_fourier.wasm'),
            {
                Animation: createAnimationImports(),
                Browser: createBrowserImports(),
                Canvas: createCanvasImports(),
                Console: createConsoleImports(),
            }
        );

        const expo = wasmModule.instance.exports as WebAssembly.Exports;
        wasmMemory = expo.memory as WebAssembly.Memory;

        window.plot_step = expo.plot_step;

        // New self-contained Rust animation functions
        window.step_animation = expo.step_animation;
        window.play_pause_animation = expo.play_pause_animation;
        window.stop_animation = expo.stop_animation;
        window.increase_animation_speed = expo.increase_animation_speed;
        window.decrease_animation_speed = expo.decrease_animation_speed;

        console.log("WebAssembly loaded successfully!");

    } catch (error) {
        console.error("Failed to load WebAssembly:", error);
    }
}

// Initialize when page loads
document.addEventListener('DOMContentLoaded', async () => {
    // Set up the canvas
    canvas = document.getElementById('canvas') as HTMLCanvasElement;
    if (!canvas) {
        console.error("Canvas element not found");
        return;
    }

    ctx = canvas.getContext('2d')!; // Use non-null assertion to ensure ctx is not null

    canvas.width = CANVAS_WIDTH;
    canvas.height = CANVAS_HEIGHT;

    // Load the WebAssembly module
    await loadWasm();

    // Add event listeners for buttons
    const stepFunctionButton = document.getElementById('plot-step')! as HTMLButtonElement;
    const playPauseButton = document.getElementById('play-pause')! as HTMLButtonElement;
    const stopButton = document.getElementById('stop')! as HTMLButtonElement;
    const backwardButton = document.getElementById('backward')! as HTMLButtonElement;
    const forwardButton = document.getElementById('forward')! as HTMLButtonElement;
    const cutoffInput = document.getElementById('cutoff')! as HTMLInputElement;

    stepFunctionButton.addEventListener('click', () => {
        cutoffControl = true;
        window.plot_step(parseInt(cutoffInput.value, 10));
    });

    cutoffInput.addEventListener('change', () => {
        if (cutoffControl) {
            stepFunctionButton.click();
        } else {
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
