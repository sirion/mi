import ScaledChartElement from "./scaledchartelement.js";
import DrawUtil from "./drawutil.js";

export default class Axis extends ScaledChartElement {
	type = "x"
	position = "top-right"

	constructor(chart, options = {}) {
		super(...arguments);
		if (options.type) {
			this.type = options.type;
		}
		if (options.text) {
			this.text = options.text;
		}
		if (options.position) {
			this.position = options.position;
		}
	}

	render(ctx, x, y, w, h, data = null) {
		const cs = ctx.canvas.width + ctx.canvas.height;
		const lw = cs / 1200;
		const al = cs / 150;
		const aw = cs / 425;

		if (this.type === "x") {
			const ly = this.position === "top-right" ? y : y + h;

			DrawUtil.drawLine(ctx, [x - al, ly], [x + w, ly], lw, "black");
			DrawUtil.drawArrow(ctx, [x + w, ly], [al, aw], "right", "black");
		} else {
			const lx = this.position === "top-right" ? x + w : x;

			DrawUtil.drawLine(ctx, [lx, y + h + al], [lx, y], lw, "black");
			DrawUtil.drawArrow(ctx, [lx, y], [al, aw], "top", "black");
		}
	}
}