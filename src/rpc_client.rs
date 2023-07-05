use discord_rich_presence::{
    activity::{
        ActivityBuilder, AssetsBuilder, Button, PartyBuilder, SecretsBuilder, TimestampsBuilder,
    },
    DiscordIpcClient,
};
use tracing::info;

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
        info!("client {}: set activity", &self.client.get_client_id());

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
                ts = ts.start(start);
            }

            if let Some(end) = activity.presence_data.end_timestamp {
                ts = ts.end(end);
            }

            act = act.timestamps(ts.build());
        }

        if let Some(buttons) = activity.presence_data.buttons {
            act = act.buttons(
                buttons
                    .iter()
                    .map(|b| Button::new(b.label.clone(), b.url.clone()))
                    .collect(),
            );
        }

        if activity.presence_data.large_image_key != None
            || activity.presence_data.large_image_text != None
            || activity.presence_data.small_image_key != None
            || activity.presence_data.small_image_text != None
        {
            let mut assets = AssetsBuilder::default();

            if let Some(large_image_key) = activity.presence_data.large_image_key {
                assets = assets.large_image(large_image_key);
            }

            if let Some(large_image_text) = activity.presence_data.large_image_text {
                assets = assets.large_text(large_image_text);
            }

            if let Some(small_image_key) = activity.presence_data.small_image_key {
                assets = assets.small_image(small_image_key);
            }

            if let Some(small_image_text) = activity.presence_data.small_image_text {
                assets = assets.small_text(small_image_text);
            }

            act = act.assets(assets.build());
        }

        if activity.presence_data.party_id != None
            || activity.presence_data.party_size != None
            || activity.presence_data.party_max != None
        {
            let mut party = PartyBuilder::default();

            if let Some(party_id) = activity.presence_data.party_id {
                party = party.id(party_id);
            }

            if activity.presence_data.party_size != None && activity.presence_data.party_max != None
            {
                let party_size = [
                    activity.presence_data.party_size.unwrap(),
                    activity.presence_data.party_max.unwrap(),
                ];
                party = party.size(party_size);
            }

            act = act.party(party.build());
        }

        if activity.presence_data.join_secret != None
            || activity.presence_data.spectate_secret != None
            || activity.presence_data.match_secret != None
        {
            let mut secrets = SecretsBuilder::default();

            if let Some(join) = activity.presence_data.join_secret {
                secrets = secrets.join_secret(join);
            }

            if let Some(spectate) = activity.presence_data.spectate_secret {
                secrets = secrets.spectate_secret(spectate);
            }

            if let Some(match_) = activity.presence_data.match_secret {
                secrets = secrets.match_secret(match_);
            }

            act = act.secrets(secrets.build());
        }

        self.client.set_activity(act.build()).unwrap();
    }

    pub fn clear_activity(&mut self) {
        info!("client {}: clear activity", &self.client.get_client_id());
        self.client.clear_activity().unwrap();
    }

    pub fn get_client_id(&self) -> String {
        self.client.get_client_id().clone()
    }

    pub fn get_user(&self) -> &DiscordUser {
        info!("client {}: get user", &self.client.get_client_id());
        &self.user
    }
}

impl Default for RpcClient {
    fn default() -> Self {
        Self::new(DEFAULT_CLIENT_ID.into())
    }
}
