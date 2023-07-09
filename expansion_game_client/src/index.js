import { Game, start_ui } from "expansion_game_wasm";
// import { memory } from "expansion_game_wasm/expansion_game_wasm_bg";
import "./style.css";

function main() {
  const canvas = document.getElementById("game-canvas");
  canvas.height = 720;
  canvas.width = 1280;

  const ctx = canvas.getContext("2d");
  const game = Game.new();

  const tick = () => {
    let counter = game.tick();
    requestAnimationFrame(tick);
  };

  start_ui();
  requestAnimationFrame(tick);
}

main();
