import ChartData from "./chartdata.js";


export default class Chart extends EventTarget {
	// /** @type {HTMLCanvasElement} **/
	// canvas = null

	/** @type {CanvasRenderingContext2D} **/
	ctx = null

	/** @type {ChartArea[]} */
	areas = []

	scales = {
		"default": null
	}

	/** @type {ChartData} */
	_data = null
	get data() {
		return this._data;
	}
	set data(data) {
		if (!(data instanceof ChartData)) {
			data = new ChartData(data);
		}

		if (this._data) {
			this._data.removeEventListener("change", this._boundOnDataChange);
		}
		if (data) {
			data.addEventListener("change", this._boundOnDataChange);
		}

		this._data = data;
		this.onDataChange();
	}

	_boundDraw = null
	set updateOnWindowResize(updateOnWindowResize) {
		if (this._boundDraw) {
			window.removeEventListener("resize", this._boundDraw);
		}

		if (updateOnWindowResize) {
			if (!this._boundDraw) {
				this._boundDraw = this.draw.bind(this);
			}
			window.addEventListener("resize", this._boundDraw);
		} else {
			this._boundDraw = null;
		}
		
		this._boundDraw = this.draw.bind(this);
	}

	get updateOnWindowResize() {
		return !!this._boundDraw;
	}

	_lastSizes = {}
	_resizeCheckInterval =  null
	set resizeCheck(resizeCheck) {
		if (this._resizeCheckInterval) {
			clearInterval(this._resizeCheckInterval);
		}

		if (resizeCheck) {
			// There is no resize event on the canvas, so we need to check manually.
			this._resizeCheckInterval = setInterval(() => {
				if (
					// Canvas exists in the DOM
					this.ctx && this.ctx.canvas.parentElement && 
					// Canvas sizes changed
					(
						this._lastSizes.w !== this.ctx.canvas.width ||
						this._lastSizes.h !== this.ctx.canvas.height
					)
				) {
					this._lastSizes.w = this.ctx.canvas.width;
					this._lastSizes.h = this.ctx.canvas.height;
					this.draw();
				}
			}, 300);		
		} else {
			this._resizeCheckInterval = null;
		}
	}

	get resizeCheck() {
		return this._resizeCheckInterval !== null;
	}

	constructor(options = {}) {
		super(...arguments);

		// Assigned here so if can be added and removed as event listener
		this._boundOnDataChange = this.onDataChange.bind(this);

		if (options.updateOnWindowResize) {
			this.updateOnWindowResize = options.updateOnWindowResize;
		} else {
			// Default to on
			this.updateOnWindowResize = true;
		}

		if (options.resizeCheck) {
			this.resizeCheck = options.resizeCheck;
		}

		if (options.data) {
			this.data = options.data;
		}
	}

	attach(canvas) {
		this.ctx = canvas.getContext("2d");
		this.draw();
	}

	release() {
		if (this.ctx) {
			this.clear();
		}
		this.ctx = null;
	}

	calibrate() {
		const w = this.ctx.canvas.clientWidth;
		const h = this.ctx.canvas.clientHeight;
		
		this.ctx.canvas.width = w;
		this.ctx.canvas.height = h;
	}

	onDataChange(event) { // Might be called with an event argument
		this.dispatchEvent(Object.assign(new Event("dataChange"), {
			chartData: this.data
		}));
		this.draw();
	};



	clear() {
		this.ctx.clearRect(0, 0, this.ctx.canvas.width, this.ctx.canvas.height);
	}

	draw(event) { // Might be called with an event argument
		if (!this.ctx) {
			return;
		}
		
		window.cancelAnimationFrame(this._nextFrame);
		this._nextFrame = window.requestAnimationFrame(() => {
			this.calibrate()
			this.clear();
			this.areas.forEach(area => area.render(this.ctx, this._data));
		});
	}
}
