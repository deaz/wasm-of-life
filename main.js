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
    WebAssembly.instantiate(bytes, {
      env: {
        log: function(ptr) {
          let str = copyCStr(module, ptr);
          console.log(str);
        },
      },
    }),
  )
  .then(results => {
    module = results.instance.exports;

    const cell_size = 5;

    const width = Math.max(
      document.documentElement.clientWidth,
      window.innerWidth || 0,
    );
    const height = Math.max(
      document.documentElement.clientHeight,
      window.innerHeight || 0,
    );

    const canvas = document.getElementById('screen');
    canvas.width = width;
    canvas.height = height;

    if (canvas.getContext) {
      const ctx = canvas.getContext('2d');

      const byteSize = width * height * 4;
      const pointer = module.alloc(byteSize);

      const buffer = new Uint8ClampedArray(
        module.memory.buffer,
        pointer,
        byteSize,
      );
      let img = new ImageData(buffer, width, height);

      module.init(
        Math.floor(width / cell_size),
        Math.floor(height / cell_size),
      );

      let start = null;

      function step(timestamp) {
        if (start === null) {
          start = timestamp;
        }
        const progress = timestamp - start;
        if (progress > 20) {
          module.draw(pointer, width, height);

          // Hack: buffer becomes empty sometimes
          if (buffer.length === 0) {
            const buffer = new Uint8ClampedArray(
              module.memory.buffer,
              pointer,
              byteSize,
            );
            img = new ImageData(buffer, width, height);
          }

          ctx.putImageData(img, 0, 0);

          start = timestamp;
        }
        window.requestAnimationFrame(step);
      }

      window.requestAnimationFrame(step);
    }
  });
