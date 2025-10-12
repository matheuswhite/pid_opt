use std::process::Command;

use crate::genetic_algorithm::GeneticAlgorithmBuilder;
use aule::prelude::*;

mod genetic_algorithm;
mod individual;
mod input;
mod population;
mod work;

fn main() {
    println!("Removing last outputs...");
    let _ = std::fs::remove_dir_all("output");
    std::fs::create_dir_all("output").unwrap();

    println!("Generating initial population...");

    let mut ga = GeneticAlgorithmBuilder::default()
        .with_population_size(5_000)
        .with_parallel_works(4)
        .build();

    let mut best_individual = None;
    let generations = Time::from((1.0, 500.0));

    for _ in generations {
        println!("Evolving generation {}", ga.generation());
        let Some(best) = ga.eval(0.3, 0.1) else {
            println!("No best individual found in this generation.");
            break;
        };

        best_individual = Some(best.clone());

        println!(
            "Generation {}:\n  Size: {}\n  Best PID = (kp: {:.2}, ki: {:.2}, kd: {:.2}) with fitness {:.5}",
            ga.generation(),
            ga.len(),
            best.kp(),
            best.ki(),
            best.kd(),
            best.fitness()
        );
    }

    if let Some(best) = best_individual {
        println!(
            "Best individual found: PID = (kp: {:.2}, ki: {:.2}, kd: {:.2}) with fitness {:.5}",
            best.kp(),
            best.ki(),
            best.kd(),
            best.fitness()
        );
        best.show();

        let cmd = Command::new("python").arg("plot.py").output().unwrap();
        if cmd.status.success() {
            println!("Plot generated successfully.");
        } else {
            eprintln!(
                "Error generating plot: {}",
                String::from_utf8_lossy(&cmd.stderr)
            );
        }
    } else {
        println!("No best individual found.");
    }
}
