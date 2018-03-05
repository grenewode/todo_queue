use std::ops::{Add, Mul};
use std::cmp::PartialOrd;

pub enum Limit<T> {
    Includes(T),
    Excludes(T),
    Inf,
}

impl<T> Limit<T> {
    pub fn is_lower_bound_of<O>(&self, v: &O) -> bool
    where
        O: PartialOrd<T>,
        T: PartialOrd<O>,
    {
        match *self {
            Limit::Excludes(ref low) => low < v,
            Limit::Includes(ref low) => low <= v,
            Limit::Inf => true,
        }
    }

    pub fn is_upper_bound_of<O>(&self, v: &O) -> bool
    where
        O: PartialOrd<T>,
        T: PartialOrd<O>,
    {
        match *self {
            Limit::Excludes(ref high) => high > v,
            Limit::Includes(ref high) => high >= v,
            Limit::Inf => true,
        }
    }

    pub fn is_lower_bound<O>(&self, other: &Limit<O>) -> bool
    where
        O: PartialOrd<T>,
        T: PartialOrd<O>,
    {
        use self::Limit::*;
        match (self, other) {
            (&Includes(ref low), &Includes(ref high))
            | (&Excludes(ref low), &Excludes(ref high))
            | (&Includes(ref low), &Excludes(ref high)) => low <= high,
            (&Excludes(ref low), &Includes(ref high)) => low < high,
            (&Inf, _) => true,
            (_, &Inf) => false,
        }
    }

    pub fn is_upper_bound<O>(&self, other: &Limit<O>) -> bool
    where
        O: PartialOrd<T>,
        T: PartialOrd<O>,
    {
        other.is_lower_bound(self)
    }
}

pub struct Range<T> {
    low: Limit<T>,
    high: Limit<T>,
}

impl<T> Range<T>
where
    T: PartialOrd,
{
    pub fn new(low: Limit<T>, high: Limit<T>) -> Self {
        Self { low, high }
    }

    pub fn contains(&self, v: &T) -> bool {
        self.low.is_lower_bound_of(v) && self.high.is_upper_bound_of(v)
    }

    pub fn is_subrange(&self, other: &Range<T>) -> bool {
        self.low.is_lower_bound(&other.low) && self.high.is_upper_bound(&other.high)
    }

    fn union(self, other: Range<T>) -> Self {
        let low = if self.low.is_lower_bound(&other.low) {
            self.low
        } else {
            other.low
        };
        let high = if self.high.is_upper_bound(&other.high) {
            self.high
        } else {
            other.high
        };

        Self { low, high }
    }

    fn intersection(self, other: Range<T>) -> Self {
        let low = if self.low.is_upper_bound(&other.low) {
            self.low
        } else {
            other.low
        };
        let high = if self.high.is_lower_bound(&other.high) {
            self.high
        } else {
            other.high
        };

        Self { low, high }
    }
}

impl<T> Add for Range<T>
where
    T: PartialOrd,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.intersection(rhs)
    }
}

impl<T> Mul for Range<T>
where
    T: PartialOrd,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}
