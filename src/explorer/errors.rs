// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Debug, Display};
use tide_disco::StatusCode;

/// [ExplorerAPIError] is a trait that represents an error that can be returned
/// returned from the ExplorerAPI for various reasons.
///
/// It aims to be Serializable for the purposes of conveying the error to the
/// client.
pub trait ExplorerAPIError: Display + Debug {
    /// The code for this error will uniquely identify this specific error.
    ///
    /// > This value **SHOULD** match the rename field for the `serde` tag,
    ///   if applicable.
    fn code(&self) -> &str;
}

/// [ExplorerAPIErrorWithCause] is a trait that represents an extension to the
/// normal [ExplorerAPIError] in that it also includes an underlying cause
/// for the given error.
pub trait ExplorerAPIErrorWithCause: ExplorerAPIError {
    type Cause: ExplorerAPIError;
    fn cause(&self) -> Self::Cause;
}

impl std::error::Error for dyn ExplorerAPIError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

/// [Unimplemented] is an error that indicates that the feature in question is
/// no implemented, but in addition, that this status can be conveyed to the
/// called with serializable error code.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "code", rename = "UNIMPLEMENTED")]
pub struct Unimplemented {}

impl Unimplemented {
    pub fn status(&self) -> StatusCode {
        StatusCode::NotImplemented
    }
}

impl ExplorerAPIError for Unimplemented {
    fn code(&self) -> &str {
        "UNIMPLEMENTED"
    }
}

/// Implement [Display] for an error message.
impl Display for Unimplemented {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this feature is not yet implemented")
    }
}

impl std::error::Error for Unimplemented {}

impl Serialize for Unimplemented {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("Unimplemented", 2)?;
        st.serialize_field("code", &self.code())?;
        st.serialize_field("message", &format!("{}", self))?;
        st.end()
    }
}

/// [InvalidLimit] is an error that represents that the there was a problem
/// with the given limit parameter.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "code", rename = "INVALID_LIMIT")]
pub struct InvalidLimit {}

impl InvalidLimit {
    pub fn status(&self) -> StatusCode {
        StatusCode::BadRequest
    }
}

impl ExplorerAPIError for InvalidLimit {
    fn code(&self) -> &str {
        "INVALID_LIMIT"
    }
}

impl Display for InvalidLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "limit must be provided, and must be a positive integer less than 100"
        )
    }
}

impl std::error::Error for InvalidLimit {}

impl Serialize for InvalidLimit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("InvalidLimit", 2)?;
        st.serialize_field("code", &self.code())?;
        st.serialize_field("message", &format!("{}", self))?;
        st.end()
    }
}

#[cfg(test)]
mod test {
    use super::Unimplemented;

    #[test]
    fn test_serialize_deserialize_unimplemented() {
        let unimplemented = Unimplemented {};
        {
            let serialize_result = serde_json::to_string(&unimplemented);
            assert!(
                serialize_result.is_ok(),
                "failed to serialize Unimplemented: {}",
                serialize_result.err().unwrap(),
            );
            let serialized = serialize_result.unwrap();
            {
                let have = &serialized;
                let want =
                    r#"{"code":"UNIMPLEMENTED","message":"this feature is not yet implemented"}"#;
                assert_eq!(
                    have, want,
                    "serialized Unimplemented mismatch: have: {have}, want: {want}"
                );
            }

            let deserialize_result: Result<Unimplemented, _> = serde_json::from_str(&serialized);
            assert!(
                deserialize_result.is_ok(),
                "failed to deserialize Unimplemented: {}",
                deserialize_result.err().unwrap(),
            );
            let deserialized = deserialize_result.unwrap();
            {
                let have = deserialized;
                let want = unimplemented;
                assert_eq!(
                    have, want,
                    "deserialized Unimplemented mismatch: have: {have}, want: {want}"
                );
            }
        }
    }
}
