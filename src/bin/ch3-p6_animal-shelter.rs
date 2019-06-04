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
#[derive(PartialEq)]
#[derive(Clone)]
enum Animal {
    Dog(Instant),
    Cat(Instant),
}

// NOTE: this approach is very verbose:
// #[derive(Sized)]
// trait Animal {}

struct AnimalShelter {
    // NOTE: We could have done the following, but it's a bit silly:
    // dogs: LinkedList<Dog>,
    // cats: LinkedList<Cat>,
    dogs: LinkedList<Animal>,
    cats: LinkedList<Animal>,
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
            Animal::Dog(dog) => self.dogs.push_back(Animal::Dog(dog)),
            Animal::Cat(cat) => self.cats.push_back(Animal::Cat(cat)),
        }
    }

    fn dequeue_dog(&mut self) -> Option<Animal> {
        self.dogs.pop_front()
    }

    fn dequeue_cat(&mut self) -> Option<Animal> {
        self.cats.pop_front()
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_dog_and_cat() {
        let _ = Animal::Dog(Instant::now());
        let _ = Animal::Cat(Instant::now());
        assert!(true);
    }

    #[test]
    fn enqueue_dog_and_cat() {
        let mut shelter = AnimalShelter::new();
        let dog = Animal::Dog(Instant::now());
        let cat = Animal::Cat(Instant::now());
        shelter.enqueue(dog);
        shelter.enqueue(cat);
        assert!(true);
    }

    #[test]
    fn dequeue_dog() {
        let mut shelter = AnimalShelter::new();
        let dog = Animal::Dog(Instant::now());
        let cat = Animal::Cat(Instant::now());
        shelter.enqueue(dog.clone());
        shelter.enqueue(cat.clone());
        assert_eq!(shelter.dequeue_dog(), Some(dog));
        assert_eq!(shelter.dequeue_cat(), Some(cat));
    }
}
