mod bubblesort;
mod mergesort;
mod vecrng_make;

pub trait Sorter {
    fn sort<T>(slice: &mut [T])
        where
            T: Ord;

}

pub fn sort<T, S>(slice: &mut [T])
    where 
        T: Ord, 
        S: Sorter, 
{
    S::sort(slice)
}

#[cfg(test)]
mod tests {
    // use core::slice::sort::mergesort;

    use super::*;
    use crate::vecrng_make::vecrng_make;
    use crate::mergesort::Mergesort;
    
    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord {
                slice.sort();
            }
    }
    #[test]
        fn std_works() {
            let mut things = vec![4, 2, 3, 1];
            sort::<_, StdSorter>(&mut things);
            assert_eq!(things, [1, 2, 3, 4]);
        }
    #[test]
    fn test_vecrng_make() {
        let test_vec = vecrng_make(50, 1000);
        let mut sorted_test_vec = test_vec.clone();
        sorted_test_vec.sort();
        println!("{:?}", sorted_test_vec);
    }

    #[test]
    fn test_mergesort() {
        let ms_test_vec = vecrng_make(50, 1000);
        let mut mergesorted_test_vec = ms_test_vec.clone();
        let printable_test_vec = ms_test_vec.clone();
        print!("{:?}", printable_test_vec);
        let result = Mergesort::mergesort(&mut mergesorted_test_vec);
        let mergesorted_test_vec = result;
        let printable_two = mergesorted_test_vec.clone();
        println!("{:?}", printable_two);
    }
}
