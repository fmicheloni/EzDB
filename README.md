# EzDB

A tiny in-memory key-value database over TCP.

## Run

```sh
cargo run
```

In another terminal:

```sh
telnet 127.0.0.1 6379
```

## Commands

```
INSERT <key> <value>
GET <key>
DELETE <key>
```
