// Ord trait が必用な反面、PartialOrd しか実装
//   されていないケースをどう扱うか

use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BinaryHeap;

    #[test]
    fn test_binary_heap_normal() {
        let temp_vec = vec![4., 2., 6., 2., 1.5, 10.3];
        let mut bh: BinaryHeap<Total<f64>> = BinaryHeap::new();
        for temp_vec_i in temp_vec {
            bh.push(Total(temp_vec_i));
        }
        assert_eq!(bh.peek(), Some(&Total(10.3)));
        bh.pop();
        assert_eq!(bh.peek(), Some(&Total(6.)));
    }

    #[test]
    #[should_panic]
    fn test_binary_heap_panic() {
        let temp_vec = vec![4., 2., 6., 2., 1.5, 10.3, f64::NAN];
        let mut bh: BinaryHeap<Total<f64>> = BinaryHeap::new();
        for temp_vec_i in temp_vec {
            bh.push(Total(temp_vec_i));
        }
        assert_eq!(bh.peek(), Some(&Total(10.3)));
    }
}
