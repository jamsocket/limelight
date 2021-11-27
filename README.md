# limelight

[![GitHub Repo stars](https://img.shields.io/github/stars/drifting-in-space/limelight?style=social)](https://github.com/drifting-in-space/limelight)
[![crates.io](https://img.shields.io/crates/v/limelight.svg)](https://crates.io/crates/limelight)
[![docs.rs](https://img.shields.io/badge/docs-release-brightgreen)](https://docs.rs/limelight/)
[![Rust](https://github.com/drifting-in-space/limelight/actions/workflows/rust.yml/badge.svg)](https://github.com/drifting-in-space/limelight/actions/workflows/rust.yml)

Limelight is a `WebGL2` wrapper with a focus on making high-performance graphics code easier to
write and maintain.

In particular, it:
- Provides a functional interface that **abstracts away the statefulness of WebGL**.
  It accomplishes this by using a *shadow GPU* that tracks the GPU's state, diffs it with the
  desired state, and sends only the necessary instructions to WebGL.
- Provides abstractions for buffers and uniforms that **defer GPU calls until the draw cycle**.
  (See [WebGL Insights](http://www.webglinsights.com/) section 14.2, *Deferring until the Draw Cycle*.)
- Provides a **typed interface to uniforms and buffers**, and automatically generates vertex array objects
  (VAOs) from Rust data types through a derive macro.

## Getting started

See the [examples](https://github.com/drifting-in-space/limelight/tree/main/examples) directory for
runnable examples.

This tutorial assumes you're familiar with basic WebGL terminology, like vertex and fragment shaders,
uniforms, and buffers.

### Drawing a triangle

([full code](https://github.com/drifting-in-space/limelight/tree/main/examples/01-triangle),
[demo](https://drifting-in-space.github.io/limelight/01-triangles/))

[![A colorful triangle](https://github.com/drifting-in-space/limelight/raw/main/assets/01-triangle.png)](https://drifting-in-space.github.io/limelight/01-triangle/)

This example demonstrates the three main steps to produce an image with limelight:
1. Create a `Program` object. A `Program` in limelight contains the vertex and fragment shader pair
   (a `WebGLProgram` object), and also contains program-specific state. The `Program` object itself acts
   as a [builder](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html) for a `GlProgram`, which
   we obtain by calling `gpu_init(&gl)` where `gl` is a [`WebGl2RenderingContext`](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext).
2. Create a `Renderer`. After we have initialized all of our programs with the GL context, we transfer ownership
   of the GL context into a `Renderer`, which then becomes responsible for all GL-side state transitions.
3. We call `renderer.render(program, buffer)`, which causes the triangle to be drawn. We have not attached a
   vertex attribute buffer in this example, and instead use the vertex shader to generate the vertices. We
   still need to tell WebGL *how many* vertices (3) we want to generate, so we pass in a `DummyBuffer` of size `3`.

```rust
use web_sys::WebGl2RenderingContext;
use limelight::{Program, Renderer, DummyBuffer, DrawMode};

fn render_triangle(gl: WebGl2RenderingContext) {
  // limelight doesn't touch the DOM at all. Use your preferred
  // framework to create a canvas and create a WebGL2 context
  // from it.

  // Create a shader program by passing in GLSL code as strings for
  // the fragment and vertex shaders.
  let program = Program::new(
      include_str!("../../examples/01-triangle/shaders/shader.frag"),
      include_str!("../../examples/01-triangle/shaders/shader.vert"),
      DrawMode::Triangles
  ).gpu_init(&gl).unwrap();

  // Create a renderer. The renderer becomes the owner of the
  // WebGl2RenderingContext, to ensure that its internal representation
  // of the GPU state is always accureate.
  let renderer = Renderer::new(gl);

  // Run the program, rendering the results to the screen. We are
  // not passing any vertex attribute data, so we use a `DummyBuffer`
  // which renders three vertices: one for each corner of a triangle.
  renderer.render(&program, &DummyBuffer::new(3)).unwrap();
}
```

### Using buffers

([full code](https://github.com/drifting-in-space/limelight/tree/main/examples/02-buffer),
[demo](https://drifting-in-space.github.io/limelight/02-buffer/))

[![Two small triangles](https://github.com/drifting-in-space/limelight/raw/main/assets/02-buffer.png)](https://drifting-in-space.github.io/limelight/02-buffer/)

Buffers enable arbitrary vertex attribute data to be passed into the shaders. Limelight provides a
procedural macro (`vertex_attribute`) for mapping from a Rust-side `struct` to a GPU-side set of
vertex attributes.

`buffer.set_data` is *lazy*: it does not result in any GPU activity until the next time the buffer is used
in a render call. If a buffer is unchanged between render calls, it is not re-written to the GPU.

```rust
use web_sys::WebGl2RenderingContext;
use limelight::{Program, Renderer, AttributeBuffer, DrawMode, BufferUsageHint, vertex_attribute};

// This attribute macro derives a number of traits, including `VertexAttribute`, which
// is required for a type to be used in an `AttributeBuffer`.
#[vertex_attribute]
struct VertexDescription {
    position: [f32; 2], // field names are mapped to variables in the shader.
}

impl VertexDescription {
    pub fn new(x: f32, y: f32) -> Self {
        VertexDescription { position: [x, y] }
    }
}

fn render_triangles(gl: WebGl2RenderingContext) {
  let program = Program::new(
      include_str!("../../examples/02-buffer/shaders/shader.frag"),
      include_str!("../../examples/02-buffer/shaders/shader.vert"),
      DrawMode::Triangles
  ).gpu_init(&gl).unwrap();

  let renderer = Renderer::new(gl);

  // Declare a buffer.
  let mut buffer: AttributeBuffer<VertexDescription> =
    AttributeBuffer::new(BufferUsageHint::DynamicDraw);

  buffer.set_data(vec![
    // Triangle #1.
    VertexDescription::new(-0.3, -0.3),
    VertexDescription::new(-0.5, -0.3),
    VertexDescription::new(-0.5, -0.5),

    // Triangle #2.
    VertexDescription::new(0.3, 0.3),
    VertexDescription::new(0.5, 0.3),
    VertexDescription::new(0.5, 0.5),
  ]);

  renderer.render(&program, &buffer).unwrap();
}
```

### Uniforms

([full code](https://github.com/drifting-in-space/limelight/tree/main/examples/03-uniform),
[demo](https://drifting-in-space.github.io/limelight/03-uniform/))

[![A scaled and rotated triangle](https://github.com/drifting-in-space/limelight/raw/main/assets/03-uniform.png)](https://drifting-in-space.github.io/limelight/03-uniform/)

Uniforms are values that can be ready in both shader and fragment programs. They can vary
between `render` calls, but for a given render call each uniform has a constant value
across all vertices and fragments.

```rust
use limelight::{DrawMode, DummyBuffer, Program, Renderer, Uniform};
use web_sys::WebGl2RenderingContext;

fn render_triangles_with_uniform(gl: WebGl2RenderingContext) {
    // This will correspond to "uniform float u_rotate" in the vertex shader.
    let rotate_uniform = Uniform::new(std::f32::consts::PI / 3.4);
    
    // This will correspond to "uniform vec2 u_rotate" in the vertex shader.
    let scale_uniform = Uniform::new([0.5, 0.8]);

    // This will correspond to "uniform vec3 u_color" in the fragment shader.
    let color_uniform = Uniform::new([0.9, 0.2, 0.3]);

    let program = Program::new(
      include_str!("../../examples/03-uniform/shaders/shader.frag"),
      include_str!("../../examples/03-uniform/shaders/shader.vert"),
        DrawMode::Triangles,
    )
    // We need to map the uniforms when we create the program.
    // The GPU-side types are automatically inferred from the Rust types.
    .with_uniform("u_rotate", rotate_uniform)
    .with_uniform("u_scale", scale_uniform)
    .with_uniform("u_color", color_uniform)
    .gpu_init(&gl)
    .unwrap();

    let renderer = Renderer::new(gl);
    renderer.render(&program, &DummyBuffer::new(3)).unwrap();
}
```

`Uniform::new` returns an `Rc<Uniform>`, allowing you to attach it to a program (or multiple programs)
while still retaining a handle through which you can set it. To do so, use `.clone()` in the call to
`with_uniform`:

```rust
use limelight::{DrawMode, DummyBuffer, Program, Renderer, Uniform};
use web_sys::WebGl2RenderingContext;

fn render_triangles_with_uniform(gl: WebGl2RenderingContext) {
    // We construct with placeholder values.
    let rotate_uniform = Uniform::new(0.);
    let scale_uniform = Uniform::new([0., 0.]);
    let color_uniform = Uniform::new([0., 0., 0.]);

    let program = Program::new(
      include_str!("../../examples/03-uniform/shaders/shader.frag"),
      include_str!("../../examples/03-uniform/shaders/shader.vert"),
        DrawMode::Triangles,
    )
    // Note the addition of `.clone()` to each uniform.
    .with_uniform("u_rotate", rotate_uniform.clone())
    .with_uniform("u_scale", scale_uniform.clone())
    .with_uniform("u_color", color_uniform.clone())
    .gpu_init(&gl)
    .unwrap();

    // Now we still have handles to the uniforms that we can use to change their values.
    rotate_uniform.set_value(std::f32::consts::PI / 3.4);
    scale_uniform.set_value([0.5, 0.8]);
    color_uniform.set_value([0.9, 0.2, 0.3]);

    let renderer = Renderer::new(gl);
    renderer.render(&program, &DummyBuffer::new(3)).unwrap();
}
```

### TODO: structuring animation
