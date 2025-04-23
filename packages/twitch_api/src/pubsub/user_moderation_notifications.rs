//! PubSub messages for user moderation notifications
use crate::{pubsub, types};
use serde_derive::{Deserialize, Serialize};

/// A user follows the channel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(into = "String", try_from = "String")]
pub struct UserModerationNotifications {
    /// The currently authenticated user for whose automod messages will be reported on
    pub current_user_id: u32,
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(
    UserModerationNotifications,
    "user-moderation-notifications",
    current_user_id,
    channel_id // FIXME: add trailing comma
);

impl pubsub::Topic for UserModerationNotifications {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];

    fn into_topic(self) -> pubsub::Topics { super::Topics::UserModerationNotifications(self) }
}

/// Reply from [UserModerationNotifications]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(tag = "type", content = "data")]
#[non_exhaustive]
pub enum UserModerationNotificationsReply {
    /// Message held by automod
    #[serde(rename = "automod_caught_message")]
    AutoModCaught {
        // FIXME: twitch says message.id, it's message.message_id
        /// Identifier of the message
        message_id: types::MsgId,
        /// Current status of the message
        status: types::AutomodStatus,
    },
}
#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;
    #[test]
    fn automodcaught() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "user-moderation-notifications.27620241.268131879",
        "message": "{\"type\":\"automod_caught_message\",\"data\":{\"message_id\":\"d6f608f8-8f34-4f65-947c-0a92e31b0bfc\",\"status\":\"PENDING\"}}"
    }
}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::UserModerationNotifications { .. },
            }
        ));
    }

    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "user-moderation-notifications.123.1234";
        assert_eq!(
            UserModerationNotifications {
                channel_id: 1234,
                current_user_id: 123
            },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    fn check_ser() {
        let s = "user-moderation-notifications.123.1234";
        let right: String = UserModerationNotifications {
            channel_id: 1234,
            current_user_id: 123,
        }
        .into();
        assert_eq!(s.to_string(), right);
    }
}
