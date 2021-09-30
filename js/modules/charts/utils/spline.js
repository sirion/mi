// This code was copied from one of my very old projects and I do not know the original source
export default class Spline {
	factor = 8

	_values = {}
	_entries = [];

	_xValues = []
	_yValues = []
	_yValuesSD = [] // Second derivative




	constructor(options) {
		this._factor = options?.factor || 10;

		if (options?.data) {
			// Set data from map (key => value - x => y)
			this.entries = Object.entries(options.data);
		}

		if (options?.entries) {
			this.entries = options.entries;
		}
	}

	get factor() {
		return this._factor;
	}

	set factor(factor) {
		if (!Number.isInteger(factor)) {
			throw new Error("Factor must be an integer");
		}

		const changed = factor != this._factor;

		this._factor = factor;
		if (changed) {
			this._calculate();
		}
	}


	// Returns the value map made from the entries
	get values() {
		return Object.fromEntries(this._entries);
	}

	// Retuirn the original data as map
	get data() {
		const data = {};
		this._xValues.forEach((v, i) => data[v] = this._yValues[i]);
		return data;
	}

	// Returns the entry list as array of [x, y] arrays.
	get entries() {
		return this._entries
	}



	set entries(entries) {
		this._xValues = [];
		this._yValues = [];
		this._yValuesSD = [];
		this._values = {};


		// Sort entries by X value
		const yValues = {};
		entries.forEach(e => {
			const x = Number(e[0]);
			const y = Number(e[1]);
			this._xValues.push(x);
			yValues[x] = y;
		});
		this._xValues.sort();
		this._xValues.forEach(x => {
			this._yValues.push(yValues[x]);
		});

		this._calculate();
	}

	_calculate() {
		if (this._xValues.length === 0) {
			return;
		}

		this._initializeSecondDerivatives();
		this._calculateSplines();
	}


	_calculateSplines() {
		const n = this._xValues.length;
		const num = n * this._factor;
		this._entries = [];
		const step = Math.round((this._xValues[n - 1] - this._xValues[0]) / (num - 1));

		this._entries.push([ this._xValues[0], this._yValues[0] ]);

		for (let i = 1; i < num; i++) {
			const x = this._xValues[0] + i * step;
			const y = this._interpolateY(x);
			this._entries.push([ x, y ]);
		}
	}

	_interpolateY(x) {
		let max = this._xValues.length - 1;
		let min = 0;

		// Find value window around x
		let floor = true;
		while (max - min > 1) { // Window size of 1
			let v = Math.floor((max + min) / 2);

			if (this._xValues[v] > x) {
				max = v;
			} else {
				min = v;
			}
		}

		const deltaX = this._xValues[max] - this._xValues[min]; // h

		if (deltaX == 0) {
			throw new Error("X data values must be consecutive.");
		}


		const a = (this._xValues[max] - x) / deltaX;
		const b = (x - this._xValues[min]) / deltaX;

		const c = a * this._yValues[min];
		const d = b * this._yValues[max];

		const e = a * a * a - a;
		const f = b * b * b - b;
		const g = deltaX * deltaX;

		return c + d + ( e * this._yValuesSD[min] + f * this._yValuesSD[max]) * g / this.factor;
	}

	_initializeSecondDerivatives() {
		// Initialize spline endpoints - second derivative is 0 at the ends
		const deltas = [];
		this._yValuesSD[0] = 0;
		this._yValuesSD[this._xValues.length - 1] = 0;
		deltas[0] = 0;

		// Calculate second derivatives for other points
		for (let i = 1; i < this._xValues.length - 1; i++) {
			const twoStep = this._xValues[i + 1] - this._xValues[i - 1];
			if (twoStep == 0) {
				throw new Error("X data values must be consecutive.");
			}

			const step = (this._xValues[i] - this._xValues[i - 1]) / twoStep;

			const p = step * this._yValuesSD[i - 1] + 2;
			this._yValuesSD[i] = (step - 1) / p;

			const deltaXForward = (this._xValues[i + 1] - this._xValues[i])
			const deltaXBackward = (this._xValues[i] - this._xValues[i - 1])
			const deltaYForward = (this._yValues[i + 1] - this._yValues[i])
			const deltaYBackward = (this._yValues[i] - this._yValues[i - 1])

			const deltaXOuter = this._xValues[i + 1] - this._xValues[i - 1];

			const delta = deltaYForward / deltaXForward - deltaYBackward / deltaXBackward;
			const deltaLast = deltas[i - 1];


			deltas[i] = (this.factor * delta / (deltaXOuter) - step * deltaLast) / p;
		}

		// Update second derivatives from back to front
		for (let i = this._xValues.length - 2; i >= 0; i--) {
			this._yValuesSD[i] = this._yValuesSD[i] * this._yValuesSD[i + 1] + deltas[i];
		}
	}

}

