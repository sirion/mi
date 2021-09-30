import ScaledChartElement from "./scaledchartelement.js";
import DrawUtil from "./drawutil.js";

export default class CircleLines extends ScaledChartElement {

	render(ctx, x, y, w, h, data = null) {
		const cs = ctx.canvas.width + ctx.canvas.height;
		const circlesize = cs / 300;
		const linesize = cs / 1200;

		ctx.lineWidth = linesize;

		ctx.save();
		data.ids.forEach(id => {
			const color = data.getInfo(id, "color") || DrawUtil.colorFromString(id);
			const values = data.getValues(id);
			
			const colorString = color.join(",");
			ctx.strokeStyle = "rgb(" + colorString + ")";
			ctx.fillStyle = "rgba(" + colorString + ", 0.5)";

			let last = null;
			for (let i = 0; i < values.length; ++i) {
				const coords = [
					x + w * this.scales.scaleX(values.x[i]),
					y + h * (1 - this.scales.scaleY(values.y[i]))
				];

				if (last) {
					ctx.beginPath();
					ctx.moveTo(...last);
					ctx.lineTo(...coords);
					ctx.stroke();
				}

				ctx.beginPath();
				ctx.arc(coords[0], coords[1], circlesize, 0, 2* Math.PI);
				ctx.fill();

				last = coords;
			}

		});	
		ctx.restore();
	}
}
