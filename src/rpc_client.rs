use discord_presence::{Client, Event};
use serde_json::Value;

use crate::{
    constants::DEFAULT_CLIENT_ID,
    types::{DiscordUser, Presence},
};

pub struct RpcClient {
    pub client_id: u64,
    client: Client,
    user: DiscordUser,
}

impl RpcClient {
    pub fn new(client_id: u64) -> Self {
        let mut client = Client::new(client_id);

        client.on_ready(move |ctx| {
            println!("[RPC Client {}] Ready: {:?}", &client_id, ctx.event);
        });

        client.on_error(move |ctx| {
            println!("[RPC Client {}] Error: {:?}", &client_id, ctx.event);
        });

        let _ = client.start();

        let ctx = client.block_until_event(Event::Ready).unwrap();

        let user: Value = ctx
            .event
            .as_object()
            .unwrap()
            .get("user")
            .unwrap()
            .to_owned();
        let user: DiscordUser = serde_json::from_value(user).unwrap();

        Self {
            client,
            client_id,
            user,
        }
    }

    pub fn set_activity(&mut self, activity: Presence) {
        println!("[RPC Client {}] set activity", &self.client_id);
        self.client
            .set_activity(|act| {
                // TODO: add more fields
                let mut act = act;

                if let Some(details) = activity.presence_data.details {
                    act = act.details(details);
                }

                if let Some(instance) = activity.presence_data.instance {
                    act = act.instance(instance);
                }

                if let Some(state) = activity.presence_data.state {
                    act = act.state(state);
                }

                if activity.presence_data.start_timestamp != None
                    || activity.presence_data.end_timestamp != None
                {
                    act = act.timestamps(|ts| {
                        let mut ts = ts;

                        if let Some(start) = activity.presence_data.start_timestamp {
                            ts = ts.start(start);
                        }

                        if let Some(end) = activity.presence_data.end_timestamp {
                            ts = ts.end(end);
                        }

                        ts
                    });
                }

                act
            })
            .unwrap();
    }

    pub fn clear_activity(&mut self) {
        println!("[RPC Client {}] clear activity", &self.client_id);
        self.client.clear_activity().unwrap();
    }

    pub fn get_user(&self) -> &DiscordUser {
        println!("[RPC Client {}] get user", &self.client_id);
        &self.user
    }
}

impl Default for RpcClient {
    fn default() -> Self {
        Self::new(DEFAULT_CLIENT_ID)
    }
}
