# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/haqq-network/haqq-clients/releases/tag/haqq-build-v0.1.0) - 2023-11-23

### Added
- haqq, cosmos, evmos: proto, grpc and rest

### Other
- don't fail if tmp dir doesn't exists
- adopt to new haqq-node protos
- remove REST endpoints, openapi-generator-cli can't handle IBC swagger
- remove cosmos nft module as it's not used by haqq network
