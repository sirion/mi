import Axis from "./charts/axis.js";
import Chart from "./charts/chart.js";
import ChartArea from "./charts/chartarea.js";
import CircleLines from "./charts/circlelines.js";
import Grid from "./charts/grid.js";
import LinearAxisNumbers from "./charts/linearaxisnumbers.js";
import LinearChartScale from "./charts/linearchartscale.js";
import LinearTrend from "./charts/lineartrend.js";
import Targets from "./charts/targets.js";

// import ChartElement from "./chartelement.js";

export default class LinearChart extends Chart {

	constructor(options = {}) {
		super(...arguments);

		this.scales["default"] = new LinearChartScale(this);

		this.areas.push(new ChartArea(this, {
			top: 0.05,
			height: 0.85,
			left: 0.05,
			width: 0.925,
			elements: [
				new Grid(this),
				new Targets(this),
				new LinearTrend(this),
				new CircleLines(this)
			]
		}));

		this.areas.push(new ChartArea(this, {
			top: 0.9,
			height: 0.1,
			left: 0.05,
			width: 0.925,
			elements: [
				new Axis(this, { type: "x" }),
				new LinearAxisNumbers(this, Object.assign({
					type: "x"
				}, options.axes ? options.axes.x : {})),
			]
		}));

		this.areas.push(new ChartArea(this, {
			top: 0.05,
			height: 0.85,
			left: 0,
			width: 0.05,
			elements: [
				new Axis(this, { type: "y" }),
				new LinearAxisNumbers(this, Object.assign({
					type: "y",
					stepsize: 1
				}, options.axes ? options.axes.y : {})),
			]
		}));

	}
}
