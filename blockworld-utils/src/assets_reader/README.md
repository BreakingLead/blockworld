# License
MIT License

Copyright (c) 2022 Ben Reeves

# minecraft-assets

[![Crates.io](https://img.shields.io/crates/v/minecraft-assets.svg)](https://crates.io/crates/minecraft-assets)
[![Docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/minecraft-assets)
[![Tests](https://github.com/bgr360/minecraft-assets-rs/actions/workflows/tests.yml/badge.svg)](https://github.com/bgr360/minecraft-assets-rs/actions/workflows/tests.yml)

A Rust library for reading asset files and resource packs for any version of
Minecraft.

## Example

```rust,no_run
use minecraft_assets::api::AssetPack;

let assets = AssetPack::at_path("~/.minecraft/");

// Load the block states for `oak_planks`
let states = assets.load_blockstates("oak_planks").unwrap();
let variants = states.variants().unwrap();

assert_eq!(variants.len(), 1);

let model_properties = &variants[""].models()[0];
assert_eq!(model_properties.model, "block/oak_planks");
```

## Documentation

This library is `#![warn(missing_docs)]`, so the documentation is very complete:

* [Main Branch (github.io)](https://bgr360.github.io/minecraft-assets-rs/minecraft_assets/)
* [Latest Release (docs.rs)](https://docs.rs/minecraft-assets)

## Feature checklist

#### Assets parsing

- [x] `assets/<namespace>/blockstates/*.json`
- [ ] `assets/<namespace>/font/*.json`
- [ ] `assets/<namespace>/lang/*.json`
- [x] `assets/<namespace>/models/block/*.json`
- [x] `assets/<namespace>/models/item/*.json`
- [ ] `assets/<namespace>/particles/*.json`
- [ ] `assets/<namespace>/shaders/{post,program}/*.json`
- [ ] `assets/<namespace>/textures/*.mcmeta`
- [ ] `assets/<namespace>/sounds.json`
- [ ] `assets/pack.mcmeta`

#### Data parsing

- [ ] `data/<namespace>/advancements/**/*.json`
- [ ] `data/<namespace>/loot_tables/**/*.json`
- [ ] `data/<namespace>/recipes/*.json`
- [ ] `data/<namespace>/structures/**/*.json`
- [ ] `data/<namespace>/tags/**/*.json`

## Projects using `minecraft-assets`

* [Brine]: A multi-version Minecraft client written using Bevy.
* *Maybe your project here! :)*

## Tests

Integration tests in [`tests/`](tests/) use the actual asset files from the
[`minecraft-assets`] repository.

That repository is fairly large (~1 GB), so the tests in `tests/` do not run by
default. If you'd like to run them, use the [`tests/setup.sh`](tests/setup.sh)
script:

```txt
$ ./tests/setup.sh
```

This script will fetch the [`minecraft-assets`] repository and check out a few
different versions at various paths in [`tests/`](tests/). Then you can run the
tests by enabling the `tests` feature:

```txt
$ cargo test --features tests
```

[`minecraft-assets`]: https://github.com/InventivetalentDev
[Brine]: https://github.com/BGR360/brine

## License

Licensed under either of

 * Apache License, Version 2.0
   (<http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   (<http://opensource.org/licenses/MIT>)

at your option.

Copyright Ben Reeves 2022

[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
