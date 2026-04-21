pub fn multiline_ret<S>(n: u8) -> bool {
    matches!(
        n, // multiline
        1
    )
}

pub fn multiline<S>(n: u8) {
    matches!(
        n, // multiline
        1
    );
}

#[test]
fn test() {
    use std::hint::black_box;

    black_box(multiline::<()>(black_box(1)));
    black_box(multiline::<()>(black_box(2)));
    black_box(multiline_ret::<()>(black_box(1)));
    black_box(multiline_ret::<()>(black_box(2)));
}
