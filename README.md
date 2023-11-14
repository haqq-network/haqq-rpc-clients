# Haqq RPC Clients

This repo contains RPC clients &mdash; generated with [buf.build](https://buf.build), patched rough edges and improved with some useful helpers.

## Rust Client

Rust client is fully functional, supports [gRPC](https://github.com/haqq-network/haqq-rpc-clients/blob/master/tests/grpc.rs) and [Rest](https://github.com/haqq-network/haqq-rpc-clients/blob/master/tests/rest.rs) enpoints.

Not published in crates yet but will be any time soon. In order to use, add to your project's `Cargo.toml`:

* `haqq-proto = {git = "https://github.com/haqq-network/haqq-rpc-clients"}` &mdash; for gRPC endpoints
* `haqq-rest = {git = "https://github.com/haqq-network/haqq-rpc-clients"}` &mdash; for REST endpoints

## TypeScript Client

Currently only structs are generated in [web](web) folder.

TODO:

[ ] package.json and other packaging-related files
[ ] tests with usage examples
