use std::cmp::Ordering;

use range::Range;
use list::Item;
use list::Status;

pub enum Filter {
    Priority(Range<i32>),
    Status(Range<Status>),
    Tag(String),
    Name(String),
    And(Vec<Filter>),
    Or(Vec<Filter>),
    Not(Box<Filter>),
}

impl Filter {
    pub fn matches(&self, item: &Item) -> bool {
        use self::Filter::*;
        match *self {
            Priority(_) => unimplemented!(),
            Status(ref status) => status.contains(item.get_status()),
            Tag(_) => unimplemented!(),
            Name(ref name) => name == item.get_name(),
            And(ref all) => all.iter().all(|cond| cond.matches(item)),
            Or(ref any) => any.iter().any(|cond| cond.matches(item)),
            Not(ref cond) => !cond.matches(item),
        }
    }
}

pub enum SimpleSort {
    ByName,
    ByPriority,
    ByStatus,
}

impl SimpleSort {
    pub fn cmp(&self, a: &Item, b: &Item) -> Ordering {
        use self::SimpleSort::*;
        match *self {
            ByName => a.get_name().cmp(b.get_name()),
            ByStatus => a.get_status().cmp(b.get_status()),
            ByPriority => unimplemented!(),
        }
    }
}

pub struct Sort(Vec<SimpleSort>);

impl Sort {
    pub fn cmp(&self, a: &Item, b: &Item) -> Ordering {
        let Sort(ref sorts) = *self;

        sorts
            .iter()
            .map(|sort| sort.cmp(a, b))
            .fold(Ordering::Equal, |init, ord| init.then(ord))
    }
}

pub type Query = (Filter, Sort);
