use std::convert;

pub const fn compose<X, Y, Z, F, G>(f: F, g: G) -> impl Fn(X) -> Z
where
    F: Fn(Y) -> Z,
    G: Fn(X) -> Y,
{
    move |x| f(g(x))
}

pub const fn id<T>(x: T) -> T {
    x
}

#[cfg(test)]
mod test {
    use crate::chapter1::{self, compose};

    #[test]
    fn sqrt() {
        std::convert::identity(1);
        let sqrt = |x: f32| x.sqrt();
        let square = |x: f32| x.powi(2);

        let comp = chapter1::compose(square, sqrt);

        for i in 1..100 {
            assert_eq!(comp(i as f32).round(), i as f32);
        }
    }

    #[test]
    fn id() {
        for i in 0..100 {
            assert_eq!(i, chapter1::id(i));
        }
    }
}
