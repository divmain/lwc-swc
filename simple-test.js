const { transpile } = require('./index')

const inputA = Buffer.from(`
  function main (obj) {
    return obj?.stuff?.optional?.("maybe");
  }
`);

const inputB = Buffer.from(`
  import { LightningElement, api } from 'lwc';

  class Example extends LightningElement {
      @api name = 'World!';

      @track myMethod() {
        return true;
      }
  }

  export default Example;
`);

(async () => {
  console.log(await transpile("my-file.js", inputA))
  console.log(await transpile("my-other-file.js", inputB));
})().catch(console.error);
