use memory_management::{show_access, PRIVILEGES};

#[cfg(test)]
mod tests_constants {
    use super::*;

    #[test]
    fn test_constants() {
        let access = PRIVILEGES.get("eduairet");
        println!("eduairet: {:?}", access);
        assert_eq!(show_access("eduairet"), (Some("eduairet"), access));

        let access = PRIVILEGES.get("plandgrave");
        println!("plandgrave: {:?}", access);
        assert_eq!(show_access("plandgrave"), (Some("plandgrave"), access));
    }
}
