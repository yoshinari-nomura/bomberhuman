const debug = true;

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

  // Player 3
  "h":          [2, Key.Left],
  "l":          [2, Key.Right],
  "k":          [2, Key.Up],
  "j":          [2, Key.Down],
  "u":          [2, Key.Button1],

  // Player 4
  "1":          [3, Key.Left],
  "2":          [3, Key.Right],
  "3":          [3, Key.Up],
  "4":          [3, Key.Down],
  "5":          [3, Key.Button1],
};

function process_key(key, state) {
  if (debug) console.log(key);
  let bind = KeyBind[key];
  if (bind)
    gs.toggle_key(bind[0], bind[1], state);
}

////////////////////////////////////////////////////////////////
// Gamepads handling
////////////////////////////////////////////////////////////////

// Needs "gamepadconnected" handler even if empty.
function init_gamepads(gp) {
  if (debug) {
    console.log("Gamepad connected at index:%d buttons:%d axes:%d [%s]",
                gp.index, gp.buttons.length, gp.axes.length, gp.id);
  }
}

function scan_gamepads() {
  // Chrome should refresh gamepads everytime you read.
  var gamepads = navigator.getGamepads ? navigator.getGamepads() : [];

  for (var i = 0; i < gamepads.length; i++) {
    var pad = gamepads[i];

    if (pad) {
      // Send state to WASM
      gs.toggle_key(i, Key.Left,    pad.axes[0] < -0.5);
      gs.toggle_key(i, Key.Right,   pad.axes[0] >  0.5);
      gs.toggle_key(i, Key.Up,      pad.axes[1] < -0.5);
      gs.toggle_key(i, Key.Down,    pad.axes[1] >  0.5);
      gs.toggle_key(i, Key.Button1, pad.buttons[0].pressed);
    }
  }
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

  scan_gamepads();
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
  document.addEventListener("gamepadconnected", e => init_gamepads(e.gamepad));
  game_loop();
}

start_game();
