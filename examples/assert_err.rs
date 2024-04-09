// FROM HERE
// https://stackoverflow.com/questions/53124930/how-do-you-test-for-a-specific-rust-error/53124931#53124931

#[allow(unused_imports)]
use url::{Url, Host, Position};

#[allow(dead_code)]
fn main() {}

#[derive(Debug)]
pub enum Error {
    UrlCreationFailed,
    CanonicalizationFailed(std::io::Error),
    FileOpenFailed(std::io::Error),
    UrlParsingFailed(url::ParseError),
}

pub fn your_func() -> Result<(), Error> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{your_func, Error};
    use std::io::ErrorKind;

    macro_rules! assert_err {
        ($expression:expr, $($pattern:tt)+) => {
            match $expression {
                $($pattern)+ => (),
                ref e => panic!("expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
            }
        }
    }

    #[test]
    fn test_failures() {
        // Few examples are here:
        assert_err!(your_func(), Err(Error::UrlParsingFailed(_)));
        assert_err!(your_func(), Err(Error::CanonicalizationFailed(_)));
        assert_err!(your_func(), Err(Error::FileOpenFailed(er)) if er.kind() == ErrorKind::NotFound);
    }
}


// cargo test --package rust_serde_json  --example  assert_err -- test --nocapture