// Animal Shelter: An animal shelter, which holds only dogs and cats,
// operates on a strictly "first in, first out" basis. People must
// adopt either the "oldest" (based on arrival time) of all animals at
// the shelter, or they can select whether they would prefer a dog or
// a cat (and will receive the oldest animal of that type). They
// cannot select which specific animal they would like.

// Create the data structures to maintain this system and implement
// operations such as enqueue, dequeueAny, dequeueDog, and dequeueCat.
// You may use the built-in Linked List data structure.

use std::collections::LinkedList;
use std::time::{Duration, Instant};

struct Dog {
    arrival_time: Instant,
}
impl Dog {
    fn new() -> Self {
        Self {
            arrival_time: Instant::now(),
        }
    }
}

struct Cat {
    arrival_time: Instant,
}

impl Cat {
    fn new() -> Self {
        Self {
            arrival_time: Instant::now(),
        }
    }
}

struct AnimalShelter {
    dogs: LinkedList<Dog>,
    cats: LinkedList<Cat>,
}

impl AnimalShelter {
    fn new() -> Self {
        Self {
            dogs: LinkedList::new(),
            cats: LinkedList::new(),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_dog_and_cat() {
        let _ = Dog::new();
        let _ = Cat::new();
        assert!(true);
    }
}
