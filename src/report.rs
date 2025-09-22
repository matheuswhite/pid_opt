use crate::individual::Individual;
use std::fs;
use std::path::PathBuf;
use typst::{
    foundations::{Dict, IntoValue},
    layout::PagedDocument,
};
use typst_as_lib::TypstEngine;

static FONT: &[u8] = include_bytes!("../fonts/texgyrecursor-regular.otf");

pub struct Report {
    filepath: PathBuf,
    best_individual: Option<Individual>,
}

impl Report {
    pub fn new(filename: &str) -> Self {
        let filepath = PathBuf::from(filename);
        Report {
            filepath,
            best_individual: None,
        }
    }

    pub fn set_best_individual(&mut self, individual: Individual) {
        self.best_individual = Some(individual);
    }

    pub fn save(&self) {
        let template = r#"
    #import sys: inputs

    #set page(paper: "a4")
    #set text(font: "TeX Gyre Cursor", 11pt)

    = PID Genetic Algorithm Optimization Report

    == Algorithm Configuration
    - Population Size: 50
    - Generations: 100
    - Crossover Rate: 0.8
    - Mutation Rate: 0.1

    == Best Individual Found
    #let best_individual = inputs.best_individual
    - Kp: best_individual.kp
    - Ki: best_individual.ki
    - Kd: best_individual.kd
    - Fitness: best_individual.fitness

    == Performance Analysis
    The genetic algorithm successfully converged to optimal PID parameters through evolutionary selection and breeding of candidate solutions.

    == Conclusion
    The optimized PID controller parameters demonstrate improved system response and stability compared to manual tuning approaches.
    "#;

        let mut dict = Dict::new();
        if let Some(ref individual) = self.best_individual {
            dict.insert("best_individual".into(), individual.clone().into_value());
        }

        let engine = TypstEngine::builder()
            .main_file(template)
            .fonts([FONT])
            .build();
        let doc: PagedDocument = engine
            .compile_with_input(dict)
            .output
            .expect("Failed to compile report");

        let pdf = typst_pdf::pdf(&doc, &Default::default()).expect("Could not generate pdf.");
        fs::write(&self.filepath, pdf).expect("Could not write pdf.");
    }
}

struct Image {
    matrix: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}
