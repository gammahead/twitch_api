//! A a broadcaster raids another broadcaster’s channel.
use super::*;

/// [`channel.raid`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelraid): a a broadcaster raids another broadcaster’s channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelRaidV1 {
    /// The broadcaster user ID that created the channel raid you want to get notifications for. Use this parameter if you want to know when a specific broadcaster raids another broadcaster.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub from_broadcaster_user_id: Option<types::UserId>,
    /// The broadcaster user ID that received the channel raid you want to get notifications for. Use this parameter if you want to know when a specific broadcaster is raided by another broadcaster.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub to_broadcaster_user_id: Option<types::UserId>,
}

impl ChannelRaidV1 {
    /// The broadcaster user ID that created the channel raid you want to get notifications for.
    pub fn from_broadcaster_user_id(from_broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            from_broadcaster_user_id: Some(from_broadcaster_user_id.into()),
            to_broadcaster_user_id: None,
        }
    }

    /// The broadcaster user ID that received the channel raid you want to get notifications for.
    pub fn to_broadcaster_user_id(to_broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            from_broadcaster_user_id: None,
            to_broadcaster_user_id: Some(to_broadcaster_user_id.into()),
        }
    }
}

impl EventSubscription for ChannelRaidV1 {
    type Payload = ChannelRaidV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelRaid;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
    const VERSION: &'static str = "1";
}

/// [`channel.raid`](ChannelRaidV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelRaidV1Payload {
    /// The broadcaster ID that created the raid.
    pub from_broadcaster_user_id: types::UserId,
    /// The broadcaster login that created the raid.
    pub from_broadcaster_user_login: types::UserName,
    /// The broadcaster display name that created the raid.
    pub from_broadcaster_user_name: types::DisplayName,
    /// The broadcaster ID that received the raid.
    pub to_broadcaster_user_id: types::UserId,
    /// The broadcaster login that received the raid.
    pub to_broadcaster_user_login: types::UserName,
    /// The broadcaster display name that received the raid.
    pub to_broadcaster_user_name: types::DisplayName,
    /// The number of viewers in the raid.
    pub viewers: i64,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.raid",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "to_broadcaster_user_id": "1337"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.123Z"
        },
        "event": {
            "from_broadcaster_user_id": "1234",
            "from_broadcaster_user_login": "cool_user",
            "from_broadcaster_user_name": "Cool_User",
            "to_broadcaster_user_id": "1337",
            "to_broadcaster_user_login": "cooler_user",
            "to_broadcaster_user_name": "Cooler_User",
            "viewers": 9001
        }
    }
    "#;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}
