### INTRODUCTION

Rather than including full packages for application/web styling (Bulma.io, ForkAwesome, etc) we are using WebPack.
The alternative would be to simply include CDN links (ex: https://bulma.io/path/to/bulma-min.css), however there may
be folks that would prefer to have local copies of everything.

This document will describe how to use WebPack

### Requirements
#### Install Node
[Node documentation](https://github.com/nodesource/distributions/blob/master/README.md)

#### Install npm
[npm documentation](https://www.npmjs.com/package/npm)

Update npm (Just to be safe)

    $ npm install npm@latest -g

### Setup WebPack for Aardwolf (Work in Progress)

Go into the aardwolf directory

    $ cd /path/to/aardwolf

Install the dependencies

    $ npm install --dev

Regarding warnings:
On Linux you may see a couple of warnings such as the ones below.  Don't fret because these dependencies are for Mac OS.

```
npm WARN optional SKIPPING OPTIONAL DEPENDENCY: fsevents@1.2.7 (node_modules/fsevents):
npm WARN notsup SKIPPING OPTIONAL DEPENDENCY: Unsupported platform for fsevents@1.2.7: wanted {"os":"darwin","arch":"any"} (current: {"os":"linux","arch":"x64"})
```

Build the packages

    $ npm run build

Thats it!

Source URL:
https://bulma.io/documentation/customize/with-webpack/
