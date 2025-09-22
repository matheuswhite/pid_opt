use aule::prelude::*;
use typst::foundations::IntoValue;

#[derive(Clone)]
pub struct Individual {
    kp: f32,
    ki: f32,
    kd: f32,
    fitness: f32,
}

impl IntoValue for Individual {
    fn into_value(self) -> typst::foundations::Value {
        let d = typst::foundations::dict!(
            "kp" => format!("{:?}", self.kp).into_value(),
            "ki" => format!("{:?}", self.ki).into_value(),
            "kd" => format!("{:?}", self.kd).into_value(),
            "fitness" => format!("{:?}", self.fitness).into_value(),
        );
        typst::foundations::Value::Dict(d)
    }
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
        let time = Time::from((1e-4, 10.0));

        let mut input = Step::new();
        let mut good_hart = GoodHart::new(0.3, 0.3, 0.4);
        let mut pid = PID::new(kp, ki, kd);
        let mut plant: SS<Euler> = Tf::new(&[-0.3183, 1.0], &[1.013e-1, 0.0318, 1.0]).into();
        let mut plotter = if plotter_en {
            Some(Plotter::new())
        } else {
            None
        };

        for dt in time {
            let signal = dt >> input.as_input();
            let error = signal - plant.last_output();
            let control_signal = pid.as_siso() * error;
            let output = control_signal * plant.as_siso();

            let _ = (error, control_signal) >> good_hart.as_error_metric();

            if let Some(plotter) = &mut plotter {
                let _ = (output) >> plotter.as_monitor();
            }
        }

        if let Some(plotter) = &mut plotter {
            plotter.display();
            plotter.join();
        }

        good_hart.value()
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
