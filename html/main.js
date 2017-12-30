fetch("wasm_of_life.wasm")
  .then(response => response.arrayBuffer())
  .then(bytes =>
    // the Rust side needs a cos function
    WebAssembly.instantiate(bytes, { env: { cos: Math.cos } })
  )
  .then(results => {
    const module = results.instance.exports;

    const width = 500;
    const height = 500;

    const canvas = document.getElementById("screen");
    if (canvas.getContext) {
      const ctx = canvas.getContext("2d");

      const byteSize = width * height * 4;
      const pointer = module.alloc(byteSize);

      const buffer = new Uint8ClampedArray(
        module.memory.buffer,
        pointer,
        byteSize
      );
      const img = new ImageData(buffer, width, height);

      let start = null;

      function step(timestamp) {
        if (start === null) {
          start = timestamp;
        }
        const progress = timestamp - start;
        if (progress > 100) {
          module.fill(pointer, width, height, timestamp);
          ctx.putImageData(img, 0, 0);

          start = timestamp;
        }
        window.requestAnimationFrame(step);
      }

      window.requestAnimationFrame(step);
    }
  });
