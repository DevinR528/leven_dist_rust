# Lev Distance
> Wasm levenshtein distance implementation

[![Travis][build-badge]][build]
[[npm-badge]][npm]

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
[build-badge]: https://travis-ci.com/DevinR528/leven_dist_rust.svg?branch=master
[build]: https://travis-ci.com/DevinR528/leven_dist_rust

[npm-badge]: https://img.shields.io/npm/v/npm-package.png?style=flat-square
[npm]: https://www.npmjs.com/package/leven_dist