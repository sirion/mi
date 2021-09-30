
export default class LinearChartScale {

	chart = null

	constructor(chart, options = {}) {
		this.chart = chart;
	}


	scaleX(value) {
		const b = this.chart.data.bounds;
		return (value - b.x[0]) / (b.x[1] - b.x[0]);
	}

	scaleY(value) {
		const b = this.chart.data.bounds;
		return (value - b.y[0]) / (b.y[1] - b.y[0]);
	}
}
