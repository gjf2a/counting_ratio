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
//! assert_eq!("15/100 (15.00%)", format!("{observations}").as_str());
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
//! assert_eq!("3/8 (37.50%)", format!("{observations}").as_str());
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
//! assert_eq!("20/120 (16.67%)", format!("{obs3}").as_str());
//! ```
//!
//! `CountingRatio` objects might also be created from other sources. This can 
//! be useful for comparing ratios without resorting to floating-point calculations, 
//! as `CountingRatio` objects implement `Ord` purely with integer arithmetic.
//! 
//! ```
//! use counting_ratio::CountingRatio;
//! 
//! let v1 = vec![1, 2, 3];
//! let v2 = vec![4, 5, 6, 7, 8, 9, 10];
//! let ratio1 = CountingRatio::ratio(v1.len() as u64, (v1.len() + v2.len()) as u64);
//! assert_eq!("3/10 (30.00%)", format!("{ratio1}").as_str());
//! 
//! let ratio2 = CountingRatio::ratio(3, 7);
//! assert!(ratio1 < ratio2);
//! ```
//! 
//! `CountingRatio` objects lend themselves to Bayesian calculations. 
//! 
//! ```
//! use counting_ratio::{CountingRatio, BayesianCounter};
//! 
//! let nums1 = vec![1, 3, 3, 5, 6, 7, 9, 11, 12, 13];
//! let nums2 = vec![0, 2, 3, 6, 8, 9];
//! let mut bayesian = BayesianCounter::new();
//! for num in nums1.iter() {
//!     bayesian.observe(*num, "One");
//! }
//! for num in nums2.iter() {
//!     bayesian.observe(*num, "Two");
//! }
//! 
//! assert_eq!(bayesian.label_count("One"), 10);
//! assert_eq!(bayesian.label_count("Two"), 6);
//! assert_eq!(bayesian.p_label("One"), CountingRatio::ratio(10, 16));
//! assert_eq!(bayesian.p_label("Two"), CountingRatio::ratio(6, 16));
//! assert_eq!(bayesian.p_example(1), CountingRatio::ratio(1, 16));
//! assert_eq!(bayesian.p_example(3), CountingRatio::ratio(3, 16));
//! assert_eq!(bayesian.p_example_given_label(3, "One"), CountingRatio::ratio(2, 10));
//! assert_eq!(bayesian.p_example_given_label(3, "Two"), CountingRatio::ratio(1, 6));
//! assert_eq!(bayesian.p_label_given_example("One", 3), CountingRatio::ratio(2 * 10 * 16, 10 * 16 * 3));
//! ```

use core::fmt::{Display, Formatter, Debug};
use core::ops::{Add, AddAssign, Mul, MulAssign, Div, DivAssign};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use trait_set::trait_set;
use histogram_macros::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord)]
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

impl Mul for CountingRatio {
    type Output = CountingRatio;

    fn mul(self, rhs: CountingRatio) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl MulAssign for CountingRatio {
    fn mul_assign(&mut self, rhs: CountingRatio) {
        self.matches *= rhs.matches;
        self.observations *= rhs.observations;
    }
}

impl Mul<u64> for CountingRatio {
    type Output = CountingRatio;

    fn mul(self, rhs: u64) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl Div for CountingRatio {
    type Output = CountingRatio;

    fn div(self, rhs: CountingRatio) -> Self::Output {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl DivAssign for CountingRatio {
    fn div_assign(&mut self, rhs: Self) {
        self.matches *= rhs.observations;
        self.observations *= rhs.matches;
    }
}

impl MulAssign<u64> for CountingRatio {
    fn mul_assign(&mut self, rhs: u64) {
        self.matches *= rhs;
    }
}

impl PartialOrd for CountingRatio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.matches == 0 && other.matches == 0 {
            Some(Ordering::Equal)
        } else if self.matches == 0 {
            Some(Ordering::Less)
        } else if other.matches == 0 {
            Some(Ordering::Greater)
        } else if self.observations == other.observations {
            self.observations.partial_cmp(&other.observations)
        } else {
            (self.matches * other.observations).partial_cmp(&(other.matches * self.observations))
        }
    }
}

trait_set! {
    pub trait Countable = Copy + Ord + Debug;
}

pub struct BayesianCounter<L: Countable, S: Countable> {
    counts: BTreeMap<L,BTreeMap<S,u64>>,
    total: u64
}

impl <L: Countable, S: Countable> BayesianCounter<L, S> {
    pub fn new() -> Self {
        Self {counts: BTreeMap::new(), total: 0}
    }

    pub fn observe(&mut self, example: S, label: L) {
        match self.counts.get_mut(&label) {
            Some(counter) => {
                bump!(counter, example);
            },
            None => {
                let mut counter = BTreeMap::new();
                bump!(counter, example);
                self.counts.insert(label, counter);
            },
        };
        self.total += 1;
    }

    pub fn count(&self, example: S, label: L) -> u64 {
        self.counts.get(&label).map_or(0, |t| t.get(&example).copied().unwrap_or(0))
    }

    pub fn label_count(&self, label: L) -> u64 {
        self.counts.get(&label).map_or(0, |t| t.values().sum())
    }

    pub fn example_count(&self, example: S) -> u64 {
        self.counts.keys().map(|label| self.count(example, *label)).sum()
    }

    pub fn p_label(&self, label: L) -> CountingRatio {
        CountingRatio::ratio(self.label_count(label), self.total)
    }

    pub fn p_example(&self, example: S) -> CountingRatio {
        CountingRatio::ratio(self.example_count(example), self.total)
    }

    pub fn p_example_given_label(&self, example: S, label: L) -> CountingRatio {
        CountingRatio::ratio(self.count(example, label), self.label_count(label))
    }

    pub fn p_label_given_example(&self, label: L, example: S) -> CountingRatio {
        self.p_example_given_label(example, label) * self.p_label(label) / self.p_example(example)
    }

    pub fn label_ranking_for(&self, example: S) -> Vec<L> {
        let mut result: Vec<(CountingRatio, L)> = self.counts.keys().map(|label| (self.p_example_given_label(example, *label) * self.label_count(*label), *label)).collect();
        result.sort();
        result.iter().map(|(_,label)| *label).collect()
    }
}