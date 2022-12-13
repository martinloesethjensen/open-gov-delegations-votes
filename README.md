# open-gov-delegations-votes

A tool to get an insight into how many delegated votes and capital (in KSM) you have for each OpenGov track.

## How to run

```sh
cargo run -- -a <ksm_address>  
```

> If you'd like to output into a file then you can easily do it like this.

```sh
cargo run -- -a <ksm_address> >> output.txt
```

## Generating SCALE file

You need the `subxt-cli` to run this.

```sh
subxt metadata -f bytes --url wss://kusama-rpc.dwellir.com:443 > kusama_metadata.scale
```
