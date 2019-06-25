use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct Timestamp {
    ts: String,
}

impl Timestamp {
    pub fn none() -> Option<&'static Timestamp> {
        None
    }

    pub fn new(ts: u64) -> Self {
        Self { ts: ts.to_string() }
    }

    pub fn into_inner(self) -> String {
        self.ts
    }

    pub fn as_str(&self) -> &str {
        self.ts.as_str()
    }
}

impl<'ts> AsRef<str> for Timestamp {
    fn as_ref(&self) -> &str {
        self.ts.as_str()
    }
}

impl From<u64> for Timestamp {
    fn from(ts: u64) -> Self {
        Self::new(ts)
    }
}

impl From<SystemTime> for Timestamp {
    fn from(st: SystemTime) -> Self {
        let ts = match st.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(ts) => ts.as_secs(),
            _ => 0,
        };

        Self::new(ts)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Timestamp::from(SystemTime::now())
    }
}
