use std::collections::VecDeque;
use crate::vote_request::VoteRequest;

#[derive(Clone)]
pub struct VoteCache {
    cache: VecDeque<VoteRequest>
}

impl VoteCache {
    pub fn new() -> VoteCache {
        return VoteCache {
            cache: VecDeque::new()
        };
    }

    pub fn cache_failed_vote(&mut self, vote: VoteRequest) {
        self.cache.push_back(vote);
    }

    pub fn return_failed_retry(&mut self, vote: VoteRequest) {
        self.cache.push_front(vote);
    }

    pub fn poll(&mut self) -> Option<VoteRequest> {
        return self.cache.pop_front();
    }

    pub fn size(&self) -> usize {
        self.cache.len()
    }
}