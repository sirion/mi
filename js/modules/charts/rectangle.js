import ScaledChartElement from "./scaledchartelement.js";

export default class Rectangle extends ScaledChartElement {

	color = "white";

	constructor(options) {
		super(options);

		if (options.color) {
			this.color = options.color;
		}
	}

	/**
	 * @param {CanvasRenderingContext2D} ctx
	 */
	render(ctx, x, y, w, h, data = null) {
		ctx.save();

		ctx.fillStyle = this.color;
		ctx.fillRect(x, y, w, h);

		ctx.restore();
	}
}
