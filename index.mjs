import make_queue from './make_queue.mjs';

// Initialization:
(async () => {
	let context, canvas, progress_el, render_el, aspect_el, width_el;
	(() => {
		// Setup the UI:
		const main = document.createElement('main');
		main.innerHTML = `
			<button>Render</button><progress max="1" style="visibility: hidden;"></progress><br>
			<label>Aspect Ratio: <input name="aspect" type="number" min="0.1" value="0.7" step="0.1"></label><br>
			<label>Width: <input name="width" type="number" min="0" step="1" value="500"></label>
			<canvas></canvas>
		`;
		canvas = main.querySelector('canvas');
		context = canvas.getContext('2d');
		context.imageSmoothingEnabled = false;
		progress_el = main.querySelector('progress');
		render_el = main.querySelector('button');
		aspect_el = main.querySelector('input[name="aspect"]');
		width_el = main.querySelector('input[name="width"]');
		document.body.insertAdjacentElement('afterbegin', main);
	})();

	// Spawn the worker that will host the renderer (this let's the renderer run without blocking the main thread):
	// TODO: Once the renderer can be parallelized, then maybe switch this to be a pool of workers:
	const worker = new Worker('./worker.mjs', {
		type: 'module'
	});
	const worker_queue = make_queue(worker);

	// Compile the renderer module:
	const module = await WebAssembly.compileStreaming(fetch('./renderer/target/wasm32-unknown-unknown/release/renderer.wasm'));
	worker.postMessage({
		type: 'module',
		module
	});

	// The main render loop:
	while (true) {
		try {
			// Wait for the render button to be clicked:
			await new Promise(resolve => render_el.onclick = resolve);
			render_el.setAttribute('disabled', '');
			progress_el.style.visibility = "visible";

			// Get input properties:
			const width = width_el.valueAsNumber;
			const aspect_ratio = aspect_el.valueAsNumber;
			const height = Math.floor(aspect_ratio * width);
			// Apply input properties:
			canvas.width = width;
			canvas.height = height; 

			// Ask the worker to render:
			worker.postMessage({
				type: 'render',
				width,
				aspect: aspect_ratio
			});

			// Wait for progress and render-finished messages
			for await (const message of worker_queue) {
				if (message.type == 'progress') {
					progress_el.value = message.progress;
				} else if (message.type == 'render-finished') {
					const image_data = new ImageData(new Uint8ClampedArray(message.data), width, height);
					context.putImageData(image_data, 0, 0);
					break;
				}
			}
		} finally {
			render_el.removeAttribute('disabled');
			progress_el.style.visibility = "hidden";
		}
	}
})();