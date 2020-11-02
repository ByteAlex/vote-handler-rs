use crate::vote_request::VoteRequest;
use crate::constants::{CACHE_TASK_OP_RESEND, CACHE_TASK_OP_VOTE};

pub struct CacheTask {
    pub op: i32,
    pub auth: Option<String>,
    pub vote: Option<VoteRequest>,
    pub msg: Option<String>,
}

impl CacheTask {
    pub fn create_vote_task(auth: String, vote: VoteRequest) -> CacheTask {
        return CacheTask {
            op: CACHE_TASK_OP_VOTE,
            auth: Some(auth),
            vote: Some(vote),
            msg: None,
        };
    }
    pub fn create_resend_task() -> CacheTask {
        return CacheTask {
            op: CACHE_TASK_OP_RESEND,
            auth: None,
            vote: None,
            msg: None,
        };
    }
}