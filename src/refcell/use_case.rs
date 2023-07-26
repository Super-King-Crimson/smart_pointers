pub fn explain() {
    println!("I know this is probably obvious, but you can't mutate immutable stuff");

    let vec = vec![1, 2, 3, 4, 5];
    // vec.push(6);

    //pointers don't work either...
    // let vec_mut = &mut vec;
    
    //Sometimes we WANT to mutate immutable stuff tho
    //like if we wanted something to mutate itself in its methods
    //but otherwise be mutable (like readonly in a TS class)

    //Let's walk through one of those examples now
}

//Sometimes programmers use a type in place of another type during testing  
//this is to check specific behavior to make sure it's implemented correctly
    //this is called a test double, and it's like a stunt double for tests
//Mock objects are test doubles that returns what happens during a test,
//so we can ensure was supposed to happen actually happened

//Let's create a mock object, that tests our methods in a library
//this library will track a current value against a maximum value, 
//and send messages based on how close we are to the max
    //like how close we are to a rate limit of API calls

pub trait Messenger {
    //controls how exactly the message will be sent: email, text, call, print in file..
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    //lifetime req: the reference should still live at least as long as the struct 
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percent_of_max = value as f64 / self.max as f64;

        if percent_of_max >= 1.0 {
            self.messenger.send("You have surpassed your quota!");
        } else if percent_of_max >= 0.75 {
            self.messenger.send("You are approaching your quota!");
        } else if percent_of_max >= 0.50 {
            self.messenger.send("You are halfway to your quota!");
        }
        //note: we use the send method on messenger,  
        //which takes an immutable ref to (the messenger it)SELF, and the message
    }
    //Think for a second on how we'd test this method.
    //It doesn't return anything we can assert_eq! on, 
    //and all of its fields are private
}

//How about we make an object that impl's Messenger
//and make its method just... keep track of what it's told to send?

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    
    struct MockMessenger {
        // sent_messages: Vec<String>,
        
        //Use a RefCell instead for interior mutability
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(Vec::new())}
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            //this originally doesn't work: self is borrowed immutably with send
            //also, we CAN'T just change that to &mut self,
            //as then it wouldn't match the Messenger trait signature

            // self.sent_messages.push(msg.to_string());

            //...don't try changing the trait definition too,
            //that restricts your client's use requirements

            //To fix this, we can use a RefCell with interior mutability
            //we can borrow it mutably and push a string onto it, 
            //even though self is immutable
            self.sent_messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn limit_tracker_sends_over_50_percent_msg() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(
            &mock_messenger,
            100
        );

        limit_tracker.set_value(60);

        // assert!(mock_messenger.sent_messages.[0].contains("halfway"));
        assert!(mock_messenger.sent_messages.borrow()[0].contains("halfway"));
        //ok it works no
    }
}