# bevy_ui_anchor

[![crates.io](https://img.shields.io/crates/v/bevy_ui_anchor)](https://crates.io/crates/bevy_ui_anchor)
[![docs.rs](https://docs.rs/bevy_ui_anchor/badge.svg)](https://docs.rs/bevy_ui_anchor)
[![License](https://img.shields.io/crates/l/bevy_ui_anchor)](https://opensource.org/licenses/MIT)

A Rust crate for anchoring UI elements to specific points or entities in the world using the Bevy game engine.

![](follow.gif)

## Features

Provides an AnchorUiNode component that:

- Anchor UI nodes to world positions or entities.
- Supports horizontal and vertical anchoring.
- Compatible with Bevy's ECS architecture.

| Bevy version | Crate version |
| ------------ | ------------------------ |
| 0.15         | 0.3 - 0.5                |
| 0.14         | 0.1 - 0.2                |

## Example

``` rust
