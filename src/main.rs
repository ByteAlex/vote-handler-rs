use std::ops::Deref;
use crate::vote_handler::VoteHandler;
use warp::Filter;
use crate::vote_request::{VoteRequest, Vote, TopVoteRequest, DblComVoteRequest, BfdVoteRequest, DBoatsVoteRequest, DBoatsBotData, DiscordListVoteRequest};
use crate::cache_task::CacheTask;
use crate::constants::{CACHE_TASK_OP_VOTE, CACHE_TASK_OP_RESEND, PAGE_KEY_TOPGG, PAGE_KEY_DBL, PAGE_KEY_BFD, PAGE_KEY_DBOATS, PAGE_KEY_DLIST};
use warp::http::StatusCode;
use tokio::sync::mpsc::Sender;
use log::{info, debug, warn};
use warp::hyper::body::Bytes;
use crate::snowflake::Snowflake;

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

    let scheduler_tx = tx.clone();
    tokio::spawn(async move {
        info!("Started resend scheduler");
        loop {
            debug!("Starting resend task");
            let result = scheduler_tx.send(CacheTask::create_resend_task()).await;
            if result.is_err() {
                warn!("Failed to start resend task")
            }
            tokio::time::sleep(*constants::VOTE_RESEND_DELAY).await;
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
                        .accept_vote_request(task.vote.unwrap()).await;
                } else if task.op == CACHE_TASK_OP_RESEND {
                    vote_handler.resend_votes().await;
                }
            }
        }
    });

    let options = warp::options().map(|| { Ok(Box::new("OPTIONS")) });
    let rest_tx = tx.clone();
    let generic_vote = warp::path!("vote" / "generic")
        .and(warp::header("authorization"))
        .and(warp::body::json())
        .and(warp::any().map(move || { rest_tx.clone() }))
        .and_then(|authorization: String, body: VoteRequest, tx: Sender<CacheTask>| async move {
            return process_vote_request(tx, authorization, body, true).await;
        });
    let rest_tx = tx.clone();
    let top_vote = warp::path!("vote" / "topgg")
        .and(warp::header("authorization"))
        .and(warp::body::json())
        .and(warp::any().map(move || { rest_tx.clone() }))
        .and_then(|authorization: String, body: TopVoteRequest, tx: Sender<CacheTask>| async move {
            return process_vote_request(tx, authorization, body, false).await;
        });
    let rest_tx = tx.clone();
    let bfd_vote = warp::path!("vote" / "bfd")
        .and(warp::header("authorization"))
        .and(warp::body::json())
        .and(warp::any().map(move || { rest_tx.clone() }))
        .and_then(|authorization: String, body: BfdVoteRequest, tx: Sender<CacheTask>| async move {
            return process_vote_request(tx, authorization, body, false).await;
        });
    let rest_tx = tx.clone();
    let dbl_vote = warp::path!("vote" / "dbl" / u64)
        .and(warp::header("authorization"))
        .and(warp::body::json())
        .and(warp::any().map(move || { rest_tx.clone() }))
        .and_then(|param: u64, authorization: String, mut body: DblComVoteRequest, tx: Sender<CacheTask>| async move {
            body.bot = Some(Snowflake(param));
            return process_vote_request(tx, authorization, body, false).await;
        });
    let rest_tx = tx.clone();
    let dboats_vote = warp::path!("vote" / "dboats")
        .and(warp::header("authorization"))
        .and(warp::body::json())
        .and(warp::any().map(move || { rest_tx.clone() }))
        .and_then(|authorization: String, body: DBoatsVoteRequest, tx: Sender<CacheTask>| async move {
            return process_vote_request(tx, authorization, body, false).await;
        });
    let rest_tx = tx.clone();
    let dlist_vote = warp::path!("vote" / "dlist")
        .and(warp::header("authorization"))
        .and(warp::body::bytes())
        .and(warp::any().map(move || { rest_tx.clone() }))
        .and_then(|authorization: String, body: Bytes, tx: Sender<CacheTask>| async move {
            use jwt::VerifyWithKey;
            let content = String::from_utf8(body.to_vec()).expect("Failed to get body as text");

            match content.verify_with_key(constants::VOTE_AUTH_KEY_DLIST.deref()) {
                Ok(body) => {
                    let body: DiscordListVoteRequest = body;
                    process_vote_request(tx, authorization, body, false).await
                }
                Err(err) => {
                    warn!("Failed to verify dlist request: {}", err);
                    let res: Result<Box<dyn warp::Reply>, warp::Rejection> = Ok(Box::new(StatusCode::UNAUTHORIZED));
                    res
                }
            }
        });
    let rest_tx = tx.clone();
    let dboats_vote_old = warp::path!("vote" / "dboats" / u64)
        .and(warp::header("authorization"))
        .and(warp::body::json())
        .and(warp::any().map(move || { rest_tx.clone() }))
        .and_then(|param: u64, authorization: String, mut body: DBoatsVoteRequest, tx: Sender<CacheTask>| async move {
            body.bot = Some(DBoatsBotData {
                id: Snowflake(param),
                name: "Bot".to_owned()
            });
            return process_vote_request(tx, authorization, body, false).await;
        });

    info!("Starting rest server");
    warp::serve(options.or(warp::post().and(generic_vote.or(top_vote)
        .or(bfd_vote).or(dbl_vote).or(dlist_vote).or(dboats_vote).or(dboats_vote_old))))
        .run(([0, 0, 0, 0], 8080))
        .await;
}

async fn process_vote_request<V: Vote>(sender: Sender<CacheTask>, auth: String, generic_vote: V,
                                       generic: bool)
                                       -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let vote = map_request(generic_vote);
    let expected_auth;
    if generic {
        expected_auth = Some(constants::VOTE_AUTH_TOKEN.clone());
    } else {
        expected_auth = get_auth(vote.clone());
    }
    return if expected_auth.is_none() || auth.eq(expected_auth.unwrap().as_str()) {
        let result = sender.send(CacheTask::create_vote_task(auth, vote)).await;
        if result.is_ok() {
            Ok(Box::new(r#"{"status":"OK"}"#))
        } else {
            Err(warp::reject::not_found())
        }
    } else {
        warn!("Dropping unauthorized request!");
        Ok(Box::new(StatusCode::UNAUTHORIZED))
    };
}

fn map_request<V: Vote>(vote: V) -> VoteRequest {
    return vote.get_as_generic();
}

pub fn get_auth(vote: VoteRequest) -> Option<String> {
    if vote.src.is_none() {
        return Some(constants::VOTE_AUTH_TOKEN.clone());
    }
    return match vote.src.unwrap().as_str() {
        PAGE_KEY_TOPGG => Some(constants::VOTE_AUTH_TOKEN_TOPGG.clone()),
        PAGE_KEY_DBL => Some(constants::VOTE_AUTH_TOKEN_DBL.clone()),
        PAGE_KEY_BFD => Some(constants::VOTE_AUTH_TOKEN_BFD.clone()),
        PAGE_KEY_DBOATS => Some(constants::VOTE_AUTH_TOKEN_DBOATS.clone()),
        PAGE_KEY_DLIST => None,
        _ => Some(constants::VOTE_AUTH_TOKEN.clone()),
    };
}
