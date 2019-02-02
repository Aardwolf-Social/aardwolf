# INTRODUCTION

Rather than including full packages for application/web styling (Bulma.io, ForkAwesome, etc) we are using WebPack.
The alternative would be to simply include CDN links (ex: https://bulma.io/path/to/bulma-min.css), however there may
be folks that would prefer to have local copies of everything.

This document will describe how to use WebPack

## Requirements
Install Node 
(Node documentation)[https://github.com/nodesource/distributions/blob/master/README.md]

Install npm
(npm documentation)[https://www.npmjs.com/package/npm]


(( WIP ))


Go into the aardwolf directory
`cd /path/to/aardwolf`

 Initialize the setup script (it is okay to accept the default values)
 `init npm`

 Install the dependencies
```
npm install bulma --save-dev
npm install css-loader --save-dev
npm install extract-text-webpack-plugin@next --save-dev
npm install node-sass --save-dev
npm install sass-loader --save-dev
npm install style-loader --save-dev
npm install webpack --save-dev
npm install webpack-cli --save-dev
```
Build the packages
`npm run build`

Thats it!

Source URL:
https://bulma.io/documentation/customize/with-webpack/

