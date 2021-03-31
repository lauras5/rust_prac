//! review iterators
pub trait IteratorExt: Iterator + Sized {
    fn our_flatten(self) -> Flatten<Self>
        where
            Self::Item: IntoIterator;
}

impl<T> IteratorExt for T
    where
        T: Iterator,
{
    fn our_flatten(self) -> Flatten<Self>
        where
            Self::Item: IntoIterator,
    {
        flatten(self)
    }
}

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
    where
        I: IntoIterator,
        I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>, // optional, may not have one
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
  O: Iterator,
  O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O:Iterator, // trait bounds
    O::Item: IntoIterator, // items of iter to impl into iter
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next() {
                    // if calling next gives us item return
                    return Some(i);
                }
                self.inner = None;
            }

            let next_inner_iter = self.outer.next()?.into_iter(); // call next on outer iter
            self.inner = Some(next_inner_iter); // store
        }
     }
}

impl <O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator, // bounds need to apply at every level
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next_back() {
                    return Some(i);
                }
                self.inner = None;
            }

            let next_inner_iter = self.outer.next_back()?.into_iter();
            self.inner = Some(next_inner_iter);
        }
    }
}

fn main() {
    // let vs = vec![1, 2, 3];
    // for v in vs {
    //     // consumes vs, owned v
    // }
    // for v in vs.iter() {
    //     // borrows vs, & to v
    // }
    // for v in &vs {
    //     // equivalent to vs.iter()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(),0)
    }

    #[test]
    fn one() { // one item
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1)
    }

    #[test]
    fn two() { // one item
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2)
    }

    #[test]
    fn two_wide() { // one item
        assert_eq!(flatten(vec!(vec!["a"], vec!["b"])).count(), 2)
    }

    #[test]
    fn reverse() { // one item
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
               .rev()
               .collect::<Vec<_>>(),
           vec!["b", "a"]
        );
    }
}