/// Extensions for [Result](std::result::Result)
pub trait ResultExt<S, E> {
    /// As [Result::expect()], but with a callback to format the message
    fn expect_fmt<F>(self, format_fn: F) -> S where F: FnOnce(E) -> String;
}

impl<S, E> ResultExt<S, E> for Result<S, E> {
    /// As [expect](Result::expect()), but with a callback to format the message
    fn expect_fmt<F>(self, format_fn: F) -> S where F: FnOnce(E) -> String {
        match self {
            Ok(success) => success,
            Err(err) => panic!("{}", format_fn(err)),
        }
    }
}
