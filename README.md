actors
======

A [rust](http://www.rust-lang.org/) library to provide 
[actor](http://en.wikipedia.org/wiki/Actor_model)-like message-based concurrency.

Stories: 
[![Stories in Ready](https://badge.waffle.io/kolloch/actors.png?label=ready&title=Ready)](https://waffle.io/kolloch/actors)
[![Stories in Progress](https://badge.waffle.io/kolloch/actors.png?label=in%20progress&title=In%20Progress)](https://waffle.io/kolloch/actors)

[![Build Status](https://travis-ci.org/kolloch/actors.svg?branch=master)](https://travis-ci.org/kolloch/actors)

[API Documentation](https://kolloch.github.io/actors/doc/actors/index.html)

Warning: This library is in a very early stage, it is not recomended for production 
and all APIs are subject to change.

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

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]

actors = "0.1"
```