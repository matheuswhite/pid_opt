use crate::{
    individual::Individual,
    work::{Work, work_pool},
};

#[derive(Clone)]
pub struct Population {
    individuals: Vec<Individual>,
}

impl Population {
    pub fn new(size: usize) -> Self {
        let individuals = NewRandomPopulation::default().work((0..size).map(|_| ()).collect());

        Self { individuals }.sorted()
    }

    pub fn new_parallel(size: usize, works: usize) -> Self {
        let individuals = work_pool(
            works,
            (0..size).map(|_| ()).collect(),
            NewRandomPopulation::default(),
        );

        Self { individuals }.sorted()
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
        self.individuals.drain(3_000.min(self.individuals.len())..);
        self
    }

    pub fn merge(self, other: Population) -> Population {
        let mut individuals = self.individuals;
        individuals.extend(other.individuals);

        Population { individuals }.sorted()
    }

    pub fn get_nth_bests(&self, n: usize) -> Population {
        self.individuals
            .iter()
            .take(n)
            .cloned()
            .collect::<Vec<_>>()
            .into()
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
        let index = rand::random::<u32>() as usize % self.individuals.len();
        self.individuals.remove(index)
    }

    pub fn get_random_individual(&self) -> &Individual {
        let index = rand::random::<u32>() as usize % self.individuals.len();
        &self.individuals[index]
    }
}

impl From<Vec<Individual>> for Population {
    fn from(individuals: Vec<Individual>) -> Self {
        Self { individuals }.sorted()
    }
}

#[derive(Default)]
struct NewRandomPopulation {
    id: usize,
}

impl NewRandomPopulation {
    const MAX_KP: f32 = 5.0;
    const MAX_KI: f32 = 5.0;
    const MAX_KD: f32 = 1.0;
}

impl Work for NewRandomPopulation {
    type Input = ();
    type Output = Individual;

    fn work(&self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        let size = input.len();
        let mut individuals = Vec::with_capacity(size);
        for _ in 0..size {
            let kp = rand::random::<f32>() * Self::MAX_KP;
            let ki = rand::random::<f32>() * Self::MAX_KI;
            let kd = rand::random::<f32>() * Self::MAX_KD;
            individuals.push(Individual::new(kp, ki, kd));
        }

        individuals
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}
