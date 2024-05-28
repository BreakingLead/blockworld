//! Information about the different versions of Minecraft assets.
//!
//! **This documentation is a work in progress.**
//!
//! # The Flattening (1.13)
//!
//! **TODO**
//!
//! See the wiki page on [The Flattening].
//!
//! # Assets / Resource Packs Changelog
//!
//! This information is taken directly from the Minecraft [wiki page].
//!
//! #### Note on Pack Versions / Formats
//!
//! Although resource packs were introduced in 1.6.1[^1], the resource pack
//! format number was not enforced (introduced?) until 1.8.8-pre[^2].
//! Additionally, the organization of assets (and even their format!) went
//! through multiple changes during pack format `1`. Notably, block/item models
//! were not supported until 1.8[^3][^4].
//!
//!
//! ## Pack Format 1
//!
//! #### 1.6.1
//!
//! * Added resource packs, replacing the functionality of texture packs.
//!
//! #### 1.7.2
//!
//! * Added the ability to apply multiple resource packs at once.
//! * Moved files from `assets/minecraft/music` to
//!   `assets/minecraft/sounds/music` and files from `assets/minecraft/sound` to
//!   `assets/minecraft/sounds`.
//!
//! #### 1.7.3
//!
//! * The `description` value of `pack.mcmeta` can now be raw JSON text format.
//!
//! #### 1.7.4
//!
//! * Removed the ability to change the Mojang logo.
//!
//! #### 1.8
//!
//! * Added the ability to change the block and item models.
//! * Textures can now be specified for blocks and items.
//! * Added the `interpolate` tag for animations.
//!
//! #### 1.8.8
//!
//! * Resource packs now display an error if the format number is wrong. At this
//!   time, it requires a format number of 1.
//!
//!
//! ## Pack Format 2
//!
//! #### 1.9
//!
//! * Changed format number to `2`, due to changes in the model system, such as
//!   item tags, multipart, and changes to display tags.
//!   * **TODO:** Get more detailed information about these changes.
//!
//! ## Pack Format 3
//!
//! #### 1.11
//!
//! * Changed format number to `3`, due to the change that all files should have
//!   lowercase letters.
//!
//! ## Pack Format 4
//!
//! #### 1.13
//!
//! * Changed format number to `4`, due to [The Flattening].
//!
//! #### 1.14
//!
//! * Particles textures are now split into individual files.
//! * Painting textures are now split into individual files.
//! * Status effect textures are now split into individual files.
//! * Particles are now configurable.
//!
//! ## Pack Format 5
//!
//! #### 1.15
//!
//! * Changed format number to `5`, due to texture mechanic changes in earlier
//!   1.14 snapshots.
//!
//! ## Pack Format 6
//!
//! #### 1.16.2
//!
//! * Changed format number to `6`, due to changes to wall blocks made in 1.16
//!   according to [MC-197275].
//!
//! ## Pack Format 7
//!
//! #### 1.17
//!
//! * Changed format number to `7`.
//! * The file `credits.txt` in `assets/minecraft/texts` was changed to
//!   `credits.json`, and the format also changed from plain text to structured
//!   JSON format.
//!
//! ## Pack Format 8
//!
//! #### 1.18
//!
//! * Changed format number to `8`, because `inventory.png` now contains an
//!   extra sprite for a thin-layout version of the effect list in the
//!   inventory.
//!
//!
//! [wiki page]: <https://minecraft.fandom.com/wiki/Resource_Pack#History>
//!
//! [The Flattening]: https://minecraft.fandom.com/wiki/The_Flattening
//!
//! [MC-197275]: <https://bugs.mojang.com/browse/MC-197275>
//!
//! [^1]: <https://minecraft.fandom.com/wiki/Java_Edition_13w24a#General>
//!
//! [^2]: <https://minecraft.fandom.com/wiki/Java_Edition_1.8.8#General_2>
//!
//! [^3]: <https://minecraft.fandom.com/wiki/Java_Edition_14w06a#General_2>
//!
//! [^4]: <https://minecraft.fandom.com/wiki/Java_Edition_14w25a#Command_format>
