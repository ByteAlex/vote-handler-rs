use crate::constants::{VOTE_ENDPOINT, VOTE_AUTH_TOKEN, VOTE_RESEND_BULK_COUNT};
use crate::vote_cache::VoteCache;
use crate::vote_request::VoteRequest;
use serde::{Serialize, Deserialize};
use reqwest::Client;
use log::{info, debug, warn, error};
use std::time::SystemTime;

#[derive(Clone)]
pub struct VoteHandler {
    cache: VoteCache,
    http_client: Client,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct VoteResponse {
    status: String
}

impl VoteHandler {
    pub fn new() -> VoteHandler {
        return VoteHandler {
            cache: VoteCache::new(),
            http_client: reqwest::Client::builder()
                .http1_title_case_headers()
                .build()
                .unwrap(),
        };
    }

    pub async fn accept_vote_request(&mut self, auth: String, vote: VoteRequest) {
        let start = SystemTime::now();
        if !auth.eq(VOTE_AUTH_TOKEN.clone().as_str()) {
            warn!("Dropping unauthorized vote");
            return;
        }
        if !self.forward_vote(vote.clone()).await {
            warn!("Adding send-failed vote to cache!");
            self.cache.cache_failed_vote(vote.clone());
        }
        let elapsed_ms = start.elapsed()
            .map(|duration| { duration.as_millis() })
            .unwrap_or(0);
        info!("Processed vote request from {} in {}ms", vote.user.0.to_string().as_str(), elapsed_ms);
    }

    pub async fn resend_votes(&mut self) {
        debug!("Resending votes...");
        let start = SystemTime::now();
        let mut count: u32 = 0;
        while let Some(vote) = self.cache.poll() {
            if !self.forward_vote(vote.clone()).await {
                self.cache.return_failed_retry(vote);
                break;
            }
            count = count + 1;
            if count >= *VOTE_RESEND_BULK_COUNT {
                break;
            }
        }
        let elapsed_ms = start.elapsed()
            .map(|duration| { duration.as_millis() })
            .unwrap_or(0);
        info!("Done resending votes ({} / {} | in {}ms)", count, self.cache.size(), elapsed_ms);
    }

    pub async fn forward_vote(&self, vote: VoteRequest) -> bool {
        let start = SystemTime::now();
        let serialized_vote = serde_json::to_string(&vote).unwrap();
        let response = self.http_client.post(VOTE_ENDPOINT.clone().as_str())
            .header("Authorization", VOTE_AUTH_TOKEN.clone().as_str())
            .body(serialized_vote)
            .send()
            .await;
        if response.is_ok() {
            let response = response.unwrap();
            let response = response.text().await;
            if response.is_ok() {
                let body = response.unwrap().clone();
                let response = serde_json::from_str(body.as_str());
                if response.is_ok() {
                    let response: VoteResponse = response.unwrap();
                    debug!("Response Status: {}", response.status);
                    return response.status.eq("OK");
                } else {
                    error!("Serde: FAIL | body: {}", body);
                }
            } else {
                error!("Body: FAIL | ???");
            }
        } else {
            let elapsed_ms = start.elapsed()
                .map(|duration| { duration.as_millis() })
                .unwrap_or(0);
            warn!("Request to vote-endpoint failed after {}ms!", elapsed_ms)
        }
        return false;
    }
}