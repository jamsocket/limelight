# {{name}}

{{name}} is a `WebGL2` wrapper with a focus on making high-performance graphics code easier to
write and maintain.

In particular, it:
- Provides a functional interface that abstracts away the statefullness of WebGL.
  It accomplishes this by using a *shadow GPU* that tracks the GPU's state, diffs it with the
  desired state, and sends only the necessary instructions to WebGL.
- Provides abstractions for buffers and uniforms that are automatically lazy, so that they only
  invoke GPU calls during the draw cycle. (See [WebGL Insights](http://www.webglinsights.com/) section 14.2, *Deferring until the Draw Cycle*.)
- Provides a typed interface to uniforms and buffers, and automatically generates vertex array objects
  (VAOs) from Rust data types through a derive macro.

## Getting started

See the [examples](examples) directory for full examples.


