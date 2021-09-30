import ScaledChartElement from "./scaledchartelement.js";
import DrawUtil from "./drawutil.js";
import Spline from "./utils/spline.js";

export default class CircleLines extends ScaledChartElement {

	steps = 7;
	color = null;

	splines = {};

	constructor(chart, options = {}) {
		super(...arguments);
		if (options.color) {
			this.color = options.color;
		}
		if (options.steps) {
			this.steps = options.steps;
		}

		chart.addEventListener("dataChange", this.onDataChange.bind(this));
		this.onDataChange();
	}

	onDataChange(e) {
		const data = this.chart.data;

		const averages = {};
		let vals = [0, 0], step = 1;

		data.ids.forEach(id => {
			const values = data.getValues(id);

			averages[id] = [];

			for (let i = 0; i < values.length; ++i) {
				// if (i == 0) {
				// 	// Start value will be used instead of average
				// 	averages[id].push([ values.x[i], values.y[i] ]);
				// }

				vals[0] += values.x[i]
				vals[1] += values.y[i]
				if (step < this.steps && i != values.length - 1) {
					step++;
				} else {
					let x = vals[0] / step
					let y = vals[1] / step
					if (i == values.length - 1) {
						x = values.x[i];
					}

					if (averages[id].length === 0) {
						// First average will be at the beginning of the x axis
						x = values.x[0];
					} else if (i === values.length - 1) {
						// First average will be at the end of the x axis
						x = values.x[i];
					}

					averages[id].push([ x, y ]);
					step = 1
					vals = [0, 0];
				}
			}
		});

		this.splines = {};
		data.ids.forEach(id => {
			this.splines[id] = new Spline({
				factor: 10,
				entries: averages[id]
			});
		});
	}

	render(ctx, x, y, w, h, data = null) {
		const cs = ctx.canvas.width + ctx.canvas.height;
		const circlesize = cs / 300;
		const linesize = cs / 1200;

		ctx.save();

		const color = this.color !== null ? this.color : (data.getInfo(id, "color") || DrawUtil.colorFromString(id));
		const colorString = color.join(",");
		ctx.lineWidth = linesize;
		ctx.strokeStyle = "rgb(" + colorString + ")";
		ctx.fillStyle = "rgba(" + colorString + ", 0.5)";

		data.ids.forEach(id => {
			let last = null;

			const entries = this.splines[id].entries;

			for (let i = 0; i < entries.length; ++i) {
				const coords = [
					x + w * this.scales.scaleX(entries[i][0]),
					y + h * (1 - this.scales.scaleY(entries[i][1]))
				];

				if (last) {
					ctx.beginPath();
					ctx.moveTo(...last);
					ctx.lineTo(...coords);
					ctx.stroke();
				}

				last = coords;
			}

		});


		ctx.restore();
	}
}
