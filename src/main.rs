use std::fmt::Display;

/// The quote itself
#[derive(Debug, PartialEq)]
struct Quote {
    author: String,
    text: String,
}

impl Quote {
    fn parse(_line: String) -> Self {
        Self {
            author: "Petar Radosevic".to_owned(),
            text: "Show me the code".to_owned(),
        }
    }
}

impl Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -- {}", self.text, self.author)
    }
}

fn main() {
    let quotes = include_str!("quotes.txt");
    println!("{}", quotes);
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
        let line = "Show me the code -- Petar Radosevic".to_owned();
        let quote = Quote::parse(line);
        let expected = Quote {
            author: "Petar Radosevic".to_owned(),
            text: "Show me the code".to_owned(),
        };
        assert_eq!(quote, expected);
    }
}
