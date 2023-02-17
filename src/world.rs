//! Physics simulation world.

use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};
use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Canvas, RenderTarget},
};

pub struct World {
    size: (f64, f64),

    balls: Vec<Ball>,
}

struct Ball {
    pos: (f64, f64),
    vel: (f64, f64),

    color: Color,
}

impl World {
    pub fn draw<T>(&self, canvas: &mut Canvas<T>)
    where
        T: RenderTarget,
    {
        for ball in &self.balls {
            ball.draw(canvas);
        }
    }

    pub fn new() -> Self {
        Self {
            size: (800.0, 600.0),
            // balls: (0..1000).map(|_| Ball::random((800.0, 600.0))).collect(),
            balls: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        self.balls.push(Ball::random(self.size));
        self.balls
            .par_iter_mut()
            .for_each(|ball| ball.update_vel(&self.size));
        self.balls.par_iter_mut().for_each(|ball| ball.tick());
    }
}

impl Ball {
    pub fn new(pos: (f64, f64), vel: (f64, f64)) -> Self {
        Self {
            pos,
            vel,
            color: Color::RGB(thread_rng().gen(), thread_rng().gen(), thread_rng().gen()),
        }
    }

    pub fn random(size: (f64, f64)) -> Self {
        let mut rng = thread_rng();
        let vel_max = 5.0;
        Self::new(
            (rng.gen::<f64>() * size.0, rng.gen::<f64>() * size.1),
            (
                (rng.gen::<f64>() * 2.0 - 1.0) * vel_max,
                (rng.gen::<f64>() * 2.0 - 1.0) * vel_max,
            ),
        )
    }

    pub fn update_vel(&mut self, world: &(f64, f64)) {
        if self.pos.0 < 0.0 || self.pos.0 > world.0 {
            self.vel.0 *= -1.0;
        }
        if self.pos.1 < 0.0 || self.pos.1 > world.1 {
            self.vel.1 *= -1.0;
        }
    }

    pub fn tick(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.vel.1 += 0.8;
    }

    pub fn draw<T>(&self, canvas: &mut Canvas<T>)
    where
        T: RenderTarget,
    {
        crate::draw_circle(
            canvas,
            self.color,
            Point::new(self.pos.0 as _, self.pos.1 as _),
            4.0,
        );
    }
}
