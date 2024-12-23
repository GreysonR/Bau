const path = require('path');
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
	mode: "development",
	entry: "./src/js/index.js",
	output: {
		filename: "index.js",
		clean: false,

		library: {
			name: "bau",
			type: "umd",
			umdNamedDefine: true,
		},
	},
	optimization: {
		minimize: false,
	},
	devtool: "source-map",
	watchOptions: {
		ignored: [
			"src/rust/**"
		]
	},
	devServer: {
		port: 80,
		open: false,
		hot: false,
		compress: true,
		historyApiFallback: true,
	},
	plugins: [
		new HtmlWebpackPlugin({
			filename: "index.html",
			template: "./src/index.html",
			scriptLoading: "blocking",
		}),
	],
	experiments: {
		asyncWebAssembly: true,
	},
};