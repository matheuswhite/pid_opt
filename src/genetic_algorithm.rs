use rand::{SeedableRng, rngs::StdRng};

use crate::{
    individual::{Individual, Model},
    population::Population,
};

pub struct GeneticAlgorithm {
    population: Population,
    generation: usize,
    mutation_step: f32,
    digit_range: (i32, i32),
    seed: u64,
    rng: StdRng,
}

#[derive(Default)]
pub struct GeneticAlgorithmBuilder {
    population_size: usize,
    parellel_works: usize,
    mutation_step: f32,
    model: Model,
    digit_range: (i32, i32),
    dir: &'static str,
    max_kp: f32,
    max_ki: f32,
    max_kd: f32,
    seed: u64,
}

impl GeneticAlgorithmBuilder {
    pub fn with_population_size(mut self, size: usize) -> Self {
        self.population_size = size;
        self
    }

    pub fn with_parallel_works(mut self, works: usize) -> Self {
        self.parellel_works = works;
        self
    }

    pub fn with_model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }

    pub fn with_mutation_step(mut self, step: f32) -> Self {
        self.mutation_step = step;
        self
    }

    pub fn with_digit_range(mut self, range: (i32, i32)) -> Self {
        self.digit_range = range;
        self
    }

    pub fn with_output_dir(mut self, dir: &'static str) -> Self {
        self.dir = dir;
        self
    }

    pub fn with_max_kp(mut self, max_kp: f32) -> Self {
        self.max_kp = max_kp;
        self
    }

    pub fn with_max_ki(mut self, max_ki: f32) -> Self {
        self.max_ki = max_ki;
        self
    }

    pub fn with_max_kd(mut self, max_kd: f32) -> Self {
        self.max_kd = max_kd;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn build(self) -> GeneticAlgorithm {
        let rng = StdRng::seed_from_u64(self.seed);

        GeneticAlgorithm {
            population: if self.parellel_works == 0 {
                Population::new(
                    self.population_size,
                    self.model,
                    self.dir,
                    self.max_kp,
                    self.max_ki,
                    self.max_kd,
                    self.seed,
                )
            } else {
                Population::new_parallel(
                    self.population_size,
                    self.parellel_works,
                    self.model,
                    self.dir,
                    self.max_kp,
                    self.max_ki,
                    self.max_kd,
                    self.seed,
                )
            },
            generation: 0,
            mutation_step: self.mutation_step,
            digit_range: self.digit_range,
            seed: self.seed,
            rng,
        }
    }
}

impl GeneticAlgorithm {
    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn len(&self) -> usize {
        self.population.len()
    }

    pub fn tournament_section(&mut self, tournament_size: usize) -> Population {
        if self.population.len() < tournament_size {
            return self.population.clone();
        }

        let mut selected = vec![];
        let n_tournaments = self.population.len() / tournament_size;
        for _ in 0..n_tournaments {
            let fighters = (0..tournament_size)
                .map(|_| self.population.get_random_individual())
                .collect::<Vec<_>>();

            let winner = fighters
                .into_iter()
                .max_by(|a, b| {
                    let res = b.partial_cmp(a);
                    if res.is_none() {
                        println!(
                            "Warning: NaN fitness detected during tournament selection: {} vs {}",
                            a.fitness(),
                            b.fitness()
                        );
                    }
                    res.unwrap()
                })
                .unwrap();

            selected.push(winner);
        }

        Population::from_individuals(selected, self.seed)
    }

    pub fn eval(&mut self, mutation_rate: f32, replace_rate: f32) -> Option<Individual> {
        assert!(
            0.0 <= mutation_rate && mutation_rate <= 1.0,
            "Mutation rate must be between 0 and 1"
        );
        assert!(
            0.0 <= replace_rate && replace_rate <= 1.0,
            "Replace rate must be between 0 and 1"
        );

        println!("Selection by tournament...");
        let mut to_reproduce = self.tournament_section(3);

        let mut all_children = vec![];
        let total_crossovers = to_reproduce.len() / 2;
        for _ in 0..total_crossovers {
            let Some((father, mother)) = to_reproduce.pop_parents() else {
                break;
            };

            let children = father.crossover(&mother, self.digit_range, &mut self.rng);
            all_children.extend(children);
        }

        let all_children = Population::from_individuals(
            all_children
                .into_iter()
                .map(|child| child.mutate(mutation_rate, self.mutation_step, &mut self.rng))
                .collect::<Vec<_>>()
                .into(),
            self.seed,
        );

        println!("Replacing...");
        let n_retain = (self.population.len() as f32 * (1.0 - replace_rate)) as usize;
        let best_parents = self.population.get_nth_bests(n_retain);
        self.population = best_parents.merge(all_children);

        self.generation += 1;

        self.population.get_best().map(|ind| ind.clone())
    }
}
