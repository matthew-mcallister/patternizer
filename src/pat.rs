use regex::Regex;
use smartstring::alias::String as SString;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Segment {
    /// Represents a stretch of raw text data up until the first
    /// instance of the following token occurs.
    Any,
    /// Represents an exact sequence of characters.
    Token(SString),
    /// Represents a string of 1 or more decimal digits.
    Decimal,
}

/// A rule for matching samples.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pat {
    segments: Vec<Segment>,
}

impl Pat {
    pub fn regex(&self) -> Regex {
        let parts: Vec<&str> = Default::default();
        for segment in self.segments.iter() {
            let part = match segment {
                Segment::Any => ".*?",
                Segment::Token(s) => &s,
                Segment::Decimal(s) => "\d+",
            };
            parts.push(part);
        }
        Regex::new(parts.concat())
    }

    /// Returns a measure of how precisely the pattern specifies what
    /// text it matches. For example, a pattern which matches all text
    /// has zero specificity, while a pattern that matches exactly one
    /// string has infinite specificity.
    ///
    /// When evaluating the fitness of a collection of patterns, each
    /// sample generates a reward proportional to the specificity of the
    /// most specific pattern which matches it. I.e. for a collection of
    /// patterns P,
    ///
    ///   C(P) ∝ max{specificity(pat) | pat ∈ P}.
    pub fn specificity(&self) -> f32 {
        unimplemented!();
    }
}

impl Default for Pat {
    fn default() -> Self {
        Pat { segments: vec![Segment::Any] }
    }
}

#[derive(Debug, Default)]
pub struct Solution {
    patterns: Vec<Pat>,
}
