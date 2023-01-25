let vm;

const wasmedge = require("wasmedge-core");
const path = require("path").join(
  __dirname,
  "..",
  "..",
  "target/wasm32-wasi/debug/custom_framework_wasm.wasm"
);
console.log(path);

vm = new wasmedge.VM(path, {
  EnableWasiStartFunction: false,
  env: process.env,
  args: process.argv,
  preopens: { "/": __dirname },
});

console.log(vm);

module.exports = {
  start: () => {
    vm.Run("start");
  },
  test: () => {
    return vm.RunInt("test");
  },
};
