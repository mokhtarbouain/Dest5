

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 8 {
        eprintln!("Usage: {} <initial_position_x> <initial_position_y> <initial_velocity_x> <initial_velocity_y> <process_noise> <measurement_noise> <time_step>", args[0]);
        process::exit(1);
    }

    let initial_position_x: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid initial position x");
            process::exit(1);
        }
    };

    let initial_position_y: f64 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid initial position y");
            process::exit(1);
        }
    };

    let initial_velocity_x: f64 = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid initial velocity x");
            process::exit(1);
        }
    };

    let initial_velocity_y: f64 = match args[4].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid initial velocity y");
            process::exit(1);
        }
    };

    let process_noise: f64 = match args[5].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid process noise");
            process::exit(1);
        }
    };

    let measurement_noise: f64 = match args[6].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid measurement noise");
            process::exit(1);
        }
    };

    let time_step: f64 = match args[7].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid time step");
            process::exit(1);
        }
    };

    if time_step == 0.0 {
        eprintln!("Time step cannot be zero");
        process::exit(1);
    }

    if initial_position_x.is_nan() || initial_position_y.is_nan() || initial_velocity_x.is_nan() || initial_velocity_y.is_nan() {
        eprintln!("Initial position and velocity cannot be NaN");
        process::exit(1);
    }

    let mut state = [initial_position_x, initial_velocity_x, initial_position_y, initial_velocity_y];
    let mut covariance = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    let measurements = [
        [1.0, 1.0],
        [2.0, 2.0],
        [3.0, 3.0],
        [4.0, 4.0],
        [5.0, 5.0],
    ];

    for measurement in measurements.iter() {
        predict(&mut state, &mut covariance, process_noise, time_step);
        update(&mut state, &mut covariance, measurement, measurement_noise);
        println!("Estimated position: ({}, {})", state[0], state[2]);
    }
}

fn predict(state: &mut [f64; 4], covariance: &mut [[f64; 4]; 4], process_noise: f64, time_step: f64) {
    let f = [
        [1.0, time_step, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, time_step],
        [0.0, 0.0, 0.0, 1.0],
    ];

    let q = [
        [0.25 * time_step.powi(4), 0.5 * time_step.powi(3), 0.0, 0.0],
        [0.5 * time_step.powi(3), time_step.powi(2), 0.0, 0.0],
        [0.0, 0.0, 0.25 * time_step.powi(4), 0.5 * time_step.powi(3)],
        [0.0, 0.0, 0.5 * time_step.powi(3), time_step.powi(2)],
    ];

    let mut temp_state = [0.0; 4];
    let mut temp_covariance = [[0.0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            temp_state[i] += f[i][j] * state[j];
            for k in 0..4 {
                temp_covariance[i][j] += f[i][k] * covariance[k][j] * f[j][k];
            }
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            temp_covariance[i][j] += q[i][j];
        }
    }

    *state = temp_state;
    *covariance = temp_covariance;
}

fn update(state: &mut [f64; 4], covariance: &mut [[f64; 4]; 4], measurement: &[f64; 2], measurement_noise: f64) {
    let h = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ];

    let r = [
        [measurement_noise, 0.0],
        [0.0, measurement_noise],
    ];

    let mut s = [[0.0; 2]; 2];
    let mut k = [[0.0; 4]; 2];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..4 {
                s[i][j] += h[i][k] * covariance[k][j] * h[j][k];
            }
            s[i][j] += r[i][j];
        }
    }

    for i in 0..2 {
        for j in 0..4 {
            for k in 0..2 {
                k[i][j] += covariance[i][k] * h[k][j] * s[j][k];
            }
        }
    }

    let mut temp_state = [0.0; 4];
    let mut temp_covariance = [[0.0; 4]; 4];

    for i in 0..4 {
        temp_state[i] = state[i];
        for j in 0..2 {
            temp_state[i] += k[j][i] * (measurement[j] - h[j][i] * state[i]);
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            temp_covariance[i][j] = covariance[i][j];
            for k in 0..2 {
                temp_covariance[i][j] -= k[k][i] * h[k][j] * covariance[j][k];
            }
        }
    }

    *state = temp_state;
    *covariance = temp_covariance;
}