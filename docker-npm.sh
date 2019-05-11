#!/bin/sh

# WEBPACK for Aardwolf
npm install npm@latest -g
npm install
npm init -y
npm install webpack --save-dev 
npm install webpack-cli --save-dev
npm install bulma --save-dev 
npm install clean-webpack-plugin --save-dev 
npm install css-loader --save-dev 
npm install extract-text-webpack-plugin@next --save-dev 
npm install mini-css-extract-plugin --save-dev 
npm install node-sass --save-dev 
npm install sass-loader --save-dev 
npm install style-loader --save-dev
npm run build