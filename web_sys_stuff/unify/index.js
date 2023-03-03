// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('./pkg')
  .catch(console.error);

rust
  .then(m => m.greet('World!'))
  .catch(console.error);

// rust
//   .then(m => alert('1 + 2 = ' + m.add(1, 2)))
//   .catch(console.error);


/* eslint-disable no-unused-vars */
import { chars } from './chars-list.js';
let mod;

let counters = [];
rust
  .then(wasm => {
      mod = wasm;
      addCounter();
      let b = document.getElementById('add-counter');
      if (!b) throw new Error('Unable to find #add-counter');
      b.addEventListener('click', ev => addCounter());
  })
  .catch(console.error);

function addCounter() {
    let ctr = mod.Counter.new(randomChar(), 0);
    counters.push(ctr);
    update();
}

function update() {
    let container = document.getElementById('container');
    if (!container) throw new Error('Unable to find #container in dom');
    while (container.hasChildNodes()) {
        if (container.lastChild.id == 'add-counter') break;
        container.removeChild(container.lastChild);
    }
    for (var i = 0; i < counters.length; i++) {
        let counter = counters[i];
        container.appendChild(newCounter(counter.key(), counter.count(), ev => {
            counter.increment();
            update();
        }));
    }
}

function randomChar() {
    console.log('randomChar');
    let idx = Math.floor(Math.random() * (chars.length - 1));
    console.log('index', idx);
    let ret = chars.splice(idx, 1)[0];
    console.log('char', ret);
    return ret;
}

function newCounter(key, value, cb) {
    let container = document.createElement('div');
    container.setAttribute('class', 'counter');
    let title = document.createElement('h1');
    title.appendChild(document.createTextNode('Counter ' + key));
    container.appendChild(title);
    container.appendChild(newField('Count', value));
    let plus = document.createElement('button');
    plus.setAttribute('type', 'button');
    plus.setAttribute('class', 'plus-button');
    plus.appendChild(document.createTextNode('+'));
    plus.addEventListener('click', cb);
    container.appendChild(plus);
    return container;
}

function newField(key, value) {
    let ret = document.createElement('div');
    ret.setAttribute('class', 'field');
    let name = document.createElement('span');
    name.setAttribute('class', 'name');
    name.appendChild(document.createTextNode(key));
    ret.appendChild(name);
    let val = document.createElement('span');
    val.setAttribute('class', 'value');
    val.appendChild(document.createTextNode(value));
    ret.appendChild(val);
    return ret;
}


rust
  .then(m => {
      return m.run_fetch("rustwasm/wasm-bindgen").then((data) => {
          console.log(data);

          console.log("The latest commit to the wasm-bindgen %s branch is:", data.name);
          console.log("%s, authored by %s <%s>", data.commit.sha, data.commit.commit.author.name, data.commit.commit.author.email);
      })
  })
  .catch(console.error);

  
import('./crate/pkg')
  .catch(console.error);

rust
  .then(wasm => {
    const canvas = document.getElementById('drawing');
    const ctx = canvas.getContext('2d');

    const realInput = document.getElementById('real');
    const imaginaryInput = document.getElementById('imaginary');
    const renderBtn = document.getElementById('render');

    renderBtn.addEventListener('click', () => {
      const real = parseFloat(realInput.value) || 0;
      const imaginary = parseFloat(imaginaryInput.value) || 0;
      wasm.draw_julia(ctx, 600, 600, real, imaginary);
    });

    wasm.draw_julia(ctx, 600, 600, -0.15, 0.65);
  })
  .catch(console.error);

rust.then(rust_module => {
    let fm = null;

    const play_button = document.getElementById("play");
    play_button.addEventListener("click", event => {
      if (fm === null) {
        fm = new rust_module.FmOsc();
        fm.set_note(50);
        fm.set_fm_frequency(0);
        fm.set_fm_amount(0);
        fm.set_gain(0.8);
      } else {
        fm.free();
        fm = null;
      }
    });

    const primary_slider = document.getElementById("primary_input");
    primary_slider.addEventListener("input", event => {
      if (fm) {
        fm.set_note(parseInt(event.target.value));
      }
    });

    const fm_freq = document.getElementById("fm_freq");
    fm_freq.addEventListener("input", event => {
      if (fm) {
        fm.set_fm_frequency(parseFloat(event.target.value));
      }
    });

    const fm_amount = document.getElementById("fm_amount");
    fm_amount.addEventListener("input", event => {
      if (fm) {
        fm.set_fm_amount(parseFloat(event.target.value));
      }
    });
  })
  .catch(console.error);

window.addEventListener('load', async () => {
  await rust.then(lol => {lol.start_webrtc()});
});
