# `limelight-primitives`

This crate implements a number of 2D shapes that can be drawn with a
limelight [`Renderer`](https://docs.rs/limelight/latest/limelight/renderer/struct.Renderer.html).

Each primitive comes with two parts: a data structure (like `Rect` or `Circle`) representing
the raw shape data that is sent to the GPU, and a layer (like `RectLayer` and `CircleLayer`)
that implements [`Drawable`](https://docs.rs/limelight/latest/limelight/renderer/trait.Drawable.html)
and can draw itself when passed a `Renderer` instance.

All layers are capable of drawing multiple instances of the shape they represent.

```rust
use limelight_primitives::{Circle, CircleLayer};
use limelight::Renderer;

fn draw_circles(renderer: &Renderer) {
    let circles = CircleLayer::new();
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

    self.circles.draw(renderer)?;
}
```

The `.buffer()` method returns a [`Buffer`](https://docs.rs/limelight/latest/limelight/buffer/struct.Buffer.html) of the relevant type, e.g. `RectLayer::buffer()` returns a `Buffer<Rect>`, which you
can use to update the rectangle data at any time.

Layers also expose a `Uniform<[[f32; 4]; 4]>` that acts as a [transformation matrix](https://en.wikipedia.org/wiki/Transformation_matrix) on the points.

For an example that uses uniforms, see the [primitive scene demo](https://drifting-in-space.github.io/limelight/primitive-scene/) ([code](https://github.com/drifting-in-space/limelight/tree/main/examples/primitive-scene)).

## Primitives

- `Circle`: filled circles.
- `Rect`: filled rectangle.
- `Line`: straight line of arbitrary (scaled) thickness.
- `Hairline`: axis-aligned line with unscaled thickness (i.e. thickness is independent of zoom level; useful for grids and axes).
