/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    pub matcher: Box<Fn(T) -> bool>,
    pub subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F>(matcher: F, subs: &str) -> Matcher<T>
    where
        F: 'static + Fn(T) -> bool,
    {
        Matcher {
            matcher: Box::new(matcher),
            subs: String::from(subs),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    pub matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: std::fmt::Display + Copy,
{
    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an interator, returning a new iterator
    pub fn apply(self, iter: impl Iterator<Item = T>) -> impl Iterator<Item = String> {
        iter.map(move |e| {
            let mut output = String::new();
            for m in &self.matchers {
                if (&m.matcher)(e) {
                    output.push_str(&m.subs);
                }
            }
            if output.is_empty() {
                output = e.to_string();
            }
            output
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: std::ops::Rem + std::fmt::Display + From<u8> + PartialEq + Copy,
    <T as std::ops::Rem>::Output: PartialEq,
{
    Fizzy::new()
        .add_matcher(Matcher::new(
            |n: T| n % T::from(3) == T::from(0) % T::from(1),
            "fizz",
        ))
        .add_matcher(Matcher::new(
            |n: T| n % T::from(5) == T::from(0) % T::from(1),
            "buzz",
        ))
}
