const debug = false;

import { GameState, Key } from "bomberhuman";

////////////////////////////////////////////////////////////////
// Key handling
////////////////////////////////////////////////////////////////

/// `Key` is imported from WASM
const KeyBind = {
  // Player 1
  "ArrowLeft":  [0, Key.Left],
  "ArrowRight": [0, Key.Right],
  "ArrowUp":    [0, Key.Up],
  "ArrowDown":  [0, Key.Down],
  " ":          [0, Key.Button1],

  // Player 2
  "a":          [1, Key.Left],
  "d":          [1, Key.Right],
  "w":          [1, Key.Up],
  "s":          [1, Key.Down],
  "q":          [1, Key.Button1],
};

function process_key(key, state) {
  if (debug) console.log(key);
  let bind = KeyBind[key];
  if (bind)
    gs.toggle_key(bind[0], bind[1], state);
}

////////////////////////////////////////////////////////////////
// Main loop
////////////////////////////////////////////////////////////////

let gs = null;
let start = null;
let prev_timestamp = null;

let game_loop = (timestamp) => {
  if (!prev_timestamp) {
    start = timestamp;
    prev_timestamp = timestamp;
    requestAnimationFrame(game_loop);
    return;
  }

  let delta = (timestamp - prev_timestamp);
  gs.update(delta);  // WASM
  gs.draw();  // WASM

  prev_timestamp = timestamp;
  requestAnimationFrame(game_loop);
};

////////////////////////////////////////////////////////////////
// Main
////////////////////////////////////////////////////////////////

function start_game() {
  gs = GameState.new(900, 780); // WASM
  document.addEventListener('keydown', e => process_key(e.key, true));
  document.addEventListener('keyup',   e => process_key(e.key, false));
  game_loop();
}

start_game();
