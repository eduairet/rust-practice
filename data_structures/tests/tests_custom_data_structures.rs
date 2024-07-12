use data_structures::MyFlags;

#[cfg(test)]
mod tests_custom_data_structures {
    use super::*;

    #[test]
    fn test_my_flags() {
        let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
        let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
        assert_eq!((e1 | e2), MyFlags::FLAG_ABC);
        assert_eq!((e1 & e2), MyFlags::FLAG_C);
        assert_eq!((e1 - e2), MyFlags::FLAG_A);
        assert_eq!(!e2, MyFlags::FLAG_A);

        let mut flags = MyFlags::FLAG_ABC;

        let flags_assertion = "00000000000000000000000000000111";
        assert_eq!(format!("{}", flags), flags_assertion);

        let clear_flags_assertion = "00000000000000000000000000000000";
        assert_eq!(format!("{}", flags.clear()), clear_flags_assertion);

        let flag_b_assertion = "MyFlags(FLAG_B)";
        assert_eq!(format!("{:?}", MyFlags::FLAG_B), flag_b_assertion);

        let flag_a_b_assertion = "MyFlags(FLAG_A | FLAG_B)";
        assert_eq!(
            format!("{:?}", MyFlags::FLAG_A | MyFlags::FLAG_B),
            flag_a_b_assertion
        );
    }
}
