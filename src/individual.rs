use aule::prelude::{
    AsInput, AsMetric, AsOutput, AsSISO, Euler, ITAE, Input, Metric, PID, SISO, SS, Signal, Tf,
    Time, Writter,
};
use std::{f32::consts::PI, time::Duration};

use crate::input::*;

#[derive(Clone)]
pub struct Individual {
    kp: f32,
    ki: f32,
    kd: f32,
    fitness: f32,
}

impl Individual {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            fitness: Self::eval_fitness(kp, ki, kd, false),
        }
    }

    pub fn crossover(&self, other: &Individual) -> Vec<Individual> {
        let (kp1, kp2) = (rand::random::<f32>() <= 0.5)
            .then(|| (self.kp, other.kp))
            .unwrap_or((self.kp, other.kp));
        let (ki1, ki2) = (rand::random::<f32>() <= 0.5)
            .then(|| (self.ki, other.ki))
            .unwrap_or((self.ki, other.ki));
        let (kd1, kd2) = (rand::random::<f32>() <= 0.5)
            .then(|| (self.kd, other.kd))
            .unwrap_or((self.kd, other.kd));

        vec![
            Individual::new(kp1, ki1, kd1),
            Individual::new(kp2, ki2, kd2),
        ]
    }

    pub fn mutate(self, mutation_rate: f32, mutation_step: f32) -> Individual {
        let kp = self.kp
            + if rand::random::<f32>() < mutation_rate {
                lerp(rand::random::<f32>(), -mutation_step, mutation_step)
            } else {
                0.0
            };

        let ki = self.ki
            + if rand::random::<f32>() < mutation_rate {
                lerp(rand::random::<f32>(), -mutation_step, mutation_step)
            } else {
                0.0
            };

        let kd = self.kd
            + if rand::random::<f32>() < mutation_rate {
                lerp(rand::random::<f32>(), -mutation_step, mutation_step)
            } else {
                0.0
            };

        Individual::new(kp, ki, kd)
    }

    pub fn show(&self) {
        Self::eval_fitness(self.kp, self.ki, self.kd, true);
    }

    pub fn eval_fitness(kp: f32, ki: f32, kd: f32, plotter_en: bool) -> f32 {
        let time = Time::from((1e-2, 6.0));

        let inputs: [(&'static str, Box<dyn Input>); _] = [
            ("step", Box::new(Step::new(1.0))),
            ("sinusoidal", Box::new(Sinusoidal::new(PI / 2.0, 1.0, 0.0))),
            ("square", Box::new(Square::new(PI / 2.0, 1.0, 0.0))),
            ("sawtooth", Box::new(Sawtooth::new(PI / 2.0, 1.0, 0.0))),
            (
                "random",
                Box::new(Random::new(0.0, 1.0, PI / 4.0, PI / 2.0)),
            ),
        ];
        let mut sims = inputs.map(|(name, input)| {
            let name = if plotter_en {
                Some(name.to_string())
            } else {
                None
            };
            Simulation::new(kp, ki, kd, input, name)
        });

        for dt in time {
            if plotter_en {
                for sim in sims.iter_mut() {
                    let _ = dt >> sim.as_input();
                }
            } else {
                let _ = dt >> sims[0].as_input();
                let _ = dt >> sims[2].as_input();
            }
        }

        (sims[0].error_metric_value() + sims[2].error_metric_value()) / 2.0
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
    error_metric: ITAE,
    pid: PID,
    plant: SS<Euler>,
    writter: Option<Writter>,
}

impl Simulation {
    pub fn new(kp: f32, ki: f32, kd: f32, input: Box<dyn Input>, name: Option<String>) -> Self {
        let k = 1.0;
        let a = 1.0;

        Self {
            input,
            error_metric: ITAE::new(),
            pid: PID::new(kp, ki, kd),
            plant: Tf::new(&[k], &[1.0, k * a]).into(),
            writter: name
                .map(|name| Writter::new(&format!("output/{}.csv", name), ["input", "output"])),
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
