use crate::vote_handler::VoteHandler;
use warp::Filter;
use crate::vote_request::VoteRequest;
use crate::cache_task::CacheTask;
use crate::constants::{CACHE_TASK_OP_VOTE, CACHE_TASK_OP_RESEND, VOTE_AUTH_TOKEN};
use warp::http::StatusCode;
use tokio::sync::mpsc::Sender;
use log::{info, debug, warn};

mod snowflake;
mod vote_request;
mod constants;
mod vote_cache;
mod vote_handler;
mod cache_task;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting vote-handler using proxy url {}", constants::VOTE_ENDPOINT.clone().as_str());
    let (tx, mut rx) = tokio::sync::mpsc::channel(128);

    let rest_tx = tx.clone();
    let vote = warp::path("vote")
        .and(warp::header("authorization"))
        .and(warp::body::json())
        .and(warp::any().map(move || { rest_tx.clone() }))
        .and_then(|authorization: String, body: VoteRequest, tx: Sender<CacheTask>| async move {
            return process_vote_request(tx, authorization, body).await;
        });

    let mut scheduler_tx = tx.clone();
    tokio::spawn(async move {
        info!("Started resend scheduler");
        loop {
            debug!("Starting resend task");
            let result = scheduler_tx.send(CacheTask::create_resend_task()).await;
            if result.is_err() {
                warn!("Failed to start resend task")
            }
            tokio::time::delay_for(*constants::VOTE_RESEND_DELAY).await;
        }
    });

    tokio::spawn(async move {
        info!("Started processing loop");
        let mut vote_handler = VoteHandler::new();
        loop {
            debug!("Awaiting next inbound");
            let rec = rx.recv().await;
            if rec.is_some() {
                let task = rec.unwrap();
                if task.op == CACHE_TASK_OP_VOTE {
                    vote_handler
                        .accept_vote_request(task.auth.unwrap(), task.vote.unwrap()).await;
                } else if task.op == CACHE_TASK_OP_RESEND {
                    vote_handler.resend_votes().await;
                }
            }
        }
    });
    info!("Starting rest server");
    warp::serve(vote).run(([0, 0, 0, 0], 8080)).await;
}

async fn process_vote_request(mut sender: Sender<CacheTask>, auth: String, vote: VoteRequest)
                              -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    return if auth.starts_with(VOTE_AUTH_TOKEN.clone().as_str()) {
        let result = sender.send(CacheTask::create_vote_task(auth, vote)).await;
        if result.is_ok() {
            Ok(Box::new("Ok"))
        } else {
            Err(warp::reject::not_found())
        }
    } else {
        Ok(Box::new(StatusCode::UNAUTHORIZED))
    };
}

