# Lev Distance
> Wasm levenshtein distance implementation

[![Build Status](https://travis-ci.com/DevinR528/leven_dist_rust.svg?branch=master)](https://travis-ci.com/DevinR528/leven_dist_rust)
[![npm package][https://img.shields.io/npm/v/npm-package.png?style=flat-square]](https://www.npmjs.com/package/leven_dist)

Simple function written in rust compiled to wasm for use in .js.

```bash
npm i leven-dist
```

```js
const lev = require('leven-dist');

console.log(lev('kitten', 'sitting'))
// 3
})
```