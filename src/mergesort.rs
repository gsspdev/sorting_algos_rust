use super::Sorter;

#[Debug]
pub struct Mergesort;

impl Sorter for Mergesort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        // Implementation of the sort function
    }
}

impl Mergesort {
    pub fn mergesort<T>(slice: &mut [T]) {
        let middle = slice.len() / 2;

        fn split<T>(slice: &mut [T]) -> (Vec<T>, Vec<T>) {
            let left = slice[..middle].to_vec();
            let right = slice[middle..].to_vec();
            (left, right)
        }

        let (left, right) = split(slice);
        println!("left, {:?}", left);
        println!("right, {:?}", right);
    }
}