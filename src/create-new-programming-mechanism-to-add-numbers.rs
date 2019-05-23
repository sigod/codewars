macro_rules! add {
    ( $( $x:expr ),* ) =>  {
        {
            let mut temp = 0;
            $(
                temp += $x;
            )*
            temp
        }
    }
}

#[test]
fn test1() {
    assert_eq!(add![1, 2], 3);
}
