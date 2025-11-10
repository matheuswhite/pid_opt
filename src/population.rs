use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::{
    individual::{Individual, Model},
    work::{Work, work_pool},
};

#[derive(Clone)]
pub struct Population {
    individuals: Vec<Individual>,
    rng: StdRng,
}
//0.0031834461
impl Population {
    pub fn new(
        size: usize,
        model: Model,
        dir: &'static str,
        max_kp: f32,
        max_ki: f32,
        max_kd: f32,
        seed: u64,
    ) -> Self {
        let individuals = NewRandomPopulation::new(model, dir, max_kp, max_ki, max_kd, seed)
            .work((0..size).map(|_| ()).collect());
        let rng = StdRng::seed_from_u64(seed);

        Self { individuals, rng }.sorted()
    }

    pub fn new_parallel(
        size: usize,
        works: usize,
        model: Model,
        dir: &'static str,
        max_kp: f32,
        max_ki: f32,
        max_kd: f32,
        seed: u64,
    ) -> Self {
        let individuals = work_pool(
            works,
            (0..size).map(|_| ()).collect(),
            NewRandomPopulation::new(model, dir, max_kp, max_ki, max_kd, seed),
        );
        let rng = StdRng::seed_from_u64(seed);

        Self { individuals, rng }.sorted()
    }

    pub fn len(&self) -> usize {
        self.individuals.len()
    }

    fn sorted(mut self) -> Self {
        let size_before_filter = self.individuals.len();
        let inds = self
            .individuals
            .into_iter()
            .filter(|ind| ind.fitness().is_finite())
            .collect::<Vec<_>>();
        let size_after_filter = inds.len();
        println!(
            "Filtered {} individuals with non-finite fitness ({} remaining)",
            size_before_filter - size_after_filter,
            size_after_filter
        );
        self.individuals = inds;
        self.individuals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.individuals.drain(1_000.min(self.individuals.len())..);
        self
    }

    pub fn merge(self, other: Population) -> Population {
        let mut individuals = self.individuals;
        individuals.extend(other.individuals);

        Population {
            individuals,
            rng: self.rng,
        }
        .sorted()
    }

    pub fn get_nth_bests(&self, n: usize) -> Population {
        let individuals = self
            .individuals
            .iter()
            .take(n)
            .cloned()
            .collect::<Vec<_>>()
            .into();

        Population {
            individuals,
            rng: self.rng.clone(),
        }
    }

    pub fn get_best(&self) -> Option<&Individual> {
        self.individuals.get(0)
    }

    pub fn pop_parents(&mut self) -> Option<(Individual, Individual)> {
        if self.individuals.len() < 2 {
            return None;
        }

        let father = self.pop_random_individual();
        let mother = self.pop_random_individual();

        Some((father, mother))
    }

    pub fn pop_random_individual(&mut self) -> Individual {
        let index = self.rng.random::<u32>() as usize % self.individuals.len();
        self.individuals.remove(index)
    }

    pub fn get_random_individual(&mut self) -> Individual {
        let index = self.rng.random::<u32>() as usize % self.individuals.len();
        self.individuals[index].clone()
    }

    pub fn from_individuals(individuals: Vec<Individual>, seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);

        Self { individuals, rng }.sorted()
    }
}

#[derive(Clone)]
struct NewRandomPopulation {
    id: usize,
    model: Model,
    dir: &'static str,
    max_kp: f32,
    max_ki: f32,
    max_kd: f32,
    rng: StdRng,
    seed: u64,
}

impl NewRandomPopulation {
    pub fn new(
        model: Model,
        dir: &'static str,
        max_kp: f32,
        max_ki: f32,
        max_kd: f32,
        seed: u64,
    ) -> Self {
        let rng = StdRng::seed_from_u64(seed);

        Self {
            id: 0,
            model,
            dir,
            max_kp,
            max_ki,
            max_kd,
            rng,
            seed,
        }
    }
}

impl Work for NewRandomPopulation {
    type Input = ();
    type Output = Individual;

    fn work(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        let size = input.len();
        let mut individuals = Vec::with_capacity(size);
        for _ in 0..size {
            let kp = self.rng.random::<f32>() * self.max_kp;
            let ki = self.rng.random::<f32>() * self.max_ki;
            let kd = self.rng.random::<f32>() * self.max_kd;
            individuals.push(Individual::new(kp, ki, kd, self.model, self.dir, self.seed));
        }

        individuals
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}
