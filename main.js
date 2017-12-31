function copyCStr(module, ptr) {
  const orig_ptr = ptr;
  const collectCString = function*() {
    let memory = new Uint8Array(module.memory.buffer);
    while (memory[ptr] !== 0) {
      if (memory[ptr] === undefined) {
        throw new Error('Tried to read undef mem');
      }
      yield memory[ptr];
      ptr += 1;
    }
  };

  const buffer_as_u8 = new Uint8Array(collectCString());
  const utf8Decoder = new TextDecoder('UTF-8');
  const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
  module.dealloc_str(orig_ptr);
  return buffer_as_utf8;
}

let module;

fetch('wasm_of_life.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes =>
    // the Rust side needs a cos function
    WebAssembly.instantiate(bytes, {
      env: {
        floor: Math.floor,
        log: function(ptr) {
          let str = copyCStr(module, ptr);
          console.log(str);
        },
      },
    }),
  )
  .then(results => {
    module = results.instance.exports;

    const width = 500;
    const height = 500;

    const canvas = document.getElementById('screen');
    if (canvas.getContext) {
      const ctx = canvas.getContext('2d');

      const byteSize = width * height * 4;
      const pointer = module.alloc(byteSize);

      const buffer = new Uint8ClampedArray(
        module.memory.buffer,
        pointer,
        byteSize,
      );
      const img = new ImageData(buffer, width, height);

      let start = null;

      function step(timestamp) {
        if (start === null) {
          start = timestamp;
        }
        const progress = timestamp - start;
        if (progress > 1000) {
          module.draw(pointer, width, height);
          ctx.putImageData(img, 0, 0);

          start = timestamp;
        }
        window.requestAnimationFrame(step);
      }

      window.requestAnimationFrame(step);
    }
  });
