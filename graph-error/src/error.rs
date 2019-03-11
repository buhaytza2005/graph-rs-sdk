use std::error::Error;
use std::fmt;
use std::string::ToString;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GraphError {
    pub error_info: String,
    pub error_type: ErrorType,
    pub code: u16,
}

impl GraphError {
    pub fn new(error_info: &str, error_type: ErrorType, code: u16) -> GraphError {
        GraphError {
            error_info: error_info.to_string(),
            error_type,
            code,
        }
    }

    pub fn is_error(status: u16) -> bool {
        ErrorType::from_u16(status).is_some()
    }
}

impl Error for GraphError {
    fn description(&self) -> &str {
        self.error_info.as_str()
    }

    fn source<'a>(&'a self) -> Option<&(dyn Error + 'static)> {
        unimplemented!()
    }
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\nError Code: {:#?}\nError Message: {:#?}",
            &self.code, &self.error_info
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorInfo {
    code: String,
    message: String,
    #[serde(rename = "innerError")]
    inner_error: InnerError,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerError {
    #[serde(rename = "request-id")]
    request_id: String,
    date: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ErrorType {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    RequestEntityTooLarge,
    UnsupportedMediaType,
    RequestRangeNotSatisfiable,
    UnprocessableEntity,
    TooManyRequests,
    InternalServerError,
    NotImplemented,
    ServiceUnavailable,
    InsufficientStorage,
    BandwidthLimitExceeded,
    UnknownError,
}

impl ErrorType {}

impl ErrorType {
    pub fn as_str(&self) -> &str {
        match *self {
            ErrorType::BadRequest => "Cannot process the request because it is malformed or incorrect.",
            ErrorType::Unauthorized => "Required authentication information is either missing or not valid for the resource.",
            ErrorType::Forbidden => "Access is denied to the requested resource. The user might not have enough permission.",
            ErrorType::NotFound => "The requested resource doesnt exist.",
            ErrorType::MethodNotAllowed => "The HTTP method in the request is not allowed on the resource.",
            ErrorType::NotAcceptable => "This service doesnt support the format requested in the Accept header.",
            ErrorType::Conflict => "The current state conflicts with what the request expects. For example, the specified parent folder might not exist",
            ErrorType::Gone => "The requested resource is no longer available at the server.",
            ErrorType::LengthRequired => "A Content-Length header is required on the request.",
            ErrorType::PreconditionFailed=> "A precondition provided in the request (such as an if-match header) does not match the resource's current state.",
            ErrorType::RequestEntityTooLarge => "The request size exceeds the maximum limit.",
            ErrorType::UnsupportedMediaType => "The content type of the request is a format that is not supported by the service.",
            ErrorType::RequestRangeNotSatisfiable => "The specified byte range is invalid or unavailable.",
            ErrorType::UnprocessableEntity => "Cannot process the request because it is semantically incorrect.",
            ErrorType::TooManyRequests => "Client application has been throttled and should not attempt to repeat the request until an amount of time has elapsed.",
            ErrorType::InternalServerError => "There was an internal server error while processing the request.",
            ErrorType::NotImplemented => "The requested feature isn’t implemented.",
            ErrorType::ServiceUnavailable => "The service is temporarily unavailable. You may repeat the request after a delay. There may be a Retry-After header.",
            ErrorType::InsufficientStorage => "The maximum storage quota has been reached.",
            ErrorType::BandwidthLimitExceeded => "Your app has been throttled for exceeding the maximum bandwidth cap. Your app can retry the request again after more time has elapsed.",
            ErrorType::UnknownError => "Unknown error or failure",
        }
    }

    pub fn from_u16(num: u16) -> Option<ErrorType> {
        match num {
            400 => Some(ErrorType::BadRequest),
            401 => Some(ErrorType::Unauthorized),
            403 => Some(ErrorType::Forbidden),
            404 => Some(ErrorType::NotFound),
            405 => Some(ErrorType::MethodNotAllowed),
            406 => Some(ErrorType::NotAcceptable),
            409 => Some(ErrorType::Conflict),
            410 => Some(ErrorType::Gone),
            411 => Some(ErrorType::LengthRequired),
            412 => Some(ErrorType::PreconditionFailed),
            413 => Some(ErrorType::RequestEntityTooLarge),
            415 => Some(ErrorType::UnsupportedMediaType),
            416 => Some(ErrorType::RequestRangeNotSatisfiable),
            422 => Some(ErrorType::UnprocessableEntity),
            429 => Some(ErrorType::TooManyRequests),
            500 => Some(ErrorType::InternalServerError),
            501 => Some(ErrorType::NotImplemented),
            503 => Some(ErrorType::ServiceUnavailable),
            507 => Some(ErrorType::InsufficientStorage),
            509 => Some(ErrorType::BandwidthLimitExceeded),
            _ => None,
        }
    }

    /// Returns the matching GraphError for a u16 wrapped
    /// in an option.
    ///
    /// # Example
    /// ```
    /// use graph_error::GraphError;
    /// use graph_error::ErrorType;
    ///
    /// let error: Option<GraphError> = ErrorType::graph_error(400);
    /// assert_eq!(
    ///     Some(GraphError::new(ErrorType::BadRequest.as_str(),
    ///         ErrorType::BadRequest,
    ///         400)),
    ///     error
    /// );
    /// ```
    pub fn graph_error(status: u16) -> Option<GraphError> {
        let error_type = ErrorType::from_u16(status);
        if error_type.is_some() {
            let error_type = match error_type {
                Some(t) => t,
                None => {
                    return Some(GraphError::new(
                        ErrorType::BadRequest.as_str(),
                        ErrorType::BadRequest,
                        400,
                    ));
                },
            };
            return Some(GraphError {
                error_info: String::from(error_type.as_str()),
                error_type,
                code: status,
            });
        }
        None
    }

    pub fn is_error(status: u16) -> bool {
        ErrorType::from_u16(status).is_some()
    }
}

impl ToString for ErrorType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

/// Returns the matching GraphError for a u16.
/// This method will panic with a NoneError if there is no corresponding u16 to match.
/// Only use this method if you are sure the u16 given will match
/// or you if the method results with a panic.
impl From<u16> for GraphError {
    fn from(err: u16) -> Self {
        let error_type = ErrorType::from_u16(err).unwrap();
        GraphError {
            error_info: error_type.to_string(),
            error_type,
            code: err,
        }
    }
}
