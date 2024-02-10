use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Source {
    Pile(usize),
    Foundation(usize),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Destination {
    Pile(usize),
    Foundation(usize),
}

#[derive(Debug)]
struct Move {
    source: Option<Source>,
    destination: Option<Destination>,
}

impl Move {
    fn new() -> Self {
        Self {
            source: None,
            destination: None,
        }
    }

    fn source(&self, source: usize) -> Self {
        Self {
            source: Some(Source::Pile(source)),
            destination: self.destination,
        }
    }

    fn destination(&self, destination: usize) -> Self {
        Self {
            source: self.source,
            destination: Some(Destination::Pile(destination)),
        }
    }

    fn play(&self) {
    }
}
