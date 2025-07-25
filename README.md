# Portfolio website
This is my portfolio website made in Rust with [Yew](https://yew.rs/).

# Installation
1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Install rust for wasm: `rustup target add wasm32-unknown-unknow`.
3. Install `trunk` using `cargo`: `cargo install trunk`.
4. Install the Tailwind CLI (`tailwindcss`), either by using `npm` (`npm install tailwindcss @tailwindcss/cli`) or through downloading the [binary](https://github.com/tailwindlabs/tailwindcss/releases).
5. Make the `client/src/tailwind.css` file by doing `tailwindcss -o client/src/tailwind.css`.

# Running
1. To build the css, use the Tailwind CLI by doing `tailwindcss -o client/src/tailwindcss` (you can also add the `-w` flag to make it watch for changes and auto-compile the CSS file).
2. To run the backend, run `cargo run --package portfolio-server --release` (this will start the backend server on `http://localhost:8000`).
3. In a new terminal, navigate to the `client` directory (`cd client`).
4. Run `trunk serve --release` to run the frontend. Add the `--open` to open the URL in your default browser.
> If trunk is not found, make sure that the cargo bin directory is added to your PATH.
