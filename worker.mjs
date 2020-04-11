import make_queue from './make_queue.mjs';
// import init, { render } from './renderer/pkg/renderer.js';

const queue = make_queue(self);

let last_progress = false;

// Run:
(async () => {
	let memory;
	const imports = {
		env: {
			set_progress(progress) {
				let now = Date.now();
				if (!last_progress || last_progress + 20 < now) {
					postMessage({
						type: 'progress',
						progress
					});
					last_progress = now;
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
	const { init, render } = instance.exports;
	memory = instance.exports.memory;

	// Let the module do any initialization
	init();

	while (true) {
		// Main render loop - wait for render messages:
		for await (const msg of queue) {
			if (msg.type == 'render') {
				const { width, aspect } = msg;
				const length = width * width * aspect * 4;

				// Call the renderer:
				const ptr = render(aspect, width);
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