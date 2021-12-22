use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use limelight::{renderer::Drawable, Renderer};
use limelight_primitives::{Line3D, Line3DLayer, Color};
use limelight_yew::{
    LimelightComponent, LimelightComponentProps, LimelightController, ShouldRequestAnimationFrame,
};
use palette::{Gradient, Hsl, Srgb};
use rand::prelude::ThreadRng;
use rand::{Rng, thread_rng};
use std::f32::consts::TAU;
use palette::chromatic_adaptation::AdaptInto;
use rand::seq::SliceRandom;

const MAX_LINES: usize = 500;

struct Snowflake {
    snowflake: Line3DLayer,
    lines: Vec<Line3D>,
    rng: ThreadRng,
    colors: Vec<Srgb>,
}

impl LimelightController for Snowflake {
    fn draw(&mut self, renderer: &mut Renderer, _ts: f64) -> Result<ShouldRequestAnimationFrame> {
        if self.lines.len() < MAX_LINES {
            let start_on_line = self.rng.gen_range(0. .. 0.8);
            let rot = self.rng.gen_range(0. ..TAU);
            let z_delta = self.rng.gen_range(-0.1 .. 0.1);
            let size = self.rng.gen_range(0. .. 0.02f32).sqrt();
            let color = self.colors.choose(&mut self.rng).unwrap();
            let cc = Color(*limelight::bytemuck::from_bytes(&[
                (color.red * 255.) as u8,
                (color.green * 255.) as u8,
                (color.blue * 255.) as u8,
                0xff,
            ]));

            for i in 0..6 {
                for j in [-1., 1.] {
                    let theta = TAU * (i as f32 / 6.);
                    let xx = start_on_line * theta.cos();
                    let yy = start_on_line * theta.sin();
    
                    let theta2 = theta + rot * j;
                    let xx_delta = theta2.cos() * size;
                    let yy_delta = theta2.sin() * size;
    
                    self.lines.push(Line3D {
                        start: [xx, yy, 0.],
                        end: [xx + xx_delta, yy + yy_delta, z_delta],
                        width: 0.003,
                        color: cc,
                    });
                }
            }

            self.snowflake.buffer().set_data(self.lines.clone());
        }

        self.snowflake.draw(renderer)?;

        Ok(true)
    }

    fn handle_mousemove(&mut self, x: f32, y: f32) -> ShouldRequestAnimationFrame {
        let rot = nalgebra::Rotation3::from_euler_angles(-y / 2., x / 2., 0.);

        self.snowflake.transform().set_value(rot.to_homogeneous().into());

        true
    }
}

impl Snowflake {
    fn new() -> Rc<RefCell<Self>> {
        let mut rng = thread_rng();
        let c1 = Hsl::new(rng.gen_range(0. .. 360.), rng.gen_range(0.6 .. 1.), rng.gen_range(0.6 .. 0.8));
        let c2 = Hsl::new(rng.gen_range(0. .. 360.), rng.gen_range(0. .. 1.), rng.gen_range(0.6 .. 1.));
        let c3 = Hsl::new(rng.gen_range(0. .. 360.), rng.gen_range(0.6 .. 1.), rng.gen_range(0.6 .. 0.7));
        let colors: Vec<Srgb> = Gradient::new(vec![c1, c2, c3]).take(10).map(|d| d.adapt_into()).collect();

        Rc::new(RefCell::new(Snowflake {
            snowflake: Line3DLayer::default(),
            lines: Vec::new(),
            rng,
            colors,
        }))
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    yew::start_app_with_props::<LimelightComponent<Snowflake>>(LimelightComponentProps {
        controller: Snowflake::new(),
        height: 500,
        width: 500,
    });
}
