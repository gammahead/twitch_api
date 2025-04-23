//! Removes a VIP from the broadcaster’s chat room.
//! [`remove-channel-vip`](https://dev.twitch.tv/docs/api/reference#remove-channel-vip)
//!
//! # Accessing the endpoint
//!
//! ## Request: [RemoveChannelVipRequest]
//!
//! To use this endpoint, construct a [`RemoveChannelVipRequest`] with the [`RemoveChannelVipRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::remove_channel_vip;
//! let request =
//!     remove_channel_vip::RemoveChannelVipRequest::new("1234", "1337");
//! ```
//!
//! ## Response: [RemoveChannelVipResponse]
//!
//! Send the request to receive the response with [`HelixClient::req_delete()`](helix::HelixClient::req_delete).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, channels::remove_channel_vip};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = remove_channel_vip::RemoveChannelVipRequest::new("1234", "1337");
//! let response: remove_channel_vip::RemoveChannelVipResponse = client.req_delete(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`RemoveChannelVipRequest::parse_response(None, &request.get_uri(), response)`](RemoveChannelVipRequest::parse_response)

use super::*;
use helix::RequestDelete;

/// Query Parameters for [Remove Channel VIP](super::remove_channel_vip)
///
/// [`remove-channel-vip`](https://dev.twitch.tv/docs/api/reference#remove-channel-vip)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct RemoveChannelVipRequest<'a> {
    /// The ID of the broadcaster that’s removing VIP status from the user.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the user to remove as a VIP from the broadcaster’s chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
}

impl<'a> RemoveChannelVipRequest<'a> {
    /// Remove channel VIP
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::helix::channels::RemoveChannelVipRequest;
    /// let request = RemoveChannelVipRequest::new("1337", "1234");
    /// ```
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            user_id: user_id.into_cow(),
        }
    }
}

/// Return Values for [Remove Channel VIP](super::remove_channel_vip)
///
/// [`remove-channel-vip`](https://dev.twitch.tv/docs/api/reference#remove-channel-vip)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum RemoveChannelVipResponse {
    /// Successfully removed the VIP.
    Success,
}

impl Request for RemoveChannelVipRequest<'_> {
    type Response = RemoveChannelVipResponse;

    const PATH: &'static str = "channels/vips";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelManageVips];
}

impl RequestDelete for RemoveChannelVipRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestDeleteError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT => Ok(helix::Response::with_data(
                RemoveChannelVipResponse::Success,
                request,
            )),
            _ => Err(helix::HelixRequestDeleteError::InvalidResponse {
                reason: "unexpected status",
                response: response.to_string(),
                status,
                uri: uri.clone(),
            }),
        }
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = RemoveChannelVipRequest::new("123", "456");

    dbg!(req.create_request("token", "clientid").unwrap());

    // From twitch docs
    let data = vec![];

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels/vips?broadcaster_id=123&user_id=456"
    );

    dbg!(RemoveChannelVipRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
