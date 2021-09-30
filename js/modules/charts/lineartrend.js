import ScaledChartElement from "./scaledchartelement.js";
import DrawUtil from "./drawutil.js";

export default class LinearTrend extends ScaledChartElement {
	
	render(ctx, x, y, w, h, data = null) {
		const cs = ctx.canvas.width + ctx.canvas.height;
		const dashsize = cs / 300;
		const linesize = cs / 1200;
		
		ctx.save();
		ctx.lineWidth = linesize;
		ctx.setLineDash([dashsize, dashsize]);
		
		ctx.beginPath();
		ctx.rect(x, y, w, h);
		ctx.clip();
		
		data.ids.forEach(id => {
			const color = data.getInfo(id, "color") || DrawUtil.colorFromString(id);
			const values = data.getValues(id);
			
			ctx.strokeStyle = "rgba(" + color.join(",") + ", 0.25)";

			const st =
				(values.y[values.length - 1] - values.y[0]) /
				(values.x[values.length - 1] - values.x[0]);
			
			const fx = x => (x - values.x[0]) * st + values.y[0];

			const p1 = [
				x + w * this.scales.scaleX(data.bounds.x[0]),
				y + h * (1 - this.scales.scaleY(fx(data.bounds.x[0])))
			];
			const p2 = [
				x + w * this.scales.scaleX(data.bounds.x[1]),
				y + h * (1 - this.scales.scaleY(fx(data.bounds.x[1])))
			];

			ctx.beginPath();
			ctx.moveTo(...p1);
			ctx.lineTo(...p2);
			ctx.stroke();
		});
		ctx.restore();
	}
}

