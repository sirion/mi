import ScaledChartElement from "./scaledchartelement.js";

export default class LinearAxisNumbers extends ScaledChartElement {
	type = "x"

	stepsize = 1
	
	formatter = d => d

	constructor(chart, options = {}) {
		super(...arguments);
		if (options.type) {
			this.type = options.type;
		}
		if (options.stepsize) {
			this.stepsize = options.stepsize;
		}
		if (options.formatter) {
			this.formatter = options.formatter;
		}
	}

	render(ctx, x, y, w, h, data = null) {
		const cs = ctx.canvas.width + ctx.canvas.height;
		const font = cs / 150;

		// Numbers
		ctx.save();
		ctx.textBaseline = "middle";
		ctx.font = font + "px sans-serif";

		if (this.type === "x") {
			ctx.textAlign = "center";

			const [start, end] = data.bounds.x;
			const steps = (end - start) / this.stepsize;

			for (let i = 0; i <= steps; ++i) {
				ctx.save();
				ctx.translate(x + (w * (i / steps)), y);
				ctx.rotate(90 * Math.PI / 180);
				ctx.fillText(this.formatter(start + i * this.stepsize), h / 2, 0, h);
				ctx.restore();
			}

		} else {
			ctx.textAlign = "left";

			const [start, end] = data.bounds.y;
			const steps = (end - start) / this.stepsize;


			for (let i = 0; i <= steps; ++i) {
				ctx.fillText(this.formatter(start + i * this.stepsize), x + w / 2, y + (h * (1 - i / steps)), w);
			}
		}

		ctx.restore();
	}
}
