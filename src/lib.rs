/*! `url-query-cleaner` provides facilities to clean up Url query parameters

# url-query-cleaner

Rust library to provide facilities to clean up url query parameters.

- [Usage and Examples](#usage-and-examples)
- [API Reference][API reference]

# Usage and Examples

## Basic

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

## Advanced

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

 */

#![deny(missing_docs)]

use url::{ParseError, Url};

/// `clean` removes all query parameters that match any of the `filters` and
/// returns a new simplified url.
///
/// **Note**: It should not be used directly.
pub fn clean<'a>(url: &str, filters: Vec<&'a str>) -> Result<String, ParseError> {
    let mut uri = Url::parse(url)?;
    let query = uri
        .query_pairs()
        .filter(|(name, _)| !filters.iter().any(|filter| name.starts_with(filter)))
        .map(|(name, value)| format!("{}={}", name, value))
        .collect::<Vec<String>>()
        .join("&");
    if query.is_empty() {
        uri.set_query(None);
    } else {
        uri.set_query(Some(&query));
    }
    Ok(uri.to_string())
}

/// `AllowedTracking` allows you to toggle which tracking to be allowed so that `untrack`
/// doesn't touch it
#[derive(Default, Copy, Clone)]
pub struct AllowedTracking {
    /// Marketing tracking - see `AllowedMarketingTracking`
    pub marketing: AllowedMarketingTracking,
}

/// `AllowedMarketingTracking` allows you to toggle which marketing tracking to be
/// allowed, so that `untrack` doesn't touch it.
#[derive(Default, Copy, Clone)]
pub struct AllowedMarketingTracking {
    /// Urchin Tracking Module
    pub utm: bool,
    /// Google Click Identifier
    pub gclid: bool,
    /// Google Ads
    pub gclsrc: bool,
    /// DoubleClick click identifier, now Google
    pub dclid: bool,
    /// Facebook click identifier
    pub fbclid: bool,
    /// Microsoft Bing Ads click identifier
    pub mscklid: bool,
    /// zanox click identifier, now Awin
    pub zanpid: bool,
}

/// `untrack` removes all tracking query parameters from a `url`, while keeping any set in
/// `opts`
pub fn untrack(url: &'static str, opts: AllowedTracking) -> Result<String, ParseError> {
    let mut filters = Vec::new();
    if !opts.marketing.utm {
        filters.push("utm_");
    }
    if !opts.marketing.gclid {
        filters.push("gclid");
    }
    if !opts.marketing.gclsrc {
        filters.push("gclsrc");
    }
    if !opts.marketing.dclid {
        filters.push("dclid");
    }
    if !opts.marketing.fbclid {
        filters.push("fbclid");
    }
    if !opts.marketing.mscklid {
        filters.push("mscklid");
    }
    if !opts.marketing.zanpid {
        filters.push("zanpid");
    }
    clean(url, filters)
}

#[cfg(test)]
mod tests {
    use super::*;

    static NONE_ALLOWED: AllowedTracking = AllowedTracking {
        marketing: AllowedMarketingTracking {
            utm: false,
            gclid: false,
            gclsrc: false,
            fbclid: false,
            mscklid: false,
            zanpid: false,
            dclid: false,
        },
    };

    static GOOGLE_ALLOWED: AllowedTracking = AllowedTracking {
        marketing: AllowedMarketingTracking {
            utm: false,
            gclid: true,
            gclsrc: true,
            fbclid: false,
            mscklid: false,
            zanpid: false,
            dclid: false,
        },
    };

    #[test]
    fn valid_url_remove_utm() {
        assert_eq!(
            untrack(
                "https://www.example.com/?utm_content=buffercf3b2",
                NONE_ALLOWED
            )
            .unwrap(),
            "https://www.example.com/"
        );
        assert_eq!(untrack("https://www.example.com/?utm_content=buffercf3b2&utm_medium=social&utm_source=facebook.com&utm_campaign=buffer", NONE_ALLOWED).unwrap(), "https://www.example.com/");

        assert_eq!(
            untrack(
                "https://www.example.com?utm_content=buffercf3b2",
                NONE_ALLOWED
            )
            .unwrap(),
            "https://www.example.com/"
        );
    }

    #[test]
    fn valid_url_remove_utm_mixed() {
        assert_eq!(
            untrack(
                "https://www.example.com/?utm_content=buffercf3b2&name=ferret",
                NONE_ALLOWED
            )
            .unwrap(),
            "https://www.example.com/?name=ferret"
        );
        assert_eq!(untrack("https://www.example.com/?utm_content=buffercf3b2&name=ferret&utm_medium=social&color=purple&utm_source=facebook.com&utm_campaign=buffer", NONE_ALLOWED).unwrap(), "https://www.example.com/?name=ferret&color=purple");

        assert_eq!(
            untrack(
                "https://www.example.com?utm_content=buffercf3b2&name=ferret",
                NONE_ALLOWED
            )
            .unwrap(),
            "https://www.example.com/?name=ferret"
        );
    }

    #[test]
    fn valid_url_remove_utm_mixed_with_anchor() {
        assert_eq!(
            untrack(
                "https://www.example.com/?utm_content=buffercf3b2&name=ferret&gclid=someid#dope",
                NONE_ALLOWED
            )
            .unwrap(),
            "https://www.example.com/?name=ferret#dope"
        );
    }

    #[test]
    fn valid_url_remove_all_mixed_except_google() {
        assert_eq!(
            untrack(
                "https://www.example.com/?utm_content=buffercf3b2&name=ferret&gclid=someid",
                GOOGLE_ALLOWED
            )
            .unwrap(),
            "https://www.example.com/?name=ferret&gclid=someid"
        );
        assert_eq!(untrack("https://www.example.com/?utm_content=buffercf3b2&name=ferret&utm_medium=social&gclsrc=somesrc&color=purple&utm_source=facebook.com&utm_campaign=buffer", GOOGLE_ALLOWED).unwrap(), "https://www.example.com/?name=ferret&gclsrc=somesrc&color=purple");

        assert_eq!(
            untrack(
                "https://www.example.com?utm_content=buffercf3b2&name=ferret&mscklid=somemsid",
                GOOGLE_ALLOWED
            )
            .unwrap(),
            "https://www.example.com/?name=ferret"
        );
    }

    #[test]
    #[should_panic]
    fn invalid_url() {
        assert_eq!(untrack("http://[:::1]/", NONE_ALLOWED).unwrap(), "asdf");
    }
}
