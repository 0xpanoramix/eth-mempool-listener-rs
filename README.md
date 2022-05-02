# eth-mempool-listener-rs
A simple program able to listen to the pending transactions of the Ethereum mempool, but in Rust.

## How does it work ?

It creates a set of clients to query the mempool from a node using the endpoint you've provided as an environment
variable.
Then, it converts the transaction to a readable message so that you can use its fields properly.

Take a look at the code and feel free to use it for your own needs !

## Getting started !

### Installation

Make sure everything is ready to go using :
```shell
cargo check
```

### Quickstart

Provide the node endpoint in a .envrc file or export it in the shell environment :
```shell
export WSS_NODE_ENDPOINT=RUN_YOUR_OWN_NODE_TO_IMPROVE_DECENTRALISATION_;)
```

Then, run the following command to compile the binary :
```shell
cargo build --release
```

You can start the listener by running :
```shell
./target/release/eth-mempool-listener-rs
```

## Author

Made with ❤️ by 🤖 [Luca Georges François](https://github.com/PtitLuca) 🤖
