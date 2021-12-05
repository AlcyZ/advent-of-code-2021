#[derive(Debug)]
struct ThreeMeasurement {
    measurements: Option<Vec<i32>>,
    sum: Option<i32>,
}

impl ThreeMeasurement {
    fn new() -> ThreeMeasurement {
        ThreeMeasurement {
            measurements: Some(vec![]),
            sum: None,
        }
    }

    fn add(&mut self, value: i32) {
        if let Some(elements) = &mut self.measurements {
            if elements.len() < 3 {
                elements.push(value);

                if elements.len() >= 3 {
                    let sum = elements.iter().sum();

                    self.sum = Some(sum);
                    self.measurements = None;
                }
            }
        }
    }
}

struct Measurement {
    measurements: Vec<ThreeMeasurement>,
}

impl Measurement {
    fn new() -> Measurement {
        Measurement {
            measurements: vec![],
        }
    }

    fn add(&mut self, value: i32) {
        let measurement = ThreeMeasurement::new();
        self.measurements.push(measurement);

        for measurement in self.measurements.iter_mut() {
            measurement.add(value);
        }
    }

    fn find_increases(self) -> i32 {
        let mut counter = 0;
        let mut previous = None;

        for measurement in self.measurements {
            if let Some(current) = measurement.sum {
                if let Some(previous) = previous {
                    if previous < current {
                        counter += 1;
                    }
                }

                previous = Some(current);
            }
        }

        counter
    }
}

pub(super) fn three_measurement(input: &Vec<i32>) -> i32 {
    let mut measurement = Measurement::new();

    for current in input {
        measurement.add(*current);
    }

    measurement.find_increases()
}
