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
    };
}

function createconsoleImports() {
    return {
        log: (ptr: number, len: number) => {
            const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
            const msg = new TextDecoder("utf-8").decode(bytes);
            console.log("[WASM]", msg);
        }
    };
}

function createMathImports() {
    return {
        random: Math.random,
    }
}

async function loadWasm(): Promise<void> {
    try {
        setupCanvas();

        const wasmModule = await WebAssembly.instantiateStreaming(
            fetch('./wasm_rust.wasm'),
            {
                Math: createMathImports(),
                Console: createconsoleImports(),
                Canvas: createCanvasImports(),
            }
        );

        const expo = wasmModule.instance.exports as WebAssembly.Exports;
        wasmMemory = expo.memory as WebAssembly.Memory;

        window.draw_random_pattern = expo.draw_random_pattern;
        window.say_hi = expo.say_hi;
        window.foo = expo.foo;


        console.log("WebAssembly loaded successfully!");

    } catch (error) {
        console.error("Failed to load WebAssembly:", error);
    }
}

// Initialize when page loads
document.addEventListener('DOMContentLoaded', async () => {
    await loadWasm();
    window.draw_random_pattern();
});
