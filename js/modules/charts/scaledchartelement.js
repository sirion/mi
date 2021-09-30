import ChartElement from "./chartelement.js";

export default class ScaledChartElement extends ChartElement {

	scaleName = "default"

	get scales() {
		return this.chart.scales[this.scaleName];
	}

	constructor(chart, options = {}) {
		super(...arguments);
		if (options.scaleName) {
			this.scaleName = options.scaleName;
		}
	}

}
