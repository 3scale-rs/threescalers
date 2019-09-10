use std::{
    convert::TryFrom,
    time::SystemTime,
};

// The implementation of SystemTime has different properties according to the underlying platform.
// This module declares MAX_SECS and MIN_SECS for usable values for an isize value.
mod systime {
    // A platform-specific module must define the following attributes of SystemTime:
    //
    // RESOLUTION: number of values in the isize type to represent one second.
    // SECONDS_TO_UNIX_EPOCH: Seconds to add (or subtract if negative) to the timestamp of 0isize.
    // UNSIGNED_VALUES: Set to 0 if negative values of isize are acceptable, 1 otherwise.

    // SystemTime uses Windows' FILETIME structure as of 1.37.0.
    // This structure splits time in intervals, which are defined as 100ns, so the i64 space of
    // usable values is shrunk. Additionally, a value of 0 refers to 1601, so an offset must be
    // added so it becomes UNIX_EPOCH.
    #[cfg(windows)]
    mod internals {
        const NANOS_PER_SEC: u64 = 1_000_000_000;
        const INTERVALS_PER_SEC: u64 = NANOS_PER_SEC / 100;

        pub(in super) const RESOLUTION: isize = INTERVALS_PER_SEC as isize;
        pub(in super) const SECONDS_TO_UNIX_EPOCH: isize = 11_644_473_600; // from 1601 to 1970
        pub(in super) const UNSIGNED_VALUES: isize = 0;
    }

    // Most platforms won't allow negative timestamps.
    #[cfg(not(windows))]
    mod internals {
        pub(in super) const RESOLUTION: isize = 1;
        pub(in super) const SECONDS_TO_UNIX_EPOCH: isize = 0;
        pub(in super) const UNSIGNED_VALUES: isize = 1;
    }

    use internals::{RESOLUTION, SECONDS_TO_UNIX_EPOCH, UNSIGNED_VALUES};

    const MAX_0_UNIX_EPOCH_DIFFERENTIAL: isize =
        [0, SECONDS_TO_UNIX_EPOCH][(SECONDS_TO_UNIX_EPOCH > 0) as usize];
    const MIN_0_UNIX_EPOCH_DIFFERENTIAL: isize =
        [0, SECONDS_TO_UNIX_EPOCH][(SECONDS_TO_UNIX_EPOCH < 0) as usize];
    const MIN_HARD: isize = [isize::min_value(), 0isize][(UNSIGNED_VALUES > 0) as usize];

    #[cfg(feature = "nightly")]
    pub const MAX_SECS: isize = (isize::max_value() / RESOLUTION)
        .saturating_sub(MAX_0_UNIX_EPOCH_DIFFERENTIAL);
    #[cfg(not(feature = "nightly"))]
    pub const MAX_SECS: isize = (isize::max_value() / RESOLUTION) - MAX_0_UNIX_EPOCH_DIFFERENTIAL;

    #[cfg(feature = "nightly")]
    pub const MIN_SECS: isize = (MIN_HARD / RESOLUTION)
        .saturating_add(MIN_0_UNIX_EPOCH_DIFFERENTIAL);
    #[cfg(not(feature = "nightly"))]
    pub const MIN_SECS: isize = (MIN_HARD / RESOLUTION) + MIN_0_UNIX_EPOCH_DIFFERENTIAL;
}

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

    pub const fn min_value() -> i64 {
        systime::MIN_SECS as i64
    }

    pub const fn max_value() -> i64 {
        systime::MAX_SECS as i64
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
pub type TimestampOffsetInt = i64;
// The type representing a positive number of seconds for a Duration (ie. the ret type of as_secs).
pub type AsSecsUnsignedInt = u64;

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

    fn test_offset(offset: isize) -> SystemTime {
        use std::time::Duration;
        println!("*** TESTING OFFSET {:?}", offset);

        let offset = TimestampOffsetInt::try_from(offset);
        // barf if we cannot even represent the offset in a TimestampOffsetInt.
        assert!(offset.is_ok());
        let offset = offset.unwrap();

        let st = if offset < 0 {
            // Where isize is >= TimestampOffsetInt, there is no valid abs() for the minimum value,
            // so add 1 before computing it and then subtract 1 to the unsigned type, assuming the
            // unsigned int is at least the same width as TimestampOffsetInt. If it isn't, the test
            // will panic.
            let duration_secs = AsSecsUnsignedInt::try_from((offset + 1).abs()).unwrap() + 1;
            println!("*** TESTING SUB DURATION_SECS {:?}", duration_secs);
            SystemTime::UNIX_EPOCH.checked_sub(Duration::from_secs(duration_secs))
        } else {
            let duration_secs = AsSecsUnsignedInt::try_from(offset).unwrap();
            println!("*** TESTING ADD DURATION_SECS {:?}", duration_secs);
            SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(duration_secs))
        };
        // assert we have a SystemTime
        assert!(st.is_some());
        println!("*** SYSTEMTIME created");

        let st = st.unwrap();
        let ts = Timestamp::try_from(st);
        // assert we can represent this SystemTime
        assert!(ts.is_ok());

        st
    }

    #[test]
    fn try_from_maximum_system_time() {
        let st = test_offset(Timestamp::max_value() as isize);
        let since = SystemTime::UNIX_EPOCH.duration_since(st);
        match since {
            Ok(secs) => println!("[F] Before UNIX: {:#?}", secs.as_secs()),
            Err(s) => println!("[F] After UNIX: {:#?}", s.duration().as_secs()),
        };
        // errors are returned below _only_ if st is strictly later than the Epoch
        assert!(SystemTime::UNIX_EPOCH.duration_since(st).is_err());
    }

    #[test]
    fn try_from_minimum_system_time() {
        let st = test_offset(Timestamp::min_value() as isize);
        // Ok only if Epoch is >= than st
        let since = SystemTime::UNIX_EPOCH.duration_since(st);
        match since {
            Ok(secs) => println!("[P] Before UNIX: {:#?}", secs.as_secs()),
            Err(s) => println!("[P] After UNIX: {:#?}", s.duration().as_secs()),
        };
        //assert!(since.is_ok());
    }
}
