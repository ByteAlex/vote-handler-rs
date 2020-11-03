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
    Authorization token provided to the endpoint in Authorization header
    */
    pub static ref VOTE_ENDPOINT_AUTH_TOKEN: String = var("VOTE_ENDPOINT_AUTH_TOKEN")
        .unwrap_or(var("VOTE_AUTH_TOKEN").unwrap_or("secret".to_owned()));

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


    /**
    Authorization token provided in the Authorization header for vote/generic endpoint
    */
    pub static ref VOTE_AUTH_TOKEN: String = var("VOTE_AUTH_TOKEN")
        .unwrap_or("secret".to_owned());
    /**
    Authorization token provided in the Authorization header for vote/topgg endpoint
    */
    pub static ref VOTE_AUTH_TOKEN_TOPGG: String = var("VOTE_AUTH_TOKEN_TOPGG")
        .unwrap_or(VOTE_AUTH_TOKEN.clone());
    /**
    Authorization token provided in the Authorization header for vote/dbl endpoint
    */
    pub static ref VOTE_AUTH_TOKEN_DBL: String = var("VOTE_AUTH_TOKEN_DBL")
        .unwrap_or(VOTE_AUTH_TOKEN.clone());
    /**
    Authorization token provided in the Authorization header for vote/bfd endpoint
    */
    pub static ref VOTE_AUTH_TOKEN_BFD: String = var("VOTE_AUTH_TOKEN_BFD")
        .unwrap_or(VOTE_AUTH_TOKEN.clone());
    /**
    Authorization token provided in the Authorization header for vote/dboats endpoint
    */
    pub static ref VOTE_AUTH_TOKEN_DBOATS: String = var("VOTE_AUTH_TOKEN_DBOATS")
        .unwrap_or(VOTE_AUTH_TOKEN.clone());
}

pub const CACHE_TASK_OP_VOTE: u8 = 0;
pub const CACHE_TASK_OP_RESEND: u8 = 1;
pub const PAGE_KEY_TOPGG: &str = "topgg";
pub const PAGE_KEY_DBL: &str = "dbl";
pub const PAGE_KEY_BFD: &str = "bfd";
pub const PAGE_KEY_DBOATS: &str = "dboats";