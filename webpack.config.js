const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const path = require("path");

var config = {
  entry: './web/javascript/app.js',
  output: {
    filename: 'app.js'
  },
   optimization: {
    splitChunks: {
      cacheGroups: {
        styles: {
          name: 'styles',
          test: /\.css$/,
          chunks: 'all',
          enforce: true
        }
      }
    }
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: "app.css",
    })
  ],
  module: {
    rules: [{
      test: /\.css$/,
      use: [
	MiniCssExtractPlugin.loader,
        {
          loader: "css-loader",
          options: {
            includePaths: [
              path.resolve("./node_modules/fork-awesome/css"),
              path.resolve("./node_modules/bulma/css")
            ]
          }
        }
      ]
    }, {
      test: /\.woff2?$|\.ttf$|\.eot$|\.svg|\.png|\.jpg|\.jpeg|\.gif$/,
      use: [{
        loader: "file-loader"
      }]
    }]
  }
};

module.exports = config;
