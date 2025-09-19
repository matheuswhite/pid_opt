use crate::{individual::Individual, population::Population, progress_bar};

pub struct GeneticAlgorithm {
    population: Population,
    generation: usize,
    parallel_works: usize,
}

#[derive(Default)]
pub struct GeneticAlgorithmBuilder {
    population_size: usize,
    parellel_works: usize,
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

    pub fn build(self) -> GeneticAlgorithm {
        GeneticAlgorithm {
            population: if self.parellel_works == 0 {
                Population::new(self.population_size)
            } else {
                Population::new_parallel(self.population_size, self.parellel_works)
            },
            generation: 0,
            parallel_works: self.parellel_works,
        }
    }
}

impl GeneticAlgorithm {
    const MUTATION_STEP: f32 = 5.0;

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
                .max_by(|a, b| b.partial_cmp(a).unwrap())
                .unwrap();

            selected.push(winner.clone());
        }

        Population::from(selected)
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
        for i in 0..total_crossovers {
            let Some((father, mother)) = to_reproduce.pop_parents() else {
                break;
            };

            let children = father.crossover(&mother);
            all_children.extend(children);

            progress_bar("crossover".to_string(), i + 1, total_crossovers);
        }

        let children_count = all_children.len();
        let all_children = all_children
            .into_iter()
            .enumerate()
            .map(|(i, child)| {
                progress_bar("mutation".to_string(), i + 1, children_count);

                child.mutate(mutation_rate, Self::MUTATION_STEP)
            })
            .collect::<Vec<_>>()
            .into();

        println!("Replacing...");
        let n_retain = (self.population.len() as f32 * (1.0 - replace_rate)) as usize;
        let best_parents = self.population.get_nth_bests(n_retain);
        self.population = best_parents.merge(all_children);

        self.generation += 1;

        self.population.get_best().map(|ind| ind.clone())
    }
}
