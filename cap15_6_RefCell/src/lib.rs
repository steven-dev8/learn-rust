pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTrack<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTrack<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTrack<'a, T> {
        LimitTrack {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let porcetage_of_max = self.value as f64 / self.max as f64;

        if porcetage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if porcetage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if porcetage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn test_value_75_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_track = LimitTrack::new(&mock_messenger, 100);

        limit_track.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}




