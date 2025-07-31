// Define constants for canvas dimensions
const CANVAS_WIDTH = 800;
const CANVAS_HEIGHT = 400;
const SPECTRUM_CANVAS_HEIGHT = 200;

interface CanvasInfo {
    canvas: HTMLCanvasElement;
    context: CanvasRenderingContext2D;
    animationId: number | null; // Animation loop id for this canvas
}

interface WasmExports extends WebAssembly.Exports {
    memory: WebAssembly.Memory;
    plot_example: (kMin: number, kMax: number, kind: number) => void;

    // Canvas functions
    canvas_mouse_move: (canvasId: number, x: number, y: number) => void;
    get_example_canvas_id: () => number;
    get_spectrum_canvas_id: () => number;
    get_animation_canvas_id: () => number;

    // Animation functions
    step_animation: () => void;
    play_pause_animation: (canvasId: number, kMin: number, kMax: number) => void;
    stop_animation: () => void;
    increase_animation_speed: () => void;
    decrease_animation_speed: () => void;
}

let WASM: WasmExports;
let CANVAS_REGISTRY: Map<number, CanvasInfo> = new Map();

// Helper function to decode WASM strings
function decodeWasmString(ptr: number, len: number): string {
    const bytes = new Uint8Array(WASM.memory.buffer, ptr, len);
    return new TextDecoder("utf-8").decode(bytes);
}

// Helper function to get canvas and context by integer ID
function getCanvasInfo(canvasId: number): CanvasInfo {
    const canvasInfo = CANVAS_REGISTRY.get(canvasId);
    if (!canvasInfo) {
        throw new Error(`Canvas with id ${canvasId} not found`);
    }
    return canvasInfo;
}

function createCanvasImports() {
    return {
        arc: (canvasId: number, x: number, y: number, radius: number, startAngle: number, endAngle: number) => {
            getCanvasInfo(canvasId).context.arc(x, y, radius, startAngle, endAngle);
        },
        begin_path: (canvasId: number) => {
            getCanvasInfo(canvasId).context.beginPath();
        },
        clear_rect: (canvasId: number, x: number, y: number, width: number, height: number) => {
            getCanvasInfo(canvasId).context.clearRect(x, y, width, height);
        },
        fill: (canvasId: number) => {
            getCanvasInfo(canvasId).context.fill();
        },
        fill_rect: (canvasId: number, x: number, y: number, width: number, height: number) => {
            getCanvasInfo(canvasId).context.fillRect(x, y, width, height);
        },
        height: (canvasId: number): number => {
            return getCanvasInfo(canvasId).canvas.height;
        },
        line_to: (canvasId: number, x: number, y: number) => {
            getCanvasInfo(canvasId).context.lineTo(x, y);
        },
        move_to: (canvasId: number, x: number, y: number) => {
            getCanvasInfo(canvasId).context.moveTo(x, y);
        },
        set_fill_style_color: (canvasId: number, r: number, g: number, b: number, a: number) => {
            getCanvasInfo(canvasId).context.fillStyle = `rgba(${r}, ${g}, ${b}, ${a})`;
        },
        set_line_width: (canvasId: number, width: number) => {
            getCanvasInfo(canvasId).context.lineWidth = width;
        },
        set_stroke_style_color: (canvasId: number, r: number, g: number, b: number, a: number) => {
            getCanvasInfo(canvasId).context.strokeStyle = `rgba(${r}, ${g}, ${b}, ${a})`;
        },
        stroke: (canvasId: number) => {
            getCanvasInfo(canvasId).context.stroke();
        },
        stroke_rect: (canvasId: number, x: number, y: number, width: number, height: number) => {
            getCanvasInfo(canvasId).context.strokeRect(x, y, width, height);
        },
        width: (canvasId: number): number => {
            return getCanvasInfo(canvasId).canvas.width;
        },

        fill_text: (canvasId: number, textPtr: number, textLen: number, x: number, y: number) => {
            const text = decodeWasmString(textPtr, textLen);
            getCanvasInfo(canvasId).context.fillText(text, x, y);
        },
        set_font: (canvasId: number, fontPtr: number, fontLen: number)  => {
            getCanvasInfo(canvasId).context.font = decodeWasmString(fontPtr, fontLen);
        },
        set_text_align: (canvasId: number, alignPtr: number, alignLen: number) => {
            getCanvasInfo(canvasId).context.textAlign = decodeWasmString(alignPtr, alignLen) as CanvasTextAlign;
        },
        measure_text_width: (canvasId: number, textPtr: number, textLen: number, fontPtr: number, fontLen: number): number => {
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
        log:   (ptr: number, len: number) => { console.log("[WASM]", decodeWasmString(ptr, len));   },
        error: (ptr: number, len: number) => { console.error("[WASM]", decodeWasmString(ptr, len)); },
    };
}

function createBrowserImports() {
    return {
        alert:    (ptr: number, len: number) => { window.alert(decodeWasmString(ptr, len)); },
        time_now: (): number => performance.now(),
    };
}

function createAnimationImports() {
    return {
        start_animation_loop:  (canvasId: number) => {
            const canvasInfo = getCanvasInfo(canvasId);
            if (canvasInfo.animationId !== null) return; // Already running
            function animationFrame() {
                WASM.step_animation();
                canvasInfo.animationId = requestAnimationFrame(animationFrame);
            }
            canvasInfo.animationId = requestAnimationFrame(animationFrame);
        },
        stop_animation_loop:   (canvasId: number) => {
            const canvasInfo = getCanvasInfo(canvasId);
            if (canvasInfo.animationId !== null) {
                cancelAnimationFrame(canvasInfo.animationId);
                canvasInfo.animationId = null;
            }
        },
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

        WASM = wasmModule.instance.exports as WasmExports;
        console.log("WebAssembly loaded successfully!");

    } catch (error) {
        console.error("Failed to load WebAssembly:", error);
    }
}

// Function to register a canvas and return its integer ID
function registerCanvas(canvasName: string, canvasId: number, width: number, height: number) {
    const canvas = document.getElementById(canvasName)! as HTMLCanvasElement;
    const context = canvas.getContext('2d')!;

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

    registerCanvas('step-canvas',           EXAMPLE_CANVAS_ID,   CANVAS_WIDTH, CANVAS_HEIGHT);
    registerCanvas('animation-canvas',      ANIMATION_CANVAS_ID, CANVAS_WIDTH, CANVAS_HEIGHT);
    registerCanvas('power-spectrum-canvas', SPECTRUM_CANVAS_ID,  CANVAS_WIDTH, SPECTRUM_CANVAS_HEIGHT);


    // Add event listeners for buttons
    const playPauseButton = document.getElementById('play-pause')! as HTMLButtonElement;
    const stopButton = document.getElementById('stop')! as HTMLButtonElement;
    const backwardButton = document.getElementById('backward')! as HTMLButtonElement;
    const forwardButton = document.getElementById('forward')! as HTMLButtonElement;

    const exampleFreqMinInput = document.getElementById('example-freq-min')! as HTMLInputElement;
    const exampleMaxFreqInput = document.getElementById('example-freq-max')! as HTMLInputElement;

    const animationFreqMinInput = document.getElementById('animation-freq-min')! as HTMLInputElement;
    const animationFreqMaxInput = document.getElementById('animation-freq-max')! as HTMLInputElement;

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

    stopButton.addEventListener('click',     () => { WASM.stop_animation();           });
    backwardButton.addEventListener('click', () => { WASM.decrease_animation_speed(); });
    forwardButton.addEventListener('click',  () => { WASM.increase_animation_speed(); });

    // Initial plot and highlight
    document.querySelector('.example-btn[data-example="step"]')?.classList.add('active');
    plotCurrentExample();

    playPauseButton.click();
    setTimeout(() => {
        playPauseButton.click();
    }, 10);
});
