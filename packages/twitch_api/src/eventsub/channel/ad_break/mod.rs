#![doc(alias = "channel.ad_break")]
//! Ad break on channel has begun
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod begin;

#[doc(inline)]
pub use begin::{ChannelAdBreakBeginV1, ChannelAdBreakBeginV1Payload};
