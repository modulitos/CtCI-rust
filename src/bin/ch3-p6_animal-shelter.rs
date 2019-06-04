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

#[derive(Debug)]
struct Dog {
    arrival: Instant,
}

impl Dog {
    fn new() -> Self {
        Self {
            arrival: Instant::now(),
        }
    }
}

#[derive(Debug)]
struct Cat {
    arrival: Instant,
}

impl Cat {
    fn new() -> Self {
        Self {
            arrival: Instant::now(),
        }
    }
}

#[derive(Debug)]
enum Animal {
    Dog(Dog),
    Cat(Cat),
}

struct AnimalShelter {
    dogs: LinkedList<Dog>,
    cats: LinkedList<Cat>,
    // NOTE: The current implementation is a bit overdone, but I
    // wanted to explore a robust solution. We could just do the
    // following and save a lot of troble:

    // dogs: LinkedList<Animal>,
    // cats: LinkedList<Animal>,

    // where:
    // enum Animal {
    //   Dog(Instant),
    //   Cat(Instant),
    // }
}

impl AnimalShelter {
    fn new() -> Self {
        Self {
            dogs: LinkedList::new(),
            cats: LinkedList::new(),
        }
    }

    fn enqueue(&mut self, animal: Animal) {
        match animal {
            Animal::Dog(animal) => self.dogs.push_back(animal),
            Animal::Cat(animal) => self.cats.push_back(animal),
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

    #[test]
    fn enqueue_dog_and_cat() {
        let mut shelter = AnimalShelter::new();
        let dog = Animal::Dog(Dog::new());
        let cat = Animal::Cat(Cat::new());
        shelter.enqueue(dog);
        shelter.enqueue(cat);
        assert!(true);
    }
}
