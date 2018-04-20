use ordered_iter::OrderedMapIterator;

pub struct IntoIter<I>
where
    I: IntoIterator,
{
    index: usize,
    inner: <I as IntoIterator>::IntoIter,
}

impl<I> IntoIter<I>
where
    I: IntoIterator,
{
    pub fn new(iter: I) -> Self {
        IntoIter { index: 0, inner: iter.into_iter() }
    }
}

impl<T, I> Iterator for IntoIter<I>
where
    I: IntoIterator<Item = T>
{
    type Item = (usize, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if let Some(value) = self.inner.next() {
            self.index += 1;
            Some((index, value))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<I> ExactSizeIterator for IntoIter<I>
where
    I: IntoIterator,
    <I as IntoIterator>::IntoIter: ExactSizeIterator
{
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T, I> OrderedMapIterator for IntoIter<I>
where
    I: IntoIterator<Item = T>,
{
    type Key = usize;
    type Val = T;
}

pub struct Iter<'a, T>
where
    T: 'a
{
    index: usize,
    inner: <&'a [T] as IntoIterator>::IntoIter,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(iter: &'a [T]) -> Self {
        Iter { index: 0, inner: iter.into_iter() }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Copy
{
    type Item = (usize, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|value| {
            let index = self.index;
            self.index += 1;
            (index, *value)
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T>
where
    T: Copy
{
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, T> OrderedMapIterator for Iter<'a, T>
where
    T: Copy
{
    type Key = usize;
    type Val = T;
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn into_iter() {
        let values = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let iter = IntoIter::new(values);
        let subject: Vec<_> = iter.collect();
        let expected = vec![(0, 0.1), (1, 0.2), (2, 0.3), (3, 0.4), (4, 0.5)];
        expect!(subject).to(be_equal_to(expected));
    }

    #[test]
    fn iter() {
        let values = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let iter = Iter::new(&values[..]);
        let subject: Vec<_> = iter.collect();
        let expected = vec![(0, 0.1), (1, 0.2), (2, 0.3), (3, 0.4), (4, 0.5)];
        expect!(subject).to(be_equal_to(expected));
    }
}
