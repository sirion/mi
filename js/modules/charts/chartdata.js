
export default class ChartData extends EventTarget {

	/**
	 * Type of data (to distinguis when using mutliple scales)
	 */
	type = "default"

	values = {}
	options = {}

	/**
	 * Contains the IDs of the data entry groups (graps) for the data type
	 */
	get ids() {
		return Object.keys(this.values);
	}

	/**
	 * Mounds are the minimum and maximum values for all values (for all IDs).
	 */
	_calculatedBounds = {
		x: [Infinity, -Infinity],
		y: [Infinity, -Infinity]
	}
	_manualBounds = {
		x: [undefined, undefined],
		y: [undefined, undefined]
	}

	get bounds() {
		const m = this._manualBounds;
		const c = this._calculatedBounds;
		return {
			x: [
				m.x[0] !== undefined ? m.x[0] : c.x[0],
				m.x[1] !== undefined ? m.x[1] : c.x[1]
			],
			y: [
				m.y[0] !== undefined ? m.y[0] : c.y[0],
				m.y[1] !== undefined ? m.y[1] : c.y[1]
			]
		}
	}

	/**
	 * Formatters are called when setting the data to convert values
	 * into numerical representations
	 */
	formatters = {
		key: d => d,
		value: d => d
	}

	constructor(options = {}) {
		super(...arguments);
		if (options.formatters) {
			if (options.formatters.key) {
				this.formatters.key = options.formatters.key;
			}
			if (options.formatters.value) {
				this.formatters.value = options.formatters.value;
			}
		}
		if (options.type) {
			this.type = options.type;
		}
		if (options.values) {
			Object.entries(options.values).forEach(e => {
				this.setValues(e[0], e[1]);
			});
		}
		if (options.infos) {
			Object.entries(options.infos).forEach(e => {
				const key = e[0];
				Object.entries(e[1]).forEach(d => {
					this.setInfo(d[0], key, d[1]);
				});
			});
		}
		if (options.options) {
			Object.entries(options.options).forEach(e => {
				this.setOptions(e[0], e[1]);
			});
		}
		if (options.bounds) {
			this.setBounds(options.bounds);
		}
	}

	/**
	 * Fire one change event delayed
	 */
	aggregateChangeEvents() {
		this.dispatchEvent(Object.assign(new Event("change"), {
			chartData: this
		}));
	}

	/**
	 * Set additional info (that is not related to any ID)
	 *
	 * @param {string|map} key - Either the key fot the info or a map/object of infos
	 * @param {any} info - If key is a string this contains the info, els it is ignored
	 */
	setOptions(key, value = null) {
		if (typeof key === "string") {
			this.options[key] = value;
		} else if (key && typeof key === "object") {
			Object.assign(this.options, key);
		} else {
			console.error("Invalid Info format.")
		}
		this.aggregateChangeEvents();
	}

	/**
	 * Set additional info (that is no display value, but related to them) for the given ID
	 *
	 * @param {string} id - The ID as used in the values
	 * @param {string|map} key - Either the key fot the info or a map/object of infos
	 * @param {any} info - If key is a string this contains the info, els it is ignored
	 */
	setInfo(id, key, info = null) {
		if (!this.values[id]) {
			this.values[id] = {};
		}
		if (!this.values[id].info) {
			this.values[id].info = {};
		}

		if (typeof key === "string") {
			this.values[id].info[key] = info;
		} else if (key && typeof key === "object") {
			Object.assign(this.values[id].info, key);
		} else {
			console.error("Invalid Info format.")
		}
		this.aggregateChangeEvents();
	}

	getInfo(id, key) {
		return this.values[id] && this.values[id].info ? this.values[id].info[key] : undefined;
	}

	/**
	 * Sets the display data for one ID
	 *
	 * @param {sting} id - The ID to identify the given display values
	 * @param {map} data - The data as key-value-pairs
	 */
	setValues(id, data) {
		// Format
		const keys = [];
		const vals = {};
		for (let k in Object.assign({}, data)) {
			const key = this.formatters.key(k);
			const value = this.formatters.value(data[k]);
			keys.push(key);
			vals[key] = value;
		}

		// Sort
		keys.sort((a, b) => a - b);
		const values = keys.map(k => vals[k]);

		this.values[id] = Object.assign(this.values[id] || {}, {
			length: keys.length,
			data: data,
			x: keys,
			y: values
		});

		const len = keys.length;
		if (len > 0) {
			const minX = keys[0];
			const maxX = keys[len - 1];
			const [ minY, maxY ] = values.reduce((minMax, val) => {
				return [ val < minMax[0] ? val : minMax[0], val > minMax[1] ? val : minMax[1] ];
			}, [ Infinity, -Infinity ]);

			this._calculatedBounds.x[0] = minX < this._calculatedBounds.x[0] ? minX : this._calculatedBounds.x[0];
			this._calculatedBounds.x[1] = maxX > this._calculatedBounds.x[1] ? maxX : this._calculatedBounds.x[1];
			this._calculatedBounds.y[0] = minY < this._calculatedBounds.y[0] ? minY : this._calculatedBounds.y[0];
			this._calculatedBounds.y[1] = maxY > this._calculatedBounds.y[1] ? maxY : this._calculatedBounds.y[1];
		}
		this.aggregateChangeEvents();
	}

	getValues(id) {
		return this.values[id];
	}

	/**
	 * Override bounds of the displayed data manually
	 *
	 * @param {ChartBounds} bounds
	 */
	setBounds(bounds) {
		const m = this._manualBounds;
		if (bounds.x) {
			m.x[0] = bounds.x[0] !== undefined ? bounds.x[0] : m.x[0];
			m.x[1] = bounds.x[1] !== undefined ? bounds.x[1] : m.x[1];
		}
		if (bounds.y) {
			m.y[0] = bounds.y[0] !== undefined ? bounds.y[0] : m.y[0];
			m.y[1] = bounds.y[1] !== undefined ? bounds.y[1] : m.y[1];
		}
		this.aggregateChangeEvents();
	}
}