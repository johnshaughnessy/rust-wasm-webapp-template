# Wasm app template

I built this app template as part of a game jam.

I want to save it separately because it's a convenient start point for a rust -> web assembly -> web application.

There's a `cargo` project uses `yew` and `wasm-pack` to build some `wasm` and `js` libraries.

Then it has a separate `npm` project that uses `webpack` and `tailwind css` to put it all together.

There might be some leftover nonsense in here from the gamejam. I may clean it up later as I use this as the starting point for my next rust -> wasm -> web app.

I do _not_ recommend using this. I don't have any plans to document how it works and if you know what all the pieces do then it's probably not helpful to you anyway.

# Requirements

Optional dependencies in `[`brackets`]`.

- `wasm-pack`
- [`zip`]
- [`butler`]
- `cargo`/`rustc`
- [`cargo-watch`]
- `node`/`npm`
- [`alacritty`]
