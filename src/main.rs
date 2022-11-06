use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::input::mouse::*;

use glam::*;
use std::f32::consts::{FRAC_PI_2, PI};

use typed_arena::Arena;

use rand::distributions::uniform::SampleRange;
use rand::{thread_rng, Rng};

mod boids;
mod xy;
use xy::XY;
use boids::*;

#[derive(PartialEq,Debug,Clone,Copy)]
pub struct Bird {
    pub pos: [f32; 2],
    pub vel: [f32; 2],
    acc: [f32; 2],
    color: Color
}
impl XY for Bird {
    fn xy(&self) -> [f32; 2] {
        self.pos
    }
}
impl Bird {
    fn random<P, V>(vec_range: V, pos_range: P) -> Self
    where
        P: SampleRange<u16> + Clone,
        V: SampleRange<i32> + Clone,
        //creates random velocity and starting placement for bird, within
        //defined ranges
    {
        let mut rng = thread_rng();
        let vel_sign = || -> f32 {
            if thread_rng().gen_bool(0.5) {
                1.0
            } else {
                -1.0
            }
        };
        let vel = [rng.gen_range(vec_range.clone()) as f32 * vel_sign(),
                rng.gen_range(vec_range) as f32 * vel_sign()];
        let pos = [rng.gen_range(pos_range.clone()) as f32,
                rng.gen_range(pos_range) as f32];
        Bird {
            vel, pos, acc: [0.0, 0.0],
            color: Color::new(rng.gen(), rng.gen(), rng.gen(), 1.0),
        }
    }
}

#[derive(Debug)]
struct MainState {
    birds: Vec<Bird>,
    bird_shape: graphics::MeshBuilder,
}
impl MainState {
    fn new() -> GameResult<MainState> {
        Ok(MainState {
            birds: vec![],
            bird_shape: graphics::MeshBuilder::new()
                .line(
                    &[//should make an equilateral triangle
                        [-2.0, -4.0],
                        [0.0, 4.124],
                        [2.0, -4.0],
                        [-2.0, -4.0]
                    ], 1.0, Color::WHITE
                )?.to_owned(),
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if button_pressed(ctx, MouseButton::Left) {
            self.birds.push(Bird::random(2..=4, 40..760));
        }//spawn birds on click

        let (width, height) = graphics::size(ctx);
        let all_birds = self.birds.clone();

        self.birds = self.birds.iter_mut().map(|bird| {
            //get birds within a range of this bird
            let local_boid = bird.get_local_boid(&all_birds[..], 50.0);
            //apply the three rules of a flock 
            bird.acc = bird.flock_rules(&local_boid[..]);

            //update velocities
            bird.vel[0] = bird.acc[0] + bird.vel[0];
            bird.vel[1] = bird.acc[1] + bird.vel[1];

            //update positions
            bird.pos[0] = bird.vel[0] + bird.pos[0] % width;
            bird.pos[1] = bird.vel[1] + bird.pos[1] % height;

            //wrap to opposite border when position is less than 0
            if bird.pos[0] <= 0.0 {
                bird.pos[0] += width;
            }
            if bird.pos[1] <= 0.0 {
                bird.pos[1] += height;
            }
            bird.to_owned()
        }).collect();//update bird positions according to velocity
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let mesh = &self.bird_shape.build(ctx)?;

        for bird in self.birds.iter() {
            graphics::draw(ctx, mesh, (bird.pos, bird.vel[1].atan2(bird.vel[0]) - FRAC_PI_2, bird.color))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("boids", "stefano");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
