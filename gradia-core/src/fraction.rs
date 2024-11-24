use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Fraction {
    numerator: isize,
    denominator: isize,
}

impl Fraction {
    pub fn new(number: f64) -> Self {
        fn convert(number: f64) -> (isize, isize) {
            const MAX_DENOMINATOR: isize = 1_000_000;

            if number == 0.0 {
                return (0, 1);
            }

            let mut best_numerator = 0;
            let mut best_denominator = 1;
            let mut best_diff = f64::MAX;

            for denominator in 1..=MAX_DENOMINATOR {
                let numerator = (number * denominator as f64).round() as isize;
                let fraction = numerator as f64 / denominator as f64;
                let diff = (number - fraction).abs();

                if diff < best_diff {
                    best_diff = diff;
                    best_numerator = numerator;
                    best_denominator = denominator;

                    if diff == 0.0 {
                        break;
                    }
                }
            }

            (best_numerator, best_denominator)
        }

        let result = convert(number);
        let mut frac = Fraction {
            numerator: result.0,
            denominator: result.1,
        };

        frac.simplify();
        frac
    }

    pub fn from(value: String) -> Option<Fraction> {
        let (numerator, denominator) = value.split_once("/")?;

        let mut fraction = Fraction {
            numerator: if let Ok(i) = numerator.trim().parse() {
                i
            } else {
                return None;
            },
            denominator: if let Ok(i) = denominator.trim().parse() {
                i
            } else {
                return None;
            },
        };
        fraction.simplify();
        Some(fraction)
    }

    pub fn display(&self) -> String {
        let mut selfs = self.clone();
        selfs.simplify();

        if selfs.denominator == 1 {
            selfs.numerator.to_string()
        } else {
            format!("{}/{}", selfs.numerator, selfs.denominator)
        }
    }

    // Function to simplify the fraction
    fn simplify(&mut self) {
        // Function to find the greatest common divisor (GCD) using Euclid's algorithm
        fn gcd(mut a: isize, mut b: isize) -> isize {
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        }

        let gcd = gcd(self.numerator.abs(), self.denominator.abs());
        self.numerator /= gcd;
        self.denominator /= gcd;
    }

    // Function to convert the fraction to a floating-point number
    pub fn to_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        let number = other;
        fn lcm(a: isize, b: isize) -> isize {
            let gcd = |mut a: isize, mut b: isize| {
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                a
            };

            (a * b) / gcd(a, b)
        }

        let denominator = lcm(self.denominator, number.denominator);
        let numerator1 = self.numerator * (denominator / self.denominator);
        let numerator2 = other.numerator * (denominator / other.denominator);

        let mut result = Fraction {
            numerator: numerator1 + numerator2,
            denominator,
        };
        result.simplify();
        result
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, other: Fraction) -> Fraction {
        let number = other;
        fn lcm(a: isize, b: isize) -> isize {
            let gcd = |mut a: isize, mut b: isize| {
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                a
            };

            (a * b) / gcd(a, b)
        }

        let denominator = lcm(self.denominator, number.denominator);
        let numerator1 = self.numerator * (denominator / self.denominator);
        let numerator2 = other.numerator * (denominator / other.denominator);

        let mut result = Fraction {
            numerator: numerator1 - numerator2,
            denominator,
        };
        result.simplify();
        result
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Fraction {
        let number = other;
        let mut result = Fraction {
            numerator: self.numerator * number.numerator,
            denominator: self.denominator * number.denominator,
        };
        result.simplify();
        result
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, other: Fraction) -> Fraction {
        let mut number = other;
        (number.denominator, number.numerator) = (number.numerator, number.denominator);

        let mut result = self * number;
        result.simplify();
        result
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        let mut selfs = self.clone();
        let mut other = other.clone();
        selfs.simplify();
        other.simplify();
        self.denominator == other.denominator && self.numerator == other.numerator
    }
}
