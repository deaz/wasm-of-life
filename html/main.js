fetch("wasm_of_life.wasm").then(response =>
  response.arrayBuffer()
).then(bytes =>
  // the Rust side needs a cos function
  WebAssembly.instantiate(bytes, {env: {cos: Math.cos}})
).then(results => {
  let module = {};
  let mod = results.instance;
  module.alloc = mod.exports.alloc;
  module.dealloc = mod.exports.dealloc;
  module.fill = mod.exports.fill;

  let width = 500;
  let height = 500;

  let canvas = document.getElementById('screen');
  if (canvas.getContext) {
    let ctx = canvas.getContext('2d');

    let byteSize = width * height * 4;
    let pointer = module.alloc(byteSize);

    let usub = new Uint8ClampedArray(mod.exports.memory.buffer, pointer, byteSize);
    let img = new ImageData(usub, width, height);

    let start = null;

    function step(timestamp) {
      let progress;
      if (start === null) start = timestamp;
      progress = timestamp - start;
      if (progress > 100) {
        module.fill(pointer, width, height, timestamp);

        start = timestamp;

        window.requestAnimationFrame(draw);
      } else {
        window.requestAnimationFrame(step);
      }
    }

    function draw() {
      ctx.putImageData(img, 0, 0);
      window.requestAnimationFrame(step);
    }

    window.requestAnimationFrame(step);
  }

});
