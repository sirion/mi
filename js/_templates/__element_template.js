// import constants from "PATH/TO/constants.js";

const template = document.createElement("template");
template.innerHTML = `
<style>
	:host {
	}

</style>
<span>classname content</span>
`;
export default class CLASSNAME extends HTMLElement {
	//////////////////////////////////////////// Constructor ///////////////////////////////////////////

	constructor(...args) {
		super(...args);

		// Init shdow DOM template
		this._shadow = this.attachShadow({ mode: 'open' });
		this._shadow.appendChild(template.content.cloneNode(true));
	}

	////////////////////////////////////// Custom Element Methods //////////////////////////////////////

	connectedCallback() {}

	disconnectedCallback() {}

	attributeChangedCallback(name, oldValue, newValue) {
		this[name] = newValue;
	}

	static get observedAttributes() {
		return [];
	}


	////////////////////////////////////// Property Getter/Setter //////////////////////////////////////

	get AAA() {
		return this._AAA;
	}

	set AAA(AAA) {
		this._AAA = AAA;
	}

	////////////////////////////////////////// Public Methods //////////////////////////////////////////
	/////////////////////////////////////////// Event Handler //////////////////////////////////////////
	////////////////////////////////////////// Private Methods /////////////////////////////////////////

}
customElements.define("doon-CLASSNAME", CLASSNAME);

///////////////////////////////////////// Hidden Functions /////////////////////////////////////////
