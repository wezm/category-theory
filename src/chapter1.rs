pub fn id<T>(x: T) -> T {
    x
}

pub fn compose<A, B, C, F: (Fn(A) -> B), G: (Fn(B) -> C)>(f1: F, f2: G) -> (impl Fn(A) -> C) {
    move |x| f2(f1(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add1(x: u32) -> u32 {
        x + 1
    }

    fn addref(x: &str) -> usize {
        x.len()
    }

    fn add2(x: u32) -> u32 {
        x + 2
    }

    fn to_string(x: u32) -> String {
        x.to_string()
    }

    fn to_ustring(x: usize) -> String {
        x.to_string()
    }

    #[test]
    fn compose_identity() {
        let f = compose(id, id);
        assert_eq!(f(1), 1);
    }

    #[test]
    fn compose_same() {
        let add3 = compose(add1, add2);
        assert_eq!(add3(1), 4);
    }

    #[test]
    fn compose_different() {
        let add_into_string = compose(add1, to_string);
        assert_eq!(add_into_string(1), String::from("2"));
    }

    #[test]
    fn compose_reference() {
        let len_into_string = compose(addref, to_ustring);
        assert_eq!(len_into_string("asdf"), String::from("4"));
    }
}
