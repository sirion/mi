import ScaledChartElement from "./scaledchartelement.js";
import DrawUtil from "./drawutil.js";

export default class Targets extends ScaledChartElement {

	render(ctx, x, y, w, h, data = null) {
		ctx.save();
		data.ids.forEach(id => {
			const target = data.getInfo(id, "target");
			const color = data.getInfo(id, "color") || DrawUtil.colorFromString(id);

			if (!target) {
				return;
			}

			const ly = y + (h * (1 - this.scales.scaleY(target)));

			ctx.strokeStyle = "rgba(" + color.join(",") + ",0.75)";
			ctx.beginPath();
			ctx.moveTo(x, ly);
			ctx.lineTo(x + w, ly);
			ctx.stroke();
		});	
		ctx.restore();
	}
}