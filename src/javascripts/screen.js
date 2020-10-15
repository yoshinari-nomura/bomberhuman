const debug = false;

////////////////////////////////////////////////////////////////
/// Sprite
////////////////////////////////////////////////////////////////

const dimension = 60;

class Sprite {
  constructor(image_path) {
    this.image = new Image();
    this.image.src = image_path;
  }

  draw_on(ctx, x, y, action = 0) {
    ctx.drawImage(this.image,
                  dimension * action, 0,
                  dimension, dimension,
                  x, y,
                  dimension, dimension);
  }
}

////////////////////////////////////////////////////////////////
/// export functions to WASM
////////////////////////////////////////////////////////////////

let ctx = document.getElementById('canvas').getContext('2d');
let sprites = new Sprite('assets/sprites.png');

export function screen_clear_rect(x, y, width, height) {
  ctx.clearRect(x, y, width, height);
}

/// put sprite on x,y
export function screen_put_sprite(x, y, class_id, action = 0) {
  ctx.drawImage(sprites.image,
                dimension * action, dimension * class_id,
                dimension, dimension,
                x, y,
                dimension, dimension);
}
