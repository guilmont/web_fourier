// Define constants for canvas dimensions
import { loadWasm, getWasmExports, WasmExports } from './wasm-utils.js';
import { getCanvasImports } from './canvas-wasm.js';


interface FourierExports extends WasmExports {
    plot_example: (kMin: number, kMax: number, kind: number) => void;
    step_animation: () => void;
    play_pause_animation: (kMin: number, kMax: number, example: number) => void;
    stop_animation: () => void;
    increase_animation_speed: () => void;
    decrease_animation_speed: () => void;
}

/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

// Initialize when page loads
document.addEventListener('DOMContentLoaded', async () => {
    // Load the WebAssembly module with Canvas imports
    await loadWasm('./web_fourier.wasm', getCanvasImports());
    const WASM = getWasmExports() as FourierExports;


    const exampleFreqMinInput = document.getElementById('example-freq-min')! as HTMLInputElement;
    const exampleMaxFreqInput = document.getElementById('example-freq-max')! as HTMLInputElement;

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

    // Initial plot and highlight
    document.querySelector('.example-btn[data-example="step"]')?.classList.add('active');
    plotCurrentExample();

    // Animation controls
    const animationFreqMinInput = document.getElementById('animation-freq-min')! as HTMLInputElement;
    const animationFreqMaxInput = document.getElementById('animation-freq-max')! as HTMLInputElement;

    // Animation button handlers
    const playPauseBtn = document.getElementById('play-pause')! as HTMLButtonElement;
    const stopBtn = document.getElementById('stop')! as HTMLButtonElement;

    let currentAnimationExample = 0; // Track current animation example
    playPauseBtn.addEventListener('click', () => {
        const kMin = parseInt(animationFreqMinInput.value, 10);
        const kMax = parseInt(animationFreqMaxInput.value, 10);
        WASM.play_pause_animation(kMin, kMax, currentAnimationExample);
    });

    stopBtn.addEventListener('click', () => { WASM.stop_animation(); });
    document.getElementById('forward')!.addEventListener('click',  () => { WASM.increase_animation_speed(); });
    document.getElementById('backward')!.addEventListener('click', () => { WASM.decrease_animation_speed(); });

    animationFreqMinInput.addEventListener('change', () => { stopBtn.click(); playPauseBtn.click(); });
    animationFreqMaxInput.addEventListener('change', () => { stopBtn.click(); playPauseBtn.click(); });

    // Animation example buttons
    document.querySelectorAll('.animation-example-btn').forEach((btn, idx) => {
        btn.addEventListener('click', (e) => {
            // Stop current animation
            stopBtn.click();

            // Update current example
            currentAnimationExample = idx;
            // Highlight selected
            document.querySelectorAll('.animation-example-btn').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');

            // Change to the new example
            const kMin = parseInt(animationFreqMinInput.value, 10);
            const kMax = parseInt(animationFreqMaxInput.value, 10);
            WASM.play_pause_animation(kMin, kMax, currentAnimationExample);
        });
    });

    // Highlight first animation example by default
    document.querySelector('.animation-example-btn')?.classList.add('active');

    // Quick initialization so we see something in the screen
    playPauseBtn.click();
    setTimeout(() => { playPauseBtn.click(); }, 5);

});
