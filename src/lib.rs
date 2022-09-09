//! Each `CountingRatio` represents a set of observations. For example, if we observe 20
//! marbles removed from a jar, and we further observe that 12 of them are red, we can use a
//! `CountingRatio` to represent this collection of observations.
//!
//! Unlike conventional rational-number data types, because we are making observations we do
//! not reduce our numerator and denominator - we preserve them as originally counted.
//!
//! ```
//! use counting_ratio::CountingRatio;
//!
//! let mut observations = CountingRatio::new();
//!
//! for i in 0..100 {
//!     observations.observe(&i, |n| n % 7 == 0);
//! }
//!
//! assert_eq!(0.15, observations.into());
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
//!     observations.observe_with_prior(&s, |s| s.len() > 0, |s| s.contains("a"));
//! }
//!
//! assert_eq!(0.375, observations.into());
//! ```

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CountingRatio {
    matches: u64, observations: u64
}

impl CountingRatio {
    pub fn new() -> Self {CountingRatio { matches: 0, observations: 0}}

    pub fn observe<T, F: Fn(&T) -> bool>(&mut self, observation: &T, condition: F) {
        self.observations += 1;
        if condition(observation) {
            self.matches += 1;
        }
    }

    pub fn observe_with_prior<T, P: Fn(&T) -> bool, O: Fn(&T) -> bool>(&mut self, observation: &T,
                                                                       prior_condition: P,
                                                                       posterior_condition: O) {
        if prior_condition(observation) {
            self.observations += 1;
            if posterior_condition(observation) {
                self.matches += 1;
            }
        }
    }
}

impl From<CountingRatio> for f64 {
    fn from(cr: CountingRatio) -> Self {
        cr.matches as f64 / cr.observations as f64
    }
}