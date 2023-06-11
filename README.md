# Strdiff

Strdiff is a library for calculating string differences. It leverages efficient algorithms such as the Levenshtein distance and the Damerau-Levenshtein distance.

## How to use:
*NOTE:* This requires a language that [supports WebAssembly](https://docs.wasmer.io/)

### Yarn/NPM example
- Install wapm package to yarn project
`wasmer add --yarn willdera/strdiff`

- Install wapm package to npm project
`wasmer add --npm willdera/strdiff`

- Import and use
```js
// index.js
import {bindings} from "@willdera/strdiff";

async function main() {
    const strdiff = await bindings.strdiff();
    
    // tag: 0 -> string
    // tag: 1 -> Vector of string
    let strdiff_check = strdiff.lvd({tag: 0, val: "Hello"}, {tag: 0, val: "Hella"});
    
    // tag: "ok" -> Successful execution
    // tag: "err" -> Failed execution
    console.log(strdiff_check); // { tag: 'ok', val: { tag: 0, val: 1 } }
}
```
 The example above returns a Levenshtein distance of 1, represented as `val: 1` in `{ tag: 0, val: 1 }`
*NOTE:* The example above showcases Levenshtein distance, same approach can be followed for Damerau-Levenshtein distance by replacing `strdiff.lvd(...)` with `strdiff.dlvd(...)`, same argument data-types would work.
