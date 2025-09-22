use crate::{genetic_algorithm::GeneticAlgorithmBuilder, individual::Individual, report::Report};

mod genetic_algorithm;
mod individual;
mod input;
mod population;
mod report;
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
    // let mut report = Report::new("report.pdf");
    // let individual = Individual::new(0.03, 2.75, 0.28);
    // report.set_best_individual(individual);
    // report.save();

    genetic_algorithm_example();
}

fn genetic_algorithm_example() {
    println!("Generating initial population...");

    let mut report = Report::new("report.pdf");
    let mut ga = GeneticAlgorithmBuilder::default()
        .with_population_size(5_000)
        .with_parallel_works(4)
        .build();

    let mut best_individual = None;

    loop {
        println!("Evolving generation {}", ga.generation());
        let Some(best) = ga.eval(0.25, 0.1) else {
            println!("No best individual found in this generation.");
            break;
        };

        best_individual = Some(best.clone());
        report.set_best_individual(best.clone());

        println!(
            "Generation {}:\n  Size: {}\n  Best PID = (kp: {:.2}, ki: {:.2}, kd: {:.2}) with fitness {:.5}",
            ga.generation(),
            ga.len(),
            best.kp(),
            best.ki(),
            best.kd(),
            best.fitness()
        );

        if best.fitness() <= 3.5 {
            println!("Stopping criteria reached.");
            break;
        }
    }

    if let Some(best) = best_individual {
        println!(
            "Best individual found: PID = (kp: {:.2}, ki: {:.2}, kd: {:.2}) with fitness {:.5}",
            best.kp(),
            best.ki(),
            best.kd(),
            best.fitness()
        );
        report.save();
        best.show();
    } else {
        println!("No best individual found.");
    }
}
