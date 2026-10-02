#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Timestamp {
    seconds: i64,
    nanoseconds: u32,
}

impl Timestamp {
    pub fn new(seconds: i64, nanoseconds: u32) -> Self {
        Self { seconds, nanoseconds }
    }

    pub fn now() -> Self {
        let duration = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();

        Self {
            seconds: duration.as_secs() as i64,
            nanoseconds: duration.as_nanos() as u32,
        }
    }

    pub fn secs(self) -> i64 {
        self.seconds
    }

    pub fn nanos(self) -> u32 {
        self.nanoseconds
    }
}

impl From<std::time::SystemTime> for Timestamp {
    fn from(time: std::time::SystemTime) -> Self {
        let duration = time.duration_since(std::time::UNIX_EPOCH).unwrap();

        Self {
            seconds: duration.as_secs() as i64,
            nanoseconds: duration.as_nanos() as u32,
        }
    }
}

impl std::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", display(self.seconds), self.nanoseconds)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TimestampTz {
    time: Timestamp,
    offset: i32,
}

impl TimestampTz {
    pub fn new(time: Timestamp, offset: i32) -> Self {
        Self { time, offset }
    }

    pub fn now() -> Self {
        Self {
            time: Timestamp::now(),
            offset: 0,
        }
    }

    pub fn secs(self) -> i64 {
        self.time.secs()
    }

    pub fn nanos(self) -> u32 {
        self.time.nanos()
    }

    pub fn offset(self) -> i32 {
        self.offset
    }
}

impl std::fmt::Debug for TimestampTz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for TimestampTz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", display(self.time.seconds + self.offset as i64 * 60))
    }
}

fn display(seconds: i64) -> String {
    let days = seconds.div_euclid(86_400);
    let secs_of_day = seconds.rem_euclid(86_400);
    let hour = secs_of_day / 3600;
    let minute = (secs_of_day % 3600) / 60;
    let second = secs_of_day % 60;
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}")
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    // Howard Hinnant's civil calendar conversion.
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };

    year += if month <= 2 { 1 } else { 0 };
    (year, month, day)
}
