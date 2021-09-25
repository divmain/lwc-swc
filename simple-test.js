const { transpile } = require('./index')

const inputA = Buffer.from(`
  function main (obj) {
    return obj?.stuff?.optional?.("maybe");
  }
`);

const inputB = Buffer.from(`
  import { LightningElement, api } from 'lwc';

  export default class Example extends LightningElement {
      @api name = 'World!';

      myMethod() {
        return true;
      }
  }
`);

(async () => {
  console.log(await transpile("my-file.js", inputA))
  console.log(await transpile("my-other-file.js", inputB));
})().then(console.log, console.error);

// // require("./lwc-swc.darwin-x64.node").transpile("myfile.js", Buffer.from(`function main (obj) { return obj?.stuff?.optional ?? "default"; }`)).then(console.log, console.error)

// // console.assert(sync(0) === 100, 'Simple test failed')

// console.info('Simple test passed')
