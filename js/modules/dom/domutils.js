const c = document.createElement.bind(document);

function d(options) {
	const type = options.type || "div";

	let namespace = null;
	if (!options.namespace) {
		if (type.toLowerCase() === "svg") {
			namespace = "http://www.w3.org/2000/svg";
		}
	} else {
		namespace = options.namespace;
	}

	let el;
	if (namespace) {
		el = document.createElementNS(namespace, type);
	} else {
		el = document.createElement(type);
	}


	if (options.id) {
		el.id = options.id;
	}

	if (options.classes) {
		const classes = Array.isArray(options.classes) ? options.classes : [ options.classes ];
		classes.forEach(cl => el.classList.add(cl));
	}

	if (options.textContent) {
		el.textContent = options.textContent;
	}

	if (options.content) {
		if (typeof options.content === "string") {
			el.textContent = options.content;
		} else {
			const content = Array.isArray(options.content) ? options.content : [ options.content ];
			el.append(...content);
		}
	}

	if (options.attributes) {
		Object.keys(options.attributes).forEach(name => {
			if (options.attributes[name] !== undefined) {
				el.setAttribute(name, options.attributes[name]);
			}
		});
	}
	if (options.style) {
		Object.assign(el.style, options.style);
	}

	if (options.events) {
		Object.keys(options.events).forEach(name => {
			el.addEventListener(name, options.events[name]);
		});
	}

	if (options.children) {
		el.append(...options.children.map(c => {
			if (c instanceof Node) {
				// Allow using Nodes in children-array
				return c;
			}

			if (namespace) {
				c = Object.assign({ namespace: namespace }, c);
			}
			return d(c)
		}));
	}

	return el;
}

export { c, d };