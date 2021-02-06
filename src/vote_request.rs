use crate::snowflake::Snowflake;
use crate::constants::{PAGE_KEY_TOPGG, PAGE_KEY_DBL, PAGE_KEY_BFD, PAGE_KEY_DBOATS};
use serde::{Serialize, Deserialize};

pub trait Vote {
    fn get_bot(&self) -> Snowflake;
    fn get_user(&self) -> Snowflake;
    fn get_source(&self) -> String;
    fn get_as_generic(&self) -> VoteRequest;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VoteRequest {
    pub bot: Snowflake,
    pub user: Snowflake,
    pub r#type: String,
    pub is_weekend: bool,
    pub query: Option<String>,
    pub src: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TopVoteRequest {
    pub bot: Snowflake,
    pub user: Snowflake,
    pub r#type: String,
    pub is_weekend: bool,
    pub query: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DblComVoteRequest {
    pub bot: Option<Snowflake>,
    pub id: Snowflake,
    pub username: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BfdVoteRequest {
    pub bot: Snowflake,
    pub user: Snowflake,
    pub r#type: String,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DBoatsBotData {
    pub id: Snowflake,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DBoatsUserData {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DBoatsVoteRequest {
    pub bot: DBoatsBotData,
    pub user: DBoatsUserData,
}

impl Vote for VoteRequest {
    fn get_bot(&self) -> Snowflake {
        return self.bot;
    }

    fn get_user(&self) -> Snowflake {
        return self.user;
    }

    fn get_source(&self) -> String {
        return self.src.clone().unwrap_or(PAGE_KEY_TOPGG.to_owned());
    }

    fn get_as_generic(&self) -> VoteRequest {
        return VoteRequest {
            bot: self.get_bot(),
            user: self.get_user(),
            r#type: self.r#type.to_owned(),
            is_weekend: self.is_weekend,
            query: self.query.to_owned(),
            src: Some(self.get_source()),
        };
    }
}

impl Vote for TopVoteRequest {
    fn get_bot(&self) -> Snowflake {
        return self.bot;
    }

    fn get_user(&self) -> Snowflake {
        return self.user;
    }

    fn get_source(&self) -> String {
        return PAGE_KEY_TOPGG.to_owned();
    }

    fn get_as_generic(&self) -> VoteRequest {
        return VoteRequest {
            bot: self.get_bot(),
            user: self.get_user(),
            r#type: if self.r#type.eq("upvote") { "vote".to_owned() } else { self.r#type.to_owned() },
            is_weekend: self.is_weekend,
            query: self.query.to_owned(),
            src: Some(self.get_source()),
        };
    }
}

impl Vote for DblComVoteRequest {
    fn get_bot(&self) -> Snowflake {
        return self.bot.unwrap_or(Snowflake(0));
    }

    fn get_user(&self) -> Snowflake {
        return self.id;
    }

    fn get_source(&self) -> String {
        return PAGE_KEY_DBL.to_owned();
    }

    fn get_as_generic(&self) -> VoteRequest {
        return VoteRequest {
            bot: self.get_bot(),
            user: self.get_user(),
            r#type: "vote".to_owned(),
            is_weekend: false,
            query: None,
            src: Some(self.get_source()),
        };
    }
}

impl Vote for BfdVoteRequest {
    fn get_bot(&self) -> Snowflake {
        return self.bot;
    }

    fn get_user(&self) -> Snowflake {
        return self.user;
    }

    fn get_source(&self) -> String {
        return PAGE_KEY_BFD.to_owned();
    }

    fn get_as_generic(&self) -> VoteRequest {
        return VoteRequest {
            bot: self.get_bot(),
            user: self.get_user(),
            r#type: self.r#type.to_owned(),
            is_weekend: false,
            query: None,
            src: Some(self.get_source()),
        };
    }
}

impl Vote for DBoatsVoteRequest {
    fn get_bot(&self) -> Snowflake {
        return self.bot.id;
    }

    fn get_user(&self) -> Snowflake {
        return self.user.id;
    }

    fn get_source(&self) -> String {
        return PAGE_KEY_DBOATS.to_owned();
    }

    fn get_as_generic(&self) -> VoteRequest {
        return VoteRequest {
            bot: self.get_bot(),
            user: self.get_user(),
            r#type: "vote".to_owned(),
            is_weekend: false,
            query: None,
            src: Some(self.get_source()),
        };
    }
}