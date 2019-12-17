const path = require('path');
const CleanWebpackPlugin = require('clean-webpack-plugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
  mode: 'development',
  entry: './web/javascript/app.js',
  output: {
    filename: 'app.js',
		path: path.resolve(__dirname, 'dist'),
  },
	plugins: [
		new CleanWebpackPlugin(['dist']),
		new MiniCssExtractPlugin({
			filename: 'app.css',
		}),
	],
	module: {
		rules: [
			{
				test: /\.css$/,
				use: [
					{
						loader: MiniCssExtractPlugin.loader,
					},
					'css-loader',
				],
			},
			{
				test: /\.woff2?$|\.ttf$|\.eot$|\.svg|\.png|\.jpg|\.jpeg|\.gif$/,
				loader: 'file-loader',
			},
		]
	},
};
