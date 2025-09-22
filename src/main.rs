use aule::prelude::*;
use ndarray::Array2;
use std::f32::consts::PI;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let kp = args
        .get(1)
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(1.0);
    let ki = args
        .get(2)
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);
    let kd = args
        .get(3)
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);

    let k = 1.0;
    let a = 1.0;
    let time = Time::from((1e-4, 2.0 * PI));

    let mut input = Sinusoid::new(1.0, 1.0 / (2.0 * PI), 0.0);
    let mut good_hart = GoodHart::new(0.3, 0.3, 0.4);
    let mut pid = PID::new(kp, ki, kd);
    let mut plant: SS<RK4> = SS::new(
        Array2::from_shape_vec((1, 1), vec![-a * k]).unwrap(),
        vec![k],
        vec![1.0],
        0.0,
    );

    for dt in time {
        let signal = dt >> input.as_input();
        let error = signal - plant.last_output();
        let control_signal = pid.as_siso() * error;
        let _output = control_signal * plant.as_siso();

        let _ = (error, control_signal) >> good_hart.as_error_metric();
    }

    println!("{}", good_hart.value());
}
