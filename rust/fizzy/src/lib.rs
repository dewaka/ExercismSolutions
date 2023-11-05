// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T> {
    pred: Box<dyn Fn(T) -> bool + 'a>,
    subs: String,
}

impl<'a, T> Matcher<'a, T> {
    pub fn new<P, S>(pred: P, subs: S) -> Matcher<'a, T>
    where
        P: Fn(T) -> bool + 'a,
        S: ToString,
    {
        Self {
            pred: Box::new(pred),
            subs: subs.to_string(),
        }
    }

    fn substitute(&self, val: T) -> Option<&str> {
        if (self.pred)(val) {
            Some(&self.subs)
        } else {
            None
        }
    }
}

pub struct Fizzy<'a, T> {
    matchers: Vec<Matcher<'a, T>>,
}

impl<'a, T> Fizzy<'a, T>
where
    T: ToString + Clone + 'a,
{
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String> + 'a
    where
        I: IntoIterator<Item = T> + 'a,
    {
        iter.into_iter().map(move |val| self.apply_val(val))
    }

    fn apply_val(&self, val: T) -> String {
        let subs: Vec<&str> = self
            .matchers
            .iter()
            .flat_map(|m| m.substitute(val.clone()))
            .collect();
        if subs.is_empty() {
            val.to_string()
        } else {
            subs.join("")
        }
    }
}

pub fn fizz_buzz<'a, T>() -> Fizzy<'a, T>
where
    T: Clone + PartialEq + std::ops::Rem<T, Output = T> + ToString + 'a,
    u8: Into<T>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
