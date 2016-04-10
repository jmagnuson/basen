
#[macro_use]
extern crate basen;

#[cfg(test)]
mod test {
    use basen::BaseN;

    #[test]
    fn test_to_base() {

        let expected: BaseN = BaseN::with_existing(
            10,
            vec![9, 6, 3, 4]
        );
        let x: BaseN = BaseN::with_existing(
            16,
            vec![1, 1, 1, 1]
        );
        
        assert_eq!( expected, x.to_base(10).unwrap() );
    }

    #[test]
    fn test_set_base() {

        let expected: BaseN = BaseN::with_existing(
            10,
            vec![9, 6, 3, 4]
        );
        let mut x: BaseN = BaseN::with_existing(
            16,
            vec![1, 1, 1, 1]
        );

        x.set_base(10);

        assert_eq!( expected, x );
    }

    #[test]
    fn test_from_usize() {

        let val10 = 4369;
        let new_base = 16;

        let expected: BaseN = BaseN::with_existing(
            16,
            vec![1, 1, 1, 1]
        );
        
        assert_eq!( expected, BaseN::from_usize(new_base, val10).unwrap() );
    }

    #[test]
    fn test_add_same_base() {

        let expected: BaseN = BaseN::with_existing(
            10,
            vec![3, 3, 3, 3]
        );
        let x: BaseN = BaseN::with_existing(
            10,
            vec![1, 1, 1, 1]
        );
        let y: BaseN = BaseN::with_existing(
            10,
            vec![2, 2, 2, 2]
        );
        
        let z: BaseN = (x + y).unwrap();

        //assert!(true);
        assert_eq!( expected, z );
    }

    #[test]
    fn test_add_different_base() {

        let expected: BaseN = BaseN::with_existing(
            16,
            vec![15, 11, 9, 1]
        );
        let x: BaseN = BaseN::with_existing(
            16,
            vec![1, 1, 1, 1]
        );
        let y: BaseN = BaseN::with_existing(
            10,
            vec![2, 2, 2, 2]
        );
        
        let z: BaseN = (x + y.to_base(16).unwrap()).unwrap();

        //assert!(true);
        assert_eq!( expected, z );
    }

    #[test]
    fn test_sub_same_base() {

        let x: BaseN = BaseN::with_existing(
            10,
            vec![3, 3, 3, 3]
        );
        let y: BaseN = BaseN::with_existing(
            10,
            vec![1, 1, 1, 1]
        );
        let expected: BaseN = BaseN::with_existing(
            10,
            vec![2, 2, 2, 2]
        );
        
        let z: BaseN = (x - y).unwrap();

        assert_eq!( expected, z );
    }

    #[test]
    fn test_sub_different_base() {

        let x: BaseN = BaseN::with_existing(
            16,
            vec![15, 11, 9, 1]
        );
        let expected: BaseN = BaseN::with_existing(
            16,
            vec![1, 1, 1, 1]
        );
        let y: BaseN = BaseN::with_existing(
            10,
            vec![2, 2, 2, 2]
        );
        
        let z: BaseN = (x - y.to_base(16).unwrap()).unwrap();

        assert_eq!( expected, z );
    }

    #[test]
    fn test_mul_same_base() {

        let x: BaseN = BaseN::with_existing(
            10,
            vec![3, 3, 3, 3]
        );
        let y: BaseN = BaseN::with_existing(
            10,
            vec![1, 1, 1, 1]
        );
        let expected: BaseN = BaseN::with_existing(
            10,
            vec![3, 6, 9, 2, 0, 7, 3]
        );

        let z: BaseN = (x * y).unwrap();

        assert_eq!( expected, z );
    }

    #[test]
    fn test_mul_different_base() {

        let x: BaseN = BaseN::with_existing(
            16,
            vec![15, 11, 9, 1]
        );
        let expected: BaseN = BaseN::with_existing(
            16,
            vec![2, 13, 7, 7, 15, 13]
        );
        let y: BaseN = BaseN::with_existing(
            10,
            vec![2, 2, 2, 2]
        );

        let z: BaseN = (x * y).unwrap();

        assert_eq!( expected, z );
    }

    #[test]
    fn test_div_same_base() {

        let expected: BaseN = BaseN::with_existing(
            10,
            vec![3, 3, 3, 3]
        );
        let y: BaseN = BaseN::with_existing(
            10,
            vec![1, 1, 1, 1]
        );
        let x: BaseN = BaseN::with_existing(
            10,
            vec![3, 6, 9, 2, 0, 7, 3]
        );

        let z: BaseN = (x / y).unwrap();

        assert_eq!( expected, z );
    }

    #[test]
    fn test_div_different_base() {

        let expected: BaseN = BaseN::with_existing(
            16,
            vec![15, 11, 9, 1]
        );
        let x: BaseN = BaseN::with_existing(
            16,
            vec![2, 13, 7, 7, 15, 13]
        );
        let y: BaseN = BaseN::with_existing(
            10,
            vec![2, 2, 2, 2]
        );

        let z: BaseN = (x / y).unwrap();

        assert_eq!( expected, z );
    }

    #[test]
    fn test_eq_same_base() {

        let x: BaseN = BaseN::with_existing(
            16,
            vec![3, 3, 3, 3]
        );
        let y: BaseN = BaseN::with_existing(
            16,
            vec![3, 3, 3, 3]
        );

        assert_eq!( x==y, true );
    }

    #[test]
    fn test_eq_different_base() {

        let x: BaseN = BaseN::with_existing(
            10,
            vec![3, 3, 3, 3]
        );
        let y: BaseN = BaseN::with_existing(
            16,
            vec![5, 0, 13]
        );

        assert_eq!( x==y, true );

    }

    #[test]
    fn test_ne_same_base() {

        let x: BaseN = BaseN::with_existing(
            16,
            vec![3, 3, 3, 3]
        );
        let y: BaseN = BaseN::with_existing(
            16,
            vec![4, 4, 4, 4]
        );

        assert_eq!( x!=y, true );
    }

    #[test]
    fn test_ne_different_base() {

        let x: BaseN = BaseN::with_existing(
            10,
            vec![3, 3, 3, 3]
        );
        let y: BaseN = BaseN::with_existing(
            16,
            vec![6, 1, 14]
        );

        assert_eq!( x!=y, true );

    }

    #[test]
    fn test_gt_same_base() {

        let x: BaseN = BaseN::with_existing(
            16,
            vec![3, 3, 3, 3]
        );
        let y: BaseN = BaseN::with_existing(
            16,
            vec![4, 4, 4, 4]
        );

        assert_eq!( x>y, false );
    }

    #[test]
    fn test_gt_different_base() {

        let x: BaseN = BaseN::with_existing(
            10,
            vec![3, 3, 3, 3]
        );
        let y: BaseN = BaseN::with_existing(
            16,
            vec![6, 1, 14]
        );

        assert_eq!( x>y, false );

    }


}
