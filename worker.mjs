import make_queue from './make_queue.mjs';
import init, { render } from './renderer/pkg/renderer.js';

const queue = make_queue(self);

// Run:
(async () => {
	await init();

	while (true) {
		// Main render loop - wait for render messages:
		for await (const msg of queue) {
			if (msg.type == 'render') {
				const { width, height } = msg;

				// Call the renderer:
				const data = render(width, height);
				console.log(data);

				// Return the data:
				postMessage({
					type: 'render-finished',
					data
				}, data.buffer)
			}
		}
	}
})();