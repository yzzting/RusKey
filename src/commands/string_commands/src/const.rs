pub const EMPTY: &str = "nil";

pub enum Accumulation {
    Incr = 1,
    Decr = -1,
}

pub struct GetExExtraArgs {
    pub ex: Option<i64>,
    pub px: Option<i64>,
    pub exat: Option<i64>,
    pub pxat: Option<i64>,
    pub persist: Option<bool>,
}

pub struct ExtraArgs {
    pub ex: Option<i64>,
    pub px: Option<i64>,
    pub exat: Option<i64>,
    pub pxat: Option<i64>,
    pub nx: Option<bool>,
    pub xx: Option<bool>,
    pub keepttl: Option<bool>,
    pub get: Option<bool>,
}

pub enum SetError {
    InvalidExpiredTime,
    KeyOfValueNotSpecified,
}
