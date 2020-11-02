use std::env::var;
use core::time::Duration;
use lazy_static::lazy_static;

lazy_static! {
    /**
    Endpoint to use for proxied requests
    */
    pub static ref VOTE_ENDPOINT: String = var("VOTE_ENDPOINT")
        .unwrap_or("http//api.server.dev/vote".to_owned());

    /**
    Authorization token provided in the Authorization header
    */
    pub static ref VOTE_AUTH_TOKEN: String = var("VOTE_AUTH_TOKEN")
        .unwrap_or("secret".to_owned());

    /**
    Delay in seconds between each resend execution, - executed per instance
    */
    pub static ref VOTE_RESEND_DELAY: Duration = Duration::from_secs(
        var("VOTE_RESEND_DELAY").unwrap_or("5".to_owned()).parse().unwrap_or(5));

    /**
    Max amount of resent votes per resend-execution
    */
    pub static ref VOTE_RESEND_BULK_COUNT: u32 = var("VOTE_RESEND_BULK_COUNT")
        .unwrap_or("100".to_owned())
        .parse()
        .unwrap_or(100);
}

pub static CACHE_TASK_OP_VOTE: u8 = 0;
pub static CACHE_TASK_OP_RESEND: u8 = 1;