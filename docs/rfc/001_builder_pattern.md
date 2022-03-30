# RFC Title

## Authors

MACS-J1149
mowkoshka

## Status

FINAL

## Summary

We should remove the serve macro and replace it with a builder design pattern similar to many other service builders and launchers.

## Motivation

In effort keep the code used by the end users simpler and cleaner.

## Design

```rust
let server = server::builder()
 .call_api(call_api::new())
 .construction_api(construction_api::new())
 .data_api(data_api::new())
 .indexer_api(indexer_api::new())
 .custom_config(custom_config::new())
 .cache(cache_struct::new()).
```

There is no option for the configuration file path, as that is better passed at run time so you can easily swap between multiple config files.
i.e.
`cargo run --bin rosetta-snarkos -- -c ./conf.toml`

## Drawbacks

N/A

## Effect on Ecosystem

N/A

## Alternatives

There are probably some other patterns we could do instead of the builder pattern.
