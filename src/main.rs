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
