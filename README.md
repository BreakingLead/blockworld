# Blockworld

(Currently Indev & Not Playable)

![](./logo.svg)

The path currently planned:
Rewrite GT: New Horizons?

TL;DR:
Blockworld is a modern **minecraft (java version) implementation** which has better performance, and minecraft modders can easily port their mods into this minecraft implementation.

## What does Blockworld do
- Provide better minecraft experience for players.
- Modular design, which means you can use Blockworld as a voxel game engine. Official updates will be provide like a mod (1.20 mod 1.16 mod etc.)
- Provide easy api to mod the game.
- Looks like minecraft. So it's not like what minetest did.
- No code obfuscation: everyone can modify the source code.
- Port your minecraft world into Blockworld easily.
- Fix some major bugs in minecraft. However some bugs which is useful(like quasi-connection) won't be deleted.
- **Multi-threading.**
- Cross-platform.
- **Vulkan, Metal support.**
- Be able to write mods in your favorite programming language by **WASM as a IR**.

## What doesn't Blockworld do
- Use the same data format(like named binary tag) as minecraft. However we will provide a converter
- Reimplement every detail of minecraft.
- Follow the offical update of minecraft. We think the content of 1.12.2 is enough. But the content of higher version of minecraft may be maintained by mods.


## Minecraft is Dying

Minecraft, as a popular game, has been loved for many years.
Its huge community created plenty of various mods and gameplay.
Ironically, mojang seems not to know that not everybody likes their tedious updates -- axolotls, suspicious sands, wolves. Do we really need them?
Isn't mods like _Create_ better than offical updates? 
Minecraft **should** be like lego bricks, like a playground, like a game framework, rather than a boring RPG game.
If it has to be a RPG game, it should be done by **mods** like _twilight forest_.

That's why I decided to make a new minecraft, which is suckless.

Our goal is giving the ability of making new gameplays.

---

## Chinese Translation

我认为Minecraft已经死了。

Minecraft 作为一款备受欢迎的游戏，确实在多年来一直备受热爱。其庞大的社区为游戏的多样性贡献良多，创造了众多各种各样的模组和玩法。然而，令人讽刺的是，Mojang 似乎未能理解并满足所有玩家的期望，尤其是那些无聊的更新，像美西螈、可疑的沙子。这些更新引起了一些玩家的不满，但是，我们真的需要这些更新吗？这些东西Mod难道做的不是更好吗？

相比之下，许多模组，比如 Create，提供了更有趣、更富创意的玩法，使得玩家能够更好地个性化和定制游戏体验。这些模组不仅丰富了游戏内容，还为玩家提供了更大的创造空间，使得 Minecraft 更像一个开放的游乐场，而不仅仅是一个受限制的无聊的RPG游戏。我们玩MC难道不就是为了自由度？

与其将 Minecraft 局限于一个乏味的 RPG 类游戏，不如将其看作是一种乐高积木，一个灵活多变的游戏框架。这样的理念更符合现代玩家的需求，他们更倾向于创意无限的游戏体验。这也是沙盒游戏的初衷。考虑到现代游戏市场的发展趋势，更加开放和自由的游戏体验更能迎合广大玩家的口味。

此外，有许多优秀的模组，比如暮色森林，已经证明了模组在为游戏注入新鲜血液方面的能力。这些模组不仅提供了全新的地图、任务和敌人，还拓展了游戏的整体玩法。因此，如果我们真的需要一个RPG风格的《我的世界》，为什么不依赖于这些成功的模组，如《暮色森林》呢？

而且，有的模组整合包已经脱离了Minecraft的范畴，例如《格雷科技：新视野》。这个整合包，说是独立游戏，都是算小的。

因此，我决定着手制作一个新的MC，旨在保持简约，对现有的Minecraft做一定程度上的兼容（详情见下）（这也是Minetest失败的原因，没有利用好MC社区）。并提供一个简单的API来制作模组（使用WASM作为IR，所以你可以用任何你喜欢的编程语言），使他们能够创造出属于自己的独特游戏体验。我们的目标是赋予玩家制作新玩法的能力，以满足不同玩家对游戏的多样化需求。

Blockworld的目标：（兼容性，兼容性，兼容性！提供Java转译层！）

- 为玩家提供更好的 MC 游玩体验。
- 采用模块化设计，这意味着，你可以将官方游戏版本的更新看成一个 Mod。比如1.16.5 Mod，1.20 Mod之类。
- 提供易用的 API 以制作 Mod。
- 能一定程度上兼容 Java 版的 Mod。
- 风格保持和 MC 一致，不会像 Minetest 那样奇怪。
- 无代码混淆：每个人都可以修改源代码。
- 轻松将MC存档移植到Blockworld。
- 修复《我的世界》中的一些陈年主要 Bug。然而，一些实用的Bug（如QC激活，BUD）将不会被删除。
- 多线程支持。
- 跨平台兼容。
- Vulkan, Metal 支持。
- 能够使用您喜欢的编程语言编写模组。

Blockworld不会去做的：

- 使用与《我的世界》相同的数据格式（如NBT）。不过，我们将提供一个转换器。
- 重新实现《我的世界》的每一个细节。
- 遵循《我的世界》的官方更新。我们认为1.12.2版本的内容已足够好玩。但是，较高版本的内容仍然会以 Mod 的形式维护。
