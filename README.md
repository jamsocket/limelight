# Limelight

[![GitHub Repo stars](https://img.shields.io/github/stars/drifting-in-space/limelight?style=social)](https://github.com/drifting-in-space/limelight)
[![crates.io](https://img.shields.io/crates/v/limelight.svg)](https://crates.io/crates/limelight)
[![docs.rs](https://img.shields.io/badge/docs-release-brightgreen)](https://docs.rs/limelight/)
[![Rust](https://github.com/drifting-in-space/limelight/actions/workflows/rust.yml/badge.svg)](https://github.com/drifting-in-space/limelight/actions/workflows/rust.yml)

Limelight is a `WebGL2` wrapper with a focus on making high-performance WebAssembly graphics code easier to
write and maintain.

Specifically, it:
- Provides a functional interface that **abstracts away the statefulness of WebGL**.
  It accomplishes this by using a *shadow GPU* that tracks the GPU's state, diffs it with the
  desired state, and sends only the necessary instructions to WebGL.
- Provides abstractions for buffers and uniforms that **defer GPU data transfer until the next draw cycle**.
- Provides a **typed interface to uniforms and buffers**, and automatically generates bindings
  between shader attributes and Rust `struct`s through a derive macro.

## Getting started

See the [examples](https://github.com/drifting-in-space/limelight/tree/main/examples) directory for
runnable examples.

This tutorial assumes you're familiar with basic WebGL terminology, like vertex and fragment shaders,
uniforms, and buffers.

### Drawing a triangle

([full code](https://github.com/drifting-in-space/limelight/tree/main/examples/01-triangle),
[demo](https://drifting-in-space.github.io/limelight/01-triangle/))

[![A colorful triangle](https://github.com/drifting-in-space/limelight/raw/main/assets/01-triangle.png)](https://drifting-in-space.github.io/limelight/01-triangle/)

This example demonstrates the three main steps to produce an image with limelight:
1. Create a `Program` object. A `Program` in limelight contains the vertex and fragment shader pair
   (a `WebGLProgram` object), and also contains program-specific state.
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
  let mut program = Program::new(
      include_str!("../../examples/01-triangle/shaders/shader.vert"),
      include_str!("../../examples/01-triangle/shaders/shader.frag"),
      DrawMode::Triangles
  );

  // Create a renderer. The renderer becomes the owner of the
  // WebGl2RenderingContext, to ensure that its internal representation
  // of the GPU state is always accureate.
  let mut renderer = Renderer::new(gl);

  // Run the program, rendering the results to the screen. We are
  // not passing any vertex attribute data, so we use a `DummyBuffer`
  // which renders three vertices: one for each corner of a triangle.
  renderer.render(&mut program, &DummyBuffer::new(3)).unwrap();
}
```

### Using buffers

([full code](https://github.com/drifting-in-space/limelight/tree/main/examples/02-buffer),
[demo](https://drifting-in-space.github.io/limelight/02-buffer/))

[![Two small triangles](https://github.com/drifting-in-space/limelight/raw/main/assets/02-buffer.png)](https://drifting-in-space.github.io/limelight/02-buffer/)

Buffers enable arbitrary vertex attribute data to be passed into the shaders. Limelight provides a
procedural macro (`attribute`) for mapping from a Rust-side `struct` to a GPU-side set of
vertex attributes. To use this macro, your crate will also have to depend on [`bytemuck`](https://docs.rs/bytemuck/latest/bytemuck/) and its `derive` feature.

```rust
use web_sys::WebGl2RenderingContext;
use limelight::{Program, Renderer, Buffer, DrawMode, BufferUsageHint, attribute};

// This attribute macro derives a number of traits, including `VertexAttribute`, which
// is required for a type to be used in an `Buffer`.
#[attribute]
struct VertexDescription {
    position: [f32; 2], // field names are mapped to variables in the shader.
}

impl VertexDescription {
    pub fn new(x: f32, y: f32) -> Self {
        VertexDescription { position: [x, y] }
    }
}

fn render_triangles(gl: WebGl2RenderingContext) {
  let mut program = Program::new(
      include_str!("../../examples/02-buffer/shaders/shader.vert"),
      include_str!("../../examples/02-buffer/shaders/shader.frag"),
      DrawMode::Triangles
  );

  let mut renderer = Renderer::new(gl);

  let data = vec![
      // Lower-left triangle.
      VertexDescription::new(-0.1, -0.1),
      VertexDescription::new(-0.5, -0.1),
      VertexDescription::new(-0.5, -0.5),
      // Upper-right triangle.
      VertexDescription::new(0.1, 0.1),
      VertexDescription::new(0.5, 0.1),
      VertexDescription::new(0.5, 0.5),
  ];

  // Declare a buffer.
  let mut buffer: Buffer<VertexDescription> =
    Buffer::new(data, BufferUsageHint::StaticDraw);

  renderer.render(&mut program, &buffer).unwrap();
}
```

### Uniforms

([full code](https://github.com/drifting-in-space/limelight/tree/main/examples/03-uniform),
[demo](https://drifting-in-space.github.io/limelight/03-uniform/))

[![A scaled and rotated triangle](https://github.com/drifting-in-space/limelight/raw/main/assets/03-uniform.png)](https://drifting-in-space.github.io/limelight/03-uniform/)

Uniforms are values that can be used in both shader and fragment programs. They can vary
between `render` calls, but for a given render call each uniform has a constant value
across all vertices and fragments.

```rust
use limelight::{DrawMode, DummyBuffer, Program, Renderer, Uniform};
use web_sys::WebGl2RenderingContext;

fn render_triangles_with_uniform(gl: WebGl2RenderingContext) {
    // This will correspond to "uniform float u_rotate" in the vertex shader.
    let rotate_uniform = Uniform::new(std::f32::consts::PI / 3.4);
    
    // This will correspond to "uniform vec2 u_scale" in the vertex shader.
    let scale_uniform = Uniform::new([0.5, 0.8]);

    // This will correspond to "uniform vec3 u_color" in the fragment shader.
    let color_uniform = Uniform::new([0.9, 0.2, 0.3]);

    let mut program = Program::new(
      include_str!("../../examples/03-uniform/shaders/shader.vert"),
      include_str!("../../examples/03-uniform/shaders/shader.frag"),
      DrawMode::Triangles,
    )
    // We need to map the uniforms when we create the program.
    // The GPU-side types are automatically inferred from the Rust types.
    .with_uniform("u_rotate", rotate_uniform)
    .with_uniform("u_scale", scale_uniform)
    .with_uniform("u_color", color_uniform);

    let mut renderer = Renderer::new(gl);
    renderer.render(&mut program, &DummyBuffer::new(3)).unwrap();
}
```

### Animation

([full code](https://github.com/drifting-in-space/limelight/tree/main/examples/04-animate),
[demo](https://drifting-in-space.github.io/limelight/04-animate/))

The previous examples have rendered static images, so we haven't had a need to separate code
that sets up the initial data structures from code that updates GPU-side data and triggers an
animation. In this example, we separate the code into a `new()` method that is called once,
and a `render` method that is called on every frame.

limelight is not a framework, and in order to integrate with other frameworks, it is not opinionated
as to how you structure your code. This example shows one way you might choose to structure code for
a simple animation (see the [full code](https://github.com/drifting-in-space/limelight/tree/main/examples/04-animate)
to see how it can be integrated with the [Yew](https://yew.rs/) web framework).

`buffer.set_data` and `uniform.set_data` are *lazy*: they do not result in any GPU activity until
the next time the buffer is used in a render call. (See [WebGL Insights](http://www.webglinsights.com/)
section 14.2, *Deferring until the Draw Cycle*.) If a buffer or uniform is unchanged between render
calls, it is not re-written to the GPU.

```rust
use limelight::{Buffer, BufferUsageHint, DrawMode, Program, Renderer, Uniform, attribute};
use web_sys::WebGl2RenderingContext;

struct Animation {
    program: Program<VertexDescription, ()>,
    buffer: Buffer<VertexDescription>,
    uniform: Uniform<[f32; 3]>,
}

impl Animation {
    pub fn new(gl: &WebGl2RenderingContext) -> Self {
        let buffer = Buffer::new(vec![], BufferUsageHint::DynamicDraw);
        let uniform = Uniform::new([0., 0., 0.]);

        let program = Program::new(
            include_str!("../../examples/04-animate/shaders/shader.vert"),
            include_str!("../../examples/04-animate/shaders/shader.frag"),
            DrawMode::Triangles,
        )
        // Note that we clone uniform, so that we can retain a handle to it.
        // Cloning a `Uniform` results in a reference-counted pointer to the
        // same uniform.
        .with_uniform("u_color", uniform.clone());       
        
        Animation {
            buffer,
            program,
            uniform
        }
    }

    pub fn render(&mut self, time: f64, renderer: &mut Renderer) {
        let theta1 = time as f32 / 1000.;
        let theta2 = theta1 + (std::f32::consts::TAU / 3.);
        let theta3 = theta2 + (std::f32::consts::TAU / 3.);
        
        self.buffer.set_data(vec![
            VertexDescription::new(theta1.cos(), theta1.sin()),
            VertexDescription::new(theta2.cos(), theta2.sin()),
            VertexDescription::new(theta3.cos(), theta3.sin()),
        ]);

        let r = (time as f32 / 3000.).sin() / 2. + 0.5;
        let g = (time as f32 / 5000.).sin() / 2. + 0.5;
        let b = (time as f32 / 7000.).sin() / 2. + 0.5;

        self.uniform.set_value([r, g, b]);

        renderer.render(&mut self.program, &self.buffer).unwrap();
    }
}

#[attribute]
struct VertexDescription {
    position: [f32; 2],
}

impl VertexDescription {
    pub fn new(x: f32, y: f32) -> Self {
        VertexDescription { position: [x, y] }
    }
}
```

## More examples

- [Instancing](https://drifting-in-space.github.io/limelight/instances/)
- [Pong](https://drifting-in-space.github.io/limelight/pong/)
- [Zooming / panning](https://drifting-in-space.github.io/limelight/zoom-pan/)
- [Full-canvas shader](https://drifting-in-space.github.io/limelight/zoom-pan/)
- [Primitives](https://drifting-in-space.github.io/limelight/primitives/)