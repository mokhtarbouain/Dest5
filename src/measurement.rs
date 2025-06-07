

mod measurement {
    pub struct Measurement {
        x: f64,
        y: f64,
    }

    impl Measurement {
        pub fn new(x: f64, y: f64) -> Measurement {
            Measurement { x, y }
        }

        pub fn simulate_noisy_measurements() -> Vec<Measurement> {
            vec![
                Measurement::new(1.1, 0.9),
                Measurement::new(2.0, 2.1),
                Measurement::new(3.0, 3.1),
                Measurement::new(4.1, 4.0),
                Measurement::new(5.0, 5.2),
            ]
        }
    }
}