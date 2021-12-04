use limelight::Uniform;

pub struct TransformUniform {
    scale: (f32, f32),
    center: (f32, f32),
    uniform: Uniform<[[f32; 4]; 4]>,
}

fn scale_center_to_matrix(
    (scale_x, scale_y): (f32, f32),
    (center_x, center_y): (f32, f32),
) -> [[f32; 4]; 4] {
    [
        [scale_x, 0., 0., -center_x],
        [0., scale_y, 0., -center_y],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]
}

impl TransformUniform {
    pub fn new() -> Self {
        let scale = (1., 1.);
        let center = (0., 0.);
        let uniform = Uniform::new(scale_center_to_matrix(scale, center));
        TransformUniform {
            scale,
            center,
            uniform,
        }
    }

    pub fn uniform(&self) -> Uniform<[[f32; 4]; 4]> {
        self.uniform.clone()
    }

    fn update_uniform(&self) {
        self.uniform
            .set_value(scale_center_to_matrix(self.scale, self.center));
    }

    /// Multiply the current scale, in such a way that the given point
    /// remains stationary.
    ///
    /// i.e. if `v * self.uniform.value = scale_center` is true before the
    /// scale is applied, it should also be true after.
    pub fn scale(&mut self, scale_factor: f32, scale_center: (f32, f32)) {
        let old_scale_x = self.scale.0;
        let old_scale_y = self.scale.1;

        self.scale.0 *= scale_factor;
        self.scale.1 *= scale_factor;

        self.center.0 =
            (self.scale.0 / old_scale_x) * (scale_center.0 + self.center.0) - scale_center.0;
        self.center.1 =
            (self.scale.1 / old_scale_y) * (scale_center.1 + self.center.1) - scale_center.1;

        self.update_uniform();
    }

    // Pan by the given amount, provided in destination units.
    pub fn pan(&mut self, vector: (f32, f32)) {
        self.center.0 -= vector.0;
        self.center.1 -= vector.1;
        self.update_uniform();
    }
}
