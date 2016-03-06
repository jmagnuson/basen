
#[macro_use]
extern crate basen;

#[cfg(test)]
mod test {
    use basen::BaseN;

    #[test]
    fn test_convert_base() {

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

}
