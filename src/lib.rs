pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

mod bubblesort;
mod insertionsort;
mod selectionsort;

pub struct StdSorter;
impl<T> Sorter<T> for StdSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![5, 2, 3, 1, 4];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5])
    }
}
