use crate::{value::dispatch_operation, Value};

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut left = *self;
        let mut right = *other;
        dispatch_operation!(left, right, l, |r| l.cmp(&r))
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
