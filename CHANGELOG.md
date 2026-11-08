# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.1.0]

### Added
- `RosettaClient` for the Data API (`/network/list`, `/network/status`,
  `/network/options`, `/block`, `/account/balance`, `/mempool`,
  `/mempool/transaction`, `/search/transactions`) and the Construction API
  (`derive`, `preprocess`, `metadata`, `payloads`, `combine`, `parse`,
  `hash`, `submit`), with `post` for any other endpoint.
- The Rosetta types as Mina's server uses them, with Mina's metadata typed:
  `BalanceMetadata` (locked, liquid, total), `AccountBalanceMetadata`
  (nonce), `TokenMetadata`, `BlockMetadata` (creator) and `OperationType`.
- `Transfer` and `Delegation` make the operations of a payment and a
  delegation.
- Retries on unreachable servers, 5xx answers and `retriable` Rosetta errors.
- Features `native-tls` (default) and `rustls-tls`.
