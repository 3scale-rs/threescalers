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
// This has to be a signed type to be able to represent SystemTime dates before the UNIX epoch in
// exchange of not being able to represent dates on or after the year 292_277_026_596.
//
// On most platforms, Rust's SystemTime is internally represented as an unsigned int of 64 bits of
// width, but only 63 bits are actually usable because sign is taken into consideration for checked
// additions and subtractions. On some other platforms, most notably Windows, Rust's SystemTime can
// represent dates before the UNIX epoch. Because of this, we can use an i64 to (as of Rust 1.37)
// have all possible values of SystemTime translated into a (potentially negative) UNIX timestamp.
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
    pub (in self) use super::TimestampOffsetInt;
    use super::*;

    // The implementation of SystemTime has different properties according to the underlying platform.
    // This module declares a few const fns to help test the SystemTime conversion.
    mod systime {
        pub (in self) use super::TimestampOffsetInt;

        // A platform-specific module must define the following attributes of SystemTime:
        //
        // RESOLUTION: number of values in the TimestampOffsetInt type to represent one second.
        // SECONDS_TO_UNIX_EPOCH: Seconds to add (or subtract if negative) to the timestamp of 0.
        // UNSIGNED_VALUES: Set to 0 if negative values of i64 are acceptable, 1 otherwise.

        // SystemTime uses Windows' FILETIME structure as of 1.37.0.
        // This structure splits time in intervals, which are defined as 100ns, so the i64 space of
        // usable values is shrunk. Additionally, a value of 0 refers to 1601, so an offset must be
        // added so it becomes UNIX_EPOCH.
        #[cfg(windows)]
        mod internals {
            use super::TimestampOffsetInt;

            const NANOS_PER_SEC: u64 = 1_000_000_000;
            const INTERVALS_PER_SEC: u64 = NANOS_PER_SEC / 100;

            pub(in super) const RESOLUTION: TimestampOffsetInt = INTERVALS_PER_SEC as TimestampOffsetInt;
            pub(in super) const SECONDS_TO_UNIX_EPOCH: TimestampOffsetInt = 11_644_473_600; // from 1601 to 1970
            pub(in super) const UNSIGNED_VALUES: bool = false;
        }

        // Most platforms won't allow negative timestamps.
        #[cfg(not(windows))]
        mod internals {
            use super::TimestampOffsetInt;

            pub(in super) const RESOLUTION: TimestampOffsetInt = 1;
            pub(in super) const SECONDS_TO_UNIX_EPOCH: TimestampOffsetInt = 0;
            pub(in super) const UNSIGNED_VALUES: bool = true;
        }

        use internals::{RESOLUTION, SECONDS_TO_UNIX_EPOCH, UNSIGNED_VALUES};

        const MAX_0_UNIX_EPOCH_DIFFERENTIAL: TimestampOffsetInt =
            [0, SECONDS_TO_UNIX_EPOCH][(SECONDS_TO_UNIX_EPOCH > 0) as usize];
        const MIN_0_UNIX_EPOCH_DIFFERENTIAL: TimestampOffsetInt =
            [0, SECONDS_TO_UNIX_EPOCH][(SECONDS_TO_UNIX_EPOCH < 0) as usize];
        const MIN_HARD: TimestampOffsetInt =
            [TimestampOffsetInt::min_value(), 0][UNSIGNED_VALUES as usize];

        #[cfg(feature = "nightly")]
        const MAX_SECS: TimestampOffsetInt = (TimestampOffsetInt::max_value() / RESOLUTION)
            .saturating_sub(MAX_0_UNIX_EPOCH_DIFFERENTIAL);
        #[cfg(not(feature = "nightly"))]
        const MAX_SECS: TimestampOffsetInt = (TimestampOffsetInt::max_value() / RESOLUTION)
            - MAX_0_UNIX_EPOCH_DIFFERENTIAL;

        #[cfg(feature = "nightly")]
        const MIN_SECS: TimestampOffsetInt = (MIN_HARD / RESOLUTION)
            .saturating_add(MIN_0_UNIX_EPOCH_DIFFERENTIAL);
        #[cfg(not(feature = "nightly"))]
        const MIN_SECS: TimestampOffsetInt = (MIN_HARD / RESOLUTION) + MIN_0_UNIX_EPOCH_DIFFERENTIAL;

        pub const fn unix_epoch_offset_as_secs() -> TimestampOffsetInt {
            SECONDS_TO_UNIX_EPOCH
        }

        pub const fn min_secs() -> TimestampOffsetInt {
            MIN_SECS
        }

        pub const fn max_secs() -> TimestampOffsetInt {
            MAX_SECS
        }
    }

    fn test_offset(offset: TimestampOffsetInt) -> Option<SystemTime> {
        use std::time::Duration;

        println!("*** Offset {}", offset);
        let maybe_st = if offset < 0 {
            // With TimestampOffsetInt being a signed integer, the minimum value representable is
            // not representable with its unsigned counter-part, so abs() would fail for that value.
            // To avoid that we add 1 before computing it so it is representable, and then add 1 to
            // the unsigned type, assuming the unsigned type is at least the same width as
            // TimestampOffsetInt. If it isn't, the test will panic.
            let duration_secs = AsSecsUnsignedInt::try_from((offset + 1).abs()).unwrap() + 1;
            println!("Negative offset: {} - Duration: {}", offset, duration_secs);
            SystemTime::UNIX_EPOCH.checked_sub(Duration::from_secs(duration_secs))
        } else {
            let duration_secs = AsSecsUnsignedInt::try_from(offset).unwrap();
            println!("Positive offset: {} - Duration: {}", offset, duration_secs);
            SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(duration_secs))
        };
        // assert we have a SystemTime
        match maybe_st {
            Some(st) => {
                // assert we can represent this SystemTime
                assert!(Timestamp::try_from(st).is_ok());
            }
            None => ()
        };

        maybe_st
    }

    #[test]
    fn try_from_maximum_system_time() {
        let target_secs = AsSecsUnsignedInt::try_from(systime::max_secs().abs()).unwrap();
        let st = test_offset(systime::max_secs());

        assert!(st.is_some());
        let st = st.unwrap();

        let since_unix_epoch = st.duration_since(SystemTime::UNIX_EPOCH);
        // We only care about timestamps that can represent values after the UNIX epoch, so let's
        // require it right now and avoid treating the st <= UNIX_EPOCH case.
        let secs = match since_unix_epoch {
            Ok(duration) => {
                // UNIX_EPOCH is at or earlier than st
                assert!(systime::unix_epoch_offset_as_secs() >= 0);
                duration.as_secs()
            },
            Err(st_err) => {
                assert!(systime::unix_epoch_offset_as_secs() <= 0);
                st_err.duration().as_secs()
            }
        };

        assert_eq!(secs, target_secs);
        assert_eq!(TimestampOffsetInt::try_from(secs).unwrap(), systime::max_secs());
    }

    #[test]
    fn try_from_minimum_system_time() {
        let target_secs = AsSecsUnsignedInt::try_from((systime::min_secs() + 1).abs()).unwrap() + 1;
        let st = test_offset(systime::min_secs());

        assert!(st.is_some());
        let st = st.unwrap();

        let to_unix_epoch = SystemTime::UNIX_EPOCH.duration_since(st);
        // The result will have an Ok variant only if UNIX_EPOCH is >= than st, but that's a detail
        // of the implementation. In the case st _is_ UNIX_EPOCH, it could also return an Err
        // variant with a 0 duration.
        let secs = match to_unix_epoch {
            Ok(duration) => {
                // The minimum value st is at or earlier than UNIX_EPOCH
                assert!(systime::unix_epoch_offset_as_secs() >= 0);
                duration.as_secs()
            },
            Err(st_err) => {
                // The minimum value st is at or later than UNIX_EPOCH.
                // (it isn't clear from the docs that being exactly UNIX_EPOCH wouldn't return Err)
                assert!(systime::unix_epoch_offset_as_secs() <= 0);
                st_err.duration().as_secs()
            },
        };

        assert_eq!(secs, target_secs);
    }

    #[test]
    fn try_from_out_of_range_max_system_time() {
        let far_in_future = systime::max_secs().saturating_add(1);
        if far_in_future > systime::max_secs() {
            let st = test_offset(far_in_future);
            assert!(st.is_none());
        }
    }

    #[test]
    fn try_from_out_of_range_min_system_time() {
        let far_in_past = systime::min_secs().saturating_sub(1);
        if far_in_past < systime::min_secs() {
            let st = test_offset(far_in_past);
            assert!(st.is_none());
        }
    }
}
