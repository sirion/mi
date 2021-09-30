import ScaledChartElement from "./scaledchartelement.js";

export default class Grid extends ScaledChartElement {

	render(ctx, x, y, w, h, data = null) {
		const stepsize = data.options.grid.stepsize;

		ctx.save();
		ctx.lineWidth = ((w + h) / 1200);
		ctx.strokeStyle = "lightgrey";
		
		if (stepsize.x) {
			for (let i = data.bounds.x[0]; i <= data.bounds.x[1]; i += stepsize.x) {
				const lx = x + this.scales.scaleX(i) * w;
				ctx.beginPath();
				ctx.moveTo(lx, y);
				ctx.lineTo(lx, y + h);
				ctx.stroke();
			}
		}

		if (stepsize.y) {
			for (let i = data.bounds.y[0]; i <= data.bounds.y[1]; i += stepsize.y) {
				const ly = y + h * (1 - this.scales.scaleY(i));
				ctx.beginPath();
				ctx.moveTo(x , ly);
				ctx.lineTo(x + w,  ly);
				ctx.stroke();
			}
		}

		ctx.restore();
	}
}
