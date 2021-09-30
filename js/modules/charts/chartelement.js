export default class ChartElement {

	chart = null

	constructor(chart) {
		this.chart = chart;
	}

	render(ctx, x, y, w, h, data = null) {
		console.error("[ChartElement] render method must be overwritten");
	}
}
