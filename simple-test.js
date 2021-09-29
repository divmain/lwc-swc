const { transpile, minify } = require('./index')

const fileOne = `
  function main (obj) {
    return obj?.stuff?.optional?.("maybe");
  }
`;
const fileOneBuffer = Buffer.from(fileOne);

const fileTwo = `
  import { LightningElement, api } from 'lwc';

  export default class Example extends LightningElement {
      @api name = 'World!';

      @track myMethod() {
        return true;
      }
  }
`;
const fileTwoBuffer = Buffer.from(fileTwo);

(async () => {
  console.log(await transpile("my-file.js", fileOneBuffer))
  console.log(await transpile("my-other-file.js", fileTwoBuffer));
  console.log(await Promise.all([
    minify(fileOne),
    minify(fileTwo)
  ]));
})().catch(console.error);
