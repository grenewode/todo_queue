use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::hash::Hash;

use query::Query;
use range::Range;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Status {
    Waiting,
    Queuing,
    Working,
    Completed,
}

impl Default for Status {
    fn default() -> Self {
        Status::Waiting
    }
}

pub struct ItemDesc {
    pub name: String,
    pub description: String,
    pub status: Status,
}

pub trait Item {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_status(&self) -> &Status;
    fn set_name(&mut self, name: &str);
    fn set_description(&mut self, description: &str);
    fn set_status(&mut self, status: Status);
}

impl ItemDesc {
    pub fn new<N, D>(name: N, description: D) -> Self
    where
        N: Into<String>,
        D: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            status: Status::default(),
        }
    }
}

pub trait List {
    fn add(&mut self, item: ItemDesc) -> &mut Item;
    fn select(&self, Query) -> Vec<&Item>;
    fn select_mut(&mut self, Query) -> Vec<&mut Item>;
}
