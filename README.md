# Limelight

[![GitHub Repo stars](https://img.shields.io/github/stars/drifting-in-space/limelight?style=social)](https://github.com/drifting-in-space/limelight)
[![crates.io](https://img.shields.io/crates/v/limelight.svg)](https://crates.io/crates/limelight)
[![docs.rs](https://img.shields.io/badge/docs-release-brightgreen)](https://docs.rs/limelight/)
[![Rust](https://github.com/drifting-in-space/limelight/actions/workflows/rust.yml/badge.svg)](https://github.com/drifting-in-space/limelight/actions/workflows/rust.yml)

Limelight is a `WebGL2` wrapper with a focus on making high-performance WebAssembly graphics code easier to write and maintain.

https://user-images.githubusercontent.com/46173/146214698-784404f2-633c-4180-acda-1e8d64189e76.mov

Specifically, `limelight`:
- Provides a functional interface that **abstracts away the statefulness of WebGL**.
  It accomplishes this by using a *shadow GPU* that tracks the GPU's state, diffs it with the
  desired state, and sends only the necessary instructions to WebGL.
- Provides abstractions for buffers and uniforms that **defer GPU data transfer until the next draw cycle**.
- Provides a **typed interface to uniforms and buffers**, and automatically generates bindings
  between shader attributes and Rust `struct`s through a derive macro.
- Provides an **interface for transforms** like zoom and pan through [`limelight-transform`](https://github.com/drifting-in-space/limelight/tree/main/transform).
- Provides 2D **shape primitives** like circles and lines through [`limelight-primitives`](https://github.com/drifting-in-space/limelight/tree/main/primitives).

# Getting started

<a href="https://drifting-in-space.github.io/limelight/05-primitives/"><img style="width: 300px;" src="https://github.com/drifting-in-space/limelight/raw/main/assets/05-primitives.png" alt="Abstract art made from circles and rectangles." /></a>

([full code](https://github.com/drifting-in-space/limelight/tree/main/examples/05-primitives),
[demo](https://drifting-in-space.github.io/limelight/05-primitives/))

This example uses [`limelight-primitives`](https://github.com/drifting-in-space/limelight/tree/main/primitives)
and [`limelight-yew`](https://github.com/drifting-in-space/limelight/tree/main/yew) to construct a basic, static image
made from circles and rectangles.

```rust
use anyhow::Result;
use limelight::{renderer::Drawable, Renderer};
use limelight_primitives::{Circle, CircleLayer, Rect, RectLayer};
use limelight_yew::{LimelightComponent, LimelightController};

struct Primitives {
    rects: RectLayer,
    circles: CircleLayer,
}

impl LimelightController for Primitives {
    fn draw(
        &mut self,
        renderer: &mut Renderer,
        _ts: f64,
    ) -> Result<limelight_yew::ShouldRequestAnimationFrame> {
        self.rects.draw(renderer)?;
        self.circles.draw(renderer)?;

        Ok(false)
    }
}

impl Default for Primitives {
    fn default() -> Self {
        let rects = RectLayer::new();
        let circles = CircleLayer::new();

        rects.buffer().set_data(vec![
            Rect {
                lower_right: [0.4, 0.1],
                upper_left: [-0.8, 0.2],
                color: palette::named::TOMATO.into(),
            },
            Rect {
                lower_right: [0.4, 0.25],
                upper_left: [-0.6, 0.5],
                color: palette::named::SLATEBLUE.into(),
            },
        ]);

        circles.buffer().set_data(vec![
            Circle {
                position: [0., 0.25],
                radius: 0.2,
                color: palette::named::WHITE.into(),
            },
            Circle {
                position: [0., 0.25],
                radius: 0.1,
                color: palette::named::ORANGERED.into(),
            },
        ]);

        Primitives { rects, circles }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<LimelightComponent<Primitives>>();
}
```

# More Examples

- [Instancing](https://drifting-in-space.github.io/limelight/instances/) ([code](https://github.com/drifting-in-space/limelight/tree/main/examples/instances))
- [Pong](https://drifting-in-space.github.io/limelight/pong/)  ([code](https://github.com/drifting-in-space/limelight/tree/main/examples/pong))
- [Zooming / panning](https://drifting-in-space.github.io/limelight/zoom-pan/) ([code](https://github.com/drifting-in-space/limelight/tree/main/examples/zoom-pan))
- [Full-canvas shader](https://drifting-in-space.github.io/limelight/shaderfun/) ([code](https://github.com/drifting-in-space/limelight/tree/main/examples/shaderfun))
- [Primitive scene](https://drifting-in-space.github.io/limelight/primitive-scene/) ([code](https://github.com/drifting-in-space/limelight/tree/main/examples/primitive-scene))
