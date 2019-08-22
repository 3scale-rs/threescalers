use std::{
    convert::TryFrom,
    time::SystemTime,
};

/// A timestamp represented as a String. Meant to be useful just for representation in API calls.
/// If you need to operate on this, consider using instead SystemTime or DateTime and try_from() it.
#[derive(Clone, Debug)]
pub struct Timestamp {
    ts: String,
}

impl Timestamp {
    pub fn none() -> Option<&'static Timestamp> {
        None
    }

    pub fn new(ts: i64) -> Self {
        Self { ts: ts.to_string() }
    }

    pub fn now() -> Self {
        match Timestamp::try_from(SystemTime::now()) {
            Ok(ts) => ts,
            // blow up if we can't represent the current time (note: this actually panics)
            _ => unreachable!(),
        }
    }

    pub fn into_inner(self) -> String {
        self.ts
    }

    pub fn as_str(&self) -> &str {
        self.ts.as_str()
    }
}

impl AsRef<str> for Timestamp {
    fn as_ref(&self) -> &str {
        self.ts.as_str()
    }
}

impl From<i64> for Timestamp {
    fn from(ts: i64) -> Self {
        Self::new(ts)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

// We are interested in the number of seconds relative to the UNIX epoch for timestamps.
// Different platforms handle system timestamps in different ways, but we can handle all of them
// with an i64 in practical terms, which gives us more than enough time both before and after the
// UNIX epoch than what we could care about.
//
// This has to be a signed type to be able to represent dates before the UNIX epoch in exchange of
// not being able to represent dates on or after the year 292_277_026_596.
//
// Switch these types below to 128-bit width to avoid the Y292277026K596 problem.
type TimestampOffsetInt = i64;
// The type representing a positive number of seconds for a Duration (ie. the ret type of as_secs).
type AsSecsUnsignedInt = u64;

impl TryFrom<SystemTime> for Timestamp {
    // When this impl fails, the range we want is too far before UNIX_EPOCH or too far after that
    // it cannot be represented by SystemTime's internal representation. This error is typically
    // TryFromIntError.
    type Error = <TimestampOffsetInt as TryFrom<AsSecsUnsignedInt>>::Error;

    fn try_from(st: SystemTime) -> Result<Self, Self::Error> {
        let ts = match st.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(ts) => TimestampOffsetInt::try_from(ts.as_secs())?,
            // when an error is returned it contains the positive duration we'd need to add to st
            // to reach UNIX_EPOCH (ie. st is some time earlier than UNIX_EPOCH), so it is always
            // a positive value that can be negated safely if it fits the positive part of the
            // internal integer type (ie. signed ints can always represent all their positive values
            // as negative ones).
            Err(e) => {
                use std::ops::Neg;
                TimestampOffsetInt::try_from(e.duration().as_secs())?.neg()
            }
        };

        Ok(Self::new(ts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_SECS: isize = isize::max_value();
    const MIN_SECS: isize = isize::min_value();

    fn test_offset(offset: isize) -> SystemTime {
        use std::time::Duration;

        let offset = TimestampOffsetInt::try_from(offset);
        // barf if we cannot even represent the offset in a TimestampOffsetInt.
        assert!(offset.is_ok());
        let offset = offset.unwrap();

        let st = if offset < 0 {
            // Where isize is >= TimestampOffsetInt, there is no valid abs() for the minimum value,
            // so add 1 before computing it and then subtract 1 to the unsigned type, assuming the
            // unsigned int is at least the same width as TimestampOffsetInt. If it isn't, the test
            // will panic.
            let duration_secs = AsSecsUnsignedInt::try_from((offset + 1).abs()).unwrap() - 1;
            SystemTime::UNIX_EPOCH.checked_sub(Duration::from_secs(duration_secs))
        } else {
            let duration_secs = AsSecsUnsignedInt::try_from(offset).unwrap();
            SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(duration_secs))
        };
        // assert we have a SystemTime
        assert!(st.is_some());

        let st = st.unwrap();
        let ts = Timestamp::try_from(st);
        // assert we can represent this SystemTime
        assert!(ts.is_ok());

        st
    }

    #[test]
    fn try_from_unreasonable_future_system_time() {
        let st = test_offset(MAX_SECS);
        // errors are returned below _only_ if st is strictly later than the Epoch
        assert!(SystemTime::UNIX_EPOCH.duration_since(st).is_err());
    }

    #[test]
    fn try_from_unreasonable_past_system_time() {
        let st = test_offset(MIN_SECS);
        // Ok only if Epoch is >= than st
        assert!(SystemTime::UNIX_EPOCH.duration_since(st).is_ok());
    }
}
