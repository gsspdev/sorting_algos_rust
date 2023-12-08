mod bubblesort;
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
    use super::*;
    
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
    fn vecrng_make_works() {
        let things = crate::vecrng_make::vecrng_make(20, 100);
        assert_eq!(things.len(), 20);
        assert!(things[5] <= 100);
        println!("{:?}", things);
    }

    #[test]
    fn test_vecrng_make() {
        let test_vec = crate::vecrng_make::vecrng_make(50, 1000);
        let mut sorted_test_vec = test_vec.clone();
        sorted_test_vec.sort();
        println!("{:?}", sorted_test_vec);
    }
    
}
