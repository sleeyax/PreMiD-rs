use discord_rich_presence::{
    activity::{ActivityBuilder, TimestampsBuilder},
    DiscordIpcClient,
};

use crate::{
    constants::DEFAULT_CLIENT_ID,
    types::{DiscordUser, Presence},
};

pub struct RpcClient {
    client: DiscordIpcClient,
    user: DiscordUser,
}

impl RpcClient {
    pub fn new(client_id: String) -> Self {
        let mut client = DiscordIpcClient::new(&client_id.to_string());

        let ctx = client.connect().unwrap();

        let raw_user = ctx
            .as_object()
            .unwrap()
            .get("data")
            .unwrap()
            .get("user")
            .unwrap()
            .to_owned();
        let user: DiscordUser = serde_json::from_value(raw_user).unwrap();

        Self { client, user }
    }

    pub fn set_activity(&mut self, activity: Presence) {
        println!("[RPC Client {}] set activity", &self.client.get_client_id());

        let mut act = ActivityBuilder::default();

        if let Some(details) = activity.presence_data.details {
            act = act.details(details);
        }

        if let Some(state) = activity.presence_data.state {
            act = act.state(state);
        }

        if activity.presence_data.start_timestamp != None
            || activity.presence_data.end_timestamp != None
        {
            let mut ts = TimestampsBuilder::default();

            if let Some(start) = activity.presence_data.start_timestamp {
                ts = ts.start(start as i64);
            }

            if let Some(end) = activity.presence_data.end_timestamp {
                ts = ts.end(end as i64);
            }

            act = act.timestamps(ts.build());
        }

        self.client.set_activity(act.build()).unwrap();
    }

    pub fn clear_activity(&mut self) {
        println!(
            "[RPC Client {}] clear activity",
            &self.client.get_client_id()
        );
        self.client.clear_activity().unwrap();
    }

    pub fn get_client_id(&self) -> String {
        self.client.get_client_id().clone()
    }

    pub fn get_user(&self) -> &DiscordUser {
        println!("[RPC Client {}] get user", &self.client.get_client_id());
        &self.user
    }
}

impl Default for RpcClient {
    fn default() -> Self {
        Self::new(DEFAULT_CLIENT_ID.into())
    }
}
