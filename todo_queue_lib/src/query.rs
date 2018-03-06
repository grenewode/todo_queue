use std::cmp::Ordering;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};
use std::mem;

use range::Range;
use list::{Item, ItemId, List};
use list::Status;

#[derive(Debug, PartialEq, Eq)]
pub enum Filter {
    All,
    None,
    Status(Range<Status>),
    Tag(String),
    Name(String),
    And(Vec<Filter>),
    Or(Vec<Filter>),
    Not(Box<Filter>),
}

impl Filter {
    pub fn status<S: Into<Range<Status>>>(status: S) -> Self {
        Filter::Status(status.into())
    }

    pub fn tag<T: Into<String>>(tag: T) -> Self {
        Filter::Tag(tag.into())
    }

    pub fn name<T: Into<String>>(name: T) -> Self {
        Filter::Name(name.into())
    }
}

impl Not for Filter {
    type Output = Self;
    fn not(self) -> Self {
        Filter::Not(Box::new(self))
    }
}

impl BitAndAssign for Filter {
    fn bitand_assign(&mut self, other: Filter) {
        use self::Filter::*;

        if let And(ref mut me) = *self {
            if let And(other) = other {
                me.extend(other);
            } else {
                me.push(other);
            }
        } else {
            let old = mem::replace(self, And(vec![]));
            *self &= old;
            *self &= other;
        }
    }
}

impl BitAnd for Filter {
    type Output = Filter;

    fn bitand(self, other: Filter) -> Filter {
        let mut s = self;
        s &= other;
        s
    }
}

impl BitOrAssign for Filter {
    fn bitor_assign(&mut self, other: Filter) {
        use self::Filter::*;

        if let Or(ref mut me) = *self {
            if let Or(other) = other {
                me.extend(other);
            } else {
                me.push(other);
            }
        } else {
            let old = mem::replace(self, Or(vec![]));
            *self |= old;
            *self |= other;
        }
    }
}

impl BitOr for Filter {
    type Output = Filter;

    fn bitor(self, other: Filter) -> Filter {
        let mut s = self;
        s |= other;
        s
    }
}

impl Filter {
    pub fn matches(&self, item: &Item) -> bool {
        use self::Filter::*;
        match *self {
            All => true,
            None => false,
            Status(ref status) => status.contains(item.get_status()),
            Tag(ref tag) => item.has_tag(tag),
            Name(ref name) => name == item.get_name(),
            And(ref all) => all.iter().all(|cond| cond.matches(item)),
            Or(ref any) => any.iter().any(|cond| cond.matches(item)),
            Not(ref cond) => !cond.matches(item),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Query {
    joins: Vec<Filter>,
}

impl From<Filter> for Query {
    fn from(filter: Filter) -> Query {
        Query {
            joins: vec![filter],
        }
    }
}

impl Query {
    pub fn then(self, filter: Filter) -> Self {
        let Self { mut joins } = self;
        joins.push(filter);
        Self { joins }
    }

    pub fn select<L>(&self, list: &L) -> Vec<ItemId>
    where
        L: List,
    {
        self.joins
            .iter()
            .flat_map(|filter| list.select(filter))
            .collect()
    }
}
