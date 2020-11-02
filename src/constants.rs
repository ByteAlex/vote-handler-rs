use std::env::var;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref VOTE_ENDPOINT: String = var("VOTE_ENDPOINT")
        .unwrap_or("http//api.server.dev/vote".to_owned());
    pub static ref VOTE_AUTH_TOKEN: String = var("VOTE_AUTH_TOKEN")
        .unwrap_or("secret".to_owned());
}

pub static CACHE_TASK_OP_VOTE: i32 = 0;
pub static CACHE_TASK_OP_RESEND: i32 = 1;