window.loadModule = async () => {

  window.AudioContext = window.AudioContext || window.webkitAudioContext;
  window.actx = new window.AudioContext({
    sampleRate: 44100
  })

  URLFromFiles(['engine.js', 'index.js']).then((e) => {
    
    window.actx.audioWorklet.addModule(e).then(() => {
      window.node = new AudioWorkletNode(window.actx, 'glicol-engine', {outputChannelCount: [2]})
      fetch('./glicol_wasm.wasm')
      .then(response => response.arrayBuffer())
      .then(arrayBuffer => {
        window.node.port.postMessage({
        type: "load", obj: arrayBuffer})
      })

      window.actx.destination.channelInterpretation = "discrete";
      window.node.connect(window.actx.destination)

      let sab2 = exports.RingBuffer.getStorageForCapacity(1024, Uint8Array);
      let rb2 = new exports.RingBuffer(sab2, Uint8Array);
      window.paramWriter = new ParameterWriter(rb2);
      window.node.port.postMessage({
          type: "sab",
          data: sab2
      });

      console.clear();
      console.log("%c"+window.art, "color: #3E999F")
      console.log(`\n\nAvailable nodes:`) //, "background: green; font-weight: bold");
      console.log(Object.keys(window.docs.about))
  
      console.log(`\n\nFetch help files by:`) //, "background: grey; font-weight: bold");
      console.log(`typing help("the node name") in the console, e.g. help("sin");\n\nor move the cursor to a keyword and press Ctrl+Shift+/`) //, "background: green");

      // paramWriter.enqueue_change(0, e.target.value)
    })
  })
}
window.loadModule();

