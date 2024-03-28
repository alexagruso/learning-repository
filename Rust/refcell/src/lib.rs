use std::cell::RefCell;

pub trait Messanger {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: Messanger> {
    messanger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messanger,
{
    pub fn new(messanger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messanger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messanger.send("URGENT: YOUR QUOTA HAS BEEN REACHED");
        } else if percentage_of_max >= 0.9 {
            self.messanger
                .send("WARNING: YOU HAVE REACHED 90% OF YOUR QUOTA");
        } else if percentage_of_max >= 0.75 {
            self.messanger
                .send("WARNING: YOU HAVE REACHED 75% OF YOUR QUOTA");
        }
    }
}

pub struct MockMessanger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessanger {
    fn new() -> MockMessanger {
        MockMessanger {
            sent_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Messanger for MockMessanger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

#[cfg(test)]
mod tests {
    use crate::{LimitTracker, MockMessanger};

    #[test]
    fn blah() {
        let mock_messanger = MockMessanger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messanger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messanger.sent_messages.borrow().len(), 1);
    }
}
