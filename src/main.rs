use rand::seq::SliceRandom;
use std::fmt::Display;

/// The quote itself
#[derive(Debug, PartialEq)]
struct Quote {
    author: String,
    text: String,
}

impl Quote {
    fn parse(line: &str) -> Self {
        let mut parts = line.split("--");

        // TODO: It should still fail to build when the string is invalid
        // but the error message should be cleaner.
        let text = parts.next().unwrap().trim().to_owned();
        let author = parts.next().unwrap().trim().to_owned();

        Self { author, text }
    }
}

impl Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -- {}", self.text, self.author)
    }
}

fn main() {
    let raw = include_str!("quotes.txt");
    let quotes: Vec<Quote> = raw.lines().map(Quote::parse).collect();

    let mut rng = rand::thread_rng();
    let quote = quotes.choose(&mut rng).unwrap();

    println!("{}", quote);
}

#[cfg(test)]
mod tests {
    use crate::Quote;

    #[test]
    fn test_display_implementation_for_quote() {
        let quote = Quote {
            author: "Petar Radosevic".to_owned(),
            text: "Show me the code".to_owned(),
        };
        assert_eq!(quote.to_string(), "Show me the code -- Petar Radosevic");
    }

    #[test]
    fn test_parsing_a_string() {
        let line = "Show me the code -- Petar Radosevic";
        let quote = Quote::parse(line);
        let expected = Quote {
            author: "Petar Radosevic".to_owned(),
            text: "Show me the code".to_owned(),
        };
        assert_eq!(quote, expected);
    }
}
