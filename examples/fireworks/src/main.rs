use micro_jam_engine::{Game, vek::Rect, console_log};
use rand::rngs::ThreadRng;
use rand::Rng;

struct Fireworks {
    fireworks: Vec<Firework>,
    sparks: Vec<Spark>,
    rng: ThreadRng,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Firework {
    location: Point,
    speed: Point,
    time: f32,
}

#[derive(Debug)]
struct Spark {
    location: Point,
    speed: Point,
    time: f32,
    exp_time: f32,
}

impl Game for Fireworks {
    const TITLE: &'static str = "Fireworks";

    type SaveData = ();

    fn init(console: &mut micro_jam_engine::Console<Self>) -> Self {
        Fireworks {
            fireworks: vec![],
            sparks: vec![],
            rng: rand::thread_rng(),
        }
    }

    fn tick(&mut self, dt: f32, console: &mut micro_jam_engine::Console<Self>) {
        // let size_x = console.graphics.width();
        // let size_y = console.graphics.height();
        let pressed = console.input.mouse_pressed(0);
        let mouse_location = console.input.mouse();
        if let (true, Some((x, y))) = (pressed, mouse_location) {
            self.fireworks.push(Firework {
                location: Point::new(
                    console.graphics.width() / 2.0,
                    console.graphics.height() - 25.0,
                ),
                speed: Point::new(
                    (self.rng.gen::<f32>() - 0.5) * 30.0,
                    -(20.0 + self.rng.gen::<f32>() * 30.0),
                ),
                time: 0.0,
            });
        }

        console.graphics.clear(0x0);
        for firework in self.fireworks.iter() {
            console.graphics.draw_rect(Rect::new(firework.location.x, firework.location.y, 3.0, 3.0), 0xffffffff, true);
        }

        let mut to_remove = vec![];
        for (idx, firework) in self.fireworks.iter_mut().enumerate() {
            firework.location.x += firework.speed.x * dt;
            firework.location.y += firework.speed.y * dt;

            firework.time += dt;

            if firework.time > 2.0 {
                to_remove.push(idx);
            }
        }

        // remove old fireworks
        for idx in to_remove.into_iter().rev() {
            let removed = self.fireworks.remove(idx);
            for _ in 0..self.rng.gen_range(20..40) {
                self.sparks.push(Spark {
                    location: removed.location,
                    speed: Point {
                        x: (self.rng.gen::<f32>() - 0.5) * 15.0,
                        y: (self.rng.gen::<f32>() - 0.5) * 15.0,
                    },
                    time: 0.0,
                    exp_time: self.rng.gen::<f32>() + 1.5,
                })
            }
        }

        for spark in self.sparks.iter() {
            console.graphics.draw_rect(Rect::new(spark.location.x, spark.location.y, 2.0, 2.0), 0xffffffff, true);
        }

        let mut sparks_to_remove = vec![];
        for (idx, spark) in self.sparks.iter_mut().enumerate() {
            spark.speed.y += 4.0 * dt;
            spark.location.x += spark.speed.x * dt;
            spark.location.y += spark.speed.y * dt;
            spark.time += dt;

            if spark.time > spark.exp_time {
                sparks_to_remove.push(idx);
            }
        }

        for idx in sparks_to_remove.into_iter().rev() {
            self.sparks.remove(idx);
        }
    }
}

fn main() {
    Fireworks::run();
}
