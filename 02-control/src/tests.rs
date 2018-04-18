#![allow(dead_code)]

//!
//! INSTRUCTIONS: Try to use the three loop types for each exercise.
//! 


/// Computes product of numbers between `from` and `to` (both included !)
/// 
/// For example: `product_range(4, 6) = 4 x 5 x 6 = 120`
/// 
fn product_range(from_input: u32, to: u32) -> u32 {
    let mut result : u32 = 1 ;
    let mut from = from_input ;
    while from <= to {
      result *= from ;
      from += 1 ;
    }
    result
}

mod product_range_should {

    use super::product_range;

    #[test]
    fn return_2_when_from_2_to_2() {
        assert_eq!(2u32, product_range(2, 2));
    }

    #[test]
    fn return_120_when_from_4_to_6() {
        assert_eq!(120u32, product_range(4, 6));
    }

    #[test]
    fn return_720_when_from_2_to_6() {
        assert_eq!(720u32, product_range(2, 6));
    }

}

/// Returns last number before `0` and returns `0` if none.
/// 
/// For example: `last_non_zero([5, 6, 0, 1, 3]) = 6`
/// 
fn last_non_zero(numbers: Vec<u32>) -> u32 {
    let mut last = 0 ;
    for e in numbers {
      if e == 0 {
        break ;
      }
      last = e ;
    }
    last
}

mod last_non_zero_should {

    use super::last_non_zero;

    #[test]
    fn return_0_when_empty() {
        assert_eq!(0u32, last_non_zero(vec![]));
    }

    #[test]
    fn return_6_when_5_6_0_1_3() {
        assert_eq!(6u32, last_non_zero(vec![5, 6, 0, 1, 3]));
    }

}
