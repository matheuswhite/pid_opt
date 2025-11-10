use std::{fs::File, process::Command};

use crate::{
    genetic_algorithm::{GeneticAlgorithm, GeneticAlgorithmBuilder},
    individual::Model,
};
use aule::prelude::*;
use gag::Redirect;

mod genetic_algorithm;
mod individual;
mod input;
mod population;
mod work;

fn main() {
    run_ga(
        "dc_motor",
        GeneticAlgorithmBuilder::default()
            .with_population_size(1_000)
            .with_parallel_works(4)
            .with_model(Model::DCMotor)
            .with_mutation_step(1.0)
            .with_digit_range((-1, 3))
            .with_output_dir("dc_motor")
            .with_max_kp(100.0)
            .with_max_ki(100.0)
            .with_seed(0x2268a378740265f9)
            .build(),
        100,
    );
    run_ga(
        "complex_system",
        GeneticAlgorithmBuilder::default()
            .with_population_size(1_000)
            .with_parallel_works(4)
            .with_model(Model::Complex)
            .with_mutation_step(0.1)
            .with_digit_range((-10, -1))
            .with_output_dir("complex_system")
            .with_max_kp(0.9)
            .with_max_kd(0.9)
            .with_seed(0x2268a378740265f9)
            .build(),
        100,
    );
}

fn run_ga(dir: &str, mut ga: GeneticAlgorithm, max_generations: usize) {
    let dir = format!("output/{dir}");
    println!("Removing {} dir...", dir);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let file = File::create(format!("{dir}/log.txt")).unwrap();
    let file_err = File::create(format!("{dir}/err.txt")).unwrap();
    let _print_gag = Redirect::stdout(file).unwrap();
    let _print_err_gag = Redirect::stderr(file_err).unwrap();

    println!("Seed: {:#x}", ga.seed());

    println!("Generating initial population...");

    let mut best_individual = None;
    let generations = Time::continuous(1.0, max_generations as f32);

    for _ in generations {
        println!("Evolving generation {}", ga.generation());
        let Some(best) = ga.eval(0.75, 0.3) else {
            println!("No best individual found in this generation.");
            break;
        };

        best_individual = Some(best.clone());

        println!(
            "Generation {}:\n  Size: {}\n  Best PID = (kp: {:.10}, ki: {:.10}, kd: {:.10}) with fitness {:.10}",
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
            "Best individual found: PID = (kp: {:.10}, ki: {:.10}, kd: {:.10}) with fitness {:.10}",
            best.kp(),
            best.ki(),
            best.kd(),
            best.fitness()
        );
        best.show();

        let cmd = Command::new("python")
            .arg("plot.py")
            .arg(dir)
            .output()
            .unwrap();
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
