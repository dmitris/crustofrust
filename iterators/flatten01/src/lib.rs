use std::iter::Iterator;

pub fn flatten<I>(iter: I) -> Flatten<I>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter)
}

pub struct Flatten<I>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    outer: I::IntoIter,
    inner: Option<<I::Item as IntoIterator>::IntoIter>,
}

impl<I> Flatten<I>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    fn new(v: I) -> Self {
        Flatten {
            outer: v.into_iter(),
            inner: None,
        }
    }
}

impl<I> Iterator for Flatten<I>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    type Item = <I::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        // check if self.inner is None which happens at the first call
        // if self.inner is None, assign self.outer.next() to self.inner
        if self.inner.is_none() {
            self.inner = Some(self.outer.next()?.into_iter());
        }

        loop {
            match self.inner {
                None => panic!("inner is None, should never happen"),
                Some(ref mut inner) => match inner.next() {
                    Some(e) => return Some(e),
                    None => match self.outer.next() {
                        None => return None, // all done - the end of the iteration
                        Some(it) => self.inner = Some(it.into_iter()),
                    }, // // more "cryptic" alternative:
                       // self.inner = Some(self.outer.next()?.into_iter()),
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_ints() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut flat_it = flatten(&v);
        // println!("{}", flatten.count());
        assert_eq!(Some(&1), flat_it.next());
        assert_eq!(Some(&2), flat_it.next());
        assert_eq!(Some(&3), flat_it.next());
        assert_eq!(Some(&4), flat_it.next());
        assert_eq!(None, flat_it.next());
        assert_eq!(None, flat_it.next()); // try to call it one more time after the iteration is completed

        // for el in flatten {
        //     println!("{:?}", el);
        // }
        flat_it = flatten(&v);
        let flat: Vec<&i32> = flat_it.collect();
        assert_eq!(vec![&1, &2, &3, &4], flat);
    }

    #[test]
    fn flatten_chars() {
        let vs = vec![vec!['a', 'b'], vec!['c']];
        let mut flat_it = flatten(&vs);
        // println!("{}", flatten.count());
        assert_eq!(Some(&'a'), flat_it.next());
        assert_eq!(Some(&'b'), flat_it.next());
        assert_eq!(Some(&'c'), flat_it.next());
        assert_eq!(None, flat_it.next());
    }

    #[test]
    fn flatten_empty_vec() {
        let v: Vec<Vec<i32>> = vec![];
        let mut flat_it = flatten(&v);
        assert_eq!(None, flat_it.next());
    }

    #[test]
    fn flatten_ints_with_empty() {
        let v = vec![vec![1, 2], vec![], vec![], vec![3, 4], vec![]];
        let mut flat_it = flatten(&v);
        // println!("{}", flatten.count());
        assert_eq!(Some(&1), flat_it.next());
        assert_eq!(Some(&2), flat_it.next());
        assert_eq!(Some(&3), flat_it.next());
        assert_eq!(Some(&4), flat_it.next());
        assert_eq!(None, flat_it.next());
        assert_eq!(None, flat_it.next()); // try to call it one more time after the iteration is completed

        // for el in flatten {
        //     println!("{:?}", el);
        // }
        flat_it = flatten(&v);
        let flat: Vec<&i32> = flat_it.collect();
        assert_eq!(vec![&1, &2, &3, &4], flat);
    }
}
