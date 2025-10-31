use std::process::Command;

use crate::{genetic_algorithm::GeneticAlgorithmBuilder, individual::Model};
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
        .with_population_size(1_000)
        .with_parallel_works(4)
        .with_model(Model::Complex)
        .build();

    let mut best_individual = None;
    let generations = Time::from((1.0, 200.0));

    for _ in generations {
        println!("Evolving generation {}", ga.generation());
        let Some(best) = ga.eval(0.5, 0.3) else {
            println!("No best individual found in this generation.");
            break;
        };

        best_individual = Some(best.clone());

        println!(
            "Generation {}:\n  Size: {}\n  Best PID = (kp: {:.5}, ki: {:.5}, kd: {:.5}) with fitness {:.5}",
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
            "Best individual found: PID = (kp: {:.5}, ki: {:.5}, kd: {:.5}) with fitness {:.5}",
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
