use std::{fmt::format, vec};

#[derive(Debug)]
struct CarRace {
    name: String,
    laps: Vec<i32>,
}

impl CarRace {
    // No receiver, a static method
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            laps: Vec::new(),
        }
    }

    // Exclusive borrowed read-write access to self
    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // Shared and read-only borrowed access to self
    fn print_laps(&self) {
        println!("Record {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    // Exclusive ownership of self (covered later)
    fn finish(self) -> i32 {
        self.laps.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::method_and_traits::{Animal, Dogs, Logger, Pets, Player, VerbosityFilter};
    use crate::method_and_traits::{CarRace, Meters, Multiply};
    use crate::method_and_traits::{Dog, Pet};

    #[test]
    fn test_car_race() {
        let mut race = CarRace::new("Monaco GP");

        assert_eq!(race.name, "Monaco GP");
        assert!(race.laps.is_empty());

        race.add_lap(72);
        race.add_lap(71);

        assert_eq!(race.laps.len(), 2);
        assert_eq!(race.laps, vec![72, 71]);

        race.print_laps();
        assert_eq!(race.laps, vec![72, 71]);

        let total = race.finish();
        assert_eq!(total, 72 + 71)
    }

    #[test]
    fn test_traits() {
        let fido = Dog {
            name: String::from("Fido"),
            age: 5,
        };

        let talk = fido.talk();
        assert_eq!(talk, "Woof, my name is Fido!");

        let age = fido.get_age();
        assert_eq!(age, fido.age);

        fido.greet();
    }

    #[test]
    fn test_super_traits() {
        let puppy = Dogs(String::from("Rex"));

        assert_eq!(puppy.name(), "Rex");
        assert_eq!(puppy.leg_count(), 4);
    }

    #[test]
    fn test_associated_types() {
        println!("{:?}", Meters(10).multiply(&Meters(20)));
    }

    #[test]
    fn test_deriving() {
        let p1: Player = Player::default();
        let mut p2 = p1.clone();
        p2.name = String::from("EldurScrollz");
        // Debug trait adds support for printing with `{:?}`.
        println!("{p1:?} vs. {p2:?}");
    }

    #[test]
    fn test_exercise() {
        let logger = VerbosityFilter {
            max_verbosity: 3,
            inner: super::StderrLogger,
        };

        logger.log(5, "FYI");
        logger.log(2, "Uhoh");
    }
}

trait Pet {
    /// Return a sentence from this pet.
    fn talk(&self) -> String;

    /// Print a string to the terminal greeting this pet.
    fn greet(&self);

    fn get_age(&self) -> i8;
}

struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }

    fn get_age(&self) -> i8 {
        self.age
    }

    fn greet(&self) {
        println!("Hallo mok")
    }
}

trait Animal {
    fn leg_count(&self) -> u32;
}

trait Pets: Animal {
    fn name(&self) -> String;
}

struct Dogs(String);

impl Animal for Dogs {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pets for Dogs {
    fn name(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug)]
struct Meters(i32);
#[derive(Debug)]
struct MetersSquared(i32);

trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
    type Output = MetersSquared;
    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}

#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Only log messages up to the given verbosity level.
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}
