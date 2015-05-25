actors
======

[![Build Status](https://travis-ci.org/kolloch/actors.svg?branch=master)](https://travis-ci.org/kolloch/actors)

[API Documentation](https://kolloch.github.io/actors/doc/actors/index.html)

A rust library to provide actor-like message-based concurrency.

Goals:

* Message-based state manipulation.
* Deal with failure by allowing actor supervision.
* Light-weighed: Each actor should consume only few resources.
* Multi-plex actor execution efficiently upon thread-pools.
* Composable: Do not try to solve everything at once.
* Rusty: Use features available in this beautiful language.

Non-Goals:

* Transparent network communication/distribution as part of this
  library.

## Usage (NOT THERE YET ;) )

Add this to your `Cargo.toml`:

```toml
[dependencies]

actors = "0.1"
```