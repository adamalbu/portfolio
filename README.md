# Portfolio website
This is my portfolio website made in Rust with [Yew](https://yew.rs/).

# Installation
1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Install `trunk` using `cargo`: `cargo install trunk`.
3. Install the Tailwind CLI (`tailwindcss`), either by using `npm` (`npm install tailwindcss @tailwindcss/cli`) or through downloading the [binary](https://github.com/tailwindlabs/tailwindcss/releases).
4. Make the `src/tailwind.css` file by doing `tailwindcss -o src/tailwind.css`.

# Running
1. To build the css, use the Tailwind CLI by doing `tailwindcss -o src/tailwindcss` (you can also add the `-w` flag to make it watch for changes and auto-compile the CSS file).
2. To run the backend, run `cargo run --package portfolio_backend --release` (this will start the backend server on `http://localhost:8000`).
2. To run the frontend, run `trunk serve --release` (you can also add the `--open` to open the URL in your default browser).
