#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn say_hello(name: String) {
    println!("Hello {name}")
}

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

#[derive(Debug)]
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Debug, Clone)]
struct Dependency {
    name: String,
    version_expression: String,
}

/// A representation of a software package.
#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    languange: Option<Language>,
}

impl Package {
    /// Return a representation of this package as a dependency, for use in
    /// building other packages.
    fn as_dependencies(&self) -> Dependency {
        Dependency {
            name: (self.name.clone()),
            version_expression: self.version.clone(),
        }
    }
}

/// A builder for a Package. Use `build()` to create the `Package` itself.
struct PackageBuilder(Package);

impl PackageBuilder {
    fn new(name: impl Into<String>) -> Self {
        Self(Package {
            name: name.into(),
            version: "0.1".into(),
            authors: Vec::new(),
            dependencies: Vec::new(),
            languange: None,
        })
    }

    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    fn authors(mut self, authors: Vec<String>) -> Self {
        self.0.authors = authors;
        self
    }

    fn dependency(mut self, depedency: Dependency) -> Self {
        self.0.dependencies.push(depedency);
        self
    }

    fn languange(mut self, lang: Language) -> Self {
        self.0.languange = Some((lang));
        self
    }

    fn build(self) -> Package {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::memory_management::{Droppable, Language, PackageBuilder, Point, say_hello};

    #[test]
    fn test_ownership() {
        {
            let p = Point(3, 4);
            dbg!(p.0);
        }
    }

    #[test]
    fn test_sematics() {
        let s1 = String::from("Hello!");
        let s2 = s1;
        dbg!(s2);
        // dbg!(s1);

        let name = String::from("Alice");
        say_hello(name);
        // say_hello(name);
    }

    #[test]
    fn test_clone() {
        let name = String::from("Alice");
        say_hello(name.clone());
        say_hello(name);
    }

    #[test]
    fn test_copy_types() {
        let p1 = Point(3, 4);
        let p2 = p1;
        println!("p1: {p1:?}");
        println!("p2: {p2:?}");
    }

    #[test]
    fn test_drop() {
        let a = Droppable { name: "a" };
        {
            let b = Droppable { name: "b" };
            {
                let c = Droppable { name: "c" };
                let d = Droppable { name: "d" };
                println!("Exiting innermost block");
            }
            println!("Exiting next block");
        }

        drop(a);
    }

    #[test]
    fn test_builder_type() {
        let base64 = PackageBuilder::new("base64").version("0.13").build();
        dbg!(&base64);

        let log = PackageBuilder::new("log")
            .version("0.4")
            .languange(Language::Rust)
            .build();
        dbg!(&log);

        let serde = PackageBuilder::new("serde")
            .authors(vec!["martin".into()])
            .version(String::from("4.0"))
            .dependency(base64.as_dependencies())
            .dependency(log.as_dependencies())
            .build();

        dbg!(&serde);
    }
}
