#![allow(dead_code)]

use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
};

fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);

    println!("x : {}", interproduct(120, 100, 248));

    let n = 20;
    println!("fib({n}) = {}", fib(n));
    println!("print_fib({n}) = {}", print_fib(n));

    scope();
    if_expression();

    let size = if x < 20 { "small" } else { "large" };
    println!("number size : {}", size);

    match_expression();
    match_like_if();
    while_condition();
    for_statement();
    loop_statement();
    label_in_loop_statement();
    dbg!(gcd(143, 52));

    let n = 4;
    println!("{n} = {}", factorial(n));
    println!("Length: {}", collatz_length(11));
    array();
    tupple();
    array_iterator();

    let tuple = (1, 5, 3);
    println!(
        "{tuple:?}: {}",
        if check_order(tuple) {
            "ordered"
        } else {
            "unordered"
        }
    );

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("Original:");
    for row in &matrix {
        println!("{:?}", row);
    }

    let transposed = transpose(matrix);

    println!("\nTransposed:");
    for row in &transposed {
        println!("{:?}", row);
    }

    strings();

    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));

    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };

    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);

    let jackie = Person {
        name: String::from("Jackie"),
        ..avery
    };

    describe(&jackie);

    let p = Point(17, 23);
    println!("{},{}", p.0, p.1);

    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);

    let digest = compute_digest("Hello");
    println!("digest : {digest:?}");

    println!("{BANNER}");

    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!(
        "The car has arrived on the ground floor: {:?}",
        car_arrived(0)
    );
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}

/// https://google.github.io/comprehensive-rust/types-and-values/arithmetic.html
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

/// https://google.github.io/comprehensive-rust/types-and-values/inference.html
fn takes_u32(x: u32) {
    println!("u32: {x}");
}

/// https://google.github.io/comprehensive-rust/types-and-values/inference.html
fn takes_i8(y: i8) {
    println!("i8: {y}");
}

/// https://google.github.io/comprehensive-rust/types-and-values/exercise.html
fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

/// optimized of https://google.github.io/comprehensive-rust/types-and-values/solution.html
fn print_fib(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }

    a
}

/// https://google.github.io/comprehensive-rust/control-flow-basics/blocks-and-scopes.html
fn scope() {
    let z = 13;
    let x = {
        let y = 10;
        dbg!(y);
        z - y
    };

    dbg!(x);
}

/// https://google.github.io/comprehensive-rust/control-flow-basics/if.html
fn if_expression() {
    let x = 10;

    if x == 0 {
        print!("zero!");
    } else if x < 100 {
        println!("biggish");
    } else {
        print!("huge");
    }
}

/// https://google.github.io/comprehensive-rust/control-flow-basics/match.html
fn match_expression() {
    let val = 1;
    match val {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ => {
            println!("something else");
        }
    }
}

/// https://google.github.io/comprehensive-rust/control-flow-basics/match.html
fn match_like_if() {
    let flag = true;
    let val = match flag {
        true => 1,
        false => 0,
    };

    println!("The value of {flag} is {val}");
}

/// https://google.github.io/comprehensive-rust/control-flow-basics/loops.html
fn while_condition() {
    let mut x = 50;
    while x >= 10 {
        x = x / 2;
    }

    dbg!(x);
}

/// https://google.github.io/comprehensive-rust/control-flow-basics/loops/for.html
fn for_statement() {
    for x in 1..5 {
        dbg!(x);
    }

    for elem in [2, 4, 8, 16, 32] {
        dbg!(elem);
    }
}

/// https://google.github.io/comprehensive-rust/control-flow-basics/loops/loop.html
/// https://google.github.io/comprehensive-rust/control-flow-basics/break-continue.html
fn loop_statement() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 6 {
            break;
        }

        if i % 2 == 0 {
            continue;
        }

        dbg!(i);
    }
}

/// https://google.github.io/comprehensive-rust/control-flow-basics/break-continue/labels.html
fn label_in_loop_statement() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }

    dbg!(elements_searched);
}

/// https://google.github.io/comprehensive-rust/control-flow-basics/functions.html
fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 { gcd(b, a % b) } else { a }
}

/// https://google.github.io/comprehensive-rust/control-flow-basics/macros.html
fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

/// https://google.github.io/comprehensive-rust/control-flow-basics/exercise.html
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }

    len
}

/// https://google.github.io/comprehensive-rust/tuples-and-arrays/arrays.html
fn array() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[get_index()] = 0;
    println!("a: {a:?}");
}

fn get_index() -> usize {
    2
}

fn tupple() {
    let t: (i8, bool) = (7, true);
    dbg!(t.0);
    dbg!(t.1);
}

/// https://google.github.io/comprehensive-rust/tuples-and-arrays/iteration.html
fn array_iterator() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}

/// https://google.github.io/comprehensive-rust/tuples-and-arrays/destructuring.html
fn check_order(tuple: (i32, i32, i32)) -> bool {
    let (left, middle, right) = tuple;
    left < middle && middle < right
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }

    result
}

/// https://google.github.io/comprehensive-rust/references/strings.html
fn strings() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");

    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[2..9];
    println!("s3: {s3}");
}

fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut mag_squared = 0.0;
    for coord in vector {
        mag_squared += coord * coord;
    }

    mag_squared.sqrt()
}

/// Change the magnitude of the vector to 1.0 without changing its direction.
fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);
    for item in vector {
        *item /= mag;
    }
}

/// https://google.github.io/comprehensive-rust/user-defined-types/named-structs.html
struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

/// https://google.github.io/comprehensive-rust/user-defined-types/tuple-structs.html
struct Point(i32, i32);
struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    todo!("Ask a rocket scientiest at NASA")
}

fn set_thruster_force(force: Newtons) {
    todo!("Set the Thruster")
}

#[repr(u32)]
enum Bar {
    A,
    B = 1000,
    C,
}

macro_rules! dbg_bits {
    () => {};
}

enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

// Aliases are more usefull with long, complex types:
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;

const DIGEST_SIZE: usize = 3;
const FILL_VALUE: u8 = calculate_fill_value();

const fn calculate_fill_value() -> u8 {
    if DIGEST_SIZE < 10 { 42 } else { 13 }
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [FILL_VALUE; DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }

    digest
}

static BANNER: &str = "Welcome to RustOS 3.14";

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    /// A button was pressed.
    ButtonPressed(Button),

    /// The car has arrived at the given floor.
    CarArrived(Floor),

    /// The car's doors have opened.
    CarDoorOpened,

    /// The car's doors have closed.
    CarDoorClosed,
}

/// A floor is represented as an integer.
type Floor = i32;

#[derive(Debug)]
/// A direction of travel.
enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
enum Button {
    /// A button in the elevator lobby on the given floor.
    LobbyCall(Direction, Floor),

    /// A floor button within the car.
    CarFloor(Floor),
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

/// A floor buton was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}
