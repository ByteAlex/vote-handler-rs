# Vote-Handler Proxy
This applications purpose is only to keep state in a local cache for 
as long as the bot's vote endpoint is unavailable.

This application can be run multiple-times across different nodes using
a load-balancer to achieve high-availability.

## Env vars
* RUST_LOG | Set logging level
* VOTE_ENDPOINT | (Mandatory) Set the endpoint to proxy requests to
* VOTE_ENDPOINT_AUTH_TOKEN | Set the token provided to the endpoint in Authorization 
header, defaults from VOTE_AUTH_TOKEN
* VOTE_RESEND_DELAY | The interval in seconds between resend 
executions, default 5
* VOTE_RESEND_BULK_COUNT | The amount of requests per resend-execution, 
default 100
* VOTE_AUTH_TOKEN | The token provided in Authorization header to validate requests
against on vote/generic endpoint
* VOTE_AUTH_TOKEN_TOPGG | The token provided in Authorization header to validate requests
against on vote/topgg endpoint
* VOTE_AUTH_TOKEN_DBL | The token provided in Authorization header to validate requests
against on vote/dbl/{botid} endpoint
* VOTE_AUTH_TOKEN_BFD | The token provided in Authorization header to validate requests
against on vote/bfd endpoint
* VOTE_AUTH_TOKEN_DBOATS | The token provided in Authorization header to validate requests
against on vote/dboats/{botid} endpoint

## Usage
Your endpoint has to return a Status-Code 200 with the response ``{"status":"OK"}``, 
at least the `status`-node with the value `Ok` must be present.

The vote-handler proxy exposes 5 different endpoints:
* vote/generic
* vote/topgg
* vote/dbl/{botid}
* vote/bfd
* vote/dboats/{botid}

The requests will be accepted and unified to the following struct:

```rust
pub struct VoteRequest {
    pub bot: Snowflake,
    pub user: Snowflake,
    pub r#type: String,
    pub is_weekend: bool,
    pub query: Option<String>,
    pub src: Option<String>,
}
```