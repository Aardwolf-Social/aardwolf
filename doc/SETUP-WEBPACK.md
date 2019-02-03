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

Update npm (Just to be safe)
`npm install npm@latest -g`

## Setup WebPack for Aardwolf (Work in Progress)

Go into the aardwolf directory
`cd /path/to/aardwolf`

 Initialize the setup script (it is okay to accept the default values)
 `npm init`

 Install the dependencies
 - webpack, and webpack-cli should be installed first the rest are in alphabetical order
```
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
```
Regarding warnings:
On Linux you may see a couple of warnings such as the ones below.  Don't fret because these dependencies are for Mac OS.
```
npm WARN optional SKIPPING OPTIONAL DEPENDENCY: fsevents@1.2.7 (node_modules/fsevents):
npm WARN notsup SKIPPING OPTIONAL DEPENDENCY: Unsupported platform for fsevents@1.2.7: wanted {"os":"darwin","arch":"any"} (current: {"os":"linux","arch":"x64"})
```

Build the packages
`npm run build`

Thats it!

Source URL:
https://bulma.io/documentation/customize/with-webpack/

