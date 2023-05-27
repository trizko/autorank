# autorank

A self-balancing multiplayer terminal based autochess written in Rust.

## How to run the client binary (arclient)
To run the client, you must first add wasm as a compilation target and install the `trunk` cargo package:
```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```
After `trunk` is installed you can run the following command from the root of the `arclient` package:
```bash
# the --open flag will automatically open a web browser.
trunk serve --open
```
This command will start up a server that serves the frontend application with automatic hot-reloading.