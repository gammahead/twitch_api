#![doc(alias = "channel.subscription")]
//! Subscription on a specified channel has changed
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod end;
pub mod gift;
pub mod message;

#[doc(inline)]
pub use end::{ChannelSubscriptionEndV1, ChannelSubscriptionEndV1Payload};
#[doc(inline)]
pub use gift::{ChannelSubscriptionGiftV1, ChannelSubscriptionGiftV1Payload};
#[doc(inline)]
pub use message::{ChannelSubscriptionMessageV1, ChannelSubscriptionMessageV1Payload};
