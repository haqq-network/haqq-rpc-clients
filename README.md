# Haqq RPC Clients

This repo contains RPC clients &mdash; generated with [buf.build](https://buf.build), patched rough edges and improved with some useful helpers.

## Rust Client

![docs.rs](https://img.shields.io/docsrs/haqq-grpc?link=https%3A%2F%2Fdocs.rs%2Fhaqq-grpc)
![Crates.io](https://img.shields.io/crates/l/haqq-grpc)

Rust client is fully functional, supports [gRPC](https://github.com/haqq-network/haqq-clients/blob/master/tests/grpc.rs).

**TODO**:

- [ ] REST endpoints (openapi-generator-cli is very buggy for rust)

## TypeScript gRPC-Web Client

Currently only structs are generated in [web](web) folder.

**TODO**:

- [ ] package.json and other packaging-related files
- [ ] tests with usage examples
