// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Parse a time zone items
//!
//! The GNU docs state:
//!
//! > Normally, dates are interpreted using the rules of the current time zone,
//! > which in turn are specified by the TZ environment variable, or by a
//! > system default if TZ is not set. To specify a different set of default
//! > time zone rules that apply just to one date, start the date with a string
//! > of the form ‘TZ="rule"’. The two quote characters (‘"’) must be present
//! > in the date, and any quotes or backslashes within rule must be escaped by
//! > a backslash.
//!
//! Does not support POSIX rules with custom timezones

use winnow::{Parser, PResult};
use winnow::ascii::escaped_transform;
use winnow::combinator::{alt, delimited};
use winnow::token::take_till;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TimeZone(String);

pub fn parse(input: &mut &str) -> PResult<TimeZone> {
    delimited(
        "TZ=\"",
        escaped_transform(
            take_till(1.., ['\\', '"']),
            '\\',
            alt(("\\".value("\\"), "\"".value("\""))),
        ),
        "\"",
    )
    .map(TimeZone)
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::{parse, TimeZone};

    #[test]
    fn all() {
        for (s, tz) in [
            (r#"TZ="Europe/Amsterdam""#, "Europe/Amsterdam"),
            (r#"TZ="Americas/New_York""#, "Americas/New_York"),
            (r#"TZ="Some \"rule\" this is""#, "Some \"rule\" this is"),
        ] {
            let mut t = s;
            assert_eq!(
                parse(&mut t),
                Ok(TimeZone(tz.to_string())),
                "Failed string: {s}"
            )
        }
    }
}
