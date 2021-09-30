
export default class ChartArea {
	top = 0
	left = 0
	width = 1
	height = 1

	elements = []

	constructor(chart, options = {}) {
		this.top    = options.top    ? options.top    : 0;
		this.left   = options.left   ? options.left   : 0;
		this.width  = options.width  ? options.width  : 1;
		this.height = options.height ? options.height : 1;
		if (options.elements) {
			this.elements.push(...options.elements);
		}
	}

	/**
	 * 
	 * @param {CanvasRenderingContext2D} ctx
	 * @param {ChartData} data
	 */
	render(ctx, data) {
		const x = ctx.canvas.width * this.left;
		const y = ctx.canvas.height * this.top;
		const w = ctx.canvas.width * this.width;
		const h = ctx.canvas.height * this.height;
		this.elements.forEach(e => e.render(ctx, x, y, w, h, data));
	}
}
