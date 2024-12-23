const path = require("path");
const minify = process.argv.includes("minify");
const clean = !process.argv.includes("noclean");

module.exports = {
	mode: "production",
	entry: "./src/js/index.js",
	output: {
		path: path.resolve(__dirname, "dist"),
		filename: `bau${ minify ? ".min" : "" }.js`,
		clean: clean,

		library: {
			name: "bau",
			type: "umd",
			umdNamedDefine: true,
		},
	},
	optimization: {
		minimize: minify,
	},
	experiments: {
		asyncWebAssembly: true,
	},
}
