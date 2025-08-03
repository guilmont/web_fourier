// Define constants for canvas dimensions
import { loadWasm, getWasmExports } from './wasm-utils.js';
import { getCanvasImports } from './canvas-wasm.js';
/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////
// Initialize when page loads
document.addEventListener('DOMContentLoaded', async () => {
    // Load the WebAssembly module with Canvas imports
    await loadWasm('./web_fourier.wasm', getCanvasImports());
    const WASM = getWasmExports();
    const exampleFreqMinInput = document.getElementById('example-freq-min');
    const exampleMaxFreqInput = document.getElementById('example-freq-max');
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
});
