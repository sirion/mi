export default class DrawUtil {

	static colors = [
		[255,   0,   0],
		[  0, 255,   0],
		[  0,   0, 255],
		[  0, 255, 255],
		[180, 180,   0],
		[255,   0, 255],
	];

	static colorFromString(str) {
		let c = 0;
		for (let i = 0; i < str.length; ++i) {
			c = (c + str.charCodeAt(i)) % DrawUtil.colors.length;
		}

		return DrawUtil.colors[c];
	}

	static drawLine(ctx, from, to, lineWidth, color) {
		ctx.save();
		ctx.lineWidth = lineWidth;
		ctx.strokeStyle = color;
		ctx.fillStyle = color;
		
		ctx.beginPath();
		ctx.moveTo(...from);
		ctx.lineTo(...to);
		ctx.stroke();

		ctx.restore();
	}

	static drawArrow(ctx, from, sizes, direction, color) {
		ctx.save();
		ctx.strokeStyle = color;
		ctx.fillStyle = color;
		
		let p1 = from.slice();
		let p2 = from.slice();
		let p3 = from.slice();

		switch (direction) {
			default:
			case "top":
				p1[1] -= sizes[0];
				p2[0] += sizes[1];
				p3[0] -= sizes[1];
				break;

			case "left":
				p1[0] -= sizes[0];
				p2[1] += sizes[1];
				p3[1] -= sizes[1];
				break;

			case "bottom":
				p1[1] += sizes[0];
				p2[0] += sizes[1];
				p3[0] -= sizes[1];
				break;

			case "right":
				p1[0] += sizes[0];
				p2[1] += sizes[1];
				p3[1] -= sizes[1];
				break;
		}

		ctx.beginPath();

		ctx.moveTo(p1[0], p1[1]);
		ctx.lineTo(p2[0], p2[1]);
		ctx.lineTo(p3[0], p3[1]);

		ctx.closePath();
		ctx.fill();
		
		ctx.restore();
	}
}
