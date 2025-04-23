#![doc(alias = "channel.prediction.lock")]
//! A user responds to a prediction on the specified channel

use super::*;
/// [`channel.prediction.lock`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelpredictionlock): an user responds to a prediction on the specified channel
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPredictionLockV1 {
    /// The broadcaster user ID of the channel for which “prediction lock” notifications will be received.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelPredictionLockV1 {
    /// The broadcaster user ID of the channel for which “prediction lock” notifications will be received.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelPredictionLockV1 {
    type Payload = ChannelPredictionLockV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelPredictionLock;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ChannelReadPredictions,
        twitch_oauth2::Scope::ChannelManagePredictions
    )];
    const VERSION: &'static str = "1";
}

/// [`channel.prediction.lock`](ChannelPredictionLockV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPredictionLockV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// Channel Points Prediction ID.
    pub id: types::PredictionId,
    /// The time the Channel Points Prediction will automatically lock.
    pub locked_at: types::Timestamp,
    /// An array of outcomes for the Channel Points Prediction. Includes top_predictors.
    pub outcomes: Vec<types::PredictionOutcome>,
    /// The time the Channel Points Prediction started.
    pub started_at: types::Timestamp,
    /// Title for the Channel Points Prediction.
    pub title: String,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    // FIXME: example has comments and trailing commas
    // FIXME: example shows user_id as an integer, when it's specified as a string. See https://github.com/twitchdev/issues/issues/390
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.prediction.lock",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "1337"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.123Z"
        },
        "event": {
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "outcomes": [
                {
                    "id": "1243456",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 10,
                    "channel_points": 15000,
                    "top_predictors": [
                        {
                            "user_name": "Cool_User",
                            "user_login": "cool_user",
                            "user_id": "1234",
                            "channel_points_won": null,
                            "channel_points_used": 500
                        },
                        {
                            "user_name": "Coolest_User",
                            "user_login": "coolest_user",
                            "user_id": "1236",
                            "channel_points_won": null,
                            "channel_points_used": 200
                        }
                    ]
                },
                {
                    "id": "2243456",
                    "title": "No!",
                    "color": "pink",
                    "top_predictors": [
                        {
                            "user_name": "Cooler_User",
                            "user_login": "cooler_user",
                            "user_id": "12345",
                            "channel_points_won": null,
                            "channel_points_used": 5000
                        }
                    ]
                }
            ],
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "locked_at": "2020-07-15T17:21:03.17106713Z"
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}
