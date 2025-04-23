#![doc(alias = "stream.offline")]
//! The specified broadcaster stops a stream.
use super::*;

/// [`stream.offline`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#streamoffline): the specified broadcaster stops a stream.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct StreamOfflineV1 {
    /// The broadcaster user ID you want to get stream offline notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl StreamOfflineV1 {
    /// The broadcaster user ID you want to get stream offline notifications for.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for StreamOfflineV1 {
    type Payload = StreamOfflineV1Payload;

    const EVENT_TYPE: EventType = EventType::StreamOffline;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
    const VERSION: &'static str = "1";
}

/// [`stream.offline`](StreamOfflineV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct StreamOfflineV1Payload {
    /// The broadcaster’s user id.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster’s user login.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster’s user display name.
    pub broadcaster_user_name: types::DisplayName,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "stream.offline",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "1337"
            },
            "created_at": "2019-11-16T10:11:12.123Z",
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            }
        },
        "event": {
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User"
        }
    }
    "#;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}
