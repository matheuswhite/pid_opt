use crate::{genetic_algorithm::GeneticAlgorithmBuilder};

mod genetic_algorithm;
mod individual;
mod population;
mod work;

pub fn progress_bar(id: String, current: usize, total: usize) {
    println!(
        "[{}] [{}>{}] {}%",
        id,
        "=".repeat(current * 20 / total),
        " ".repeat((total - current) * 20 / total),
        current * 100 / total
    );
}

fn main() {
    genetic_algorithm_example();
}

fn genetic_algorithm_example() {
    println!("Generating initial population...");

    let mut ga = GeneticAlgorithmBuilder::default()
        .with_population_size(1000)
        .with_parallel_works(4)
        .build();

    for _ in 0..100 {
        println!("Evolving generation {}", ga.generation());
        let Some(best) = ga.eval(0.25, 0.5) else {
            println!("No best individual found.");
            return;
        };
        println!(
            "Generation {}:\n  Size: {}\n  Best PID = (kp: {:.2}, ki: {:.2}, kd: {:.2}) with fitness {:.5}",
            ga.generation(),
            ga.len(),
            best.kp(),
            best.ki(),
            best.kd(),
            best.fitness()
        );
        best.show();
    }
}
