// Turn the message events into something that we can async iterate over.
export default function make_queue(target) {
	const receive_queue = {
		unread: [],
		waiting: false,
		async *[Symbol.asyncIterator]() {
			while (true) {
				if (this.unread.length) {
					yield this.unread.shift();
				} else {
					let res;
					const prom = new Promise(resolve => res = resolve);
					this.waiting = res;
					await prom;
					this.waiting = false;
				}
			}
		}
	};
	target.onmessage = ({ data }) => {
		receive_queue.unread.push(data);
		if (receive_queue.waiting) {
			receive_queue.waiting();
		}
	};
	return receive_queue;
}