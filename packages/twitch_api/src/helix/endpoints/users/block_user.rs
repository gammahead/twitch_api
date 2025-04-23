//! Blocks the specified user on behalf of the authenticated user.
//! [`block-user`](https://dev.twitch.tv/docs/api/reference#block-user)
//!
//! # Accessing the endpoint
//!
//! ## Request: [BlockUserRequest]
//!
//! To use this endpoint, construct a [`BlockUserRequest`] with the [`BlockUserRequest::block_user()`] method.
//!
//! ```rust
//! use twitch_api::helix::users::block_user::{self, Reason, SourceContext};
//! let request = block_user::BlockUserRequest::block_user("1234");
//! // Or, specifying a reason for the block
//! let request = block_user::BlockUserRequest::block_user("1234")
//!     .source_context(SourceContext::Chat)
//!     .reason(Reason::Spam);
//! ```
//!
//! ## Response: [BlockUser]
//!
//! Send the request to receive the response with [`HelixClient::req_put()`](helix::HelixClient::req_put).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, users::block_user};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = block_user::BlockUserRequest::block_user("1234");
//! let response: block_user::BlockUser = client.req_put(request, helix::EmptyBody, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPut::create_request)
//! and parse the [`http::Response`] with [`BlockUserRequest::parse_response(None, &request.get_uri(), response)`](BlockUserRequest::parse_response)

use super::*;
use helix::RequestPut;

/// Query Parameters for [Block User](super::block_user)
///
/// [`block-user`](https://dev.twitch.tv/docs/api/reference#block-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct BlockUserRequest<'a> {
    /// User ID of the follower
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub target_user_id: Cow<'a, types::UserIdRef>,
    /// Source context for blocking the user. Valid values: "chat", "whisper".
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub source_context: Option<SourceContext>,
    /// Reason for blocking the user. Valid values: "spam", "harassment", or "other".
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub reason: Option<Reason>,
}

impl<'a> BlockUserRequest<'a> {
    /// Block a user
    pub fn block_user(target_user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            target_user_id: target_user_id.into_cow(),
            source_context: None,
            reason: None,
        }
    }

    /// Set the source_context for this block.
    pub fn source_context(mut self, source_context: impl Into<SourceContext>) -> Self {
        self.source_context = Some(source_context.into());
        self
    }

    /// Set the reason for this block.
    pub fn reason(mut self, reason: impl Into<Reason>) -> Self {
        self.reason = Some(reason.into());
        self
    }
}

/// Source context for blocking the user.
#[derive(PartialEq, Eq, Deserialize, Serialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum SourceContext {
    /// Chat
    Chat,
    /// Whisper
    Whispher,
}

/// Reason for blocking the user.
#[derive(PartialEq, Eq, Deserialize, Serialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Reason {
    /// Spam
    Spam,
    /// Harassment
    Harassment,
    /// Other
    Other,
}

/// Return Values for [Block User](super::block_user)
///
/// [`block-user`](https://dev.twitch.tv/docs/api/reference#block-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum BlockUser {
    /// 204 - User blocked successfully.
    Success,
}

impl Request for BlockUserRequest<'_> {
    type Response = BlockUser;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const PATH: &'static str = "users/blocks";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserManageBlockedUsers];
}

impl RequestPut for BlockUserRequest<'_> {
    type Body = helix::EmptyBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPutError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT => {
                Ok(helix::Response::with_data(BlockUser::Success, request))
            }
            _ => Err(helix::HelixRequestPutError::InvalidResponse {
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
    let req = BlockUserRequest::block_user("41245071");

    dbg!(req.create_request(EmptyBody, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();
    // FIXME: I have not tested this in production

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users/blocks?target_user_id=41245071"
    );

    dbg!(BlockUserRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
