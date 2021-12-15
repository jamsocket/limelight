# `limelight-yew`

Provides scaffolding for building [Yew](https://yew.rs/) components that use
[limelight](https://github.com/drifting-in-space/limelight) to render to a
canvas. Apps can implement `LimelightController`, which can then be wrapped
in `LimelightComponent` and used as a Yew component directly.

For example, the [primitive scene](https://drifting-in-space.github.io/limelight/primitive-scene/) demo ([code](https://github.com/drifting-in-space/limelight/tree/main/examples/primitive-scene))
is implemented as a class `Primitives`, which implements [`std::default::Default`](https://doc.rust-lang.org/std/default/trait.Default.html) and `LimelightController`.

It is then initialized through Yew like so:

```rust
yew::start_app::<LimelightComponent<Primitives>>();
```

Implementors of `LimelightController` are only required to implement one function,
 `fn draw(&mut self, renderer: &mut Renderer, _ts: f64) -> Result<limelight_yew::ShouldRequestAnimationFrame>`. This function is called every
 animation frame to tell the controller to draw its content using the provided [`Renderer`](https://docs.rs/limelight/latest/limelight/renderer/struct.Renderer.html).

Implementors may optionally implement other functions like `handle_zoom` and `handle_drag` to
respond to mouse interactions with the component.

All methods return either a `bool` (aliased to `ShouldRequestAnimationFrame`) or a `Result<bool>`.
These methods should return `true` if they would like to
trigger a redraw. For `handle_` methods, this usually
means that they modified buffers or uniforms that should
be reflected in the image. For `draw`, returning `Ok(true)`
is the intended way to create an animation loop if the
`draw` call itself updates uniforms or buffers.
