//! Each `CountingRatio` represents a set of observations. For example, if we observe 20
//! marbles removed from a jar, and we further observe that 12 of them are red, we can use a
//! `CountingRatio` to represent this collection of observations.
//!
//! Unlike conventional rational-number data types, because we are making observations we do
//! not reduce our numerator and denominator - we preserve them as originally counted.
//!
//! The implementation of the `Display` trait is intended to give both a summative (percentage)
//! view, as well as the raw counts.
//!
//! ```
//! use counting_ratio::CountingRatio;
//!
//! let mut observations = CountingRatio::new();
//!
//! for i in 0..100 {
//!     observations.observe(i % 7 == 0);
//! }
//!
//! assert_eq!(0.15, observations.into());
//! assert_eq!("15/100 (15.00%)", format!("{}", observations).as_str());
//! ```
//!
//! In more complex situations, it may be helpful to supply a prior condition as well as a
//! posterior condition. In this example, we want to count the number of strings that contain
//! a vowel, but we want to ignore empty strings entirely.
//!
//! The prior condition, then, is that the string's length is at least one, and the posterior
//! condition is that it contains the letter `a`.
//!
//! ```
//! use counting_ratio::CountingRatio;
//!
//! let mut observations = CountingRatio::new();
//!
//! for s in ["bear", "zoo", "spin", "", "tribe", "", "grip", "lion", "", "cobra", "ape"] {
//!     observations.observe_with_prior(s.len() > 0, s.contains("a"));
//! }
//!
//! assert_eq!(0.375, observations.into());
//! assert_eq!("3/8 (37.50%)", format!("{}", observations).as_str());
//! ```
//!
//! `CountingRatio` objects can also be added together. Because they represent counted observations,
//! the numerators and denominators are added together to produce the sum.
//!
//! ```
//! use counting_ratio::CountingRatio;
//!
//! let mut obs1 = CountingRatio::new();
//! for i in 0..100 {
//!     obs1.observe(i % 7 == 0);
//! }
//!
//! let mut obs2 = CountingRatio::new();
//! for i in 0..20 {
//!     obs2.observe(i % 4 == 0);
//! }
//!
//! let obs3 = obs1 + obs2;
//! assert_eq!(0.16666666666666666, obs3.into());
//! assert_eq!("20/120 (16.67%)", format!("{}", obs3).as_str());
//! ```
//!
//! `CountingRatio` objects might also be created from other sources:
//! 
//! ```
//! use counting_ratio::CountingRatio;
//! 
//! let v1 = vec![1, 2, 3];
//! let v2 = vec![4, 5, 6, 7, 8, 9, 10];
//! let ratio = CountingRatio::ratio(v1.len() as u64, (v1.len() + v2.len()) as u64);
//! assert_eq!("3/10 (30.00%)", format!("{ratio}").as_str());
//! ```

use core::fmt::{Display, Formatter};
use core::ops::{Add, AddAssign};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CountingRatio {
    matches: u64, observations: u64
}

impl CountingRatio {
    pub fn new() -> Self {CountingRatio { matches: 0, observations: 0}}

    pub fn ratio(matches: u64, observations:u64) -> Self {
        Self {matches, observations}
    }

    pub fn observe(&mut self, condition_met: bool) {
        self.observations += 1;
        if condition_met {
            self.matches += 1;
        }
    }

    pub fn observe_with_prior(&mut self, prior_condition_met: bool, posterior_condition_met: bool) {
        if prior_condition_met {
            self.observations += 1;
            if posterior_condition_met {
                self.matches += 1;
            }
        }
    }

    pub fn defined(&self) -> bool {
        self.observations > 0
    }
}

impl From<CountingRatio> for f64 {
    fn from(cr: CountingRatio) -> Self {
        cr.matches as f64 / cr.observations as f64
    }
}

impl Display for CountingRatio {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}/{} ({:.2}%)", self.matches, self.observations, 100.0 * f64::from(*self))
    }
}

impl Add for CountingRatio {
    type Output = CountingRatio;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl AddAssign for CountingRatio {
    fn add_assign(&mut self, rhs: Self) {
        self.matches += rhs.matches;
        self.observations += rhs.observations;
    }
}