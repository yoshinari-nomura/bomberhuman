const debug = false;

import { GameState, Key } from "bomberhuman";

////////////////////////////////////////////////////////////////
// Key handling
////////////////////////////////////////////////////////////////

/// `Key` is imported from WASM
const KeyBind = {
  // Player 1
  "ArrowLeft":  Key.Left,
  "ArrowRight": Key.Right,
  "ArrowUp":    Key.Up,
  "ArrowDown":  Key.Down,

  // Player 2
  "a":          Key.Left,
  "s":          Key.Right,
  "w":          Key.Up,
  "x":          Key.Down
};

function process_key(key, state) {
  if (debug) console.log(key);
  let bind = KeyBind[key];
  if (bind)
    gs.toggle_key(bind, state);
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
