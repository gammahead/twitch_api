//! Removes the ban or timeout that was placed on the specified user.
//! [`unban-user`](https://dev.twitch.tv/docs/api/reference#unban-user)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UnbanUserRequest]
//!
//! To use this endpoint, construct a [`UnbanUserRequest`] with the [`UnbanUserRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::unban_user;
//! let request = unban_user::UnbanUserRequest::new("1234", "5678", "1337");
//! ```
//!
//! ## Response: [UnbanUserResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_delete()`](helix::HelixClient::req_delete).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::unban_user};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = unban_user::UnbanUserRequest::new("1234", "5678", "1337");
//! let response: unban_user::UnbanUserResponse = client.req_delete(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`UnbanUserRequest::parse_response(None, &request.get_uri(), response)`](UnbanUserRequest::parse_response)

use super::*;
use helix::RequestDelete;
/// Query Parameters for [Unban User](super::unban_user)
///
/// [`unban-user`](https://dev.twitch.tv/docs/api/reference#unban-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct UnbanUserRequest<'a> {
    /// The ID of the broadcaster whose chat room the user is banned from chatting in.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of a user that has permission to moderate the broadcaster’s chat room. This ID must match the user ID associated with the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
    /// The ID of the user to remove the ban or timeout from.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
}

impl<'a> UnbanUserRequest<'a> {
    /// Remove the ban or timeout that was placed on the specified user.
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
            user_id: user_id.into_cow(),
        }
    }
}

/// Return Values for [Unban User](super::unban_user)
///
/// [`unban-user`](https://dev.twitch.tv/docs/api/reference#unban-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum UnbanUserResponse {
    /// Unban was successful
    Success,
}

impl Request for UnbanUserRequest<'_> {
    type Response = UnbanUserResponse;

    const PATH: &'static str = "moderation/bans";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageBannedUsers];
}

impl RequestDelete for UnbanUserRequest<'_> {
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
                UnbanUserResponse::Success,
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
    let req = UnbanUserRequest::new("1234", "5678", "9876");

    dbg!(req.create_request("token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    "#
    .to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/bans?broadcaster_id=1234&moderator_id=5678&user_id=9876"
    );

    dbg!(UnbanUserRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
