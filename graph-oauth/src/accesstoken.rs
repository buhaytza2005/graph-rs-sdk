use crate::idtoken::IdToken;
use crate::jwt::JwtParser;
use crate::stdop::StdOp;
use chrono::{DateTime, Duration, Utc};
use chrono_humanize::HumanTime;
use from_as::*;
use graph_error::{GraphError, GraphFailure, GraphHeaders};
use reqwest::RequestBuilder;
use reqwest::Response;
use serde_json;
use std::convert::TryFrom;

/// AccessToken that is used for api calls to OneDrive and Graph.
///
/// The implementation of AccessToken is very generic. Callers should
/// utilize the builder: oauth::AccessTokenBuilder or use OAuth which
/// sets AccessTokens automatically. Those who wish to make their own
/// requests can use the provided TryFrom trait implementations for
/// converting from a reqwest::Response to an AccessToken:
///
///
/// An access token represents the metadata for a OAuth 2.0 access token.
/// The token_type, expires_in, scope, and access_token fields are required.
/// The access_token field is the field normally used to make authenticated
/// queries.
///
/// # Example
/// ```rust,ignore
/// use rust_onedrive::oauth::AccessToken;
/// let access_token = AccessToken::try_from(&mut response); // -> Result<AccessToken>
/// ```
///
/// AccessTokens are returned from the OAuth impl.
/// # Example
/// ```rust,ignore
/// let mut oauth = OAuth::default();
/// let mut oauth_code_grant = oauth.build().authorization_code_grant();
/// let req = oauth_code_grant.access_token();
/// let access_token = oauth.send().unwrap();
/// println!("{:#?}", access_token);
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, AsFile, FromFile)]
pub struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: i64,
    scope: String,
    refresh_token: Option<String>,
    user_id: Option<String>,
    id_token: Option<String>,
    state: Option<String>,
    timestamp: Option<DateTime<Utc>>,
}

impl AccessToken {
    /// Create a new AccessToken.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    /// let access_token = AccessToken::new("Bearer", 3600, "Read Read.Write", "ASODFIUJ34KJ;LADSK");
    /// ```
    pub fn new(token_type: &str, expires_in: i64, scope: &str, access_token: &str) -> AccessToken {
        AccessToken {
            token_type: token_type.into(),
            expires_in,
            scope: scope.into(),
            access_token: access_token.into(),
            refresh_token: None,
            user_id: None,
            id_token: None,
            state: None,
            timestamp: Some(Utc::now() + Duration::seconds(expires_in)),
        }
    }

    /// Set the token type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.token_type("Bearer");
    /// ```
    pub fn token_type(&mut self, s: &str) -> &mut AccessToken {
        self.token_type = s.into();
        self
    }

    /// Set the expies in time. This should usually be done in seconds.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.expires_in(3600);
    /// ```
    pub fn expires_in(&mut self, expires_in: i64) -> &mut AccessToken {
        self.expires_in = expires_in;
        self.timestamp = Some(Utc::now() + Duration::seconds(expires_in));
        self
    }

    /// Set the scope.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.scope("Read Read.Write");
    /// ```
    pub fn scope(&mut self, s: &str) -> &mut AccessToken {
        self.scope = s.into();
        self
    }

    /// Set the access token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.access_token("ASODFIUJ34KJ;LADSK");
    /// ```
    pub fn access_token(&mut self, s: &str) -> &mut AccessToken {
        self.access_token = s.into();
        self
    }

    /// Set the refresh token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.refresh_token(Some("#ASOD323U5342"));
    /// ```
    pub fn refresh_token(&mut self, s: Option<&str>) -> &mut AccessToken {
        self.refresh_token = StdOp::from(s);
        self
    }

    /// Set the user id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.user_id(Some("user_id"));
    /// ```
    pub fn user_id(&mut self, s: Option<&str>) -> &mut AccessToken {
        self.user_id = StdOp::from(s);
        self
    }

    /// Set the id token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::{AccessToken, IdToken};
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.id_token(Some("id_token".into()));
    /// ```
    pub fn id_token(&mut self, s: Option<String>) -> &mut AccessToken {
        self.id_token = s;
        self
    }

    /// Set the id token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::{AccessToken, IdToken};
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.with_id_token(IdToken::new("id_token", "code", "state", "session_state"));
    /// ```
    pub fn with_id_token(&mut self, id_token: IdToken) {
        self.id_token = Some(id_token.get_id_token());
    }

    /// Set the state.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    /// # use graph_oauth::oauth::IdToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.state(Some("state"));
    /// ```
    pub fn state(&mut self, s: Option<&str>) -> &mut AccessToken {
        self.state = StdOp::from(s);
        self
    }

    /// Reset the access token timestmap.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.timestamp();
    /// // The timestamp is in UTC.
    /// ```
    pub fn timestamp(&mut self) {
        self.timestamp = Some(Utc::now() + Duration::seconds(self.expires_in));
    }

    /// Get the token type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_token_type());
    /// ```
    pub fn get_token_type(&self) -> &str {
        self.token_type.as_str()
    }

    /// Set the user id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// // This is the original amount that was set not the difference.
    /// // To get the difference you can use access_token.elapsed().
    /// println!("{:#?}", access_token.get_expires_in());
    /// ```
    pub fn get_expires_in(&self) -> i64 {
        self.expires_in
    }

    /// Get the scopes.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_scopes());
    /// ```
    pub fn get_scopes(&self) -> &str {
        self.scope.as_str()
    }

    /// Get the access token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_access_token());
    /// ```
    pub fn get_access_token(&self) -> &str {
        self.access_token.as_str()
    }

    /// Get the user id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_user_id());
    /// ```
    pub fn get_user_id(&self) -> Option<String> {
        self.user_id.clone()
    }

    /// Get the refresh token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_refresh_token());
    /// ```
    pub fn get_refresh_token(self) -> Option<String> {
        match self.refresh_token {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }

    /// Get the id token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_id_token());
    /// ```
    pub fn get_id_token(&self) -> Option<String> {
        self.id_token.clone()
    }

    /// Get the state.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_state());
    /// ```
    pub fn get_state(&self) -> Option<String> {
        self.state.clone()
    }

    /// Get the timestamp.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_timestamp());
    /// ```
    pub fn get_timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp
    }

    /// Check whether the access token is expired. An access token is considerd
    /// expired when there is a negative difference between the timestamp set
    /// for the access token and the expires_in field.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.is_expired());
    /// ```
    pub fn is_expired(&self) -> bool {
        if let Some(human_time) = self.elapsed() {
            return human_time.le(&HumanTime::from(Duration::seconds(0)));
        }
        true
    }

    /// Get the time left in seconds until the access token expires.
    /// See the HumanTime crate. If you just need to know if the access token
    /// is expired then use the is_expired() message which returns a boolean
    /// true for the token has expired and false otherwise.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.elapsed());
    /// ```
    pub fn elapsed(&self) -> Option<HumanTime> {
        if let Some(timestamp) = self.timestamp {
            let ht = HumanTime::from(timestamp + Duration::seconds(self.expires_in));
            return Some(ht);
        }
        None
    }

    pub fn is_valid_jwt(&self) -> bool {
        JwtParser::parse(&self.access_token).is_ok()
    }
}

impl Default for AccessToken {
    fn default() -> Self {
        AccessToken {
            token_type: String::new(),
            expires_in: 0,
            scope: String::new(),
            access_token: String::new(),
            refresh_token: None,
            user_id: None,
            id_token: None,
            state: None,
            timestamp: Some(Utc::now() + Duration::seconds(0)),
        }
    }
}

impl TryFrom<&str> for AccessToken {
    type Error = GraphFailure;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut access_token: AccessToken = serde_json::from_str(value)?;
        access_token.timestamp();
        Ok(access_token)
    }
}

impl TryFrom<RequestBuilder> for AccessToken {
    type Error = GraphFailure;

    fn try_from(value: RequestBuilder) -> Result<Self, Self::Error> {
        let mut response = value.send()?;
        let access_token: AccessToken = AccessToken::try_from(&mut response)?;
        Ok(access_token)
    }
}

impl TryFrom<Result<reqwest::Response, reqwest::Error>> for AccessToken {
    type Error = GraphFailure;

    fn try_from(value: Result<Response, reqwest::Error>) -> Result<Self, Self::Error> {
        let mut response = value?;
        AccessToken::try_from(&mut response)
    }
}

impl TryFrom<&mut Response> for AccessToken {
    type Error = GraphFailure;

    fn try_from(value: &mut Response) -> Result<Self, Self::Error>
    where
        Self: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let status = value.status().as_u16();
        if GraphError::is_error(status) {
            let mut graph_error = GraphError::try_from(status)?;
            let graph_headers = GraphHeaders::from(value);
            graph_error.set_headers(graph_headers);
            return Err(GraphFailure::from(graph_error));
        }
        let mut access_token: AccessToken = value.json()?;
        access_token.timestamp();
        Ok(access_token)
    }
}
