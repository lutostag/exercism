/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    matcher: fn(T) -> bool,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new(matcher: fn(T) -> bool, subs: &str) -> Matcher<T> {
        Self {
            matcher,
            subs: String::from(subs),
        }
    }
}

/// A Fizzy is a list of matchers, which may be applied to an iterator.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: std::string::ToString + Clone,
{
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an interator, returning a new iterator
    pub fn apply(self, iter: impl Iterator<Item = T>) -> impl Iterator<Item = String> {
        iter.map(move |e| {
            let output: String = self
                .matchers
                .iter()
                .filter(|m| (m.matcher)(e.clone()))
                .map(|m| m.subs.clone())
                .collect();
            if output.is_empty() {
                return e.to_string();
            }
            output
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: std::ops::Rem<Output = T> + std::string::ToString + From<u8> + PartialEq + Clone,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
