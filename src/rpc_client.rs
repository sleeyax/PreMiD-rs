use std::thread::JoinHandle;

use discord_presence::{Client, Event};
use serde_json::Value;

use crate::types::{DiscordUser, Presence};

pub struct RpcClient {
    client: Client,
    client_thread: JoinHandle<()>,
    user: DiscordUser,
}

impl RpcClient {
    pub fn new(client_id: u64) -> Self {
        let mut client = Client::new(client_id);

        client.on_ready(|ctx| {
            println!("[RPC Client] Ready: {:?}", ctx.event);
        });

        client.on_error(|ctx| {
            println!("[RPC Client] Error: {:?}", ctx.event);
        });

        let client_thread = client.start();

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
            user,
            client_thread,
        }
    }

    pub fn set_activity(&mut self, activity: Presence) {
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
        println!("[RPC Client set] activity");
    }

    pub fn clear_activity(&mut self) {
        self.client.clear_activity().unwrap();
    }

    pub fn get_user(&self) -> &DiscordUser {
        &self.user
    }

    pub fn join_thread(self) {
        self.client_thread.join().unwrap();
    }
}
