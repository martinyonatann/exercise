use core::prelude;

fn apply_and_log(
    func: impl FnOnce(&'static str) -> String,
    func_name: &'static str,
    input: &'static str,
) {
    println!("calling {func_name}({input}): {}", func(input))
}

pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

struct Filter<L, P> {
    inner: L,
    predicate: P,
}

impl<L, P> Filter<L, P>
where
    L: Logger,
    P: Fn(u8, &str) -> bool,
{
    fn new(inner: L, predicate: P) -> Self {
        Self { inner, predicate }
    }
}

impl<L, P> Logger for Filter<L, P>
where
    L: Logger,
    P: Fn(u8, &str) -> bool,
{
    fn log(&self, verbosity: u8, message: &str) {
        if (self.predicate)(verbosity, message) {
            self.inner.log(verbosity, message);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::closures::{Filter, Logger, StderrLogger, apply_and_log};

    #[test]
    fn test_closure_syntax() {
        // Argument and return type can be inferred for lighweight syntax:
        let double_it = |n| n * 2;
        dbg!(double_it(50));

        // Or we can specify types and bracket the body to be fully explicit:
        let add_1f32 = |x: f32| -> f32 { x + 1.0 };
        dbg!(add_1f32(50.));
    }

    #[test]
    fn test_capturing() {
        let max_value = 5;
        let clamp = |v| {
            if v > max_value { max_value } else { v }
        };

        dbg!(clamp(1));
        dbg!(clamp(3));
        dbg!(clamp(5));
        dbg!(clamp(7));
        dbg!(clamp(10));
    }

    #[test]
    fn test_closure_traits() {
        let suffix = "-itis";
        let add_suffix = |x: &'static str| format!("{x}{suffix}");

        apply_and_log(&add_suffix, "add_suffix", "senior");
        apply_and_log(&add_suffix, "add_suffix", "appendix");

        let mut v = Vec::new();
        let mut accumulate = |x| {
            v.push(x);
            v.join("/")
        };

        apply_and_log(&mut accumulate, "accumulate", "red");
        apply_and_log(&mut accumulate, "accumulate", "green");
        apply_and_log(&mut accumulate, "accumulate", "blue");

        let take_and_reverse = |prefix: &'static str| {
            let mut acc: String = String::from(prefix);
            acc.push_str(&v.into_iter().rev().collect::<Vec<_>>().join("/"));

            acc
        };

        apply_and_log(take_and_reverse, "take_and_reverse", "reversed: ");
    }

    #[test]
    fn test_log_filter() {
        let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
        logger.log(5, "FYI");
        logger.log(1, "yikes, something went wrong");
        logger.log(2, "uhoh");
    }
}
