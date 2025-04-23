//! Removes the word or phrase that the broadcaster is blocking users from using in their chat room.
//! [`remove-blocked-term`](https://dev.twitch.tv/docs/api/reference#remove-blocked-term)
//!
//! # Accessing the endpoint
//!
//! ## Request: [RemoveBlockedTermRequest]
//!
//! To use this endpoint, construct a [`RemoveBlockedTermRequest`] with the [`RemoveBlockedTermRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::remove_blocked_term;
//! let request = remove_blocked_term::RemoveBlockedTermRequest::new(
//!     "1234", "5678", "DEADBEEF",
//! );
//! ```
//!
//! ## Response: [RemoveBlockedTerm]
//!
//! Send the request to receive the response with [`HelixClient::req_delete()`](helix::HelixClient::req_delete).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::remove_blocked_term};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = remove_blocked_term::RemoveBlockedTermRequest::new("1234", "5678", "DEADBEEF");
//! let response: remove_blocked_term::RemoveBlockedTerm = client.req_delete(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`RemoveBlockedTermRequest::parse_response(None, &request.get_uri(), response)`](RemoveBlockedTermRequest::parse_response)

use super::*;
use helix::RequestDelete;
/// Query Parameters for [Remove Blocked Terms](super::remove_blocked_term)
///
/// [`remove-blocked-term`](https://dev.twitch.tv/docs/api/reference#remove-blocked-term)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct RemoveBlockedTermRequest<'a> {
    /// The ID of the broadcaster that owns the list of blocked terms.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of a user that has permission to moderate the broadcaster’s chat room. This ID must match the user ID associated with the user OAuth token.
    /// If the broadcaster wants to delete the blocked term (instead of having the moderator do it), set this parameter to the broadcaster’s ID, too.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
    /// The ID of the blocked term you want to delete.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub id: Cow<'a, types::BlockedTermIdRef>,
}

impl<'a> RemoveBlockedTermRequest<'a> {
    /// Remove blocked term
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        id: impl types::IntoCow<'a, types::BlockedTermIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
            id: id.into_cow(),
        }
    }
}

/// Return Values for [Remove Blocked Terms](super::remove_blocked_term)
///
/// [`remove-blocked-term`](https://dev.twitch.tv/docs/api/reference#remove-blocked-term)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum RemoveBlockedTerm {
    /// 204 - Term removed successfully.
    Success,
}

impl Request for RemoveBlockedTermRequest<'_> {
    type Response = RemoveBlockedTerm;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const PATH: &'static str = "moderation/blocked_terms";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageBlockedTerms];
}

impl RequestDelete for RemoveBlockedTermRequest<'_> {
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
                RemoveBlockedTerm::Success,
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
    let req = RemoveBlockedTermRequest::new("1234", "5678", "c9fc79b8-0f63-4ef7-9d38-efd811e74ac2");

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/blocked_terms?broadcaster_id=1234&moderator_id=5678&id=c9fc79b8-0f63-4ef7-9d38-efd811e74ac2"
    );

    dbg!(RemoveBlockedTermRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
