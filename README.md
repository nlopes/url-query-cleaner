
[![docs.rs](https://docs.rs/url-query-cleaner/badge.svg)](https://docs.rs/url-query-cleaner)
[![crates.io](https://img.shields.io/crates/v/url-query-cleaner.svg)](https://crates.io/crates/url-query-cleaner)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/nlopes/url-query-cleaner/blob/master/LICENSE)
[![Build Status](https://travis-ci.org/nlopes/url-query-cleaner.svg?branch=master)](https://travis-ci.org/nlopes/url-query-cleaner)

# url-query-cleaner

`url-query-cleaner` provides facilities to clean up Url query parameters

## url-query-cleaner

Rust library to provide facilities to clean up url query parameters.

- [Usage and Examples](#usage-and-examples)
- [API Reference][API reference]

## Usage and Examples

### Basic

Use the simpler function `untrack` to clean up your url query parameters.

```rust
use url_query_cleaner::AllowedTracking;

fn main() {
    // By default, no tracking is allowed, meaning the `utm_content` query param below
    // will be removed.
    let opts: AllowedTracking = Default::default();
    let url = "https://www.example.com/?utm_content=buffercf3b2&name=ferret";

    assert_eq!(
        url_query_cleaner::untrack(url, opts).unwrap(),
        "https://www.example.com/?name=ferret",
    );
}
```

### Advanced

The advanced use case (not thaaat advanced) is for people that can't do what they need
through the provided function `untrack`.

```rust

use url_query_cleaner::clean;

fn main() {
    let url = "https://www.example.com/?&name=ferret&troop=12&item=vase";
    let mut filters = Vec::new();
    filters.push("name");
    filters.push("troop");

    assert_eq!(clean(url, filters).unwrap(), "https://www.example.com/?item=vase");
}
```

[API reference]: https://docs.rs/url-query-cleaner

