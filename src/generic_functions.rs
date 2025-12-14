use std::cmp::Ordering;
use std::convert::From;
use std::format;

fn pick<T>(cond: bool, left: T, right: T) -> T {
    if cond { left } else { right }
}

fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

struct NotClonable;

pub trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;
impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}")
    }
}

struct VerbosityFilter<L> {
    max_verbosity: u8,
    inner: L,
}

impl<L: Logger> Logger for VerbosityFilter<L> {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

#[derive(Debug)]
struct Foo(String);

impl From<u32> for Foo {
    fn from(from: u32) -> Foo {
        Foo(format!("Converted from integer: {from}"))
    }
}

impl From<bool> for Foo {
    fn from(value: bool) -> Foo {
        Foo(format!("Converted from bool : {value}"))
    }
}

fn min<T: Ord>(l: T, r: T) -> T {
    match l.cmp(&r) {
        Ordering::Less | Ordering::Equal => l,
        Ordering::Greater => r,
    }
}

// Syntactic sugar for:
// fn add_42_millions<T: Into<i32>>(x: T) -> i32
fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
    x.into() + 42_000_000
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}

/// https://google.github.io/comprehensive-rust/generics/dyn-trait.html
struct Dog {
    name: String,
    age: i8,
}

struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

// Uses generics and static dispatch.
fn generic(pet: &impl Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

// Uses type-erasure and dynamic dispatch.
fn dynamic(pet: &dyn Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick() {
        println!("picked a number: {:?}", pick(true, 222, 333));
        println!("picked a char: {:?}", pick(false, 'L', 'R'));
    }

    #[test]
    fn test_trail_bounds() {
        let foo = String::from("foo");
        let pair = duplicate(foo);
        println!("{pair:?}");
    }

    #[test]
    fn test_generic_data_types() {
        let logger = VerbosityFilter {
            max_verbosity: 3,
            inner: StderrLogger,
        };

        logger.log(5, "FYI");
        logger.log(2, "Uhoh");
    }

    #[test]
    fn test_generic_traits() {
        let from_int = Foo::from(123);
        let from_bool = Foo::from(true);
        dbg!(from_int);
        dbg!(from_bool);
    }

    #[test]
    fn test_impl_trait() {
        let many = add_42_millions(42_i8);
        dbg!(many);
        let many_more = add_42_millions(10_000_000);
        dbg!(many_more);
        let debuggable = pair_of(27);
        dbg!(debuggable);
    }

    #[test]
    fn test_dyn_trait() {
        let cat = Cat { lives: 9 };
        let dog = Dog {
            name: String::from("Fido"),
            age: 5,
        };

        generic(&cat);
        generic(&dog);

        dynamic(&cat);
        dynamic(&dog);
    }

    #[test]
    fn integers() {
        assert_eq!(min(0, 10), 0);
        assert_eq!(min(500, 123), 123);
    }

    #[test]
    fn chars() {
        assert_eq!(min('a', 'z'), 'a');
        assert_eq!(min('7', '1'), '1');
    }

    #[test]
    fn strings() {
        assert_eq!(min("hello", "goodbye"), "goodbye");
        assert_eq!(min("bat", "armadillo"), "armadillo");
    }
}
