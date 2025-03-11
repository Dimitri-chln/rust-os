use time::UtcDateTime;

#[repr(transparent)]
pub struct Time {
    inner: u32,
}

impl Time {
    pub fn date_time(&self) -> Result<UtcDateTime, time::error::ComponentRange> {
        UtcDateTime::from_unix_timestamp(self.inner as i64)
    }
}
