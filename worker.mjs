import make_queue from './make_queue.mjs';
// import init, { render } from './renderer/pkg/renderer.js';

const queue = make_queue(self);

let ready_for_progress = true;

// Run:
(async () => {
	let memory;
	const imports = {
		env: {
			set_progress(progress) {
				if (ready_for_progress) {
					postMessage({
						type: 'progress',
						progress
					});
					// Produce progress messages at most every 30ms.  The overhead of postMessage was really slowing down rendering.
					// A better way of doing progress would be to use a shared array buffer so that the main thread could read the progress directly from the worker while it's running.  Currently the wasm has to call out to JS at each progress point which slows it down.  Also, the shared array would allow the main thread to use animation frames to only get the updates that it needs.
					ready_for_progress = false;
					setTimeout(() => ready_for_progress = true, 30);
				}
			},
			get_random(ptr, len) {
				let buff = new Uint8Array(len);
				crypto.getRandomValues(buff);
				const target = new Uint8Array(memory.buffer, ptr, len);
				target.set(buff);
			},
			console_log(ptr, len) {
				const decoder = new TextDecoder('utf8', {
					fatal: true
				});
				const buff = memory.buffer.slice(ptr, ptr + len);
				console.log(decoder.decode(buff));
			}
		}
	};

	let instance;
	for await (const msg of queue) {
		if (msg.type == 'module') {
			instance = await WebAssembly.instantiate(msg.module, imports);
			break;
		}
	}
	const { render } = instance.exports;
	memory = instance.exports.memory;
	// await init();

	while (true) {
		// Main render loop - wait for render messages:
		for await (const msg of queue) {
			if (msg.type == 'render') {
				const { width, height } = msg;
				const length = width * height * 4;

				// Call the renderer:
				const ptr = render(width, height);
				const data = new Uint8Array(memory.buffer.slice(ptr, ptr + length));

				// Return the data:
				postMessage({
					type: 'render-finished',
					data
				}, data.buffer)
			}
		}
	}
})();