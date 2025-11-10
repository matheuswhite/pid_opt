use aule::prelude::*;
use rand::{Rng, rngs::StdRng};
use std::{f32::consts::PI, time::Duration};

use crate::input;

#[derive(Clone, Copy, Default)]
pub enum Model {
    #[default]
    DCMotor,
    Complex,
}

#[derive(Clone)]
pub struct Individual {
    kp: f32,
    ki: f32,
    kd: f32,
    fitness: f32,
    model: Model,
    dir: &'static str,
    seed: u64,
}

// father: 0.123124  mother: 0.567890 digit: 2 and random = father -> (0.003000, 0.007000)
fn crossover_digit(father: f32, mother: f32, digit: i32, rng: &mut StdRng) -> (f32, f32) {
    let factor = 10f32.powi(digit);
    let father_digit = ((father / factor) as u32 % 10) as f32;
    let mother_digit = ((mother / factor) as u32 % 10) as f32;

    let (d1, d2) = if rng.random::<f32>() <= 0.5 {
        (father_digit, mother_digit)
    } else {
        (mother_digit, father_digit)
    };

    (d1 * factor, d2 * factor)
}

fn crossover_float(father: f32, mother: f32, range: (i32, i32), rng: &mut StdRng) -> (f32, f32) {
    let mut child1 = 0.0;
    let mut child2 = 0.0;

    for digit in range.0..=range.1 {
        let (result1, result2) = crossover_digit(father, mother, digit, rng);
        child1 += result1;
        child2 += result2;
    }

    (child1, child2)
}

impl Individual {
    pub fn new(kp: f32, ki: f32, kd: f32, model: Model, dir: &'static str, seed: u64) -> Self {
        Self {
            kp,
            ki,
            kd,
            fitness: Self::eval_fitness(kp, ki, kd, false, model, dir, seed),
            model,
            dir,
            seed,
        }
    }

    pub fn crossover(
        &self,
        other: &Individual,
        digit_range: (i32, i32),
        rng: &mut StdRng,
    ) -> Vec<Individual> {
        let (kp1, kp2) = crossover_float(self.kp, other.kp, digit_range, rng);
        let (ki1, ki2) = crossover_float(self.ki, other.ki, digit_range, rng);
        let (kd1, kd2) = crossover_float(self.kd, other.kd, digit_range, rng);

        vec![
            Individual::new(kp1, ki1, kd1, self.model, self.dir, self.seed),
            Individual::new(kp2, ki2, kd2, self.model, self.dir, self.seed),
        ]
    }

    pub fn mutate(self, mutation_rate: f32, mutation_step: f32, rng: &mut StdRng) -> Individual {
        let kp = self.kp
            + if rng.random::<f32>() < mutation_rate {
                lerp(rng.random::<f32>(), -mutation_step, mutation_step)
            } else {
                0.0
            };

        let ki = self.ki
            + if rng.random::<f32>() < mutation_rate {
                lerp(rng.random::<f32>(), -mutation_step, mutation_step)
            } else {
                0.0
            };

        let kd = self.kd
            + if rng.random::<f32>() < mutation_rate {
                lerp(rng.random::<f32>(), -mutation_step, mutation_step)
            } else {
                0.0
            };

        Individual::new(
            kp.max(0.0),
            ki.max(0.0),
            kd.max(0.0),
            self.model,
            self.dir,
            self.seed,
        )
    }

    pub fn show(&self) {
        Self::eval_fitness(
            self.kp, self.ki, self.kd, true, self.model, self.dir, self.seed,
        );
    }

    pub fn eval_fitness(
        kp: f32,
        ki: f32,
        kd: f32,
        plotter_en: bool,
        model: Model,
        dir: &str,
        seed: u64,
    ) -> f32 {
        let time = Time::from((1e-2, 8.0 * PI));

        let inputs: [(&'static str, Box<dyn Input>); _] = [
            ("step", Box::new(input::Step::new(1.0))),
            (
                "sinusoidal",
                Box::new(input::Sinusoidal::new(2.0 * PI, 1.0, 0.0)),
            ),
            ("square", Box::new(input::Square::new(2.0 * PI, 1.0, 0.0))),
            (
                "sawtooth",
                Box::new(input::Sawtooth::new(2.0 * PI, 1.0, 0.0)),
            ),
            (
                "random",
                Box::new(input::Random::new(0.0, 1.0, 2.0 * PI, 2.5 * PI, seed)),
            ),
        ];
        let mut sims = inputs.map(|(name, input)| {
            let name = if plotter_en {
                Some(name.to_string())
            } else {
                None
            };
            Simulation::new(kp, ki, kd, input, name, model, dir)
        });

        for dt in time {
            if plotter_en {
                for sim in sims.iter_mut() {
                    let _ = dt >> sim.as_input();
                }
            } else {
                let _ = dt >> sims[2].as_input();
            }
        }

        sims[2].error_metric_value()
    }

    pub fn kp(&self) -> f32 {
        self.kp
    }

    pub fn ki(&self) -> f32 {
        self.ki
    }

    pub fn kd(&self) -> f32 {
        self.kd
    }

    pub fn fitness(&self) -> f32 {
        self.fitness
    }
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    (1.0 - t) * a + t * b
}

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl PartialOrd for Individual {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

struct Simulation {
    input: Box<dyn Input>,
    error_metric: IAE,
    pid: PID,
    plant: SS<Euler>,
    writter: Option<Writter>,
}

impl Simulation {
    pub fn new(
        kp: f32,
        ki: f32,
        kd: f32,
        input: Box<dyn Input>,
        name: Option<String>,
        model: Model,
        dir: &str,
    ) -> Self {
        let k = 1.0;
        let a = 1.0;
        let tf_motor = Tf::new(&[k], &[1.0, k * a]).into();
        let tf_complex = Tf::new(&[-0.3183, 1.0], &[1.013e-1, 0.0318, 1.0]).into();

        Self {
            input,
            error_metric: IAE::new(),
            pid: PID::new(kp, ki, kd),
            plant: match model {
                Model::DCMotor => tf_motor,
                Model::Complex => tf_complex,
            },
            writter: name.map(|name| {
                Writter::new(&format!("output/{}/{}.csv", dir, name), ["input", "output"])
            }),
        }
    }

    pub fn error_metric_value(&self) -> f32 {
        self.error_metric.value()
    }
}

impl Input for Simulation {
    fn output(&mut self, dt: Duration) -> Signal {
        let signal = dt >> &mut *self.input;
        let error = signal - self.plant.last_output();
        let control_signal = self.pid.as_siso() * error;
        let output = control_signal * self.plant.as_siso();

        let _ = error >> self.error_metric.as_metric();

        if let Some(writter) = &mut self.writter {
            let _ = (signal, output) >> writter.as_output();
        }

        output
    }
}

impl AsInput for Simulation {}
