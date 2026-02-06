mod progenitor_client;

#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///`DeleteByOrgApiKeysByIdId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteByOrgApiKeysByIdId(::std::string::String);
    impl ::std::ops::Deref for DeleteByOrgApiKeysByIdId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteByOrgApiKeysByIdId> for ::std::string::String {
        fn from(value: DeleteByOrgApiKeysByIdId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteByOrgApiKeysByIdId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteByOrgApiKeysByIdId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DeleteByOrgApiKeysByIdId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteByOrgApiKeysByIdId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteByOrgApiKeysByIdId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteByOrgApiKeysByIdResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "success": {
    ///      "type": "boolean",
    ///      "enum": [
    ///        true
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteByOrgApiKeysByIdResponse {
        pub success: bool,
    }

    impl DeleteByOrgApiKeysByIdResponse {
        pub fn builder() -> builder::DeleteByOrgApiKeysByIdResponse {
            Default::default()
        }
    }

    ///`DeleteByOrgApiKeysByIdResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteByOrgApiKeysByIdResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl DeleteByOrgApiKeysByIdResponseError {
        pub fn builder() -> builder::DeleteByOrgApiKeysByIdResponseError {
            Default::default()
        }
    }

    ///`DeleteByOrgByRepoBranchesByBranchBranch`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteByOrgByRepoBranchesByBranchBranch(::std::string::String);
    impl ::std::ops::Deref for DeleteByOrgByRepoBranchesByBranchBranch {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteByOrgByRepoBranchesByBranchBranch> for ::std::string::String {
        fn from(value: DeleteByOrgByRepoBranchesByBranchBranch) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteByOrgByRepoBranchesByBranchBranch {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteByOrgByRepoBranchesByBranchBranch {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DeleteByOrgByRepoBranchesByBranchBranch {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteByOrgByRepoBranchesByBranchBranch {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteByOrgByRepoBranchesByBranchBranch {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteByOrgByRepoBranchesByBranchOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteByOrgByRepoBranchesByBranchOrg(::std::string::String);
    impl ::std::ops::Deref for DeleteByOrgByRepoBranchesByBranchOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteByOrgByRepoBranchesByBranchOrg> for ::std::string::String {
        fn from(value: DeleteByOrgByRepoBranchesByBranchOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteByOrgByRepoBranchesByBranchOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteByOrgByRepoBranchesByBranchOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DeleteByOrgByRepoBranchesByBranchOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteByOrgByRepoBranchesByBranchOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteByOrgByRepoBranchesByBranchOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteByOrgByRepoBranchesByBranchRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteByOrgByRepoBranchesByBranchRepo(::std::string::String);
    impl ::std::ops::Deref for DeleteByOrgByRepoBranchesByBranchRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteByOrgByRepoBranchesByBranchRepo> for ::std::string::String {
        fn from(value: DeleteByOrgByRepoBranchesByBranchRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteByOrgByRepoBranchesByBranchRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteByOrgByRepoBranchesByBranchRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DeleteByOrgByRepoBranchesByBranchRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteByOrgByRepoBranchesByBranchRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteByOrgByRepoBranchesByBranchRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteByOrgByRepoBranchesByBranchResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "success": {
    ///      "type": "boolean",
    ///      "enum": [
    ///        true
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteByOrgByRepoBranchesByBranchResponse {
        pub success: bool,
    }

    impl DeleteByOrgByRepoBranchesByBranchResponse {
        pub fn builder() -> builder::DeleteByOrgByRepoBranchesByBranchResponse {
            Default::default()
        }
    }

    ///`DeleteByOrgByRepoBranchesByBranchResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteByOrgByRepoBranchesByBranchResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl DeleteByOrgByRepoBranchesByBranchResponseError {
        pub fn builder() -> builder::DeleteByOrgByRepoBranchesByBranchResponseError {
            Default::default()
        }
    }

    ///`DeleteByOrgByRepoOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteByOrgByRepoOrg(::std::string::String);
    impl ::std::ops::Deref for DeleteByOrgByRepoOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteByOrgByRepoOrg> for ::std::string::String {
        fn from(value: DeleteByOrgByRepoOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteByOrgByRepoOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteByOrgByRepoOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DeleteByOrgByRepoOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteByOrgByRepoOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteByOrgByRepoOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteByOrgByRepoRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteByOrgByRepoRepo(::std::string::String);
    impl ::std::ops::Deref for DeleteByOrgByRepoRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteByOrgByRepoRepo> for ::std::string::String {
        fn from(value: DeleteByOrgByRepoRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteByOrgByRepoRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteByOrgByRepoRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DeleteByOrgByRepoRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteByOrgByRepoRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteByOrgByRepoRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteByOrgByRepoResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "success": {
    ///      "type": "boolean",
    ///      "enum": [
    ///        true
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteByOrgByRepoResponse {
        pub success: bool,
    }

    impl DeleteByOrgByRepoResponse {
        pub fn builder() -> builder::DeleteByOrgByRepoResponse {
            Default::default()
        }
    }

    ///`DeleteByOrgByRepoResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteByOrgByRepoResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl DeleteByOrgByRepoResponseError {
        pub fn builder() -> builder::DeleteByOrgByRepoResponseError {
            Default::default()
        }
    }

    ///`DeleteByOrgByRepoWebhooksByWebhookIdOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteByOrgByRepoWebhooksByWebhookIdOrg(::std::string::String);
    impl ::std::ops::Deref for DeleteByOrgByRepoWebhooksByWebhookIdOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteByOrgByRepoWebhooksByWebhookIdOrg> for ::std::string::String {
        fn from(value: DeleteByOrgByRepoWebhooksByWebhookIdOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteByOrgByRepoWebhooksByWebhookIdOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteByOrgByRepoWebhooksByWebhookIdOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DeleteByOrgByRepoWebhooksByWebhookIdOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteByOrgByRepoWebhooksByWebhookIdOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteByOrgByRepoWebhooksByWebhookIdOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteByOrgByRepoWebhooksByWebhookIdRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteByOrgByRepoWebhooksByWebhookIdRepo(::std::string::String);
    impl ::std::ops::Deref for DeleteByOrgByRepoWebhooksByWebhookIdRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteByOrgByRepoWebhooksByWebhookIdRepo> for ::std::string::String {
        fn from(value: DeleteByOrgByRepoWebhooksByWebhookIdRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteByOrgByRepoWebhooksByWebhookIdRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteByOrgByRepoWebhooksByWebhookIdRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DeleteByOrgByRepoWebhooksByWebhookIdRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteByOrgByRepoWebhooksByWebhookIdRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteByOrgByRepoWebhooksByWebhookIdRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteByOrgByRepoWebhooksByWebhookIdResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "success": {
    ///      "type": "boolean",
    ///      "enum": [
    ///        true
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteByOrgByRepoWebhooksByWebhookIdResponse {
        pub success: bool,
    }

    impl DeleteByOrgByRepoWebhooksByWebhookIdResponse {
        pub fn builder() -> builder::DeleteByOrgByRepoWebhooksByWebhookIdResponse {
            Default::default()
        }
    }

    ///`DeleteByOrgByRepoWebhooksByWebhookIdResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteByOrgByRepoWebhooksByWebhookIdResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl DeleteByOrgByRepoWebhooksByWebhookIdResponseError {
        pub fn builder() -> builder::DeleteByOrgByRepoWebhooksByWebhookIdResponseError {
            Default::default()
        }
    }

    ///`DeleteByOrgByRepoWebhooksByWebhookIdWebhookId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteByOrgByRepoWebhooksByWebhookIdWebhookId(::std::string::String);
    impl ::std::ops::Deref for DeleteByOrgByRepoWebhooksByWebhookIdWebhookId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteByOrgByRepoWebhooksByWebhookIdWebhookId> for ::std::string::String {
        fn from(value: DeleteByOrgByRepoWebhooksByWebhookIdWebhookId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteByOrgByRepoWebhooksByWebhookIdWebhookId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteByOrgByRepoWebhooksByWebhookIdWebhookId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for DeleteByOrgByRepoWebhooksByWebhookIdWebhookId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for DeleteByOrgByRepoWebhooksByWebhookIdWebhookId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteByOrgByRepoWebhooksByWebhookIdWebhookId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgApiKeysResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "api_keys"
    ///  ],
    ///  "properties": {
    ///    "api_keys": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "created_at",
    ///          "expires_at",
    ///          "id",
    ///          "last_used_at",
    ///          "name",
    ///          "revoked_at",
    ///          "scopes"
    ///        ],
    ///        "properties": {
    ///          "created_at": {
    ///            "type": "string"
    ///          },
    ///          "expires_at": {
    ///            "type": [
    ///              "string",
    ///              "null"
    ///            ]
    ///          },
    ///          "id": {
    ///            "type": "string"
    ///          },
    ///          "last_used_at": {
    ///            "type": [
    ///              "string",
    ///              "null"
    ///            ]
    ///          },
    ///          "name": {
    ///            "type": [
    ///              "string",
    ///              "null"
    ///            ]
    ///          },
    ///          "revoked_at": {
    ///            "type": [
    ///              "string",
    ///              "null"
    ///            ]
    ///          },
    ///          "scopes": {
    ///            "type": "array",
    ///            "items": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgApiKeysResponse {
        pub api_keys: ::std::vec::Vec<GetByOrgApiKeysResponseApiKeysItem>,
    }

    impl GetByOrgApiKeysResponse {
        pub fn builder() -> builder::GetByOrgApiKeysResponse {
            Default::default()
        }
    }

    ///`GetByOrgApiKeysResponseApiKeysItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "expires_at",
    ///    "id",
    ///    "last_used_at",
    ///    "name",
    ///    "revoked_at",
    ///    "scopes"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "expires_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "last_used_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "revoked_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgApiKeysResponseApiKeysItem {
        pub created_at: ::std::string::String,
        pub expires_at: ::std::option::Option<::std::string::String>,
        pub id: ::std::string::String,
        pub last_used_at: ::std::option::Option<::std::string::String>,
        pub name: ::std::option::Option<::std::string::String>,
        pub revoked_at: ::std::option::Option<::std::string::String>,
        pub scopes: ::std::vec::Vec<::std::string::String>,
    }

    impl GetByOrgApiKeysResponseApiKeysItem {
        pub fn builder() -> builder::GetByOrgApiKeysResponseApiKeysItem {
            Default::default()
        }
    }

    ///`GetByOrgApiKeysResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgApiKeysResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgApiKeysResponseError {
        pub fn builder() -> builder::GetByOrgApiKeysResponseError {
            Default::default()
        }
    }

    ///`GetByOrgByRepoAgentblameBase`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoAgentblameBase(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoAgentblameBase {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoAgentblameBase> for ::std::string::String {
        fn from(value: GetByOrgByRepoAgentblameBase) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoAgentblameBase {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoAgentblameBase {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoAgentblameBase {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoAgentblameBase {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoAgentblameBase {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoAgentblameHead`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoAgentblameHead(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoAgentblameHead {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoAgentblameHead> for ::std::string::String {
        fn from(value: GetByOrgByRepoAgentblameHead) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoAgentblameHead {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoAgentblameHead {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoAgentblameHead {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoAgentblameHead {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoAgentblameHead {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoAgentblameOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoAgentblameOrg(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoAgentblameOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoAgentblameOrg> for ::std::string::String {
        fn from(value: GetByOrgByRepoAgentblameOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoAgentblameOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoAgentblameOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoAgentblameOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoAgentblameOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoAgentblameOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoAgentblameRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoAgentblameRepo(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoAgentblameRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoAgentblameRepo> for ::std::string::String {
        fn from(value: GetByOrgByRepoAgentblameRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoAgentblameRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoAgentblameRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoAgentblameRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoAgentblameRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoAgentblameRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoAgentblameResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "attributions",
    ///    "stats"
    ///  ],
    ///  "properties": {
    ///    "attributions": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "object",
    ///          "required": [
    ///            "confidence",
    ///            "lineNumber",
    ///            "match_type",
    ///            "model",
    ///            "provider"
    ///          ],
    ///          "properties": {
    ///            "confidence": {
    ///              "type": "number"
    ///            },
    ///            "lineNumber": {
    ///              "type": "number"
    ///            },
    ///            "match_type": {
    ///              "type": "string"
    ///            },
    ///            "model": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "provider": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "stats": {
    ///      "type": "object",
    ///      "required": [
    ///        "ai_lines",
    ///        "ai_percentage",
    ///        "human_lines",
    ///        "total_lines"
    ///      ],
    ///      "properties": {
    ///        "ai_lines": {
    ///          "type": "number"
    ///        },
    ///        "ai_percentage": {
    ///          "type": "number"
    ///        },
    ///        "human_lines": {
    ///          "type": "number"
    ///        },
    ///        "total_lines": {
    ///          "type": "number"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoAgentblameResponse {
        pub attributions: ::std::collections::HashMap<
            ::std::string::String,
            ::std::vec::Vec<GetByOrgByRepoAgentblameResponseAttributionsValueItem>,
        >,
        pub stats: GetByOrgByRepoAgentblameResponseStats,
    }

    impl GetByOrgByRepoAgentblameResponse {
        pub fn builder() -> builder::GetByOrgByRepoAgentblameResponse {
            Default::default()
        }
    }

    ///`GetByOrgByRepoAgentblameResponseAttributionsValueItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "confidence",
    ///    "lineNumber",
    ///    "match_type",
    ///    "model",
    ///    "provider"
    ///  ],
    ///  "properties": {
    ///    "confidence": {
    ///      "type": "number"
    ///    },
    ///    "lineNumber": {
    ///      "type": "number"
    ///    },
    ///    "match_type": {
    ///      "type": "string"
    ///    },
    ///    "model": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "provider": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoAgentblameResponseAttributionsValueItem {
        pub confidence: f64,
        #[serde(rename = "lineNumber")]
        pub line_number: f64,
        pub match_type: ::std::string::String,
        pub model: ::std::option::Option<::std::string::String>,
        pub provider: ::std::string::String,
    }

    impl GetByOrgByRepoAgentblameResponseAttributionsValueItem {
        pub fn builder() -> builder::GetByOrgByRepoAgentblameResponseAttributionsValueItem {
            Default::default()
        }
    }

    ///`GetByOrgByRepoAgentblameResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoAgentblameResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgByRepoAgentblameResponseError {
        pub fn builder() -> builder::GetByOrgByRepoAgentblameResponseError {
            Default::default()
        }
    }

    ///`GetByOrgByRepoAgentblameResponseStats`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "ai_lines",
    ///    "ai_percentage",
    ///    "human_lines",
    ///    "total_lines"
    ///  ],
    ///  "properties": {
    ///    "ai_lines": {
    ///      "type": "number"
    ///    },
    ///    "ai_percentage": {
    ///      "type": "number"
    ///    },
    ///    "human_lines": {
    ///      "type": "number"
    ///    },
    ///    "total_lines": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoAgentblameResponseStats {
        pub ai_lines: f64,
        pub ai_percentage: f64,
        pub human_lines: f64,
        pub total_lines: f64,
    }

    impl GetByOrgByRepoAgentblameResponseStats {
        pub fn builder() -> builder::GetByOrgByRepoAgentblameResponseStats {
            Default::default()
        }
    }

    ///`GetByOrgByRepoAnalyticsOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoAnalyticsOrg(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoAnalyticsOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoAnalyticsOrg> for ::std::string::String {
        fn from(value: GetByOrgByRepoAnalyticsOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoAnalyticsOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoAnalyticsOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoAnalyticsOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoAnalyticsOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoAnalyticsOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoAnalyticsPeriod`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "all",
    ///  "type": "string",
    ///  "enum": [
    ///    "24h",
    ///    "1w",
    ///    "1m",
    ///    "all"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetByOrgByRepoAnalyticsPeriod {
        #[serde(rename = "24h")]
        X24h,
        #[serde(rename = "1w")]
        X1w,
        #[serde(rename = "1m")]
        X1m,
        #[serde(rename = "all")]
        All,
    }

    impl ::std::fmt::Display for GetByOrgByRepoAnalyticsPeriod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X24h => f.write_str("24h"),
                Self::X1w => f.write_str("1w"),
                Self::X1m => f.write_str("1m"),
                Self::All => f.write_str("all"),
            }
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoAnalyticsPeriod {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "24h" => Ok(Self::X24h),
                "1w" => Ok(Self::X1w),
                "1m" => Ok(Self::X1m),
                "all" => Ok(Self::All),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoAnalyticsPeriod {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoAnalyticsPeriod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoAnalyticsPeriod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::default::Default for GetByOrgByRepoAnalyticsPeriod {
        fn default() -> Self {
            GetByOrgByRepoAnalyticsPeriod::All
        }
    }

    ///`GetByOrgByRepoAnalyticsRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoAnalyticsRepo(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoAnalyticsRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoAnalyticsRepo> for ::std::string::String {
        fn from(value: GetByOrgByRepoAnalyticsRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoAnalyticsRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoAnalyticsRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoAnalyticsRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoAnalyticsRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoAnalyticsRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoAnalyticsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "analytics",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "analytics": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "required": [
    ///        "contributors",
    ///        "history",
    ///        "summary",
    ///        "version"
    ///      ],
    ///      "properties": {
    ///        "_meta": {
    ///          "type": "object",
    ///          "required": [
    ///            "lastAttempt",
    ///            "lastError",
    ///            "lastSuccess"
    ///          ],
    ///          "properties": {
    ///            "lastAttempt": {
    ///              "type": "string"
    ///            },
    ///            "lastError": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "lastSuccess": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        "contributors": {
    ///          "type": "object",
    ///          "additionalProperties": {
    ///            "type": "object",
    ///            "required": [
    ///              "aiLines",
    ///              "commitCount",
    ///              "models",
    ///              "providers",
    ///              "totalLines"
    ///            ],
    ///            "properties": {
    ///              "aiLines": {
    ///                "type": "number"
    ///              },
    ///              "commitCount": {
    ///                "type": "number"
    ///              },
    ///              "models": {
    ///                "type": "object",
    ///                "additionalProperties": {
    ///                  "type": "number"
    ///                }
    ///              },
    ///              "providers": {
    ///                "type": "object",
    ///                "additionalProperties": {
    ///                  "type": "number"
    ///                }
    ///              },
    ///              "totalLines": {
    ///                "type": "number"
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "history": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "object",
    ///            "required": [
    ///              "added",
    ///              "aiLines",
    ///              "author",
    ///              "date",
    ///              "removed",
    ///              "sha"
    ///            ],
    ///            "properties": {
    ///              "added": {
    ///                "type": "number"
    ///              },
    ///              "aiLines": {
    ///                "type": "number"
    ///              },
    ///              "author": {
    ///                "type": "string"
    ///              },
    ///              "date": {
    ///                "type": "string"
    ///              },
    ///              "message": {
    ///                "type": "string"
    ///              },
    ///              "models": {
    ///                "type": "object",
    ///                "additionalProperties": {
    ///                  "type": "number"
    ///                }
    ///              },
    ///              "providers": {
    ///                "type": "object",
    ///                "additionalProperties": {
    ///                  "type": "number"
    ///                }
    ///              },
    ///              "removed": {
    ///                "type": "number"
    ///              },
    ///              "sha": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "summary": {
    ///          "type": "object",
    ///          "required": [
    ///            "aiLines",
    ///            "humanLines",
    ///            "models",
    ///            "providers",
    ///            "totalLines",
    ///            "updated"
    ///          ],
    ///          "properties": {
    ///            "aiLines": {
    ///              "type": "number"
    ///            },
    ///            "humanLines": {
    ///              "type": "number"
    ///            },
    ///            "models": {
    ///              "type": "object",
    ///              "additionalProperties": {
    ///                "type": "number"
    ///              }
    ///            },
    ///            "providers": {
    ///              "type": "object",
    ///              "additionalProperties": {
    ///                "type": "number"
    ///              }
    ///            },
    ///            "totalLines": {
    ///              "type": "number"
    ///            },
    ///            "updated": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        "version": {
    ///          "type": "number",
    ///          "enum": [
    ///            2.0
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "status": {
    ///      "type": "string",
    ///      "enum": [
    ///        "current",
    ///        "stale",
    ///        "none"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoAnalyticsResponse {
        pub analytics: ::std::option::Option<GetByOrgByRepoAnalyticsResponseAnalytics>,
        pub status: GetByOrgByRepoAnalyticsResponseStatus,
    }

    impl GetByOrgByRepoAnalyticsResponse {
        pub fn builder() -> builder::GetByOrgByRepoAnalyticsResponse {
            Default::default()
        }
    }

    ///`GetByOrgByRepoAnalyticsResponseAnalytics`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "contributors",
    ///    "history",
    ///    "summary",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "_meta": {
    ///      "type": "object",
    ///      "required": [
    ///        "lastAttempt",
    ///        "lastError",
    ///        "lastSuccess"
    ///      ],
    ///      "properties": {
    ///        "lastAttempt": {
    ///          "type": "string"
    ///        },
    ///        "lastError": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "lastSuccess": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "contributors": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "object",
    ///        "required": [
    ///          "aiLines",
    ///          "commitCount",
    ///          "models",
    ///          "providers",
    ///          "totalLines"
    ///        ],
    ///        "properties": {
    ///          "aiLines": {
    ///            "type": "number"
    ///          },
    ///          "commitCount": {
    ///            "type": "number"
    ///          },
    ///          "models": {
    ///            "type": "object",
    ///            "additionalProperties": {
    ///              "type": "number"
    ///            }
    ///          },
    ///          "providers": {
    ///            "type": "object",
    ///            "additionalProperties": {
    ///              "type": "number"
    ///            }
    ///          },
    ///          "totalLines": {
    ///            "type": "number"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "history": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "added",
    ///          "aiLines",
    ///          "author",
    ///          "date",
    ///          "removed",
    ///          "sha"
    ///        ],
    ///        "properties": {
    ///          "added": {
    ///            "type": "number"
    ///          },
    ///          "aiLines": {
    ///            "type": "number"
    ///          },
    ///          "author": {
    ///            "type": "string"
    ///          },
    ///          "date": {
    ///            "type": "string"
    ///          },
    ///          "message": {
    ///            "type": "string"
    ///          },
    ///          "models": {
    ///            "type": "object",
    ///            "additionalProperties": {
    ///              "type": "number"
    ///            }
    ///          },
    ///          "providers": {
    ///            "type": "object",
    ///            "additionalProperties": {
    ///              "type": "number"
    ///            }
    ///          },
    ///          "removed": {
    ///            "type": "number"
    ///          },
    ///          "sha": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "summary": {
    ///      "type": "object",
    ///      "required": [
    ///        "aiLines",
    ///        "humanLines",
    ///        "models",
    ///        "providers",
    ///        "totalLines",
    ///        "updated"
    ///      ],
    ///      "properties": {
    ///        "aiLines": {
    ///          "type": "number"
    ///        },
    ///        "humanLines": {
    ///          "type": "number"
    ///        },
    ///        "models": {
    ///          "type": "object",
    ///          "additionalProperties": {
    ///            "type": "number"
    ///          }
    ///        },
    ///        "providers": {
    ///          "type": "object",
    ///          "additionalProperties": {
    ///            "type": "number"
    ///          }
    ///        },
    ///        "totalLines": {
    ///          "type": "number"
    ///        },
    ///        "updated": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "version": {
    ///      "type": "number",
    ///      "enum": [
    ///        2.0
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoAnalyticsResponseAnalytics {
        pub contributors: ::std::collections::HashMap<
            ::std::string::String,
            GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue,
        >,
        pub history: ::std::vec::Vec<GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem>,
        #[serde(
            rename = "_meta",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub meta: ::std::option::Option<GetByOrgByRepoAnalyticsResponseAnalyticsMeta>,
        pub summary: GetByOrgByRepoAnalyticsResponseAnalyticsSummary,
        pub version: GetByOrgByRepoAnalyticsResponseAnalyticsVersion,
    }

    impl GetByOrgByRepoAnalyticsResponseAnalytics {
        pub fn builder() -> builder::GetByOrgByRepoAnalyticsResponseAnalytics {
            Default::default()
        }
    }

    ///`GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "aiLines",
    ///    "commitCount",
    ///    "models",
    ///    "providers",
    ///    "totalLines"
    ///  ],
    ///  "properties": {
    ///    "aiLines": {
    ///      "type": "number"
    ///    },
    ///    "commitCount": {
    ///      "type": "number"
    ///    },
    ///    "models": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "providers": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "totalLines": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue {
        #[serde(rename = "aiLines")]
        pub ai_lines: f64,
        #[serde(rename = "commitCount")]
        pub commit_count: f64,
        pub models: ::std::collections::HashMap<::std::string::String, f64>,
        pub providers: ::std::collections::HashMap<::std::string::String, f64>,
        #[serde(rename = "totalLines")]
        pub total_lines: f64,
    }

    impl GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue {
        pub fn builder() -> builder::GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue {
            Default::default()
        }
    }

    ///`GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "added",
    ///    "aiLines",
    ///    "author",
    ///    "date",
    ///    "removed",
    ///    "sha"
    ///  ],
    ///  "properties": {
    ///    "added": {
    ///      "type": "number"
    ///    },
    ///    "aiLines": {
    ///      "type": "number"
    ///    },
    ///    "author": {
    ///      "type": "string"
    ///    },
    ///    "date": {
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "models": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "providers": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "removed": {
    ///      "type": "number"
    ///    },
    ///    "sha": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem {
        pub added: f64,
        #[serde(rename = "aiLines")]
        pub ai_lines: f64,
        pub author: ::std::string::String,
        pub date: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub models: ::std::collections::HashMap<::std::string::String, f64>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub providers: ::std::collections::HashMap<::std::string::String, f64>,
        pub removed: f64,
        pub sha: ::std::string::String,
    }

    impl GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem {
        pub fn builder() -> builder::GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem {
            Default::default()
        }
    }

    ///`GetByOrgByRepoAnalyticsResponseAnalyticsMeta`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "lastAttempt",
    ///    "lastError",
    ///    "lastSuccess"
    ///  ],
    ///  "properties": {
    ///    "lastAttempt": {
    ///      "type": "string"
    ///    },
    ///    "lastError": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "lastSuccess": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoAnalyticsResponseAnalyticsMeta {
        #[serde(rename = "lastAttempt")]
        pub last_attempt: ::std::string::String,
        #[serde(rename = "lastError")]
        pub last_error: ::std::option::Option<::std::string::String>,
        #[serde(rename = "lastSuccess")]
        pub last_success: ::std::option::Option<::std::string::String>,
    }

    impl GetByOrgByRepoAnalyticsResponseAnalyticsMeta {
        pub fn builder() -> builder::GetByOrgByRepoAnalyticsResponseAnalyticsMeta {
            Default::default()
        }
    }

    ///`GetByOrgByRepoAnalyticsResponseAnalyticsSummary`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "aiLines",
    ///    "humanLines",
    ///    "models",
    ///    "providers",
    ///    "totalLines",
    ///    "updated"
    ///  ],
    ///  "properties": {
    ///    "aiLines": {
    ///      "type": "number"
    ///    },
    ///    "humanLines": {
    ///      "type": "number"
    ///    },
    ///    "models": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "providers": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "totalLines": {
    ///      "type": "number"
    ///    },
    ///    "updated": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoAnalyticsResponseAnalyticsSummary {
        #[serde(rename = "aiLines")]
        pub ai_lines: f64,
        #[serde(rename = "humanLines")]
        pub human_lines: f64,
        pub models: ::std::collections::HashMap<::std::string::String, f64>,
        pub providers: ::std::collections::HashMap<::std::string::String, f64>,
        #[serde(rename = "totalLines")]
        pub total_lines: f64,
        pub updated: ::std::string::String,
    }

    impl GetByOrgByRepoAnalyticsResponseAnalyticsSummary {
        pub fn builder() -> builder::GetByOrgByRepoAnalyticsResponseAnalyticsSummary {
            Default::default()
        }
    }

    ///`GetByOrgByRepoAnalyticsResponseAnalyticsVersion`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "number",
    ///  "enum": [
    ///    2.0
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoAnalyticsResponseAnalyticsVersion(f64);
    impl ::std::ops::Deref for GetByOrgByRepoAnalyticsResponseAnalyticsVersion {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoAnalyticsResponseAnalyticsVersion> for f64 {
        fn from(value: GetByOrgByRepoAnalyticsResponseAnalyticsVersion) -> Self {
            value.0
        }
    }

    impl ::std::convert::TryFrom<f64> for GetByOrgByRepoAnalyticsResponseAnalyticsVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![2.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoAnalyticsResponseAnalyticsVersion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }

    ///`GetByOrgByRepoAnalyticsResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoAnalyticsResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgByRepoAnalyticsResponseError {
        pub fn builder() -> builder::GetByOrgByRepoAnalyticsResponseError {
            Default::default()
        }
    }

    ///`GetByOrgByRepoAnalyticsResponseStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "current",
    ///    "stale",
    ///    "none"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetByOrgByRepoAnalyticsResponseStatus {
        #[serde(rename = "current")]
        Current,
        #[serde(rename = "stale")]
        Stale,
        #[serde(rename = "none")]
        None,
    }

    impl ::std::fmt::Display for GetByOrgByRepoAnalyticsResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Current => f.write_str("current"),
                Self::Stale => f.write_str("stale"),
                Self::None => f.write_str("none"),
            }
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoAnalyticsResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "current" => Ok(Self::Current),
                "stale" => Ok(Self::Stale),
                "none" => Ok(Self::None),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoAnalyticsResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoAnalyticsResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoAnalyticsResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetByOrgByRepoBranchesOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoBranchesOrg(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoBranchesOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoBranchesOrg> for ::std::string::String {
        fn from(value: GetByOrgByRepoBranchesOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoBranchesOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoBranchesOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoBranchesOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoBranchesOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoBranchesOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoBranchesRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoBranchesRepo(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoBranchesRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoBranchesRepo> for ::std::string::String {
        fn from(value: GetByOrgByRepoBranchesRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoBranchesRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoBranchesRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoBranchesRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoBranchesRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoBranchesRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoBranchesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "branches",
    ///    "has_more",
    ///    "next_cursor"
    ///  ],
    ///  "properties": {
    ///    "branches": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "head_sha",
    ///          "is_default",
    ///          "name"
    ///        ],
    ///        "properties": {
    ///          "head_sha": {
    ///            "type": "string"
    ///          },
    ///          "is_default": {
    ///            "type": "boolean"
    ///          },
    ///          "name": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "has_more": {
    ///      "type": "boolean"
    ///    },
    ///    "next_cursor": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoBranchesResponse {
        pub branches: ::std::vec::Vec<GetByOrgByRepoBranchesResponseBranchesItem>,
        pub has_more: bool,
        pub next_cursor: ::std::option::Option<::std::string::String>,
    }

    impl GetByOrgByRepoBranchesResponse {
        pub fn builder() -> builder::GetByOrgByRepoBranchesResponse {
            Default::default()
        }
    }

    ///`GetByOrgByRepoBranchesResponseBranchesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "head_sha",
    ///    "is_default",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "head_sha": {
    ///      "type": "string"
    ///    },
    ///    "is_default": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoBranchesResponseBranchesItem {
        pub head_sha: ::std::string::String,
        pub is_default: bool,
        pub name: ::std::string::String,
    }

    impl GetByOrgByRepoBranchesResponseBranchesItem {
        pub fn builder() -> builder::GetByOrgByRepoBranchesResponseBranchesItem {
            Default::default()
        }
    }

    ///`GetByOrgByRepoBranchesResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoBranchesResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgByRepoBranchesResponseError {
        pub fn builder() -> builder::GetByOrgByRepoBranchesResponseError {
            Default::default()
        }
    }

    ///`GetByOrgByRepoCommitsByShaOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoCommitsByShaOrg(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoCommitsByShaOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoCommitsByShaOrg> for ::std::string::String {
        fn from(value: GetByOrgByRepoCommitsByShaOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoCommitsByShaOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoCommitsByShaOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoCommitsByShaOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoCommitsByShaOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoCommitsByShaOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoCommitsByShaRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoCommitsByShaRepo(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoCommitsByShaRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoCommitsByShaRepo> for ::std::string::String {
        fn from(value: GetByOrgByRepoCommitsByShaRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoCommitsByShaRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoCommitsByShaRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoCommitsByShaRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoCommitsByShaRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoCommitsByShaRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoCommitsByShaResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "author",
    ///    "committer",
    ///    "message",
    ///    "sha"
    ///  ],
    ///  "properties": {
    ///    "author": {
    ///      "type": "object",
    ///      "required": [
    ///        "email",
    ///        "name"
    ///      ],
    ///      "properties": {
    ///        "date": {
    ///          "type": "string"
    ///        },
    ///        "email": {
    ///          "type": "string",
    ///          "format": "email",
    ///          "pattern":
    /// "^(?!\\.)(?!.*\\.\\.)([A-Za-z0-9_'+\\-\\.]*)[A-Za-z0-9_+-]@
    /// ([A-Za-z0-9][A-Za-z0-9\\-]*\\.)+[A-Za-z]{2,}$"
    ///        },
    ///        "name": {
    ///          "type": "string",
    ///          "minLength": 1
    ///        }
    ///      }
    ///    },
    ///    "committer": {
    ///      "type": "object",
    ///      "required": [
    ///        "email",
    ///        "name"
    ///      ],
    ///      "properties": {
    ///        "date": {
    ///          "type": "string"
    ///        },
    ///        "email": {
    ///          "type": "string",
    ///          "format": "email",
    ///          "pattern":
    /// "^(?!\\.)(?!.*\\.\\.)([A-Za-z0-9_'+\\-\\.]*)[A-Za-z0-9_+-]@
    /// ([A-Za-z0-9][A-Za-z0-9\\-]*\\.)+[A-Za-z]{2,}$"
    ///        },
    ///        "name": {
    ///          "type": "string",
    ///          "minLength": 1
    ///        }
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "sha": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoCommitsByShaResponse {
        pub author: GetByOrgByRepoCommitsByShaResponseAuthor,
        pub committer: GetByOrgByRepoCommitsByShaResponseCommitter,
        pub message: ::std::string::String,
        pub sha: ::std::string::String,
    }

    impl GetByOrgByRepoCommitsByShaResponse {
        pub fn builder() -> builder::GetByOrgByRepoCommitsByShaResponse {
            Default::default()
        }
    }

    ///`GetByOrgByRepoCommitsByShaResponseAuthor`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "date": {
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "type": "string",
    ///      "format": "email",
    ///      "pattern":
    /// "^(?!\\.)(?!.*\\.\\.)([A-Za-z0-9_'+\\-\\.]*)[A-Za-z0-9_+-]@
    /// ([A-Za-z0-9][A-Za-z0-9\\-]*\\.)+[A-Za-z]{2,}$"
    ///    },
    ///    "name": {
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoCommitsByShaResponseAuthor {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub date: ::std::option::Option<::std::string::String>,
        pub email: ::std::string::String,
        pub name: GetByOrgByRepoCommitsByShaResponseAuthorName,
    }

    impl GetByOrgByRepoCommitsByShaResponseAuthor {
        pub fn builder() -> builder::GetByOrgByRepoCommitsByShaResponseAuthor {
            Default::default()
        }
    }

    ///`GetByOrgByRepoCommitsByShaResponseAuthorName`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoCommitsByShaResponseAuthorName(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoCommitsByShaResponseAuthorName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoCommitsByShaResponseAuthorName> for ::std::string::String {
        fn from(value: GetByOrgByRepoCommitsByShaResponseAuthorName) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoCommitsByShaResponseAuthorName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoCommitsByShaResponseAuthorName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for GetByOrgByRepoCommitsByShaResponseAuthorName
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetByOrgByRepoCommitsByShaResponseAuthorName
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoCommitsByShaResponseAuthorName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoCommitsByShaResponseCommitter`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "date": {
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "type": "string",
    ///      "format": "email",
    ///      "pattern":
    /// "^(?!\\.)(?!.*\\.\\.)([A-Za-z0-9_'+\\-\\.]*)[A-Za-z0-9_+-]@
    /// ([A-Za-z0-9][A-Za-z0-9\\-]*\\.)+[A-Za-z]{2,}$"
    ///    },
    ///    "name": {
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoCommitsByShaResponseCommitter {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub date: ::std::option::Option<::std::string::String>,
        pub email: ::std::string::String,
        pub name: GetByOrgByRepoCommitsByShaResponseCommitterName,
    }

    impl GetByOrgByRepoCommitsByShaResponseCommitter {
        pub fn builder() -> builder::GetByOrgByRepoCommitsByShaResponseCommitter {
            Default::default()
        }
    }

    ///`GetByOrgByRepoCommitsByShaResponseCommitterName`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoCommitsByShaResponseCommitterName(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoCommitsByShaResponseCommitterName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoCommitsByShaResponseCommitterName>
        for ::std::string::String
    {
        fn from(value: GetByOrgByRepoCommitsByShaResponseCommitterName) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoCommitsByShaResponseCommitterName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoCommitsByShaResponseCommitterName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for GetByOrgByRepoCommitsByShaResponseCommitterName
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetByOrgByRepoCommitsByShaResponseCommitterName
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoCommitsByShaResponseCommitterName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoCommitsByShaResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoCommitsByShaResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgByRepoCommitsByShaResponseError {
        pub fn builder() -> builder::GetByOrgByRepoCommitsByShaResponseError {
            Default::default()
        }
    }

    ///`GetByOrgByRepoCommitsByShaSha`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoCommitsByShaSha(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoCommitsByShaSha {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoCommitsByShaSha> for ::std::string::String {
        fn from(value: GetByOrgByRepoCommitsByShaSha) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoCommitsByShaSha {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoCommitsByShaSha {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoCommitsByShaSha {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoCommitsByShaSha {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoCommitsByShaSha {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoCommitsOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoCommitsOrg(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoCommitsOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoCommitsOrg> for ::std::string::String {
        fn from(value: GetByOrgByRepoCommitsOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoCommitsOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoCommitsOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoCommitsOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoCommitsOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoCommitsOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoCommitsRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoCommitsRepo(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoCommitsRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoCommitsRepo> for ::std::string::String {
        fn from(value: GetByOrgByRepoCommitsRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoCommitsRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoCommitsRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoCommitsRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoCommitsRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoCommitsRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoCommitsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "commits",
    ///    "has_more",
    ///    "next_cursor"
    ///  ],
    ///  "properties": {
    ///    "commits": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "author",
    ///          "committer",
    ///          "message",
    ///          "sha"
    ///        ],
    ///        "properties": {
    ///          "author": {
    ///            "type": "object",
    ///            "required": [
    ///              "email",
    ///              "name"
    ///            ],
    ///            "properties": {
    ///              "date": {
    ///                "type": "string"
    ///              },
    ///              "email": {
    ///                "type": "string",
    ///                "format": "email",
    ///                "pattern":
    /// "^(?!\\.)(?!.*\\.\\.)([A-Za-z0-9_'+\\-\\.]*)[A-Za-z0-9_+-]@
    /// ([A-Za-z0-9][A-Za-z0-9\\-]*\\.)+[A-Za-z]{2,}$"
    ///              },
    ///              "name": {
    ///                "type": "string",
    ///                "minLength": 1
    ///              }
    ///            }
    ///          },
    ///          "committer": {
    ///            "type": "object",
    ///            "required": [
    ///              "email",
    ///              "name"
    ///            ],
    ///            "properties": {
    ///              "date": {
    ///                "type": "string"
    ///              },
    ///              "email": {
    ///                "type": "string",
    ///                "format": "email",
    ///                "pattern":
    /// "^(?!\\.)(?!.*\\.\\.)([A-Za-z0-9_'+\\-\\.]*)[A-Za-z0-9_+-]@
    /// ([A-Za-z0-9][A-Za-z0-9\\-]*\\.)+[A-Za-z]{2,}$"
    ///              },
    ///              "name": {
    ///                "type": "string",
    ///                "minLength": 1
    ///              }
    ///            }
    ///          },
    ///          "message": {
    ///            "type": "string"
    ///          },
    ///          "sha": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "has_more": {
    ///      "type": "boolean"
    ///    },
    ///    "next_cursor": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoCommitsResponse {
        pub commits: ::std::vec::Vec<GetByOrgByRepoCommitsResponseCommitsItem>,
        pub has_more: bool,
        pub next_cursor: ::std::option::Option<::std::string::String>,
    }

    impl GetByOrgByRepoCommitsResponse {
        pub fn builder() -> builder::GetByOrgByRepoCommitsResponse {
            Default::default()
        }
    }

    ///`GetByOrgByRepoCommitsResponseCommitsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "author",
    ///    "committer",
    ///    "message",
    ///    "sha"
    ///  ],
    ///  "properties": {
    ///    "author": {
    ///      "type": "object",
    ///      "required": [
    ///        "email",
    ///        "name"
    ///      ],
    ///      "properties": {
    ///        "date": {
    ///          "type": "string"
    ///        },
    ///        "email": {
    ///          "type": "string",
    ///          "format": "email",
    ///          "pattern":
    /// "^(?!\\.)(?!.*\\.\\.)([A-Za-z0-9_'+\\-\\.]*)[A-Za-z0-9_+-]@
    /// ([A-Za-z0-9][A-Za-z0-9\\-]*\\.)+[A-Za-z]{2,}$"
    ///        },
    ///        "name": {
    ///          "type": "string",
    ///          "minLength": 1
    ///        }
    ///      }
    ///    },
    ///    "committer": {
    ///      "type": "object",
    ///      "required": [
    ///        "email",
    ///        "name"
    ///      ],
    ///      "properties": {
    ///        "date": {
    ///          "type": "string"
    ///        },
    ///        "email": {
    ///          "type": "string",
    ///          "format": "email",
    ///          "pattern":
    /// "^(?!\\.)(?!.*\\.\\.)([A-Za-z0-9_'+\\-\\.]*)[A-Za-z0-9_+-]@
    /// ([A-Za-z0-9][A-Za-z0-9\\-]*\\.)+[A-Za-z]{2,}$"
    ///        },
    ///        "name": {
    ///          "type": "string",
    ///          "minLength": 1
    ///        }
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "sha": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoCommitsResponseCommitsItem {
        pub author: GetByOrgByRepoCommitsResponseCommitsItemAuthor,
        pub committer: GetByOrgByRepoCommitsResponseCommitsItemCommitter,
        pub message: ::std::string::String,
        pub sha: ::std::string::String,
    }

    impl GetByOrgByRepoCommitsResponseCommitsItem {
        pub fn builder() -> builder::GetByOrgByRepoCommitsResponseCommitsItem {
            Default::default()
        }
    }

    ///`GetByOrgByRepoCommitsResponseCommitsItemAuthor`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "date": {
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "type": "string",
    ///      "format": "email",
    ///      "pattern":
    /// "^(?!\\.)(?!.*\\.\\.)([A-Za-z0-9_'+\\-\\.]*)[A-Za-z0-9_+-]@
    /// ([A-Za-z0-9][A-Za-z0-9\\-]*\\.)+[A-Za-z]{2,}$"
    ///    },
    ///    "name": {
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoCommitsResponseCommitsItemAuthor {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub date: ::std::option::Option<::std::string::String>,
        pub email: ::std::string::String,
        pub name: GetByOrgByRepoCommitsResponseCommitsItemAuthorName,
    }

    impl GetByOrgByRepoCommitsResponseCommitsItemAuthor {
        pub fn builder() -> builder::GetByOrgByRepoCommitsResponseCommitsItemAuthor {
            Default::default()
        }
    }

    ///`GetByOrgByRepoCommitsResponseCommitsItemAuthorName`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoCommitsResponseCommitsItemAuthorName(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoCommitsResponseCommitsItemAuthorName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoCommitsResponseCommitsItemAuthorName>
        for ::std::string::String
    {
        fn from(value: GetByOrgByRepoCommitsResponseCommitsItemAuthorName) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoCommitsResponseCommitsItemAuthorName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoCommitsResponseCommitsItemAuthorName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for GetByOrgByRepoCommitsResponseCommitsItemAuthorName
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetByOrgByRepoCommitsResponseCommitsItemAuthorName
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoCommitsResponseCommitsItemAuthorName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoCommitsResponseCommitsItemCommitter`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "date": {
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "type": "string",
    ///      "format": "email",
    ///      "pattern":
    /// "^(?!\\.)(?!.*\\.\\.)([A-Za-z0-9_'+\\-\\.]*)[A-Za-z0-9_+-]@
    /// ([A-Za-z0-9][A-Za-z0-9\\-]*\\.)+[A-Za-z]{2,}$"
    ///    },
    ///    "name": {
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoCommitsResponseCommitsItemCommitter {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub date: ::std::option::Option<::std::string::String>,
        pub email: ::std::string::String,
        pub name: GetByOrgByRepoCommitsResponseCommitsItemCommitterName,
    }

    impl GetByOrgByRepoCommitsResponseCommitsItemCommitter {
        pub fn builder() -> builder::GetByOrgByRepoCommitsResponseCommitsItemCommitter {
            Default::default()
        }
    }

    ///`GetByOrgByRepoCommitsResponseCommitsItemCommitterName`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoCommitsResponseCommitsItemCommitterName(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoCommitsResponseCommitsItemCommitterName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoCommitsResponseCommitsItemCommitterName>
        for ::std::string::String
    {
        fn from(value: GetByOrgByRepoCommitsResponseCommitsItemCommitterName) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoCommitsResponseCommitsItemCommitterName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoCommitsResponseCommitsItemCommitterName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for GetByOrgByRepoCommitsResponseCommitsItemCommitterName
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetByOrgByRepoCommitsResponseCommitsItemCommitterName
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoCommitsResponseCommitsItemCommitterName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoCommitsResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoCommitsResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgByRepoCommitsResponseError {
        pub fn builder() -> builder::GetByOrgByRepoCommitsResponseError {
            Default::default()
        }
    }

    ///`GetByOrgByRepoContentOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoContentOrg(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoContentOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoContentOrg> for ::std::string::String {
        fn from(value: GetByOrgByRepoContentOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoContentOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoContentOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoContentOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoContentOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoContentOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoContentRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoContentRepo(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoContentRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoContentRepo> for ::std::string::String {
        fn from(value: GetByOrgByRepoContentRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoContentRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoContentRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoContentRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoContentRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoContentRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoContentResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "content",
    ///        "encoding",
    ///        "executable",
    ///        "last_commit_at",
    ///        "name",
    ///        "path",
    ///        "sha",
    ///        "size",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "content": {
    ///          "type": "string"
    ///        },
    ///        "encoding": {
    ///          "type": "string",
    ///          "enum": [
    ///            "base64"
    ///          ]
    ///        },
    ///        "executable": {
    ///          "type": "boolean"
    ///        },
    ///        "last_commit_at": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "date-time",
    ///          "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "sha": {
    ///          "type": "string"
    ///        },
    ///        "size": {
    ///          "type": "number"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "file"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "content",
    ///        "encoding",
    ///        "executable",
    ///        "last_commit_at",
    ///        "name",
    ///        "path",
    ///        "sha",
    ///        "size",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "content": {
    ///          "type": "string"
    ///        },
    ///        "encoding": {
    ///          "type": "string",
    ///          "enum": [
    ///            "base64"
    ///          ]
    ///        },
    ///        "executable": {
    ///          "type": "boolean"
    ///        },
    ///        "last_commit_at": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "date-time",
    ///          "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "sha": {
    ///          "type": "string"
    ///        },
    ///        "size": {
    ///          "type": "number"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "symlink"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "child_count",
    ///        "entries",
    ///        "name",
    ///        "path",
    ///        "sha",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "child_count": {
    ///          "type": "integer",
    ///          "maximum": 9007199254740991.0,
    ///          "minimum": -9007199254740991.0
    ///        },
    ///        "entries": {
    ///          "type": "array",
    ///          "items": {
    ///            "anyOf": [
    ///              {
    ///                "type": "object",
    ///                "required": [
    ///                  "executable",
    ///                  "last_commit_at",
    ///                  "name",
    ///                  "path",
    ///                  "sha",
    ///                  "size",
    ///                  "type"
    ///                ],
    ///                "properties": {
    ///                  "executable": {
    ///                    "type": "boolean"
    ///                  },
    ///                  "last_commit_at": {
    ///                    "type": [
    ///                      "string",
    ///                      "null"
    ///                    ],
    ///                    "format": "date-time",
    ///                    "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///                  },
    ///                  "name": {
    ///                    "type": "string"
    ///                  },
    ///                  "path": {
    ///                    "type": "string"
    ///                  },
    ///                  "sha": {
    ///                    "type": "string"
    ///                  },
    ///                  "size": {
    ///                    "type": "number"
    ///                  },
    ///                  "type": {
    ///                    "type": "string",
    ///                    "enum": [
    ///                      "file"
    ///                    ]
    ///                  }
    ///                }
    ///              },
    ///              {
    ///                "type": "object",
    ///                "required": [
    ///                  "executable",
    ///                  "last_commit_at",
    ///                  "name",
    ///                  "path",
    ///                  "sha",
    ///                  "size",
    ///                  "type"
    ///                ],
    ///                "properties": {
    ///                  "executable": {
    ///                    "type": "boolean"
    ///                  },
    ///                  "last_commit_at": {
    ///                    "type": [
    ///                      "string",
    ///                      "null"
    ///                    ],
    ///                    "format": "date-time",
    ///                    "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///                  },
    ///                  "name": {
    ///                    "type": "string"
    ///                  },
    ///                  "path": {
    ///                    "type": "string"
    ///                  },
    ///                  "sha": {
    ///                    "type": "string"
    ///                  },
    ///                  "size": {
    ///                    "type": "number"
    ///                  },
    ///                  "type": {
    ///                    "type": "string",
    ///                    "enum": [
    ///                      "symlink"
    ///                    ]
    ///                  }
    ///                }
    ///              },
    ///              {
    ///                "type": "object",
    ///                "required": [
    ///                  "name",
    ///                  "path",
    ///                  "sha",
    ///                  "type"
    ///                ],
    ///                "properties": {
    ///                  "name": {
    ///                    "type": "string"
    ///                  },
    ///                  "path": {
    ///                    "type": "string"
    ///                  },
    ///                  "sha": {
    ///                    "type": "string"
    ///                  },
    ///                  "type": {
    ///                    "type": "string",
    ///                    "enum": [
    ///                      "dir"
    ///                    ]
    ///                  }
    ///                }
    ///              }
    ///            ]
    ///          }
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "sha": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "dir"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum GetByOrgByRepoContentResponse {
        #[serde(rename = "file")]
        File {
            content: ::std::string::String,
            encoding: GetByOrgByRepoContentResponseEncoding,
            executable: bool,
            last_commit_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            name: ::std::string::String,
            path: ::std::string::String,
            sha: ::std::string::String,
            size: f64,
        },
        #[serde(rename = "symlink")]
        Symlink {
            content: ::std::string::String,
            encoding: GetByOrgByRepoContentResponseEncoding,
            executable: bool,
            last_commit_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            name: ::std::string::String,
            path: ::std::string::String,
            sha: ::std::string::String,
            size: f64,
        },
        #[serde(rename = "dir")]
        Dir {
            child_count: i64,
            entries: ::std::vec::Vec<GetByOrgByRepoContentResponseEntriesItem>,
            name: ::std::string::String,
            path: ::std::string::String,
            sha: ::std::string::String,
        },
    }

    ///`GetByOrgByRepoContentResponseEncoding`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "base64"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetByOrgByRepoContentResponseEncoding {
        #[serde(rename = "base64")]
        Base64,
    }

    impl ::std::fmt::Display for GetByOrgByRepoContentResponseEncoding {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Base64 => f.write_str("base64"),
            }
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoContentResponseEncoding {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "base64" => Ok(Self::Base64),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoContentResponseEncoding {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoContentResponseEncoding {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoContentResponseEncoding {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetByOrgByRepoContentResponseEntriesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "executable",
    ///        "last_commit_at",
    ///        "name",
    ///        "path",
    ///        "sha",
    ///        "size",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "executable": {
    ///          "type": "boolean"
    ///        },
    ///        "last_commit_at": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "date-time",
    ///          "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "sha": {
    ///          "type": "string"
    ///        },
    ///        "size": {
    ///          "type": "number"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "file"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "executable",
    ///        "last_commit_at",
    ///        "name",
    ///        "path",
    ///        "sha",
    ///        "size",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "executable": {
    ///          "type": "boolean"
    ///        },
    ///        "last_commit_at": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "date-time",
    ///          "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "sha": {
    ///          "type": "string"
    ///        },
    ///        "size": {
    ///          "type": "number"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "symlink"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "name",
    ///        "path",
    ///        "sha",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "sha": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "dir"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum GetByOrgByRepoContentResponseEntriesItem {
        #[serde(rename = "file")]
        File {
            executable: bool,
            last_commit_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            name: ::std::string::String,
            path: ::std::string::String,
            sha: ::std::string::String,
            size: f64,
        },
        #[serde(rename = "symlink")]
        Symlink {
            executable: bool,
            last_commit_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            name: ::std::string::String,
            path: ::std::string::String,
            sha: ::std::string::String,
            size: f64,
        },
        #[serde(rename = "dir")]
        Dir {
            name: ::std::string::String,
            path: ::std::string::String,
            sha: ::std::string::String,
        },
    }

    ///`GetByOrgByRepoContentResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoContentResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgByRepoContentResponseError {
        pub fn builder() -> builder::GetByOrgByRepoContentResponseError {
            Default::default()
        }
    }

    ///`GetByOrgByRepoDiffBase`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoDiffBase(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoDiffBase {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoDiffBase> for ::std::string::String {
        fn from(value: GetByOrgByRepoDiffBase) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoDiffBase {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoDiffBase {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoDiffBase {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoDiffBase {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoDiffBase {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoDiffHead`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoDiffHead(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoDiffHead {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoDiffHead> for ::std::string::String {
        fn from(value: GetByOrgByRepoDiffHead) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoDiffHead {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoDiffHead {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoDiffHead {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoDiffHead {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoDiffHead {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoDiffOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoDiffOrg(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoDiffOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoDiffOrg> for ::std::string::String {
        fn from(value: GetByOrgByRepoDiffOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoDiffOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoDiffOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoDiffOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoDiffOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoDiffOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoDiffRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoDiffRepo(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoDiffRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoDiffRepo> for ::std::string::String {
        fn from(value: GetByOrgByRepoDiffRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoDiffRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoDiffRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoDiffRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoDiffRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoDiffRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoDiffResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "base",
    ///    "files",
    ///    "filtered_files",
    ///    "head",
    ///    "stats"
    ///  ],
    ///  "properties": {
    ///    "base": {
    ///      "type": "string"
    ///    },
    ///    "files": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "path",
    ///          "status"
    ///        ],
    ///        "properties": {
    ///          "bytes": {
    ///            "type": "number"
    ///          },
    ///          "is_eof": {
    ///            "type": "boolean"
    ///          },
    ///          "old_path": {
    ///            "type": "string"
    ///          },
    ///          "path": {
    ///            "type": "string"
    ///          },
    ///          "raw": {
    ///            "type": "string"
    ///          },
    ///          "status": {
    ///            "type": "string",
    ///            "enum": [
    ///              "A",
    ///              "M",
    ///              "D",
    ///              "R",
    ///              "C",
    ///              "T"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "filtered_files": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "path",
    ///          "status"
    ///        ],
    ///        "properties": {
    ///          "bytes": {
    ///            "type": "number"
    ///          },
    ///          "is_eof": {
    ///            "type": "boolean"
    ///          },
    ///          "old_path": {
    ///            "type": "string"
    ///          },
    ///          "path": {
    ///            "type": "string"
    ///          },
    ///          "status": {
    ///            "type": "string",
    ///            "enum": [
    ///              "A",
    ///              "M",
    ///              "D",
    ///              "R",
    ///              "C",
    ///              "T"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "head": {
    ///      "type": "string"
    ///    },
    ///    "stats": {
    ///      "type": "object",
    ///      "required": [
    ///        "additions",
    ///        "changes",
    ///        "deletions",
    ///        "files"
    ///      ],
    ///      "properties": {
    ///        "additions": {
    ///          "type": "number"
    ///        },
    ///        "changes": {
    ///          "type": "number"
    ///        },
    ///        "deletions": {
    ///          "type": "number"
    ///        },
    ///        "files": {
    ///          "type": "number"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoDiffResponse {
        pub base: ::std::string::String,
        pub files: ::std::vec::Vec<GetByOrgByRepoDiffResponseFilesItem>,
        pub filtered_files: ::std::vec::Vec<GetByOrgByRepoDiffResponseFilteredFilesItem>,
        pub head: ::std::string::String,
        pub stats: GetByOrgByRepoDiffResponseStats,
    }

    impl GetByOrgByRepoDiffResponse {
        pub fn builder() -> builder::GetByOrgByRepoDiffResponse {
            Default::default()
        }
    }

    ///`GetByOrgByRepoDiffResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoDiffResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgByRepoDiffResponseError {
        pub fn builder() -> builder::GetByOrgByRepoDiffResponseError {
            Default::default()
        }
    }

    ///`GetByOrgByRepoDiffResponseFilesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "path",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "bytes": {
    ///      "type": "number"
    ///    },
    ///    "is_eof": {
    ///      "type": "boolean"
    ///    },
    ///    "old_path": {
    ///      "type": "string"
    ///    },
    ///    "path": {
    ///      "type": "string"
    ///    },
    ///    "raw": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string",
    ///      "enum": [
    ///        "A",
    ///        "M",
    ///        "D",
    ///        "R",
    ///        "C",
    ///        "T"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoDiffResponseFilesItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bytes: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_eof: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub old_path: ::std::option::Option<::std::string::String>,
        pub path: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub raw: ::std::option::Option<::std::string::String>,
        pub status: GetByOrgByRepoDiffResponseFilesItemStatus,
    }

    impl GetByOrgByRepoDiffResponseFilesItem {
        pub fn builder() -> builder::GetByOrgByRepoDiffResponseFilesItem {
            Default::default()
        }
    }

    ///`GetByOrgByRepoDiffResponseFilesItemStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "A",
    ///    "M",
    ///    "D",
    ///    "R",
    ///    "C",
    ///    "T"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetByOrgByRepoDiffResponseFilesItemStatus {
        A,
        M,
        D,
        R,
        C,
        T,
    }

    impl ::std::fmt::Display for GetByOrgByRepoDiffResponseFilesItemStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::A => f.write_str("A"),
                Self::M => f.write_str("M"),
                Self::D => f.write_str("D"),
                Self::R => f.write_str("R"),
                Self::C => f.write_str("C"),
                Self::T => f.write_str("T"),
            }
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoDiffResponseFilesItemStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "A" => Ok(Self::A),
                "M" => Ok(Self::M),
                "D" => Ok(Self::D),
                "R" => Ok(Self::R),
                "C" => Ok(Self::C),
                "T" => Ok(Self::T),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoDiffResponseFilesItemStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoDiffResponseFilesItemStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoDiffResponseFilesItemStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetByOrgByRepoDiffResponseFilteredFilesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "path",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "bytes": {
    ///      "type": "number"
    ///    },
    ///    "is_eof": {
    ///      "type": "boolean"
    ///    },
    ///    "old_path": {
    ///      "type": "string"
    ///    },
    ///    "path": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string",
    ///      "enum": [
    ///        "A",
    ///        "M",
    ///        "D",
    ///        "R",
    ///        "C",
    ///        "T"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoDiffResponseFilteredFilesItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bytes: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_eof: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub old_path: ::std::option::Option<::std::string::String>,
        pub path: ::std::string::String,
        pub status: GetByOrgByRepoDiffResponseFilteredFilesItemStatus,
    }

    impl GetByOrgByRepoDiffResponseFilteredFilesItem {
        pub fn builder() -> builder::GetByOrgByRepoDiffResponseFilteredFilesItem {
            Default::default()
        }
    }

    ///`GetByOrgByRepoDiffResponseFilteredFilesItemStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "A",
    ///    "M",
    ///    "D",
    ///    "R",
    ///    "C",
    ///    "T"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetByOrgByRepoDiffResponseFilteredFilesItemStatus {
        A,
        M,
        D,
        R,
        C,
        T,
    }

    impl ::std::fmt::Display for GetByOrgByRepoDiffResponseFilteredFilesItemStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::A => f.write_str("A"),
                Self::M => f.write_str("M"),
                Self::D => f.write_str("D"),
                Self::R => f.write_str("R"),
                Self::C => f.write_str("C"),
                Self::T => f.write_str("T"),
            }
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoDiffResponseFilteredFilesItemStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "A" => Ok(Self::A),
                "M" => Ok(Self::M),
                "D" => Ok(Self::D),
                "R" => Ok(Self::R),
                "C" => Ok(Self::C),
                "T" => Ok(Self::T),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoDiffResponseFilteredFilesItemStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for GetByOrgByRepoDiffResponseFilteredFilesItemStatus
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetByOrgByRepoDiffResponseFilteredFilesItemStatus
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetByOrgByRepoDiffResponseStats`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "additions",
    ///    "changes",
    ///    "deletions",
    ///    "files"
    ///  ],
    ///  "properties": {
    ///    "additions": {
    ///      "type": "number"
    ///    },
    ///    "changes": {
    ///      "type": "number"
    ///    },
    ///    "deletions": {
    ///      "type": "number"
    ///    },
    ///    "files": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoDiffResponseStats {
        pub additions: f64,
        pub changes: f64,
        pub deletions: f64,
        pub files: f64,
    }

    impl GetByOrgByRepoDiffResponseStats {
        pub fn builder() -> builder::GetByOrgByRepoDiffResponseStats {
            Default::default()
        }
    }

    ///`GetByOrgByRepoOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoOrg(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoOrg> for ::std::string::String {
        fn from(value: GetByOrgByRepoOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoRepo(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoRepo> for ::std::string::String {
        fn from(value: GetByOrgByRepoRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "default_branch",
    ///    "id",
    ///    "last_push_at",
    ///    "name",
    ///    "org",
    ///    "size_bytes",
    ///    "upstream"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "default_branch": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "last_push_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "org": {
    ///      "type": "string"
    ///    },
    ///    "size_bytes": {
    ///      "type": "number"
    ///    },
    ///    "upstream": {
    ///      "description": "Optionally add an upstream repository. You can
    /// configure automatic syncing from the upstream repository.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "required": [
    ///        "autosync",
    ///        "last_sync_attempt",
    ///        "last_sync_error",
    ///        "last_sync_success",
    ///        "uri"
    ///      ],
    ///      "properties": {
    ///        "autosync": {
    ///          "description": "Automatic sync configuration, if enabled",
    ///          "type": [
    ///            "object",
    ///            "null"
    ///          ],
    ///          "required": [
    ///            "period",
    ///            "resolution_strategy",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "period": {
    ///              "description": "Polling period in seconds",
    ///              "type": "number"
    ///            },
    ///            "resolution_strategy": {
    ///              "description": "Conflict resolution strategy. \"none\"
    /// means sync will fail on conflicts.",
    ///              "type": "string",
    ///              "enum": [
    ///                "none"
    ///              ]
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "poll"
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        "last_sync_attempt": {
    ///          "description": "Timestamp of the last sync attempt",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "date-time",
    ///          "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///        },
    ///        "last_sync_error": {
    ///          "description": "Error message from the last failed sync
    /// attempt",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "last_sync_success": {
    ///          "description": "Timestamp of the last successful sync",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "date-time",
    ///          "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///        },
    ///        "uri": {
    ///          "description": "URL of the upstream repository",
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoResponse {
        pub created_at: ::std::string::String,
        pub default_branch: ::std::string::String,
        pub id: ::std::string::String,
        pub last_push_at: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
        pub org: ::std::string::String,
        pub size_bytes: f64,
        ///Optionally add an upstream repository. You can configure automatic
        /// syncing from the upstream repository.
        pub upstream: ::std::option::Option<GetByOrgByRepoResponseUpstream>,
    }

    impl GetByOrgByRepoResponse {
        pub fn builder() -> builder::GetByOrgByRepoResponse {
            Default::default()
        }
    }

    ///`GetByOrgByRepoResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgByRepoResponseError {
        pub fn builder() -> builder::GetByOrgByRepoResponseError {
            Default::default()
        }
    }

    ///Optionally add an upstream repository. You can configure automatic
    /// syncing from the upstream repository.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Optionally add an upstream repository. You can
    /// configure automatic syncing from the upstream repository.",
    ///  "type": "object",
    ///  "required": [
    ///    "autosync",
    ///    "last_sync_attempt",
    ///    "last_sync_error",
    ///    "last_sync_success",
    ///    "uri"
    ///  ],
    ///  "properties": {
    ///    "autosync": {
    ///      "description": "Automatic sync configuration, if enabled",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "required": [
    ///        "period",
    ///        "resolution_strategy",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "period": {
    ///          "description": "Polling period in seconds",
    ///          "type": "number"
    ///        },
    ///        "resolution_strategy": {
    ///          "description": "Conflict resolution strategy. \"none\" means
    /// sync will fail on conflicts.",
    ///          "type": "string",
    ///          "enum": [
    ///            "none"
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "poll"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "last_sync_attempt": {
    ///      "description": "Timestamp of the last sync attempt",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time",
    ///      "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///    },
    ///    "last_sync_error": {
    ///      "description": "Error message from the last failed sync attempt",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "last_sync_success": {
    ///      "description": "Timestamp of the last successful sync",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time",
    ///      "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///    },
    ///    "uri": {
    ///      "description": "URL of the upstream repository",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoResponseUpstream {
        ///Automatic sync configuration, if enabled
        pub autosync: ::std::option::Option<GetByOrgByRepoResponseUpstreamAutosync>,
        ///Timestamp of the last sync attempt
        pub last_sync_attempt: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Error message from the last failed sync attempt
        pub last_sync_error: ::std::option::Option<::std::string::String>,
        ///Timestamp of the last successful sync
        pub last_sync_success: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///URL of the upstream repository
        pub uri: ::std::string::String,
    }

    impl GetByOrgByRepoResponseUpstream {
        pub fn builder() -> builder::GetByOrgByRepoResponseUpstream {
            Default::default()
        }
    }

    ///Automatic sync configuration, if enabled
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Automatic sync configuration, if enabled",
    ///  "type": "object",
    ///  "required": [
    ///    "period",
    ///    "resolution_strategy",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "period": {
    ///      "description": "Polling period in seconds",
    ///      "type": "number"
    ///    },
    ///    "resolution_strategy": {
    ///      "description": "Conflict resolution strategy. \"none\" means sync
    /// will fail on conflicts.",
    ///      "type": "string",
    ///      "enum": [
    ///        "none"
    ///      ]
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "poll"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoResponseUpstreamAutosync {
        pub period: f64,
        ///Conflict resolution strategy. "none" means sync will fail on
        /// conflicts.
        pub resolution_strategy: GetByOrgByRepoResponseUpstreamAutosyncResolutionStrategy,
        #[serde(rename = "type")]
        pub type_: GetByOrgByRepoResponseUpstreamAutosyncType,
    }

    impl GetByOrgByRepoResponseUpstreamAutosync {
        pub fn builder() -> builder::GetByOrgByRepoResponseUpstreamAutosync {
            Default::default()
        }
    }

    ///Conflict resolution strategy. "none" means sync will fail on conflicts.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Conflict resolution strategy. \"none\" means sync will
    /// fail on conflicts.",
    ///  "type": "string",
    ///  "enum": [
    ///    "none"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetByOrgByRepoResponseUpstreamAutosyncResolutionStrategy {
        #[serde(rename = "none")]
        None,
    }

    impl ::std::fmt::Display for GetByOrgByRepoResponseUpstreamAutosyncResolutionStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("none"),
            }
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoResponseUpstreamAutosyncResolutionStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "none" => Ok(Self::None),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoResponseUpstreamAutosyncResolutionStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for GetByOrgByRepoResponseUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetByOrgByRepoResponseUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetByOrgByRepoResponseUpstreamAutosyncType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "poll"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetByOrgByRepoResponseUpstreamAutosyncType {
        #[serde(rename = "poll")]
        Poll,
    }

    impl ::std::fmt::Display for GetByOrgByRepoResponseUpstreamAutosyncType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Poll => f.write_str("poll"),
            }
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoResponseUpstreamAutosyncType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "poll" => Ok(Self::Poll),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoResponseUpstreamAutosyncType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for GetByOrgByRepoResponseUpstreamAutosyncType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoResponseUpstreamAutosyncType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetByOrgByRepoSyncOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoSyncOrg(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoSyncOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoSyncOrg> for ::std::string::String {
        fn from(value: GetByOrgByRepoSyncOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoSyncOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoSyncOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoSyncOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoSyncOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoSyncOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoSyncRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoSyncRepo(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoSyncRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoSyncRepo> for ::std::string::String {
        fn from(value: GetByOrgByRepoSyncRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoSyncRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoSyncRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoSyncRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoSyncRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoSyncRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoSyncResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "last_sync_attempt",
    ///    "last_sync_success",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "last_sync_attempt": {
    ///      "description": "Timestamp of the last sync attempt",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time",
    ///      "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///    },
    ///    "last_sync_success": {
    ///      "description": "Timestamp of the last successful sync",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time",
    ///      "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///    },
    ///    "status": {
    ///      "description": "What the status of the last sync was.",
    ///      "oneOf": [
    ///        {
    ///          "description": "The last sync was successful.",
    ///          "type": "object",
    ///          "required": [
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "ok"
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "description": "The last sync failed with an error.",
    ///          "type": "object",
    ///          "required": [
    ///            "message",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "description": "Error message from the last failed sync
    /// attempt",
    ///              "type": "string"
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "error"
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoSyncResponse {
        ///Timestamp of the last sync attempt
        pub last_sync_attempt: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Timestamp of the last successful sync
        pub last_sync_success: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///What the status of the last sync was.
        pub status: GetByOrgByRepoSyncResponseStatus,
    }

    impl GetByOrgByRepoSyncResponse {
        pub fn builder() -> builder::GetByOrgByRepoSyncResponse {
            Default::default()
        }
    }

    ///`GetByOrgByRepoSyncResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoSyncResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgByRepoSyncResponseError {
        pub fn builder() -> builder::GetByOrgByRepoSyncResponseError {
            Default::default()
        }
    }

    ///What the status of the last sync was.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "What the status of the last sync was.",
    ///  "oneOf": [
    ///    {
    ///      "description": "The last sync was successful.",
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "ok"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "The last sync failed with an error.",
    ///      "type": "object",
    ///      "required": [
    ///        "message",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "message": {
    ///          "description": "Error message from the last failed sync
    /// attempt",
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "error"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "message")]
    pub enum GetByOrgByRepoSyncResponseStatus {
        #[serde(rename = "ok")]
        Ok,
        ///The last sync failed with an error.
        #[serde(rename = "error")]
        Error(::std::string::String),
    }

    ///`GetByOrgByRepoWebhooksOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoWebhooksOrg(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoWebhooksOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoWebhooksOrg> for ::std::string::String {
        fn from(value: GetByOrgByRepoWebhooksOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoWebhooksOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoWebhooksOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoWebhooksOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoWebhooksOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoWebhooksOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoWebhooksRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgByRepoWebhooksRepo(::std::string::String);
    impl ::std::ops::Deref for GetByOrgByRepoWebhooksRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgByRepoWebhooksRepo> for ::std::string::String {
        fn from(value: GetByOrgByRepoWebhooksRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoWebhooksRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoWebhooksRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgByRepoWebhooksRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgByRepoWebhooksRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgByRepoWebhooksRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgByRepoWebhooksResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "webhooks"
    ///  ],
    ///  "properties": {
    ///    "webhooks": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "branches",
    ///          "created_at",
    ///          "events",
    ///          "globs",
    ///          "id",
    ///          "updated_at",
    ///          "url"
    ///        ],
    ///        "properties": {
    ///          "branches": {
    ///            "type": [
    ///              "array",
    ///              "null"
    ///            ],
    ///            "items": {
    ///              "type": "string"
    ///            }
    ///          },
    ///          "created_at": {
    ///            "type": "string"
    ///          },
    ///          "events": {
    ///            "type": "array",
    ///            "items": {
    ///              "type": "string",
    ///              "enum": [
    ///                "push"
    ///              ]
    ///            }
    ///          },
    ///          "globs": {
    ///            "type": [
    ///              "array",
    ///              "null"
    ///            ],
    ///            "items": {
    ///              "type": "string"
    ///            }
    ///          },
    ///          "id": {
    ///            "type": "string"
    ///          },
    ///          "updated_at": {
    ///            "type": "string"
    ///          },
    ///          "url": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoWebhooksResponse {
        pub webhooks: ::std::vec::Vec<GetByOrgByRepoWebhooksResponseWebhooksItem>,
    }

    impl GetByOrgByRepoWebhooksResponse {
        pub fn builder() -> builder::GetByOrgByRepoWebhooksResponse {
            Default::default()
        }
    }

    ///`GetByOrgByRepoWebhooksResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoWebhooksResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgByRepoWebhooksResponseError {
        pub fn builder() -> builder::GetByOrgByRepoWebhooksResponseError {
            Default::default()
        }
    }

    ///`GetByOrgByRepoWebhooksResponseWebhooksItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "branches",
    ///    "created_at",
    ///    "events",
    ///    "globs",
    ///    "id",
    ///    "updated_at",
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "branches": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "events": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string",
    ///        "enum": [
    ///          "push"
    ///        ]
    ///      }
    ///    },
    ///    "globs": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "url": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgByRepoWebhooksResponseWebhooksItem {
        pub branches: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        pub created_at: ::std::string::String,
        pub events: ::std::vec::Vec<GetByOrgByRepoWebhooksResponseWebhooksItemEventsItem>,
        pub globs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        pub id: ::std::string::String,
        pub updated_at: ::std::string::String,
        pub url: ::std::string::String,
    }

    impl GetByOrgByRepoWebhooksResponseWebhooksItem {
        pub fn builder() -> builder::GetByOrgByRepoWebhooksResponseWebhooksItem {
            Default::default()
        }
    }

    ///`GetByOrgByRepoWebhooksResponseWebhooksItemEventsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "push"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetByOrgByRepoWebhooksResponseWebhooksItemEventsItem {
        #[serde(rename = "push")]
        Push,
    }

    impl ::std::fmt::Display for GetByOrgByRepoWebhooksResponseWebhooksItemEventsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Push => f.write_str("push"),
            }
        }
    }

    impl ::std::str::FromStr for GetByOrgByRepoWebhooksResponseWebhooksItemEventsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "push" => Ok(Self::Push),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgByRepoWebhooksResponseWebhooksItemEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for GetByOrgByRepoWebhooksResponseWebhooksItemEventsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetByOrgByRepoWebhooksResponseWebhooksItemEventsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetByOrgReposOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetByOrgReposOrg(::std::string::String);
    impl ::std::ops::Deref for GetByOrgReposOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetByOrgReposOrg> for ::std::string::String {
        fn from(value: GetByOrgReposOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetByOrgReposOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgReposOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetByOrgReposOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetByOrgReposOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetByOrgReposOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetByOrgReposResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "has_more",
    ///    "next_cursor",
    ///    "repos"
    ///  ],
    ///  "properties": {
    ///    "has_more": {
    ///      "type": "boolean"
    ///    },
    ///    "next_cursor": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "repos": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "created_at",
    ///          "default_branch",
    ///          "id",
    ///          "last_push_at",
    ///          "name",
    ///          "org",
    ///          "size_bytes",
    ///          "upstream"
    ///        ],
    ///        "properties": {
    ///          "created_at": {
    ///            "type": "string"
    ///          },
    ///          "default_branch": {
    ///            "type": "string"
    ///          },
    ///          "id": {
    ///            "type": "string"
    ///          },
    ///          "last_push_at": {
    ///            "type": [
    ///              "string",
    ///              "null"
    ///            ]
    ///          },
    ///          "name": {
    ///            "type": "string"
    ///          },
    ///          "org": {
    ///            "type": "string"
    ///          },
    ///          "size_bytes": {
    ///            "type": "number"
    ///          },
    ///          "upstream": {
    ///            "description": "Optionally add an upstream repository. You
    /// can configure automatic syncing from the upstream repository.",
    ///            "type": [
    ///              "object",
    ///              "null"
    ///            ],
    ///            "required": [
    ///              "autosync",
    ///              "last_sync_attempt",
    ///              "last_sync_error",
    ///              "last_sync_success",
    ///              "uri"
    ///            ],
    ///            "properties": {
    ///              "autosync": {
    ///                "description": "Automatic sync configuration, if
    /// enabled",
    ///                "type": [
    ///                  "object",
    ///                  "null"
    ///                ],
    ///                "required": [
    ///                  "period",
    ///                  "resolution_strategy",
    ///                  "type"
    ///                ],
    ///                "properties": {
    ///                  "period": {
    ///                    "description": "Polling period in seconds",
    ///                    "type": "number"
    ///                  },
    ///                  "resolution_strategy": {
    ///                    "description": "Conflict resolution strategy.
    /// \"none\" means sync will fail on conflicts.",
    ///                    "type": "string",
    ///                    "enum": [
    ///                      "none"
    ///                    ]
    ///                  },
    ///                  "type": {
    ///                    "type": "string",
    ///                    "enum": [
    ///                      "poll"
    ///                    ]
    ///                  }
    ///                }
    ///              },
    ///              "last_sync_attempt": {
    ///                "description": "Timestamp of the last sync attempt",
    ///                "type": [
    ///                  "string",
    ///                  "null"
    ///                ],
    ///                "format": "date-time",
    ///                "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///              },
    ///              "last_sync_error": {
    ///                "description": "Error message from the last failed sync
    /// attempt",
    ///                "type": [
    ///                  "string",
    ///                  "null"
    ///                ]
    ///              },
    ///              "last_sync_success": {
    ///                "description": "Timestamp of the last successful sync",
    ///                "type": [
    ///                  "string",
    ///                  "null"
    ///                ],
    ///                "format": "date-time",
    ///                "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///              },
    ///              "uri": {
    ///                "description": "URL of the upstream repository",
    ///                "type": "string"
    ///              }
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgReposResponse {
        pub has_more: bool,
        pub next_cursor: ::std::option::Option<::std::string::String>,
        pub repos: ::std::vec::Vec<GetByOrgReposResponseReposItem>,
    }

    impl GetByOrgReposResponse {
        pub fn builder() -> builder::GetByOrgReposResponse {
            Default::default()
        }
    }

    ///`GetByOrgReposResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgReposResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgReposResponseError {
        pub fn builder() -> builder::GetByOrgReposResponseError {
            Default::default()
        }
    }

    ///`GetByOrgReposResponseReposItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "default_branch",
    ///    "id",
    ///    "last_push_at",
    ///    "name",
    ///    "org",
    ///    "size_bytes",
    ///    "upstream"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "default_branch": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "last_push_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "org": {
    ///      "type": "string"
    ///    },
    ///    "size_bytes": {
    ///      "type": "number"
    ///    },
    ///    "upstream": {
    ///      "description": "Optionally add an upstream repository. You can
    /// configure automatic syncing from the upstream repository.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "required": [
    ///        "autosync",
    ///        "last_sync_attempt",
    ///        "last_sync_error",
    ///        "last_sync_success",
    ///        "uri"
    ///      ],
    ///      "properties": {
    ///        "autosync": {
    ///          "description": "Automatic sync configuration, if enabled",
    ///          "type": [
    ///            "object",
    ///            "null"
    ///          ],
    ///          "required": [
    ///            "period",
    ///            "resolution_strategy",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "period": {
    ///              "description": "Polling period in seconds",
    ///              "type": "number"
    ///            },
    ///            "resolution_strategy": {
    ///              "description": "Conflict resolution strategy. \"none\"
    /// means sync will fail on conflicts.",
    ///              "type": "string",
    ///              "enum": [
    ///                "none"
    ///              ]
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "poll"
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        "last_sync_attempt": {
    ///          "description": "Timestamp of the last sync attempt",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "date-time",
    ///          "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///        },
    ///        "last_sync_error": {
    ///          "description": "Error message from the last failed sync
    /// attempt",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "last_sync_success": {
    ///          "description": "Timestamp of the last successful sync",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "date-time",
    ///          "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///        },
    ///        "uri": {
    ///          "description": "URL of the upstream repository",
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgReposResponseReposItem {
        pub created_at: ::std::string::String,
        pub default_branch: ::std::string::String,
        pub id: ::std::string::String,
        pub last_push_at: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
        pub org: ::std::string::String,
        pub size_bytes: f64,
        ///Optionally add an upstream repository. You can configure automatic
        /// syncing from the upstream repository.
        pub upstream: ::std::option::Option<GetByOrgReposResponseReposItemUpstream>,
    }

    impl GetByOrgReposResponseReposItem {
        pub fn builder() -> builder::GetByOrgReposResponseReposItem {
            Default::default()
        }
    }

    ///Optionally add an upstream repository. You can configure automatic
    /// syncing from the upstream repository.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Optionally add an upstream repository. You can
    /// configure automatic syncing from the upstream repository.",
    ///  "type": "object",
    ///  "required": [
    ///    "autosync",
    ///    "last_sync_attempt",
    ///    "last_sync_error",
    ///    "last_sync_success",
    ///    "uri"
    ///  ],
    ///  "properties": {
    ///    "autosync": {
    ///      "description": "Automatic sync configuration, if enabled",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "required": [
    ///        "period",
    ///        "resolution_strategy",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "period": {
    ///          "description": "Polling period in seconds",
    ///          "type": "number"
    ///        },
    ///        "resolution_strategy": {
    ///          "description": "Conflict resolution strategy. \"none\" means
    /// sync will fail on conflicts.",
    ///          "type": "string",
    ///          "enum": [
    ///            "none"
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "poll"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "last_sync_attempt": {
    ///      "description": "Timestamp of the last sync attempt",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time",
    ///      "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///    },
    ///    "last_sync_error": {
    ///      "description": "Error message from the last failed sync attempt",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "last_sync_success": {
    ///      "description": "Timestamp of the last successful sync",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time",
    ///      "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///    },
    ///    "uri": {
    ///      "description": "URL of the upstream repository",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgReposResponseReposItemUpstream {
        ///Automatic sync configuration, if enabled
        pub autosync: ::std::option::Option<GetByOrgReposResponseReposItemUpstreamAutosync>,
        ///Timestamp of the last sync attempt
        pub last_sync_attempt: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Error message from the last failed sync attempt
        pub last_sync_error: ::std::option::Option<::std::string::String>,
        ///Timestamp of the last successful sync
        pub last_sync_success: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///URL of the upstream repository
        pub uri: ::std::string::String,
    }

    impl GetByOrgReposResponseReposItemUpstream {
        pub fn builder() -> builder::GetByOrgReposResponseReposItemUpstream {
            Default::default()
        }
    }

    ///Automatic sync configuration, if enabled
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Automatic sync configuration, if enabled",
    ///  "type": "object",
    ///  "required": [
    ///    "period",
    ///    "resolution_strategy",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "period": {
    ///      "description": "Polling period in seconds",
    ///      "type": "number"
    ///    },
    ///    "resolution_strategy": {
    ///      "description": "Conflict resolution strategy. \"none\" means sync
    /// will fail on conflicts.",
    ///      "type": "string",
    ///      "enum": [
    ///        "none"
    ///      ]
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "poll"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgReposResponseReposItemUpstreamAutosync {
        pub period: f64,
        ///Conflict resolution strategy. "none" means sync will fail on
        /// conflicts.
        pub resolution_strategy: GetByOrgReposResponseReposItemUpstreamAutosyncResolutionStrategy,
        #[serde(rename = "type")]
        pub type_: GetByOrgReposResponseReposItemUpstreamAutosyncType,
    }

    impl GetByOrgReposResponseReposItemUpstreamAutosync {
        pub fn builder() -> builder::GetByOrgReposResponseReposItemUpstreamAutosync {
            Default::default()
        }
    }

    ///Conflict resolution strategy. "none" means sync will fail on conflicts.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Conflict resolution strategy. \"none\" means sync will
    /// fail on conflicts.",
    ///  "type": "string",
    ///  "enum": [
    ///    "none"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetByOrgReposResponseReposItemUpstreamAutosyncResolutionStrategy {
        #[serde(rename = "none")]
        None,
    }

    impl ::std::fmt::Display for GetByOrgReposResponseReposItemUpstreamAutosyncResolutionStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("none"),
            }
        }
    }

    impl ::std::str::FromStr for GetByOrgReposResponseReposItemUpstreamAutosyncResolutionStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "none" => Ok(Self::None),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for GetByOrgReposResponseReposItemUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for GetByOrgReposResponseReposItemUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetByOrgReposResponseReposItemUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetByOrgReposResponseReposItemUpstreamAutosyncType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "poll"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetByOrgReposResponseReposItemUpstreamAutosyncType {
        #[serde(rename = "poll")]
        Poll,
    }

    impl ::std::fmt::Display for GetByOrgReposResponseReposItemUpstreamAutosyncType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Poll => f.write_str("poll"),
            }
        }
    }

    impl ::std::str::FromStr for GetByOrgReposResponseReposItemUpstreamAutosyncType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "poll" => Ok(Self::Poll),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetByOrgReposResponseReposItemUpstreamAutosyncType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for GetByOrgReposResponseReposItemUpstreamAutosyncType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetByOrgReposResponseReposItemUpstreamAutosyncType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetByOrgResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "last_commit_at",
    ///    "num_repos"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time",
    ///      "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///    },
    ///    "last_commit_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time",
    ///      "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///    },
    ///    "num_repos": {
    ///      "type": "integer",
    ///      "maximum": 9007199254740991.0,
    ///      "minimum": -9007199254740991.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgResponse {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub last_commit_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub num_repos: i64,
    }

    impl GetByOrgResponse {
        pub fn builder() -> builder::GetByOrgResponse {
            Default::default()
        }
    }

    ///`GetByOrgResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetByOrgResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl GetByOrgResponseError {
        pub fn builder() -> builder::GetByOrgResponseError {
            Default::default()
        }
    }

    ///`PatchByOrgByRepoBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "type": "string",
    ///      "maxLength": 100,
    ///      "minLength": 1
    ///    },
    ///    "upstream": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "required": [
    ///        "uri"
    ///      ],
    ///      "properties": {
    ///        "autosync": {
    ///          "description": "Optionally enable automatic sync from the
    /// upstream repository",
    ///          "oneOf": [
    ///            {
    ///              "type": "object",
    ///              "required": [
    ///                "period",
    ///                "type"
    ///              ],
    ///              "properties": {
    ///                "period": {
    ///                  "description": "Polling period in seconds (60s to
    /// 24.8d)",
    ///                  "type": "integer",
    ///                  "maximum": 2147483.0,
    ///                  "minimum": 60.0
    ///                },
    ///                "resolution_strategy": {
    ///                  "description": "Conflict resolution strategy. \"none\"
    /// means sync will fail on conflicts.",
    ///                  "default": "none",
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "none"
    ///                  ]
    ///                },
    ///                "type": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "poll"
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          ]
    ///        },
    ///        "uri": {
    ///          "description": "URL of the upstream repository",
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PatchByOrgByRepoBody {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<PatchByOrgByRepoBodyName>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub upstream: ::std::option::Option<PatchByOrgByRepoBodyUpstream>,
    }

    impl ::std::default::Default for PatchByOrgByRepoBody {
        fn default() -> Self {
            Self {
                name: Default::default(),
                upstream: Default::default(),
            }
        }
    }

    impl PatchByOrgByRepoBody {
        pub fn builder() -> builder::PatchByOrgByRepoBody {
            Default::default()
        }
    }

    ///`PatchByOrgByRepoBodyName`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "maxLength": 100,
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PatchByOrgByRepoBodyName(::std::string::String);
    impl ::std::ops::Deref for PatchByOrgByRepoBodyName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PatchByOrgByRepoBodyName> for ::std::string::String {
        fn from(value: PatchByOrgByRepoBodyName) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PatchByOrgByRepoBodyName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 100usize {
                return Err("longer than 100 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PatchByOrgByRepoBodyName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PatchByOrgByRepoBodyName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PatchByOrgByRepoBodyName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PatchByOrgByRepoBodyName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PatchByOrgByRepoBodyUpstream`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "uri"
    ///  ],
    ///  "properties": {
    ///    "autosync": {
    ///      "description": "Optionally enable automatic sync from the upstream
    /// repository",
    ///      "oneOf": [
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "period",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "period": {
    ///              "description": "Polling period in seconds (60s to 24.8d)",
    ///              "type": "integer",
    ///              "maximum": 2147483.0,
    ///              "minimum": 60.0
    ///            },
    ///            "resolution_strategy": {
    ///              "description": "Conflict resolution strategy. \"none\"
    /// means sync will fail on conflicts.",
    ///              "default": "none",
    ///              "type": "string",
    ///              "enum": [
    ///                "none"
    ///              ]
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "poll"
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      ]
    ///    },
    ///    "uri": {
    ///      "description": "URL of the upstream repository",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PatchByOrgByRepoBodyUpstream {
        ///Optionally enable automatic sync from the upstream repository
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub autosync: ::std::option::Option<PatchByOrgByRepoBodyUpstreamAutosync>,
        ///URL of the upstream repository
        pub uri: ::std::string::String,
    }

    impl PatchByOrgByRepoBodyUpstream {
        pub fn builder() -> builder::PatchByOrgByRepoBodyUpstream {
            Default::default()
        }
    }

    ///Optionally enable automatic sync from the upstream repository
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Optionally enable automatic sync from the upstream
    /// repository",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "period",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "period": {
    ///          "description": "Polling period in seconds (60s to 24.8d)",
    ///          "type": "integer",
    ///          "maximum": 2147483.0,
    ///          "minimum": 60.0
    ///        },
    ///        "resolution_strategy": {
    ///          "description": "Conflict resolution strategy. \"none\" means
    /// sync will fail on conflicts.",
    ///          "default": "none",
    ///          "type": "string",
    ///          "enum": [
    ///            "none"
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "poll"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum PatchByOrgByRepoBodyUpstreamAutosync {
        #[serde(rename = "poll")]
        Poll {
            ///Polling period in seconds (60s to 24.8d)
            period: i64,
            ///Conflict resolution strategy. "none" means sync will fail on
            /// conflicts.
            #[serde(
                default = "defaults::patch_by_org_by_repo_body_upstream_autosync_poll_resolution_strategy"
            )]
            resolution_strategy: PatchByOrgByRepoBodyUpstreamAutosyncResolutionStrategy,
        },
    }

    ///Conflict resolution strategy. "none" means sync will fail on conflicts.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Conflict resolution strategy. \"none\" means sync will
    /// fail on conflicts.",
    ///  "default": "none",
    ///  "type": "string",
    ///  "enum": [
    ///    "none"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PatchByOrgByRepoBodyUpstreamAutosyncResolutionStrategy {
        #[serde(rename = "none")]
        None,
    }

    impl ::std::fmt::Display for PatchByOrgByRepoBodyUpstreamAutosyncResolutionStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("none"),
            }
        }
    }

    impl ::std::str::FromStr for PatchByOrgByRepoBodyUpstreamAutosyncResolutionStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "none" => Ok(Self::None),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PatchByOrgByRepoBodyUpstreamAutosyncResolutionStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for PatchByOrgByRepoBodyUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PatchByOrgByRepoBodyUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::default::Default for PatchByOrgByRepoBodyUpstreamAutosyncResolutionStrategy {
        fn default() -> Self {
            PatchByOrgByRepoBodyUpstreamAutosyncResolutionStrategy::None
        }
    }

    ///`PatchByOrgByRepoOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PatchByOrgByRepoOrg(::std::string::String);
    impl ::std::ops::Deref for PatchByOrgByRepoOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PatchByOrgByRepoOrg> for ::std::string::String {
        fn from(value: PatchByOrgByRepoOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PatchByOrgByRepoOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PatchByOrgByRepoOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PatchByOrgByRepoOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PatchByOrgByRepoOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PatchByOrgByRepoOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PatchByOrgByRepoRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PatchByOrgByRepoRepo(::std::string::String);
    impl ::std::ops::Deref for PatchByOrgByRepoRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PatchByOrgByRepoRepo> for ::std::string::String {
        fn from(value: PatchByOrgByRepoRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PatchByOrgByRepoRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PatchByOrgByRepoRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PatchByOrgByRepoRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PatchByOrgByRepoRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PatchByOrgByRepoRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PatchByOrgByRepoResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "default_branch",
    ///    "id",
    ///    "last_push_at",
    ///    "name",
    ///    "org",
    ///    "size_bytes",
    ///    "upstream"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "default_branch": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "last_push_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "org": {
    ///      "type": "string"
    ///    },
    ///    "size_bytes": {
    ///      "type": "number"
    ///    },
    ///    "upstream": {
    ///      "description": "Optionally add an upstream repository. You can
    /// configure automatic syncing from the upstream repository.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "required": [
    ///        "autosync",
    ///        "last_sync_attempt",
    ///        "last_sync_error",
    ///        "last_sync_success",
    ///        "uri"
    ///      ],
    ///      "properties": {
    ///        "autosync": {
    ///          "description": "Automatic sync configuration, if enabled",
    ///          "type": [
    ///            "object",
    ///            "null"
    ///          ],
    ///          "required": [
    ///            "period",
    ///            "resolution_strategy",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "period": {
    ///              "description": "Polling period in seconds",
    ///              "type": "number"
    ///            },
    ///            "resolution_strategy": {
    ///              "description": "Conflict resolution strategy. \"none\"
    /// means sync will fail on conflicts.",
    ///              "type": "string",
    ///              "enum": [
    ///                "none"
    ///              ]
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "poll"
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        "last_sync_attempt": {
    ///          "description": "Timestamp of the last sync attempt",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "date-time",
    ///          "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///        },
    ///        "last_sync_error": {
    ///          "description": "Error message from the last failed sync
    /// attempt",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "last_sync_success": {
    ///          "description": "Timestamp of the last successful sync",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "date-time",
    ///          "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///        },
    ///        "uri": {
    ///          "description": "URL of the upstream repository",
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PatchByOrgByRepoResponse {
        pub created_at: ::std::string::String,
        pub default_branch: ::std::string::String,
        pub id: ::std::string::String,
        pub last_push_at: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
        pub org: ::std::string::String,
        pub size_bytes: f64,
        ///Optionally add an upstream repository. You can configure automatic
        /// syncing from the upstream repository.
        pub upstream: ::std::option::Option<PatchByOrgByRepoResponseUpstream>,
    }

    impl PatchByOrgByRepoResponse {
        pub fn builder() -> builder::PatchByOrgByRepoResponse {
            Default::default()
        }
    }

    ///`PatchByOrgByRepoResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PatchByOrgByRepoResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl PatchByOrgByRepoResponseError {
        pub fn builder() -> builder::PatchByOrgByRepoResponseError {
            Default::default()
        }
    }

    ///Optionally add an upstream repository. You can configure automatic
    /// syncing from the upstream repository.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Optionally add an upstream repository. You can
    /// configure automatic syncing from the upstream repository.",
    ///  "type": "object",
    ///  "required": [
    ///    "autosync",
    ///    "last_sync_attempt",
    ///    "last_sync_error",
    ///    "last_sync_success",
    ///    "uri"
    ///  ],
    ///  "properties": {
    ///    "autosync": {
    ///      "description": "Automatic sync configuration, if enabled",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "required": [
    ///        "period",
    ///        "resolution_strategy",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "period": {
    ///          "description": "Polling period in seconds",
    ///          "type": "number"
    ///        },
    ///        "resolution_strategy": {
    ///          "description": "Conflict resolution strategy. \"none\" means
    /// sync will fail on conflicts.",
    ///          "type": "string",
    ///          "enum": [
    ///            "none"
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "poll"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "last_sync_attempt": {
    ///      "description": "Timestamp of the last sync attempt",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time",
    ///      "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///    },
    ///    "last_sync_error": {
    ///      "description": "Error message from the last failed sync attempt",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "last_sync_success": {
    ///      "description": "Timestamp of the last successful sync",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time",
    ///      "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///    },
    ///    "uri": {
    ///      "description": "URL of the upstream repository",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PatchByOrgByRepoResponseUpstream {
        ///Automatic sync configuration, if enabled
        pub autosync: ::std::option::Option<PatchByOrgByRepoResponseUpstreamAutosync>,
        ///Timestamp of the last sync attempt
        pub last_sync_attempt: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Error message from the last failed sync attempt
        pub last_sync_error: ::std::option::Option<::std::string::String>,
        ///Timestamp of the last successful sync
        pub last_sync_success: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///URL of the upstream repository
        pub uri: ::std::string::String,
    }

    impl PatchByOrgByRepoResponseUpstream {
        pub fn builder() -> builder::PatchByOrgByRepoResponseUpstream {
            Default::default()
        }
    }

    ///Automatic sync configuration, if enabled
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Automatic sync configuration, if enabled",
    ///  "type": "object",
    ///  "required": [
    ///    "period",
    ///    "resolution_strategy",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "period": {
    ///      "description": "Polling period in seconds",
    ///      "type": "number"
    ///    },
    ///    "resolution_strategy": {
    ///      "description": "Conflict resolution strategy. \"none\" means sync
    /// will fail on conflicts.",
    ///      "type": "string",
    ///      "enum": [
    ///        "none"
    ///      ]
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "poll"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PatchByOrgByRepoResponseUpstreamAutosync {
        pub period: f64,
        ///Conflict resolution strategy. "none" means sync will fail on
        /// conflicts.
        pub resolution_strategy: PatchByOrgByRepoResponseUpstreamAutosyncResolutionStrategy,
        #[serde(rename = "type")]
        pub type_: PatchByOrgByRepoResponseUpstreamAutosyncType,
    }

    impl PatchByOrgByRepoResponseUpstreamAutosync {
        pub fn builder() -> builder::PatchByOrgByRepoResponseUpstreamAutosync {
            Default::default()
        }
    }

    ///Conflict resolution strategy. "none" means sync will fail on conflicts.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Conflict resolution strategy. \"none\" means sync will
    /// fail on conflicts.",
    ///  "type": "string",
    ///  "enum": [
    ///    "none"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PatchByOrgByRepoResponseUpstreamAutosyncResolutionStrategy {
        #[serde(rename = "none")]
        None,
    }

    impl ::std::fmt::Display for PatchByOrgByRepoResponseUpstreamAutosyncResolutionStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("none"),
            }
        }
    }

    impl ::std::str::FromStr for PatchByOrgByRepoResponseUpstreamAutosyncResolutionStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "none" => Ok(Self::None),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PatchByOrgByRepoResponseUpstreamAutosyncResolutionStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for PatchByOrgByRepoResponseUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PatchByOrgByRepoResponseUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PatchByOrgByRepoResponseUpstreamAutosyncType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "poll"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PatchByOrgByRepoResponseUpstreamAutosyncType {
        #[serde(rename = "poll")]
        Poll,
    }

    impl ::std::fmt::Display for PatchByOrgByRepoResponseUpstreamAutosyncType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Poll => f.write_str("poll"),
            }
        }
    }

    impl ::std::str::FromStr for PatchByOrgByRepoResponseUpstreamAutosyncType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "poll" => Ok(Self::Poll),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PatchByOrgByRepoResponseUpstreamAutosyncType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for PatchByOrgByRepoResponseUpstreamAutosyncType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PatchByOrgByRepoResponseUpstreamAutosyncType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PostByOrgApiKeysBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string",
    ///        "enum": [
    ///          "git:read",
    ///          "git:write",
    ///          "repo:read",
    ///          "repo:create",
    ///          "repo:delete",
    ///          "webhook:read",
    ///          "webhook:write",
    ///          "admin"
    ///        ]
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgApiKeysBody {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub scopes: ::std::vec::Vec<PostByOrgApiKeysBodyScopesItem>,
    }

    impl ::std::default::Default for PostByOrgApiKeysBody {
        fn default() -> Self {
            Self {
                name: Default::default(),
                scopes: Default::default(),
            }
        }
    }

    impl PostByOrgApiKeysBody {
        pub fn builder() -> builder::PostByOrgApiKeysBody {
            Default::default()
        }
    }

    ///`PostByOrgApiKeysBodyScopesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "git:read",
    ///    "git:write",
    ///    "repo:read",
    ///    "repo:create",
    ///    "repo:delete",
    ///    "webhook:read",
    ///    "webhook:write",
    ///    "admin"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostByOrgApiKeysBodyScopesItem {
        #[serde(rename = "git:read")]
        GitRead,
        #[serde(rename = "git:write")]
        GitWrite,
        #[serde(rename = "repo:read")]
        RepoRead,
        #[serde(rename = "repo:create")]
        RepoCreate,
        #[serde(rename = "repo:delete")]
        RepoDelete,
        #[serde(rename = "webhook:read")]
        WebhookRead,
        #[serde(rename = "webhook:write")]
        WebhookWrite,
        #[serde(rename = "admin")]
        Admin,
    }

    impl ::std::fmt::Display for PostByOrgApiKeysBodyScopesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::GitRead => f.write_str("git:read"),
                Self::GitWrite => f.write_str("git:write"),
                Self::RepoRead => f.write_str("repo:read"),
                Self::RepoCreate => f.write_str("repo:create"),
                Self::RepoDelete => f.write_str("repo:delete"),
                Self::WebhookRead => f.write_str("webhook:read"),
                Self::WebhookWrite => f.write_str("webhook:write"),
                Self::Admin => f.write_str("admin"),
            }
        }
    }

    impl ::std::str::FromStr for PostByOrgApiKeysBodyScopesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "git:read" => Ok(Self::GitRead),
                "git:write" => Ok(Self::GitWrite),
                "repo:read" => Ok(Self::RepoRead),
                "repo:create" => Ok(Self::RepoCreate),
                "repo:delete" => Ok(Self::RepoDelete),
                "webhook:read" => Ok(Self::WebhookRead),
                "webhook:write" => Ok(Self::WebhookWrite),
                "admin" => Ok(Self::Admin),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgApiKeysBodyScopesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgApiKeysBodyScopesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgApiKeysBodyScopesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PostByOrgApiKeysResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "key",
    ///    "name",
    ///    "scopes"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "key": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgApiKeysResponse {
        pub created_at: ::std::string::String,
        pub id: ::std::string::String,
        pub key: ::std::string::String,
        pub name: ::std::option::Option<::std::string::String>,
        pub scopes: ::std::vec::Vec<::std::string::String>,
    }

    impl PostByOrgApiKeysResponse {
        pub fn builder() -> builder::PostByOrgApiKeysResponse {
            Default::default()
        }
    }

    ///`PostByOrgApiKeysResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgApiKeysResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl PostByOrgApiKeysResponseError {
        pub fn builder() -> builder::PostByOrgApiKeysResponseError {
            Default::default()
        }
    }

    ///`PostByOrgByRepoAnalyticsRefreshOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoAnalyticsRefreshOrg(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoAnalyticsRefreshOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoAnalyticsRefreshOrg> for ::std::string::String {
        fn from(value: PostByOrgByRepoAnalyticsRefreshOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoAnalyticsRefreshOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoAnalyticsRefreshOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoAnalyticsRefreshOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoAnalyticsRefreshOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoAnalyticsRefreshOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoAnalyticsRefreshRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoAnalyticsRefreshRepo(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoAnalyticsRefreshRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoAnalyticsRefreshRepo> for ::std::string::String {
        fn from(value: PostByOrgByRepoAnalyticsRefreshRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoAnalyticsRefreshRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoAnalyticsRefreshRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoAnalyticsRefreshRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoAnalyticsRefreshRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoAnalyticsRefreshRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoAnalyticsRefreshResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "analytics",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "analytics": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "required": [
    ///        "contributors",
    ///        "history",
    ///        "summary",
    ///        "version"
    ///      ],
    ///      "properties": {
    ///        "_meta": {
    ///          "type": "object",
    ///          "required": [
    ///            "lastAttempt",
    ///            "lastError",
    ///            "lastSuccess"
    ///          ],
    ///          "properties": {
    ///            "lastAttempt": {
    ///              "type": "string"
    ///            },
    ///            "lastError": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "lastSuccess": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        "contributors": {
    ///          "type": "object",
    ///          "additionalProperties": {
    ///            "type": "object",
    ///            "required": [
    ///              "aiLines",
    ///              "commitCount",
    ///              "models",
    ///              "providers",
    ///              "totalLines"
    ///            ],
    ///            "properties": {
    ///              "aiLines": {
    ///                "type": "number"
    ///              },
    ///              "commitCount": {
    ///                "type": "number"
    ///              },
    ///              "models": {
    ///                "type": "object",
    ///                "additionalProperties": {
    ///                  "type": "number"
    ///                }
    ///              },
    ///              "providers": {
    ///                "type": "object",
    ///                "additionalProperties": {
    ///                  "type": "number"
    ///                }
    ///              },
    ///              "totalLines": {
    ///                "type": "number"
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "history": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "object",
    ///            "required": [
    ///              "added",
    ///              "aiLines",
    ///              "author",
    ///              "date",
    ///              "removed",
    ///              "sha"
    ///            ],
    ///            "properties": {
    ///              "added": {
    ///                "type": "number"
    ///              },
    ///              "aiLines": {
    ///                "type": "number"
    ///              },
    ///              "author": {
    ///                "type": "string"
    ///              },
    ///              "date": {
    ///                "type": "string"
    ///              },
    ///              "message": {
    ///                "type": "string"
    ///              },
    ///              "models": {
    ///                "type": "object",
    ///                "additionalProperties": {
    ///                  "type": "number"
    ///                }
    ///              },
    ///              "providers": {
    ///                "type": "object",
    ///                "additionalProperties": {
    ///                  "type": "number"
    ///                }
    ///              },
    ///              "removed": {
    ///                "type": "number"
    ///              },
    ///              "sha": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "summary": {
    ///          "type": "object",
    ///          "required": [
    ///            "aiLines",
    ///            "humanLines",
    ///            "models",
    ///            "providers",
    ///            "totalLines",
    ///            "updated"
    ///          ],
    ///          "properties": {
    ///            "aiLines": {
    ///              "type": "number"
    ///            },
    ///            "humanLines": {
    ///              "type": "number"
    ///            },
    ///            "models": {
    ///              "type": "object",
    ///              "additionalProperties": {
    ///                "type": "number"
    ///              }
    ///            },
    ///            "providers": {
    ///              "type": "object",
    ///              "additionalProperties": {
    ///                "type": "number"
    ///              }
    ///            },
    ///            "totalLines": {
    ///              "type": "number"
    ///            },
    ///            "updated": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        "version": {
    ///          "type": "number",
    ///          "enum": [
    ///            2.0
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "status": {
    ///      "type": "string",
    ///      "enum": [
    ///        "current",
    ///        "stale",
    ///        "none"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoAnalyticsRefreshResponse {
        pub analytics: ::std::option::Option<PostByOrgByRepoAnalyticsRefreshResponseAnalytics>,
        pub status: PostByOrgByRepoAnalyticsRefreshResponseStatus,
    }

    impl PostByOrgByRepoAnalyticsRefreshResponse {
        pub fn builder() -> builder::PostByOrgByRepoAnalyticsRefreshResponse {
            Default::default()
        }
    }

    ///`PostByOrgByRepoAnalyticsRefreshResponseAnalytics`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "contributors",
    ///    "history",
    ///    "summary",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "_meta": {
    ///      "type": "object",
    ///      "required": [
    ///        "lastAttempt",
    ///        "lastError",
    ///        "lastSuccess"
    ///      ],
    ///      "properties": {
    ///        "lastAttempt": {
    ///          "type": "string"
    ///        },
    ///        "lastError": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "lastSuccess": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "contributors": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "object",
    ///        "required": [
    ///          "aiLines",
    ///          "commitCount",
    ///          "models",
    ///          "providers",
    ///          "totalLines"
    ///        ],
    ///        "properties": {
    ///          "aiLines": {
    ///            "type": "number"
    ///          },
    ///          "commitCount": {
    ///            "type": "number"
    ///          },
    ///          "models": {
    ///            "type": "object",
    ///            "additionalProperties": {
    ///              "type": "number"
    ///            }
    ///          },
    ///          "providers": {
    ///            "type": "object",
    ///            "additionalProperties": {
    ///              "type": "number"
    ///            }
    ///          },
    ///          "totalLines": {
    ///            "type": "number"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "history": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "added",
    ///          "aiLines",
    ///          "author",
    ///          "date",
    ///          "removed",
    ///          "sha"
    ///        ],
    ///        "properties": {
    ///          "added": {
    ///            "type": "number"
    ///          },
    ///          "aiLines": {
    ///            "type": "number"
    ///          },
    ///          "author": {
    ///            "type": "string"
    ///          },
    ///          "date": {
    ///            "type": "string"
    ///          },
    ///          "message": {
    ///            "type": "string"
    ///          },
    ///          "models": {
    ///            "type": "object",
    ///            "additionalProperties": {
    ///              "type": "number"
    ///            }
    ///          },
    ///          "providers": {
    ///            "type": "object",
    ///            "additionalProperties": {
    ///              "type": "number"
    ///            }
    ///          },
    ///          "removed": {
    ///            "type": "number"
    ///          },
    ///          "sha": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "summary": {
    ///      "type": "object",
    ///      "required": [
    ///        "aiLines",
    ///        "humanLines",
    ///        "models",
    ///        "providers",
    ///        "totalLines",
    ///        "updated"
    ///      ],
    ///      "properties": {
    ///        "aiLines": {
    ///          "type": "number"
    ///        },
    ///        "humanLines": {
    ///          "type": "number"
    ///        },
    ///        "models": {
    ///          "type": "object",
    ///          "additionalProperties": {
    ///            "type": "number"
    ///          }
    ///        },
    ///        "providers": {
    ///          "type": "object",
    ///          "additionalProperties": {
    ///            "type": "number"
    ///          }
    ///        },
    ///        "totalLines": {
    ///          "type": "number"
    ///        },
    ///        "updated": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "version": {
    ///      "type": "number",
    ///      "enum": [
    ///        2.0
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoAnalyticsRefreshResponseAnalytics {
        pub contributors: ::std::collections::HashMap<
            ::std::string::String,
            PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue,
        >,
        pub history: ::std::vec::Vec<PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem>,
        #[serde(
            rename = "_meta",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub meta: ::std::option::Option<PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta>,
        pub summary: PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary,
        pub version: PostByOrgByRepoAnalyticsRefreshResponseAnalyticsVersion,
    }

    impl PostByOrgByRepoAnalyticsRefreshResponseAnalytics {
        pub fn builder() -> builder::PostByOrgByRepoAnalyticsRefreshResponseAnalytics {
            Default::default()
        }
    }

    ///`PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "aiLines",
    ///    "commitCount",
    ///    "models",
    ///    "providers",
    ///    "totalLines"
    ///  ],
    ///  "properties": {
    ///    "aiLines": {
    ///      "type": "number"
    ///    },
    ///    "commitCount": {
    ///      "type": "number"
    ///    },
    ///    "models": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "providers": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "totalLines": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue {
        #[serde(rename = "aiLines")]
        pub ai_lines: f64,
        #[serde(rename = "commitCount")]
        pub commit_count: f64,
        pub models: ::std::collections::HashMap<::std::string::String, f64>,
        pub providers: ::std::collections::HashMap<::std::string::String, f64>,
        #[serde(rename = "totalLines")]
        pub total_lines: f64,
    }

    impl PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue {
        pub fn builder(
        ) -> builder::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue {
            Default::default()
        }
    }

    ///`PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "added",
    ///    "aiLines",
    ///    "author",
    ///    "date",
    ///    "removed",
    ///    "sha"
    ///  ],
    ///  "properties": {
    ///    "added": {
    ///      "type": "number"
    ///    },
    ///    "aiLines": {
    ///      "type": "number"
    ///    },
    ///    "author": {
    ///      "type": "string"
    ///    },
    ///    "date": {
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "models": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "providers": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "removed": {
    ///      "type": "number"
    ///    },
    ///    "sha": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem {
        pub added: f64,
        #[serde(rename = "aiLines")]
        pub ai_lines: f64,
        pub author: ::std::string::String,
        pub date: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub models: ::std::collections::HashMap<::std::string::String, f64>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub providers: ::std::collections::HashMap<::std::string::String, f64>,
        pub removed: f64,
        pub sha: ::std::string::String,
    }

    impl PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem {
        pub fn builder() -> builder::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem {
            Default::default()
        }
    }

    ///`PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "lastAttempt",
    ///    "lastError",
    ///    "lastSuccess"
    ///  ],
    ///  "properties": {
    ///    "lastAttempt": {
    ///      "type": "string"
    ///    },
    ///    "lastError": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "lastSuccess": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta {
        #[serde(rename = "lastAttempt")]
        pub last_attempt: ::std::string::String,
        #[serde(rename = "lastError")]
        pub last_error: ::std::option::Option<::std::string::String>,
        #[serde(rename = "lastSuccess")]
        pub last_success: ::std::option::Option<::std::string::String>,
    }

    impl PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta {
        pub fn builder() -> builder::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta {
            Default::default()
        }
    }

    ///`PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "aiLines",
    ///    "humanLines",
    ///    "models",
    ///    "providers",
    ///    "totalLines",
    ///    "updated"
    ///  ],
    ///  "properties": {
    ///    "aiLines": {
    ///      "type": "number"
    ///    },
    ///    "humanLines": {
    ///      "type": "number"
    ///    },
    ///    "models": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "providers": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "totalLines": {
    ///      "type": "number"
    ///    },
    ///    "updated": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary {
        #[serde(rename = "aiLines")]
        pub ai_lines: f64,
        #[serde(rename = "humanLines")]
        pub human_lines: f64,
        pub models: ::std::collections::HashMap<::std::string::String, f64>,
        pub providers: ::std::collections::HashMap<::std::string::String, f64>,
        #[serde(rename = "totalLines")]
        pub total_lines: f64,
        pub updated: ::std::string::String,
    }

    impl PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary {
        pub fn builder() -> builder::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary {
            Default::default()
        }
    }

    ///`PostByOrgByRepoAnalyticsRefreshResponseAnalyticsVersion`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "number",
    ///  "enum": [
    ///    2.0
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoAnalyticsRefreshResponseAnalyticsVersion(f64);
    impl ::std::ops::Deref for PostByOrgByRepoAnalyticsRefreshResponseAnalyticsVersion {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoAnalyticsRefreshResponseAnalyticsVersion> for f64 {
        fn from(value: PostByOrgByRepoAnalyticsRefreshResponseAnalyticsVersion) -> Self {
            value.0
        }
    }

    impl ::std::convert::TryFrom<f64> for PostByOrgByRepoAnalyticsRefreshResponseAnalyticsVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![2.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoAnalyticsRefreshResponseAnalyticsVersion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }

    ///`PostByOrgByRepoAnalyticsRefreshResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoAnalyticsRefreshResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl PostByOrgByRepoAnalyticsRefreshResponseError {
        pub fn builder() -> builder::PostByOrgByRepoAnalyticsRefreshResponseError {
            Default::default()
        }
    }

    ///`PostByOrgByRepoAnalyticsRefreshResponseStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "current",
    ///    "stale",
    ///    "none"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostByOrgByRepoAnalyticsRefreshResponseStatus {
        #[serde(rename = "current")]
        Current,
        #[serde(rename = "stale")]
        Stale,
        #[serde(rename = "none")]
        None,
    }

    impl ::std::fmt::Display for PostByOrgByRepoAnalyticsRefreshResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Current => f.write_str("current"),
                Self::Stale => f.write_str("stale"),
                Self::None => f.write_str("none"),
            }
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoAnalyticsRefreshResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "current" => Ok(Self::Current),
                "stale" => Ok(Self::Stale),
                "none" => Ok(Self::None),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoAnalyticsRefreshResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for PostByOrgByRepoAnalyticsRefreshResponseStatus
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PostByOrgByRepoAnalyticsRefreshResponseStatus
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PostByOrgByRepoBranchesBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "from",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "from": {
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "name": {
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoBranchesBody {
        pub from: PostByOrgByRepoBranchesBodyFrom,
        pub name: PostByOrgByRepoBranchesBodyName,
    }

    impl PostByOrgByRepoBranchesBody {
        pub fn builder() -> builder::PostByOrgByRepoBranchesBody {
            Default::default()
        }
    }

    ///`PostByOrgByRepoBranchesBodyFrom`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoBranchesBodyFrom(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoBranchesBodyFrom {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoBranchesBodyFrom> for ::std::string::String {
        fn from(value: PostByOrgByRepoBranchesBodyFrom) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoBranchesBodyFrom {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoBranchesBodyFrom {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoBranchesBodyFrom {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoBranchesBodyFrom {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoBranchesBodyFrom {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoBranchesBodyName`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoBranchesBodyName(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoBranchesBodyName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoBranchesBodyName> for ::std::string::String {
        fn from(value: PostByOrgByRepoBranchesBodyName) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoBranchesBodyName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoBranchesBodyName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoBranchesBodyName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoBranchesBodyName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoBranchesBodyName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoBranchesOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoBranchesOrg(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoBranchesOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoBranchesOrg> for ::std::string::String {
        fn from(value: PostByOrgByRepoBranchesOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoBranchesOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoBranchesOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoBranchesOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoBranchesOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoBranchesOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoBranchesRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoBranchesRepo(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoBranchesRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoBranchesRepo> for ::std::string::String {
        fn from(value: PostByOrgByRepoBranchesRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoBranchesRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoBranchesRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoBranchesRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoBranchesRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoBranchesRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoBranchesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "head_sha",
    ///    "is_default",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "head_sha": {
    ///      "type": "string"
    ///    },
    ///    "is_default": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoBranchesResponse {
        pub head_sha: ::std::string::String,
        pub is_default: bool,
        pub name: ::std::string::String,
    }

    impl PostByOrgByRepoBranchesResponse {
        pub fn builder() -> builder::PostByOrgByRepoBranchesResponse {
            Default::default()
        }
    }

    ///`PostByOrgByRepoBranchesResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoBranchesResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl PostByOrgByRepoBranchesResponseError {
        pub fn builder() -> builder::PostByOrgByRepoBranchesResponseError {
            Default::default()
        }
    }

    ///`PostByOrgByRepoCommitsBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "author",
    ///    "branch",
    ///    "files",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "author": {
    ///      "type": "object",
    ///      "required": [
    ///        "email",
    ///        "name"
    ///      ],
    ///      "properties": {
    ///        "date": {
    ///          "type": "string"
    ///        },
    ///        "email": {
    ///          "type": "string",
    ///          "format": "email",
    ///          "pattern":
    /// "^(?!\\.)(?!.*\\.\\.)([A-Za-z0-9_'+\\-\\.]*)[A-Za-z0-9_+-]@
    /// ([A-Za-z0-9][A-Za-z0-9\\-]*\\.)+[A-Za-z]{2,}$"
    ///        },
    ///        "name": {
    ///          "type": "string",
    ///          "minLength": 1
    ///        }
    ///      }
    ///    },
    ///    "base_sha": {
    ///      "type": "string"
    ///    },
    ///    "branch": {
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "files": {
    ///      "type": "array",
    ///      "items": {
    ///        "anyOf": [
    ///          {
    ///            "type": "object",
    ///            "required": [
    ///              "content",
    ///              "path"
    ///            ],
    ///            "properties": {
    ///              "action": {
    ///                "type": "string",
    ///                "enum": [
    ///                  "upsert"
    ///                ]
    ///              },
    ///              "content": {
    ///                "type": "string"
    ///              },
    ///              "encoding": {
    ///                "default": "utf-8",
    ///                "type": "string",
    ///                "enum": [
    ///                  "utf-8",
    ///                  "base64"
    ///                ]
    ///              },
    ///              "path": {
    ///                "type": "string",
    ///                "minLength": 1
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "type": "object",
    ///            "required": [
    ///              "action",
    ///              "path"
    ///            ],
    ///            "properties": {
    ///              "action": {
    ///                "type": "string",
    ///                "enum": [
    ///                  "delete"
    ///                ]
    ///              },
    ///              "path": {
    ///                "type": "string",
    ///                "minLength": 1
    ///              }
    ///            }
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoCommitsBody {
        pub author: PostByOrgByRepoCommitsBodyAuthor,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub base_sha: ::std::option::Option<::std::string::String>,
        pub branch: PostByOrgByRepoCommitsBodyBranch,
        pub files: ::std::vec::Vec<PostByOrgByRepoCommitsBodyFilesItem>,
        pub message: PostByOrgByRepoCommitsBodyMessage,
    }

    impl PostByOrgByRepoCommitsBody {
        pub fn builder() -> builder::PostByOrgByRepoCommitsBody {
            Default::default()
        }
    }

    ///`PostByOrgByRepoCommitsBodyAuthor`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "date": {
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "type": "string",
    ///      "format": "email",
    ///      "pattern":
    /// "^(?!\\.)(?!.*\\.\\.)([A-Za-z0-9_'+\\-\\.]*)[A-Za-z0-9_+-]@
    /// ([A-Za-z0-9][A-Za-z0-9\\-]*\\.)+[A-Za-z]{2,}$"
    ///    },
    ///    "name": {
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoCommitsBodyAuthor {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub date: ::std::option::Option<::std::string::String>,
        pub email: ::std::string::String,
        pub name: PostByOrgByRepoCommitsBodyAuthorName,
    }

    impl PostByOrgByRepoCommitsBodyAuthor {
        pub fn builder() -> builder::PostByOrgByRepoCommitsBodyAuthor {
            Default::default()
        }
    }

    ///`PostByOrgByRepoCommitsBodyAuthorName`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoCommitsBodyAuthorName(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoCommitsBodyAuthorName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoCommitsBodyAuthorName> for ::std::string::String {
        fn from(value: PostByOrgByRepoCommitsBodyAuthorName) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoCommitsBodyAuthorName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoCommitsBodyAuthorName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoCommitsBodyAuthorName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoCommitsBodyAuthorName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoCommitsBodyAuthorName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoCommitsBodyBranch`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoCommitsBodyBranch(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoCommitsBodyBranch {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoCommitsBodyBranch> for ::std::string::String {
        fn from(value: PostByOrgByRepoCommitsBodyBranch) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoCommitsBodyBranch {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoCommitsBodyBranch {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoCommitsBodyBranch {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoCommitsBodyBranch {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoCommitsBodyBranch {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoCommitsBodyFilesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "content",
    ///        "path"
    ///      ],
    ///      "properties": {
    ///        "action": {
    ///          "type": "string",
    ///          "enum": [
    ///            "upsert"
    ///          ]
    ///        },
    ///        "content": {
    ///          "type": "string"
    ///        },
    ///        "encoding": {
    ///          "default": "utf-8",
    ///          "type": "string",
    ///          "enum": [
    ///            "utf-8",
    ///            "base64"
    ///          ]
    ///        },
    ///        "path": {
    ///          "type": "string",
    ///          "minLength": 1
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "action",
    ///        "path"
    ///      ],
    ///      "properties": {
    ///        "action": {
    ///          "type": "string",
    ///          "enum": [
    ///            "delete"
    ///          ]
    ///        },
    ///        "path": {
    ///          "type": "string",
    ///          "minLength": 1
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum PostByOrgByRepoCommitsBodyFilesItem {
        Variant0 {
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            action: ::std::option::Option<PostByOrgByRepoCommitsBodyFilesItemVariant0Action>,
            content: ::std::string::String,
            #[serde(
                default = "defaults::post_by_org_by_repo_commits_body_files_item_variant0_encoding"
            )]
            encoding: PostByOrgByRepoCommitsBodyFilesItemVariant0Encoding,
            path: PostByOrgByRepoCommitsBodyFilesItemVariant0Path,
        },
        Variant1 {
            action: PostByOrgByRepoCommitsBodyFilesItemVariant1Action,
            path: PostByOrgByRepoCommitsBodyFilesItemVariant1Path,
        },
    }

    ///`PostByOrgByRepoCommitsBodyFilesItemVariant0Action`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "upsert"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostByOrgByRepoCommitsBodyFilesItemVariant0Action {
        #[serde(rename = "upsert")]
        Upsert,
    }

    impl ::std::fmt::Display for PostByOrgByRepoCommitsBodyFilesItemVariant0Action {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Upsert => f.write_str("upsert"),
            }
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoCommitsBodyFilesItemVariant0Action {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "upsert" => Ok(Self::Upsert),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoCommitsBodyFilesItemVariant0Action {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for PostByOrgByRepoCommitsBodyFilesItemVariant0Action
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PostByOrgByRepoCommitsBodyFilesItemVariant0Action
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PostByOrgByRepoCommitsBodyFilesItemVariant0Encoding`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "utf-8",
    ///  "type": "string",
    ///  "enum": [
    ///    "utf-8",
    ///    "base64"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostByOrgByRepoCommitsBodyFilesItemVariant0Encoding {
        #[serde(rename = "utf-8")]
        Utf8,
        #[serde(rename = "base64")]
        Base64,
    }

    impl ::std::fmt::Display for PostByOrgByRepoCommitsBodyFilesItemVariant0Encoding {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Utf8 => f.write_str("utf-8"),
                Self::Base64 => f.write_str("base64"),
            }
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoCommitsBodyFilesItemVariant0Encoding {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "utf-8" => Ok(Self::Utf8),
                "base64" => Ok(Self::Base64),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoCommitsBodyFilesItemVariant0Encoding {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for PostByOrgByRepoCommitsBodyFilesItemVariant0Encoding
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PostByOrgByRepoCommitsBodyFilesItemVariant0Encoding
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::default::Default for PostByOrgByRepoCommitsBodyFilesItemVariant0Encoding {
        fn default() -> Self {
            PostByOrgByRepoCommitsBodyFilesItemVariant0Encoding::Utf8
        }
    }

    ///`PostByOrgByRepoCommitsBodyFilesItemVariant0Path`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoCommitsBodyFilesItemVariant0Path(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoCommitsBodyFilesItemVariant0Path {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoCommitsBodyFilesItemVariant0Path>
        for ::std::string::String
    {
        fn from(value: PostByOrgByRepoCommitsBodyFilesItemVariant0Path) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoCommitsBodyFilesItemVariant0Path {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoCommitsBodyFilesItemVariant0Path {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for PostByOrgByRepoCommitsBodyFilesItemVariant0Path
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PostByOrgByRepoCommitsBodyFilesItemVariant0Path
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoCommitsBodyFilesItemVariant0Path {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoCommitsBodyFilesItemVariant1Action`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "delete"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostByOrgByRepoCommitsBodyFilesItemVariant1Action {
        #[serde(rename = "delete")]
        Delete,
    }

    impl ::std::fmt::Display for PostByOrgByRepoCommitsBodyFilesItemVariant1Action {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Delete => f.write_str("delete"),
            }
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoCommitsBodyFilesItemVariant1Action {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "delete" => Ok(Self::Delete),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoCommitsBodyFilesItemVariant1Action {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for PostByOrgByRepoCommitsBodyFilesItemVariant1Action
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PostByOrgByRepoCommitsBodyFilesItemVariant1Action
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PostByOrgByRepoCommitsBodyFilesItemVariant1Path`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoCommitsBodyFilesItemVariant1Path(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoCommitsBodyFilesItemVariant1Path {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoCommitsBodyFilesItemVariant1Path>
        for ::std::string::String
    {
        fn from(value: PostByOrgByRepoCommitsBodyFilesItemVariant1Path) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoCommitsBodyFilesItemVariant1Path {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoCommitsBodyFilesItemVariant1Path {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for PostByOrgByRepoCommitsBodyFilesItemVariant1Path
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PostByOrgByRepoCommitsBodyFilesItemVariant1Path
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoCommitsBodyFilesItemVariant1Path {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoCommitsBodyMessage`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoCommitsBodyMessage(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoCommitsBodyMessage {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoCommitsBodyMessage> for ::std::string::String {
        fn from(value: PostByOrgByRepoCommitsBodyMessage) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoCommitsBodyMessage {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoCommitsBodyMessage {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoCommitsBodyMessage {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoCommitsBodyMessage {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoCommitsBodyMessage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoCommitsOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoCommitsOrg(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoCommitsOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoCommitsOrg> for ::std::string::String {
        fn from(value: PostByOrgByRepoCommitsOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoCommitsOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoCommitsOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoCommitsOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoCommitsOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoCommitsOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoCommitsRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoCommitsRepo(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoCommitsRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoCommitsRepo> for ::std::string::String {
        fn from(value: PostByOrgByRepoCommitsRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoCommitsRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoCommitsRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoCommitsRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoCommitsRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoCommitsRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoCommitsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "branch",
    ///    "message",
    ///    "sha"
    ///  ],
    ///  "properties": {
    ///    "branch": {
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "sha": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoCommitsResponse {
        pub branch: ::std::string::String,
        pub message: ::std::string::String,
        pub sha: ::std::string::String,
    }

    impl PostByOrgByRepoCommitsResponse {
        pub fn builder() -> builder::PostByOrgByRepoCommitsResponse {
            Default::default()
        }
    }

    ///`PostByOrgByRepoCommitsResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoCommitsResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl PostByOrgByRepoCommitsResponseError {
        pub fn builder() -> builder::PostByOrgByRepoCommitsResponseError {
            Default::default()
        }
    }

    ///`PostByOrgByRepoSyncOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoSyncOrg(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoSyncOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoSyncOrg> for ::std::string::String {
        fn from(value: PostByOrgByRepoSyncOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoSyncOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoSyncOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoSyncOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoSyncOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoSyncOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoSyncRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoSyncRepo(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoSyncRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoSyncRepo> for ::std::string::String {
        fn from(value: PostByOrgByRepoSyncRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoSyncRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoSyncRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoSyncRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoSyncRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoSyncRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoSyncResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "success": {
    ///      "type": "boolean",
    ///      "enum": [
    ///        true
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoSyncResponse {
        pub success: bool,
    }

    impl PostByOrgByRepoSyncResponse {
        pub fn builder() -> builder::PostByOrgByRepoSyncResponse {
            Default::default()
        }
    }

    ///`PostByOrgByRepoSyncResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoSyncResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl PostByOrgByRepoSyncResponseError {
        pub fn builder() -> builder::PostByOrgByRepoSyncResponseError {
            Default::default()
        }
    }

    ///`PostByOrgByRepoWebhooksBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "branches": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "events": {
    ///      "default": [
    ///        "push"
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string",
    ///        "enum": [
    ///          "push"
    ///        ]
    ///      }
    ///    },
    ///    "globs": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "secret": {
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "url": {
    ///      "type": "string",
    ///      "format": "uri"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoWebhooksBody {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub branches: ::std::vec::Vec<::std::string::String>,
        #[serde(default = "defaults::post_by_org_by_repo_webhooks_body_events")]
        pub events: ::std::vec::Vec<PostByOrgByRepoWebhooksBodyEventsItem>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub globs: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub secret: ::std::option::Option<PostByOrgByRepoWebhooksBodySecret>,
        pub url: ::std::string::String,
    }

    impl PostByOrgByRepoWebhooksBody {
        pub fn builder() -> builder::PostByOrgByRepoWebhooksBody {
            Default::default()
        }
    }

    ///`PostByOrgByRepoWebhooksBodyEventsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "push"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostByOrgByRepoWebhooksBodyEventsItem {
        #[serde(rename = "push")]
        Push,
    }

    impl ::std::fmt::Display for PostByOrgByRepoWebhooksBodyEventsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Push => f.write_str("push"),
            }
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoWebhooksBodyEventsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "push" => Ok(Self::Push),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoWebhooksBodyEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoWebhooksBodyEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoWebhooksBodyEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PostByOrgByRepoWebhooksBodySecret`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoWebhooksBodySecret(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoWebhooksBodySecret {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoWebhooksBodySecret> for ::std::string::String {
        fn from(value: PostByOrgByRepoWebhooksBodySecret) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoWebhooksBodySecret {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoWebhooksBodySecret {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoWebhooksBodySecret {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoWebhooksBodySecret {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoWebhooksBodySecret {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoWebhooksOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoWebhooksOrg(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoWebhooksOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoWebhooksOrg> for ::std::string::String {
        fn from(value: PostByOrgByRepoWebhooksOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoWebhooksOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoWebhooksOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoWebhooksOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoWebhooksOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoWebhooksOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoWebhooksRepo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgByRepoWebhooksRepo(::std::string::String);
    impl ::std::ops::Deref for PostByOrgByRepoWebhooksRepo {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgByRepoWebhooksRepo> for ::std::string::String {
        fn from(value: PostByOrgByRepoWebhooksRepo) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoWebhooksRepo {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoWebhooksRepo {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoWebhooksRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoWebhooksRepo {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgByRepoWebhooksRepo {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgByRepoWebhooksResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "branches",
    ///    "created_at",
    ///    "events",
    ///    "globs",
    ///    "id",
    ///    "secret",
    ///    "updated_at",
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "branches": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "events": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string",
    ///        "enum": [
    ///          "push"
    ///        ]
    ///      }
    ///    },
    ///    "globs": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "url": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoWebhooksResponse {
        pub branches: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        pub created_at: ::std::string::String,
        pub events: ::std::vec::Vec<PostByOrgByRepoWebhooksResponseEventsItem>,
        pub globs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        pub id: ::std::string::String,
        pub secret: ::std::string::String,
        pub updated_at: ::std::string::String,
        pub url: ::std::string::String,
    }

    impl PostByOrgByRepoWebhooksResponse {
        pub fn builder() -> builder::PostByOrgByRepoWebhooksResponse {
            Default::default()
        }
    }

    ///`PostByOrgByRepoWebhooksResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgByRepoWebhooksResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl PostByOrgByRepoWebhooksResponseError {
        pub fn builder() -> builder::PostByOrgByRepoWebhooksResponseError {
            Default::default()
        }
    }

    ///`PostByOrgByRepoWebhooksResponseEventsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "push"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostByOrgByRepoWebhooksResponseEventsItem {
        #[serde(rename = "push")]
        Push,
    }

    impl ::std::fmt::Display for PostByOrgByRepoWebhooksResponseEventsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Push => f.write_str("push"),
            }
        }
    }

    impl ::std::str::FromStr for PostByOrgByRepoWebhooksResponseEventsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "push" => Ok(Self::Push),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgByRepoWebhooksResponseEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgByRepoWebhooksResponseEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgByRepoWebhooksResponseEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PostByOrgReposBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "default_branch": {
    ///      "default": "main",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string",
    ///      "maxLength": 100,
    ///      "minLength": 1
    ///    },
    ///    "upstream": {
    ///      "type": "object",
    ///      "required": [
    ///        "uri"
    ///      ],
    ///      "properties": {
    ///        "autosync": {
    ///          "description": "Optionally enable automatic sync from the
    /// upstream repository",
    ///          "oneOf": [
    ///            {
    ///              "type": "object",
    ///              "required": [
    ///                "period",
    ///                "type"
    ///              ],
    ///              "properties": {
    ///                "period": {
    ///                  "description": "Polling period in seconds (60s to
    /// 24.8d)",
    ///                  "type": "integer",
    ///                  "maximum": 2147483.0,
    ///                  "minimum": 60.0
    ///                },
    ///                "resolution_strategy": {
    ///                  "description": "Conflict resolution strategy. \"none\"
    /// means sync will fail on conflicts.",
    ///                  "default": "none",
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "none"
    ///                  ]
    ///                },
    ///                "type": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "poll"
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          ]
    ///        },
    ///        "uri": {
    ///          "description": "URL of the upstream repository",
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgReposBody {
        #[serde(default = "defaults::post_by_org_repos_body_default_branch")]
        pub default_branch: ::std::string::String,
        pub name: PostByOrgReposBodyName,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub upstream: ::std::option::Option<PostByOrgReposBodyUpstream>,
    }

    impl PostByOrgReposBody {
        pub fn builder() -> builder::PostByOrgReposBody {
            Default::default()
        }
    }

    ///`PostByOrgReposBodyName`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "maxLength": 100,
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgReposBodyName(::std::string::String);
    impl ::std::ops::Deref for PostByOrgReposBodyName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgReposBodyName> for ::std::string::String {
        fn from(value: PostByOrgReposBodyName) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgReposBodyName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 100usize {
                return Err("longer than 100 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgReposBodyName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgReposBodyName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgReposBodyName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgReposBodyName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgReposBodyUpstream`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "uri"
    ///  ],
    ///  "properties": {
    ///    "autosync": {
    ///      "description": "Optionally enable automatic sync from the upstream
    /// repository",
    ///      "oneOf": [
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "period",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "period": {
    ///              "description": "Polling period in seconds (60s to 24.8d)",
    ///              "type": "integer",
    ///              "maximum": 2147483.0,
    ///              "minimum": 60.0
    ///            },
    ///            "resolution_strategy": {
    ///              "description": "Conflict resolution strategy. \"none\"
    /// means sync will fail on conflicts.",
    ///              "default": "none",
    ///              "type": "string",
    ///              "enum": [
    ///                "none"
    ///              ]
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "poll"
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      ]
    ///    },
    ///    "uri": {
    ///      "description": "URL of the upstream repository",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgReposBodyUpstream {
        ///Optionally enable automatic sync from the upstream repository
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub autosync: ::std::option::Option<PostByOrgReposBodyUpstreamAutosync>,
        ///URL of the upstream repository
        pub uri: ::std::string::String,
    }

    impl PostByOrgReposBodyUpstream {
        pub fn builder() -> builder::PostByOrgReposBodyUpstream {
            Default::default()
        }
    }

    ///Optionally enable automatic sync from the upstream repository
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Optionally enable automatic sync from the upstream
    /// repository",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "period",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "period": {
    ///          "description": "Polling period in seconds (60s to 24.8d)",
    ///          "type": "integer",
    ///          "maximum": 2147483.0,
    ///          "minimum": 60.0
    ///        },
    ///        "resolution_strategy": {
    ///          "description": "Conflict resolution strategy. \"none\" means
    /// sync will fail on conflicts.",
    ///          "default": "none",
    ///          "type": "string",
    ///          "enum": [
    ///            "none"
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "poll"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum PostByOrgReposBodyUpstreamAutosync {
        #[serde(rename = "poll")]
        Poll {
            ///Polling period in seconds (60s to 24.8d)
            period: i64,
            ///Conflict resolution strategy. "none" means sync will fail on
            /// conflicts.
            #[serde(
                default = "defaults::post_by_org_repos_body_upstream_autosync_poll_resolution_strategy"
            )]
            resolution_strategy: PostByOrgReposBodyUpstreamAutosyncResolutionStrategy,
        },
    }

    ///Conflict resolution strategy. "none" means sync will fail on conflicts.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Conflict resolution strategy. \"none\" means sync will
    /// fail on conflicts.",
    ///  "default": "none",
    ///  "type": "string",
    ///  "enum": [
    ///    "none"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostByOrgReposBodyUpstreamAutosyncResolutionStrategy {
        #[serde(rename = "none")]
        None,
    }

    impl ::std::fmt::Display for PostByOrgReposBodyUpstreamAutosyncResolutionStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("none"),
            }
        }
    }

    impl ::std::str::FromStr for PostByOrgReposBodyUpstreamAutosyncResolutionStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "none" => Ok(Self::None),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgReposBodyUpstreamAutosyncResolutionStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for PostByOrgReposBodyUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PostByOrgReposBodyUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::default::Default for PostByOrgReposBodyUpstreamAutosyncResolutionStrategy {
        fn default() -> Self {
            PostByOrgReposBodyUpstreamAutosyncResolutionStrategy::None
        }
    }

    ///`PostByOrgReposOrg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostByOrgReposOrg(::std::string::String);
    impl ::std::ops::Deref for PostByOrgReposOrg {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostByOrgReposOrg> for ::std::string::String {
        fn from(value: PostByOrgReposOrg) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostByOrgReposOrg {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgReposOrg {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PostByOrgReposOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgReposOrg {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostByOrgReposOrg {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostByOrgReposResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "default_branch",
    ///    "id",
    ///    "last_push_at",
    ///    "name",
    ///    "org",
    ///    "size_bytes",
    ///    "upstream"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "default_branch": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "last_push_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "org": {
    ///      "type": "string"
    ///    },
    ///    "size_bytes": {
    ///      "type": "number"
    ///    },
    ///    "upstream": {
    ///      "description": "Optionally add an upstream repository. You can
    /// configure automatic syncing from the upstream repository.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "required": [
    ///        "autosync",
    ///        "last_sync_attempt",
    ///        "last_sync_error",
    ///        "last_sync_success",
    ///        "uri"
    ///      ],
    ///      "properties": {
    ///        "autosync": {
    ///          "description": "Automatic sync configuration, if enabled",
    ///          "type": [
    ///            "object",
    ///            "null"
    ///          ],
    ///          "required": [
    ///            "period",
    ///            "resolution_strategy",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "period": {
    ///              "description": "Polling period in seconds",
    ///              "type": "number"
    ///            },
    ///            "resolution_strategy": {
    ///              "description": "Conflict resolution strategy. \"none\"
    /// means sync will fail on conflicts.",
    ///              "type": "string",
    ///              "enum": [
    ///                "none"
    ///              ]
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "poll"
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        "last_sync_attempt": {
    ///          "description": "Timestamp of the last sync attempt",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "date-time",
    ///          "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///        },
    ///        "last_sync_error": {
    ///          "description": "Error message from the last failed sync
    /// attempt",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "last_sync_success": {
    ///          "description": "Timestamp of the last successful sync",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "date-time",
    ///          "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///        },
    ///        "uri": {
    ///          "description": "URL of the upstream repository",
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgReposResponse {
        pub created_at: ::std::string::String,
        pub default_branch: ::std::string::String,
        pub id: ::std::string::String,
        pub last_push_at: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
        pub org: ::std::string::String,
        pub size_bytes: f64,
        ///Optionally add an upstream repository. You can configure automatic
        /// syncing from the upstream repository.
        pub upstream: ::std::option::Option<PostByOrgReposResponseUpstream>,
    }

    impl PostByOrgReposResponse {
        pub fn builder() -> builder::PostByOrgReposResponse {
            Default::default()
        }
    }

    ///`PostByOrgReposResponseError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgReposResponseError {
        pub code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub details: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub message: ::std::string::String,
    }

    impl PostByOrgReposResponseError {
        pub fn builder() -> builder::PostByOrgReposResponseError {
            Default::default()
        }
    }

    ///Optionally add an upstream repository. You can configure automatic
    /// syncing from the upstream repository.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Optionally add an upstream repository. You can
    /// configure automatic syncing from the upstream repository.",
    ///  "type": "object",
    ///  "required": [
    ///    "autosync",
    ///    "last_sync_attempt",
    ///    "last_sync_error",
    ///    "last_sync_success",
    ///    "uri"
    ///  ],
    ///  "properties": {
    ///    "autosync": {
    ///      "description": "Automatic sync configuration, if enabled",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "required": [
    ///        "period",
    ///        "resolution_strategy",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "period": {
    ///          "description": "Polling period in seconds",
    ///          "type": "number"
    ///        },
    ///        "resolution_strategy": {
    ///          "description": "Conflict resolution strategy. \"none\" means
    /// sync will fail on conflicts.",
    ///          "type": "string",
    ///          "enum": [
    ///            "none"
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "poll"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "last_sync_attempt": {
    ///      "description": "Timestamp of the last sync attempt",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time",
    ///      "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///    },
    ///    "last_sync_error": {
    ///      "description": "Error message from the last failed sync attempt",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "last_sync_success": {
    ///      "description": "Timestamp of the last successful sync",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time",
    ///      "pattern":
    /// "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\
    /// d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:
    /// 0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\
    /// d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?
    /// ::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
    ///    },
    ///    "uri": {
    ///      "description": "URL of the upstream repository",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgReposResponseUpstream {
        ///Automatic sync configuration, if enabled
        pub autosync: ::std::option::Option<PostByOrgReposResponseUpstreamAutosync>,
        ///Timestamp of the last sync attempt
        pub last_sync_attempt: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Error message from the last failed sync attempt
        pub last_sync_error: ::std::option::Option<::std::string::String>,
        ///Timestamp of the last successful sync
        pub last_sync_success: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///URL of the upstream repository
        pub uri: ::std::string::String,
    }

    impl PostByOrgReposResponseUpstream {
        pub fn builder() -> builder::PostByOrgReposResponseUpstream {
            Default::default()
        }
    }

    ///Automatic sync configuration, if enabled
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Automatic sync configuration, if enabled",
    ///  "type": "object",
    ///  "required": [
    ///    "period",
    ///    "resolution_strategy",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "period": {
    ///      "description": "Polling period in seconds",
    ///      "type": "number"
    ///    },
    ///    "resolution_strategy": {
    ///      "description": "Conflict resolution strategy. \"none\" means sync
    /// will fail on conflicts.",
    ///      "type": "string",
    ///      "enum": [
    ///        "none"
    ///      ]
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "poll"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PostByOrgReposResponseUpstreamAutosync {
        pub period: f64,
        ///Conflict resolution strategy. "none" means sync will fail on
        /// conflicts.
        pub resolution_strategy: PostByOrgReposResponseUpstreamAutosyncResolutionStrategy,
        #[serde(rename = "type")]
        pub type_: PostByOrgReposResponseUpstreamAutosyncType,
    }

    impl PostByOrgReposResponseUpstreamAutosync {
        pub fn builder() -> builder::PostByOrgReposResponseUpstreamAutosync {
            Default::default()
        }
    }

    ///Conflict resolution strategy. "none" means sync will fail on conflicts.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Conflict resolution strategy. \"none\" means sync will
    /// fail on conflicts.",
    ///  "type": "string",
    ///  "enum": [
    ///    "none"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostByOrgReposResponseUpstreamAutosyncResolutionStrategy {
        #[serde(rename = "none")]
        None,
    }

    impl ::std::fmt::Display for PostByOrgReposResponseUpstreamAutosyncResolutionStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("none"),
            }
        }
    }

    impl ::std::str::FromStr for PostByOrgReposResponseUpstreamAutosyncResolutionStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "none" => Ok(Self::None),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgReposResponseUpstreamAutosyncResolutionStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for PostByOrgReposResponseUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PostByOrgReposResponseUpstreamAutosyncResolutionStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PostByOrgReposResponseUpstreamAutosyncType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "poll"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostByOrgReposResponseUpstreamAutosyncType {
        #[serde(rename = "poll")]
        Poll,
    }

    impl ::std::fmt::Display for PostByOrgReposResponseUpstreamAutosyncType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Poll => f.write_str("poll"),
            }
        }
    }

    impl ::std::str::FromStr for PostByOrgReposResponseUpstreamAutosyncType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "poll" => Ok(Self::Poll),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostByOrgReposResponseUpstreamAutosyncType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for PostByOrgReposResponseUpstreamAutosyncType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostByOrgReposResponseUpstreamAutosyncType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct DeleteByOrgApiKeysByIdResponse {
            success: ::std::result::Result<bool, ::std::string::String>,
        }

        impl ::std::default::Default for DeleteByOrgApiKeysByIdResponse {
            fn default() -> Self {
                Self {
                    success: Err("no value supplied for success".to_string()),
                }
            }
        }

        impl DeleteByOrgApiKeysByIdResponse {
            pub fn success<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.success = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for success: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<DeleteByOrgApiKeysByIdResponse>
            for super::DeleteByOrgApiKeysByIdResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DeleteByOrgApiKeysByIdResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    success: value.success?,
                })
            }
        }

        impl ::std::convert::From<super::DeleteByOrgApiKeysByIdResponse>
            for DeleteByOrgApiKeysByIdResponse
        {
            fn from(value: super::DeleteByOrgApiKeysByIdResponse) -> Self {
                Self {
                    success: Ok(value.success),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DeleteByOrgApiKeysByIdResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for DeleteByOrgApiKeysByIdResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl DeleteByOrgApiKeysByIdResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<DeleteByOrgApiKeysByIdResponseError>
            for super::DeleteByOrgApiKeysByIdResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DeleteByOrgApiKeysByIdResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::DeleteByOrgApiKeysByIdResponseError>
            for DeleteByOrgApiKeysByIdResponseError
        {
            fn from(value: super::DeleteByOrgApiKeysByIdResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DeleteByOrgByRepoBranchesByBranchResponse {
            success: ::std::result::Result<bool, ::std::string::String>,
        }

        impl ::std::default::Default for DeleteByOrgByRepoBranchesByBranchResponse {
            fn default() -> Self {
                Self {
                    success: Err("no value supplied for success".to_string()),
                }
            }
        }

        impl DeleteByOrgByRepoBranchesByBranchResponse {
            pub fn success<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.success = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for success: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<DeleteByOrgByRepoBranchesByBranchResponse>
            for super::DeleteByOrgByRepoBranchesByBranchResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DeleteByOrgByRepoBranchesByBranchResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    success: value.success?,
                })
            }
        }

        impl ::std::convert::From<super::DeleteByOrgByRepoBranchesByBranchResponse>
            for DeleteByOrgByRepoBranchesByBranchResponse
        {
            fn from(value: super::DeleteByOrgByRepoBranchesByBranchResponse) -> Self {
                Self {
                    success: Ok(value.success),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DeleteByOrgByRepoBranchesByBranchResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for DeleteByOrgByRepoBranchesByBranchResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl DeleteByOrgByRepoBranchesByBranchResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<DeleteByOrgByRepoBranchesByBranchResponseError>
            for super::DeleteByOrgByRepoBranchesByBranchResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DeleteByOrgByRepoBranchesByBranchResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::DeleteByOrgByRepoBranchesByBranchResponseError>
            for DeleteByOrgByRepoBranchesByBranchResponseError
        {
            fn from(value: super::DeleteByOrgByRepoBranchesByBranchResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DeleteByOrgByRepoResponse {
            success: ::std::result::Result<bool, ::std::string::String>,
        }

        impl ::std::default::Default for DeleteByOrgByRepoResponse {
            fn default() -> Self {
                Self {
                    success: Err("no value supplied for success".to_string()),
                }
            }
        }

        impl DeleteByOrgByRepoResponse {
            pub fn success<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.success = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for success: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<DeleteByOrgByRepoResponse> for super::DeleteByOrgByRepoResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DeleteByOrgByRepoResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    success: value.success?,
                })
            }
        }

        impl ::std::convert::From<super::DeleteByOrgByRepoResponse> for DeleteByOrgByRepoResponse {
            fn from(value: super::DeleteByOrgByRepoResponse) -> Self {
                Self {
                    success: Ok(value.success),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DeleteByOrgByRepoResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for DeleteByOrgByRepoResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl DeleteByOrgByRepoResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<DeleteByOrgByRepoResponseError>
            for super::DeleteByOrgByRepoResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DeleteByOrgByRepoResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::DeleteByOrgByRepoResponseError>
            for DeleteByOrgByRepoResponseError
        {
            fn from(value: super::DeleteByOrgByRepoResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DeleteByOrgByRepoWebhooksByWebhookIdResponse {
            success: ::std::result::Result<bool, ::std::string::String>,
        }

        impl ::std::default::Default for DeleteByOrgByRepoWebhooksByWebhookIdResponse {
            fn default() -> Self {
                Self {
                    success: Err("no value supplied for success".to_string()),
                }
            }
        }

        impl DeleteByOrgByRepoWebhooksByWebhookIdResponse {
            pub fn success<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.success = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for success: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<DeleteByOrgByRepoWebhooksByWebhookIdResponse>
            for super::DeleteByOrgByRepoWebhooksByWebhookIdResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DeleteByOrgByRepoWebhooksByWebhookIdResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    success: value.success?,
                })
            }
        }

        impl ::std::convert::From<super::DeleteByOrgByRepoWebhooksByWebhookIdResponse>
            for DeleteByOrgByRepoWebhooksByWebhookIdResponse
        {
            fn from(value: super::DeleteByOrgByRepoWebhooksByWebhookIdResponse) -> Self {
                Self {
                    success: Ok(value.success),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DeleteByOrgByRepoWebhooksByWebhookIdResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for DeleteByOrgByRepoWebhooksByWebhookIdResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl DeleteByOrgByRepoWebhooksByWebhookIdResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<DeleteByOrgByRepoWebhooksByWebhookIdResponseError>
            for super::DeleteByOrgByRepoWebhooksByWebhookIdResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DeleteByOrgByRepoWebhooksByWebhookIdResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::DeleteByOrgByRepoWebhooksByWebhookIdResponseError>
            for DeleteByOrgByRepoWebhooksByWebhookIdResponseError
        {
            fn from(value: super::DeleteByOrgByRepoWebhooksByWebhookIdResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgApiKeysResponse {
            api_keys: ::std::result::Result<
                ::std::vec::Vec<super::GetByOrgApiKeysResponseApiKeysItem>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgApiKeysResponse {
            fn default() -> Self {
                Self {
                    api_keys: Err("no value supplied for api_keys".to_string()),
                }
            }
        }

        impl GetByOrgApiKeysResponse {
            pub fn api_keys<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::GetByOrgApiKeysResponseApiKeysItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.api_keys = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for api_keys: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgApiKeysResponse> for super::GetByOrgApiKeysResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgApiKeysResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    api_keys: value.api_keys?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgApiKeysResponse> for GetByOrgApiKeysResponse {
            fn from(value: super::GetByOrgApiKeysResponse) -> Self {
                Self {
                    api_keys: Ok(value.api_keys),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgApiKeysResponseApiKeysItem {
            created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            expires_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::string::String, ::std::string::String>,
            last_used_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            revoked_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            scopes: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgApiKeysResponseApiKeysItem {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    expires_at: Err("no value supplied for expires_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    last_used_at: Err("no value supplied for last_used_at".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    revoked_at: Err("no value supplied for revoked_at".to_string()),
                    scopes: Err("no value supplied for scopes".to_string()),
                }
            }
        }

        impl GetByOrgApiKeysResponseApiKeysItem {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn expires_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.expires_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn last_used_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_used_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_used_at: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn revoked_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.revoked_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for revoked_at: {e}"));
                self
            }
            pub fn scopes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.scopes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for scopes: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgApiKeysResponseApiKeysItem>
            for super::GetByOrgApiKeysResponseApiKeysItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgApiKeysResponseApiKeysItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    expires_at: value.expires_at?,
                    id: value.id?,
                    last_used_at: value.last_used_at?,
                    name: value.name?,
                    revoked_at: value.revoked_at?,
                    scopes: value.scopes?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgApiKeysResponseApiKeysItem>
            for GetByOrgApiKeysResponseApiKeysItem
        {
            fn from(value: super::GetByOrgApiKeysResponseApiKeysItem) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    expires_at: Ok(value.expires_at),
                    id: Ok(value.id),
                    last_used_at: Ok(value.last_used_at),
                    name: Ok(value.name),
                    revoked_at: Ok(value.revoked_at),
                    scopes: Ok(value.scopes),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgApiKeysResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgApiKeysResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgApiKeysResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgApiKeysResponseError> for super::GetByOrgApiKeysResponseError {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgApiKeysResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgApiKeysResponseError> for GetByOrgApiKeysResponseError {
            fn from(value: super::GetByOrgApiKeysResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoAgentblameResponse {
            attributions: ::std::result::Result<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::vec::Vec<super::GetByOrgByRepoAgentblameResponseAttributionsValueItem>,
                >,
                ::std::string::String,
            >,
            stats: ::std::result::Result<
                super::GetByOrgByRepoAgentblameResponseStats,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoAgentblameResponse {
            fn default() -> Self {
                Self {
                    attributions: Err("no value supplied for attributions".to_string()),
                    stats: Err("no value supplied for stats".to_string()),
                }
            }
        }

        impl GetByOrgByRepoAgentblameResponse {
            pub fn attributions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        ::std::vec::Vec<
                            super::GetByOrgByRepoAgentblameResponseAttributionsValueItem,
                        >,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.attributions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for attributions: {e}"));
                self
            }
            pub fn stats<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoAgentblameResponseStats>,
                T::Error: ::std::fmt::Display,
            {
                self.stats = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for stats: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoAgentblameResponse>
            for super::GetByOrgByRepoAgentblameResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoAgentblameResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    attributions: value.attributions?,
                    stats: value.stats?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoAgentblameResponse>
            for GetByOrgByRepoAgentblameResponse
        {
            fn from(value: super::GetByOrgByRepoAgentblameResponse) -> Self {
                Self {
                    attributions: Ok(value.attributions),
                    stats: Ok(value.stats),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoAgentblameResponseAttributionsValueItem {
            confidence: ::std::result::Result<f64, ::std::string::String>,
            line_number: ::std::result::Result<f64, ::std::string::String>,
            match_type: ::std::result::Result<::std::string::String, ::std::string::String>,
            model: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            provider: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoAgentblameResponseAttributionsValueItem {
            fn default() -> Self {
                Self {
                    confidence: Err("no value supplied for confidence".to_string()),
                    line_number: Err("no value supplied for line_number".to_string()),
                    match_type: Err("no value supplied for match_type".to_string()),
                    model: Err("no value supplied for model".to_string()),
                    provider: Err("no value supplied for provider".to_string()),
                }
            }
        }

        impl GetByOrgByRepoAgentblameResponseAttributionsValueItem {
            pub fn confidence<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.confidence = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for confidence: {e}"));
                self
            }
            pub fn line_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.line_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for line_number: {e}"));
                self
            }
            pub fn match_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.match_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for match_type: {e}"));
                self
            }
            pub fn model<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.model = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for model: {e}"));
                self
            }
            pub fn provider<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.provider = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for provider: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoAgentblameResponseAttributionsValueItem>
            for super::GetByOrgByRepoAgentblameResponseAttributionsValueItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoAgentblameResponseAttributionsValueItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    confidence: value.confidence?,
                    line_number: value.line_number?,
                    match_type: value.match_type?,
                    model: value.model?,
                    provider: value.provider?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoAgentblameResponseAttributionsValueItem>
            for GetByOrgByRepoAgentblameResponseAttributionsValueItem
        {
            fn from(value: super::GetByOrgByRepoAgentblameResponseAttributionsValueItem) -> Self {
                Self {
                    confidence: Ok(value.confidence),
                    line_number: Ok(value.line_number),
                    match_type: Ok(value.match_type),
                    model: Ok(value.model),
                    provider: Ok(value.provider),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoAgentblameResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoAgentblameResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgByRepoAgentblameResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoAgentblameResponseError>
            for super::GetByOrgByRepoAgentblameResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoAgentblameResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoAgentblameResponseError>
            for GetByOrgByRepoAgentblameResponseError
        {
            fn from(value: super::GetByOrgByRepoAgentblameResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoAgentblameResponseStats {
            ai_lines: ::std::result::Result<f64, ::std::string::String>,
            ai_percentage: ::std::result::Result<f64, ::std::string::String>,
            human_lines: ::std::result::Result<f64, ::std::string::String>,
            total_lines: ::std::result::Result<f64, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoAgentblameResponseStats {
            fn default() -> Self {
                Self {
                    ai_lines: Err("no value supplied for ai_lines".to_string()),
                    ai_percentage: Err("no value supplied for ai_percentage".to_string()),
                    human_lines: Err("no value supplied for human_lines".to_string()),
                    total_lines: Err("no value supplied for total_lines".to_string()),
                }
            }
        }

        impl GetByOrgByRepoAgentblameResponseStats {
            pub fn ai_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.ai_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ai_lines: {e}"));
                self
            }
            pub fn ai_percentage<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.ai_percentage = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ai_percentage: {e}"));
                self
            }
            pub fn human_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.human_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for human_lines: {e}"));
                self
            }
            pub fn total_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.total_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_lines: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoAgentblameResponseStats>
            for super::GetByOrgByRepoAgentblameResponseStats
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoAgentblameResponseStats,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ai_lines: value.ai_lines?,
                    ai_percentage: value.ai_percentage?,
                    human_lines: value.human_lines?,
                    total_lines: value.total_lines?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoAgentblameResponseStats>
            for GetByOrgByRepoAgentblameResponseStats
        {
            fn from(value: super::GetByOrgByRepoAgentblameResponseStats) -> Self {
                Self {
                    ai_lines: Ok(value.ai_lines),
                    ai_percentage: Ok(value.ai_percentage),
                    human_lines: Ok(value.human_lines),
                    total_lines: Ok(value.total_lines),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoAnalyticsResponse {
            analytics: ::std::result::Result<
                ::std::option::Option<super::GetByOrgByRepoAnalyticsResponseAnalytics>,
                ::std::string::String,
            >,
            status: ::std::result::Result<
                super::GetByOrgByRepoAnalyticsResponseStatus,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoAnalyticsResponse {
            fn default() -> Self {
                Self {
                    analytics: Err("no value supplied for analytics".to_string()),
                    status: Err("no value supplied for status".to_string()),
                }
            }
        }

        impl GetByOrgByRepoAnalyticsResponse {
            pub fn analytics<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::GetByOrgByRepoAnalyticsResponseAnalytics>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.analytics = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for analytics: {e}"));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoAnalyticsResponseStatus>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoAnalyticsResponse>
            for super::GetByOrgByRepoAnalyticsResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoAnalyticsResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    analytics: value.analytics?,
                    status: value.status?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoAnalyticsResponse>
            for GetByOrgByRepoAnalyticsResponse
        {
            fn from(value: super::GetByOrgByRepoAnalyticsResponse) -> Self {
                Self {
                    analytics: Ok(value.analytics),
                    status: Ok(value.status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoAnalyticsResponseAnalytics {
            contributors: ::std::result::Result<
                ::std::collections::HashMap<
                    ::std::string::String,
                    super::GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue,
                >,
                ::std::string::String,
            >,
            history: ::std::result::Result<
                ::std::vec::Vec<super::GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem>,
                ::std::string::String,
            >,
            meta: ::std::result::Result<
                ::std::option::Option<super::GetByOrgByRepoAnalyticsResponseAnalyticsMeta>,
                ::std::string::String,
            >,
            summary: ::std::result::Result<
                super::GetByOrgByRepoAnalyticsResponseAnalyticsSummary,
                ::std::string::String,
            >,
            version: ::std::result::Result<
                super::GetByOrgByRepoAnalyticsResponseAnalyticsVersion,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoAnalyticsResponseAnalytics {
            fn default() -> Self {
                Self {
                    contributors: Err("no value supplied for contributors".to_string()),
                    history: Err("no value supplied for history".to_string()),
                    meta: Ok(Default::default()),
                    summary: Err("no value supplied for summary".to_string()),
                    version: Err("no value supplied for version".to_string()),
                }
            }
        }

        impl GetByOrgByRepoAnalyticsResponseAnalytics {
            pub fn contributors<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        super::GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.contributors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for contributors: {e}"));
                self
            }
            pub fn history<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.history = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for history: {e}"));
                self
            }
            pub fn meta<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::GetByOrgByRepoAnalyticsResponseAnalyticsMeta>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.meta = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for meta: {e}"));
                self
            }
            pub fn summary<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoAnalyticsResponseAnalyticsSummary>,
                T::Error: ::std::fmt::Display,
            {
                self.summary = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for summary: {e}"));
                self
            }
            pub fn version<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoAnalyticsResponseAnalyticsVersion>,
                T::Error: ::std::fmt::Display,
            {
                self.version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for version: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoAnalyticsResponseAnalytics>
            for super::GetByOrgByRepoAnalyticsResponseAnalytics
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoAnalyticsResponseAnalytics,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    contributors: value.contributors?,
                    history: value.history?,
                    meta: value.meta?,
                    summary: value.summary?,
                    version: value.version?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoAnalyticsResponseAnalytics>
            for GetByOrgByRepoAnalyticsResponseAnalytics
        {
            fn from(value: super::GetByOrgByRepoAnalyticsResponseAnalytics) -> Self {
                Self {
                    contributors: Ok(value.contributors),
                    history: Ok(value.history),
                    meta: Ok(value.meta),
                    summary: Ok(value.summary),
                    version: Ok(value.version),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue {
            ai_lines: ::std::result::Result<f64, ::std::string::String>,
            commit_count: ::std::result::Result<f64, ::std::string::String>,
            models: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, f64>,
                ::std::string::String,
            >,
            providers: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, f64>,
                ::std::string::String,
            >,
            total_lines: ::std::result::Result<f64, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue {
            fn default() -> Self {
                Self {
                    ai_lines: Err("no value supplied for ai_lines".to_string()),
                    commit_count: Err("no value supplied for commit_count".to_string()),
                    models: Err("no value supplied for models".to_string()),
                    providers: Err("no value supplied for providers".to_string()),
                    total_lines: Err("no value supplied for total_lines".to_string()),
                }
            }
        }

        impl GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue {
            pub fn ai_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.ai_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ai_lines: {e}"));
                self
            }
            pub fn commit_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.commit_count = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commit_count: {e}"));
                self
            }
            pub fn models<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.models = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for models: {e}"));
                self
            }
            pub fn providers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.providers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for providers: {e}"));
                self
            }
            pub fn total_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.total_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_lines: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue>
            for super::GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ai_lines: value.ai_lines?,
                    commit_count: value.commit_count?,
                    models: value.models?,
                    providers: value.providers?,
                    total_lines: value.total_lines?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue>
            for GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue
        {
            fn from(
                value: super::GetByOrgByRepoAnalyticsResponseAnalyticsContributorsValue,
            ) -> Self {
                Self {
                    ai_lines: Ok(value.ai_lines),
                    commit_count: Ok(value.commit_count),
                    models: Ok(value.models),
                    providers: Ok(value.providers),
                    total_lines: Ok(value.total_lines),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem {
            added: ::std::result::Result<f64, ::std::string::String>,
            ai_lines: ::std::result::Result<f64, ::std::string::String>,
            author: ::std::result::Result<::std::string::String, ::std::string::String>,
            date: ::std::result::Result<::std::string::String, ::std::string::String>,
            message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            models: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, f64>,
                ::std::string::String,
            >,
            providers: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, f64>,
                ::std::string::String,
            >,
            removed: ::std::result::Result<f64, ::std::string::String>,
            sha: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem {
            fn default() -> Self {
                Self {
                    added: Err("no value supplied for added".to_string()),
                    ai_lines: Err("no value supplied for ai_lines".to_string()),
                    author: Err("no value supplied for author".to_string()),
                    date: Err("no value supplied for date".to_string()),
                    message: Ok(Default::default()),
                    models: Ok(Default::default()),
                    providers: Ok(Default::default()),
                    removed: Err("no value supplied for removed".to_string()),
                    sha: Err("no value supplied for sha".to_string()),
                }
            }
        }

        impl GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem {
            pub fn added<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.added = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for added: {e}"));
                self
            }
            pub fn ai_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.ai_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ai_lines: {e}"));
                self
            }
            pub fn author<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.author = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for author: {e}"));
                self
            }
            pub fn date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for date: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
            pub fn models<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.models = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for models: {e}"));
                self
            }
            pub fn providers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.providers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for providers: {e}"));
                self
            }
            pub fn removed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.removed = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for removed: {e}"));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem>
            for super::GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    added: value.added?,
                    ai_lines: value.ai_lines?,
                    author: value.author?,
                    date: value.date?,
                    message: value.message?,
                    models: value.models?,
                    providers: value.providers?,
                    removed: value.removed?,
                    sha: value.sha?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem>
            for GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem
        {
            fn from(value: super::GetByOrgByRepoAnalyticsResponseAnalyticsHistoryItem) -> Self {
                Self {
                    added: Ok(value.added),
                    ai_lines: Ok(value.ai_lines),
                    author: Ok(value.author),
                    date: Ok(value.date),
                    message: Ok(value.message),
                    models: Ok(value.models),
                    providers: Ok(value.providers),
                    removed: Ok(value.removed),
                    sha: Ok(value.sha),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoAnalyticsResponseAnalyticsMeta {
            last_attempt: ::std::result::Result<::std::string::String, ::std::string::String>,
            last_error: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            last_success: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoAnalyticsResponseAnalyticsMeta {
            fn default() -> Self {
                Self {
                    last_attempt: Err("no value supplied for last_attempt".to_string()),
                    last_error: Err("no value supplied for last_error".to_string()),
                    last_success: Err("no value supplied for last_success".to_string()),
                }
            }
        }

        impl GetByOrgByRepoAnalyticsResponseAnalyticsMeta {
            pub fn last_attempt<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.last_attempt = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_attempt: {e}"));
                self
            }
            pub fn last_error<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_error = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_error: {e}"));
                self
            }
            pub fn last_success<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_success = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_success: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoAnalyticsResponseAnalyticsMeta>
            for super::GetByOrgByRepoAnalyticsResponseAnalyticsMeta
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoAnalyticsResponseAnalyticsMeta,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    last_attempt: value.last_attempt?,
                    last_error: value.last_error?,
                    last_success: value.last_success?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoAnalyticsResponseAnalyticsMeta>
            for GetByOrgByRepoAnalyticsResponseAnalyticsMeta
        {
            fn from(value: super::GetByOrgByRepoAnalyticsResponseAnalyticsMeta) -> Self {
                Self {
                    last_attempt: Ok(value.last_attempt),
                    last_error: Ok(value.last_error),
                    last_success: Ok(value.last_success),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoAnalyticsResponseAnalyticsSummary {
            ai_lines: ::std::result::Result<f64, ::std::string::String>,
            human_lines: ::std::result::Result<f64, ::std::string::String>,
            models: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, f64>,
                ::std::string::String,
            >,
            providers: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, f64>,
                ::std::string::String,
            >,
            total_lines: ::std::result::Result<f64, ::std::string::String>,
            updated: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoAnalyticsResponseAnalyticsSummary {
            fn default() -> Self {
                Self {
                    ai_lines: Err("no value supplied for ai_lines".to_string()),
                    human_lines: Err("no value supplied for human_lines".to_string()),
                    models: Err("no value supplied for models".to_string()),
                    providers: Err("no value supplied for providers".to_string()),
                    total_lines: Err("no value supplied for total_lines".to_string()),
                    updated: Err("no value supplied for updated".to_string()),
                }
            }
        }

        impl GetByOrgByRepoAnalyticsResponseAnalyticsSummary {
            pub fn ai_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.ai_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ai_lines: {e}"));
                self
            }
            pub fn human_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.human_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for human_lines: {e}"));
                self
            }
            pub fn models<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.models = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for models: {e}"));
                self
            }
            pub fn providers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.providers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for providers: {e}"));
                self
            }
            pub fn total_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.total_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_lines: {e}"));
                self
            }
            pub fn updated<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.updated = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoAnalyticsResponseAnalyticsSummary>
            for super::GetByOrgByRepoAnalyticsResponseAnalyticsSummary
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoAnalyticsResponseAnalyticsSummary,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ai_lines: value.ai_lines?,
                    human_lines: value.human_lines?,
                    models: value.models?,
                    providers: value.providers?,
                    total_lines: value.total_lines?,
                    updated: value.updated?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoAnalyticsResponseAnalyticsSummary>
            for GetByOrgByRepoAnalyticsResponseAnalyticsSummary
        {
            fn from(value: super::GetByOrgByRepoAnalyticsResponseAnalyticsSummary) -> Self {
                Self {
                    ai_lines: Ok(value.ai_lines),
                    human_lines: Ok(value.human_lines),
                    models: Ok(value.models),
                    providers: Ok(value.providers),
                    total_lines: Ok(value.total_lines),
                    updated: Ok(value.updated),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoAnalyticsResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoAnalyticsResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgByRepoAnalyticsResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoAnalyticsResponseError>
            for super::GetByOrgByRepoAnalyticsResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoAnalyticsResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoAnalyticsResponseError>
            for GetByOrgByRepoAnalyticsResponseError
        {
            fn from(value: super::GetByOrgByRepoAnalyticsResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoBranchesResponse {
            branches: ::std::result::Result<
                ::std::vec::Vec<super::GetByOrgByRepoBranchesResponseBranchesItem>,
                ::std::string::String,
            >,
            has_more: ::std::result::Result<bool, ::std::string::String>,
            next_cursor: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoBranchesResponse {
            fn default() -> Self {
                Self {
                    branches: Err("no value supplied for branches".to_string()),
                    has_more: Err("no value supplied for has_more".to_string()),
                    next_cursor: Err("no value supplied for next_cursor".to_string()),
                }
            }
        }

        impl GetByOrgByRepoBranchesResponse {
            pub fn branches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::GetByOrgByRepoBranchesResponseBranchesItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.branches = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for branches: {e}"));
                self
            }
            pub fn has_more<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.has_more = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for has_more: {e}"));
                self
            }
            pub fn next_cursor<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.next_cursor = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoBranchesResponse>
            for super::GetByOrgByRepoBranchesResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoBranchesResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    branches: value.branches?,
                    has_more: value.has_more?,
                    next_cursor: value.next_cursor?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoBranchesResponse>
            for GetByOrgByRepoBranchesResponse
        {
            fn from(value: super::GetByOrgByRepoBranchesResponse) -> Self {
                Self {
                    branches: Ok(value.branches),
                    has_more: Ok(value.has_more),
                    next_cursor: Ok(value.next_cursor),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoBranchesResponseBranchesItem {
            head_sha: ::std::result::Result<::std::string::String, ::std::string::String>,
            is_default: ::std::result::Result<bool, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoBranchesResponseBranchesItem {
            fn default() -> Self {
                Self {
                    head_sha: Err("no value supplied for head_sha".to_string()),
                    is_default: Err("no value supplied for is_default".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl GetByOrgByRepoBranchesResponseBranchesItem {
            pub fn head_sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.head_sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for head_sha: {e}"));
                self
            }
            pub fn is_default<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.is_default = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_default: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoBranchesResponseBranchesItem>
            for super::GetByOrgByRepoBranchesResponseBranchesItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoBranchesResponseBranchesItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    head_sha: value.head_sha?,
                    is_default: value.is_default?,
                    name: value.name?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoBranchesResponseBranchesItem>
            for GetByOrgByRepoBranchesResponseBranchesItem
        {
            fn from(value: super::GetByOrgByRepoBranchesResponseBranchesItem) -> Self {
                Self {
                    head_sha: Ok(value.head_sha),
                    is_default: Ok(value.is_default),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoBranchesResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoBranchesResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgByRepoBranchesResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoBranchesResponseError>
            for super::GetByOrgByRepoBranchesResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoBranchesResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoBranchesResponseError>
            for GetByOrgByRepoBranchesResponseError
        {
            fn from(value: super::GetByOrgByRepoBranchesResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoCommitsByShaResponse {
            author: ::std::result::Result<
                super::GetByOrgByRepoCommitsByShaResponseAuthor,
                ::std::string::String,
            >,
            committer: ::std::result::Result<
                super::GetByOrgByRepoCommitsByShaResponseCommitter,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
            sha: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoCommitsByShaResponse {
            fn default() -> Self {
                Self {
                    author: Err("no value supplied for author".to_string()),
                    committer: Err("no value supplied for committer".to_string()),
                    message: Err("no value supplied for message".to_string()),
                    sha: Err("no value supplied for sha".to_string()),
                }
            }
        }

        impl GetByOrgByRepoCommitsByShaResponse {
            pub fn author<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoCommitsByShaResponseAuthor>,
                T::Error: ::std::fmt::Display,
            {
                self.author = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for author: {e}"));
                self
            }
            pub fn committer<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoCommitsByShaResponseCommitter>,
                T::Error: ::std::fmt::Display,
            {
                self.committer = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for committer: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoCommitsByShaResponse>
            for super::GetByOrgByRepoCommitsByShaResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoCommitsByShaResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    author: value.author?,
                    committer: value.committer?,
                    message: value.message?,
                    sha: value.sha?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoCommitsByShaResponse>
            for GetByOrgByRepoCommitsByShaResponse
        {
            fn from(value: super::GetByOrgByRepoCommitsByShaResponse) -> Self {
                Self {
                    author: Ok(value.author),
                    committer: Ok(value.committer),
                    message: Ok(value.message),
                    sha: Ok(value.sha),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoCommitsByShaResponseAuthor {
            date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            email: ::std::result::Result<::std::string::String, ::std::string::String>,
            name: ::std::result::Result<
                super::GetByOrgByRepoCommitsByShaResponseAuthorName,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoCommitsByShaResponseAuthor {
            fn default() -> Self {
                Self {
                    date: Ok(Default::default()),
                    email: Err("no value supplied for email".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl GetByOrgByRepoCommitsByShaResponseAuthor {
            pub fn date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for date: {e}"));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoCommitsByShaResponseAuthorName>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoCommitsByShaResponseAuthor>
            for super::GetByOrgByRepoCommitsByShaResponseAuthor
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoCommitsByShaResponseAuthor,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    date: value.date?,
                    email: value.email?,
                    name: value.name?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoCommitsByShaResponseAuthor>
            for GetByOrgByRepoCommitsByShaResponseAuthor
        {
            fn from(value: super::GetByOrgByRepoCommitsByShaResponseAuthor) -> Self {
                Self {
                    date: Ok(value.date),
                    email: Ok(value.email),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoCommitsByShaResponseCommitter {
            date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            email: ::std::result::Result<::std::string::String, ::std::string::String>,
            name: ::std::result::Result<
                super::GetByOrgByRepoCommitsByShaResponseCommitterName,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoCommitsByShaResponseCommitter {
            fn default() -> Self {
                Self {
                    date: Ok(Default::default()),
                    email: Err("no value supplied for email".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl GetByOrgByRepoCommitsByShaResponseCommitter {
            pub fn date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for date: {e}"));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoCommitsByShaResponseCommitterName>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoCommitsByShaResponseCommitter>
            for super::GetByOrgByRepoCommitsByShaResponseCommitter
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoCommitsByShaResponseCommitter,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    date: value.date?,
                    email: value.email?,
                    name: value.name?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoCommitsByShaResponseCommitter>
            for GetByOrgByRepoCommitsByShaResponseCommitter
        {
            fn from(value: super::GetByOrgByRepoCommitsByShaResponseCommitter) -> Self {
                Self {
                    date: Ok(value.date),
                    email: Ok(value.email),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoCommitsByShaResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoCommitsByShaResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgByRepoCommitsByShaResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoCommitsByShaResponseError>
            for super::GetByOrgByRepoCommitsByShaResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoCommitsByShaResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoCommitsByShaResponseError>
            for GetByOrgByRepoCommitsByShaResponseError
        {
            fn from(value: super::GetByOrgByRepoCommitsByShaResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoCommitsResponse {
            commits: ::std::result::Result<
                ::std::vec::Vec<super::GetByOrgByRepoCommitsResponseCommitsItem>,
                ::std::string::String,
            >,
            has_more: ::std::result::Result<bool, ::std::string::String>,
            next_cursor: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoCommitsResponse {
            fn default() -> Self {
                Self {
                    commits: Err("no value supplied for commits".to_string()),
                    has_more: Err("no value supplied for has_more".to_string()),
                    next_cursor: Err("no value supplied for next_cursor".to_string()),
                }
            }
        }

        impl GetByOrgByRepoCommitsResponse {
            pub fn commits<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::GetByOrgByRepoCommitsResponseCommitsItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.commits = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commits: {e}"));
                self
            }
            pub fn has_more<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.has_more = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for has_more: {e}"));
                self
            }
            pub fn next_cursor<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.next_cursor = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoCommitsResponse>
            for super::GetByOrgByRepoCommitsResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoCommitsResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    commits: value.commits?,
                    has_more: value.has_more?,
                    next_cursor: value.next_cursor?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoCommitsResponse> for GetByOrgByRepoCommitsResponse {
            fn from(value: super::GetByOrgByRepoCommitsResponse) -> Self {
                Self {
                    commits: Ok(value.commits),
                    has_more: Ok(value.has_more),
                    next_cursor: Ok(value.next_cursor),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoCommitsResponseCommitsItem {
            author: ::std::result::Result<
                super::GetByOrgByRepoCommitsResponseCommitsItemAuthor,
                ::std::string::String,
            >,
            committer: ::std::result::Result<
                super::GetByOrgByRepoCommitsResponseCommitsItemCommitter,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
            sha: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoCommitsResponseCommitsItem {
            fn default() -> Self {
                Self {
                    author: Err("no value supplied for author".to_string()),
                    committer: Err("no value supplied for committer".to_string()),
                    message: Err("no value supplied for message".to_string()),
                    sha: Err("no value supplied for sha".to_string()),
                }
            }
        }

        impl GetByOrgByRepoCommitsResponseCommitsItem {
            pub fn author<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoCommitsResponseCommitsItemAuthor>,
                T::Error: ::std::fmt::Display,
            {
                self.author = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for author: {e}"));
                self
            }
            pub fn committer<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::GetByOrgByRepoCommitsResponseCommitsItemCommitter,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.committer = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for committer: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoCommitsResponseCommitsItem>
            for super::GetByOrgByRepoCommitsResponseCommitsItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoCommitsResponseCommitsItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    author: value.author?,
                    committer: value.committer?,
                    message: value.message?,
                    sha: value.sha?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoCommitsResponseCommitsItem>
            for GetByOrgByRepoCommitsResponseCommitsItem
        {
            fn from(value: super::GetByOrgByRepoCommitsResponseCommitsItem) -> Self {
                Self {
                    author: Ok(value.author),
                    committer: Ok(value.committer),
                    message: Ok(value.message),
                    sha: Ok(value.sha),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoCommitsResponseCommitsItemAuthor {
            date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            email: ::std::result::Result<::std::string::String, ::std::string::String>,
            name: ::std::result::Result<
                super::GetByOrgByRepoCommitsResponseCommitsItemAuthorName,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoCommitsResponseCommitsItemAuthor {
            fn default() -> Self {
                Self {
                    date: Ok(Default::default()),
                    email: Err("no value supplied for email".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl GetByOrgByRepoCommitsResponseCommitsItemAuthor {
            pub fn date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for date: {e}"));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::GetByOrgByRepoCommitsResponseCommitsItemAuthorName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoCommitsResponseCommitsItemAuthor>
            for super::GetByOrgByRepoCommitsResponseCommitsItemAuthor
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoCommitsResponseCommitsItemAuthor,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    date: value.date?,
                    email: value.email?,
                    name: value.name?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoCommitsResponseCommitsItemAuthor>
            for GetByOrgByRepoCommitsResponseCommitsItemAuthor
        {
            fn from(value: super::GetByOrgByRepoCommitsResponseCommitsItemAuthor) -> Self {
                Self {
                    date: Ok(value.date),
                    email: Ok(value.email),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoCommitsResponseCommitsItemCommitter {
            date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            email: ::std::result::Result<::std::string::String, ::std::string::String>,
            name: ::std::result::Result<
                super::GetByOrgByRepoCommitsResponseCommitsItemCommitterName,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoCommitsResponseCommitsItemCommitter {
            fn default() -> Self {
                Self {
                    date: Ok(Default::default()),
                    email: Err("no value supplied for email".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl GetByOrgByRepoCommitsResponseCommitsItemCommitter {
            pub fn date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for date: {e}"));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::GetByOrgByRepoCommitsResponseCommitsItemCommitterName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoCommitsResponseCommitsItemCommitter>
            for super::GetByOrgByRepoCommitsResponseCommitsItemCommitter
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoCommitsResponseCommitsItemCommitter,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    date: value.date?,
                    email: value.email?,
                    name: value.name?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoCommitsResponseCommitsItemCommitter>
            for GetByOrgByRepoCommitsResponseCommitsItemCommitter
        {
            fn from(value: super::GetByOrgByRepoCommitsResponseCommitsItemCommitter) -> Self {
                Self {
                    date: Ok(value.date),
                    email: Ok(value.email),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoCommitsResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoCommitsResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgByRepoCommitsResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoCommitsResponseError>
            for super::GetByOrgByRepoCommitsResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoCommitsResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoCommitsResponseError>
            for GetByOrgByRepoCommitsResponseError
        {
            fn from(value: super::GetByOrgByRepoCommitsResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoContentResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoContentResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgByRepoContentResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoContentResponseError>
            for super::GetByOrgByRepoContentResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoContentResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoContentResponseError>
            for GetByOrgByRepoContentResponseError
        {
            fn from(value: super::GetByOrgByRepoContentResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoDiffResponse {
            base: ::std::result::Result<::std::string::String, ::std::string::String>,
            files: ::std::result::Result<
                ::std::vec::Vec<super::GetByOrgByRepoDiffResponseFilesItem>,
                ::std::string::String,
            >,
            filtered_files: ::std::result::Result<
                ::std::vec::Vec<super::GetByOrgByRepoDiffResponseFilteredFilesItem>,
                ::std::string::String,
            >,
            head: ::std::result::Result<::std::string::String, ::std::string::String>,
            stats: ::std::result::Result<
                super::GetByOrgByRepoDiffResponseStats,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoDiffResponse {
            fn default() -> Self {
                Self {
                    base: Err("no value supplied for base".to_string()),
                    files: Err("no value supplied for files".to_string()),
                    filtered_files: Err("no value supplied for filtered_files".to_string()),
                    head: Err("no value supplied for head".to_string()),
                    stats: Err("no value supplied for stats".to_string()),
                }
            }
        }

        impl GetByOrgByRepoDiffResponse {
            pub fn base<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.base = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for base: {e}"));
                self
            }
            pub fn files<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::GetByOrgByRepoDiffResponseFilesItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.files = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for files: {e}"));
                self
            }
            pub fn filtered_files<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::GetByOrgByRepoDiffResponseFilteredFilesItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.filtered_files = value.try_into().map_err(|e| {
                    format!("error converting supplied value for filtered_files: {e}")
                });
                self
            }
            pub fn head<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.head = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for head: {e}"));
                self
            }
            pub fn stats<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoDiffResponseStats>,
                T::Error: ::std::fmt::Display,
            {
                self.stats = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for stats: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoDiffResponse> for super::GetByOrgByRepoDiffResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoDiffResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    base: value.base?,
                    files: value.files?,
                    filtered_files: value.filtered_files?,
                    head: value.head?,
                    stats: value.stats?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoDiffResponse> for GetByOrgByRepoDiffResponse {
            fn from(value: super::GetByOrgByRepoDiffResponse) -> Self {
                Self {
                    base: Ok(value.base),
                    files: Ok(value.files),
                    filtered_files: Ok(value.filtered_files),
                    head: Ok(value.head),
                    stats: Ok(value.stats),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoDiffResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoDiffResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgByRepoDiffResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoDiffResponseError>
            for super::GetByOrgByRepoDiffResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoDiffResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoDiffResponseError>
            for GetByOrgByRepoDiffResponseError
        {
            fn from(value: super::GetByOrgByRepoDiffResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoDiffResponseFilesItem {
            bytes: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            is_eof: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            old_path: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            path: ::std::result::Result<::std::string::String, ::std::string::String>,
            raw: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            status: ::std::result::Result<
                super::GetByOrgByRepoDiffResponseFilesItemStatus,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoDiffResponseFilesItem {
            fn default() -> Self {
                Self {
                    bytes: Ok(Default::default()),
                    is_eof: Ok(Default::default()),
                    old_path: Ok(Default::default()),
                    path: Err("no value supplied for path".to_string()),
                    raw: Ok(Default::default()),
                    status: Err("no value supplied for status".to_string()),
                }
            }
        }

        impl GetByOrgByRepoDiffResponseFilesItem {
            pub fn bytes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bytes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bytes: {e}"));
                self
            }
            pub fn is_eof<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_eof = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_eof: {e}"));
                self
            }
            pub fn old_path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.old_path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for old_path: {e}"));
                self
            }
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {e}"));
                self
            }
            pub fn raw<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.raw = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for raw: {e}"));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoDiffResponseFilesItemStatus>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoDiffResponseFilesItem>
            for super::GetByOrgByRepoDiffResponseFilesItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoDiffResponseFilesItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    bytes: value.bytes?,
                    is_eof: value.is_eof?,
                    old_path: value.old_path?,
                    path: value.path?,
                    raw: value.raw?,
                    status: value.status?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoDiffResponseFilesItem>
            for GetByOrgByRepoDiffResponseFilesItem
        {
            fn from(value: super::GetByOrgByRepoDiffResponseFilesItem) -> Self {
                Self {
                    bytes: Ok(value.bytes),
                    is_eof: Ok(value.is_eof),
                    old_path: Ok(value.old_path),
                    path: Ok(value.path),
                    raw: Ok(value.raw),
                    status: Ok(value.status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoDiffResponseFilteredFilesItem {
            bytes: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            is_eof: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            old_path: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            path: ::std::result::Result<::std::string::String, ::std::string::String>,
            status: ::std::result::Result<
                super::GetByOrgByRepoDiffResponseFilteredFilesItemStatus,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoDiffResponseFilteredFilesItem {
            fn default() -> Self {
                Self {
                    bytes: Ok(Default::default()),
                    is_eof: Ok(Default::default()),
                    old_path: Ok(Default::default()),
                    path: Err("no value supplied for path".to_string()),
                    status: Err("no value supplied for status".to_string()),
                }
            }
        }

        impl GetByOrgByRepoDiffResponseFilteredFilesItem {
            pub fn bytes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bytes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bytes: {e}"));
                self
            }
            pub fn is_eof<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_eof = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_eof: {e}"));
                self
            }
            pub fn old_path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.old_path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for old_path: {e}"));
                self
            }
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {e}"));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::GetByOrgByRepoDiffResponseFilteredFilesItemStatus,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoDiffResponseFilteredFilesItem>
            for super::GetByOrgByRepoDiffResponseFilteredFilesItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoDiffResponseFilteredFilesItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    bytes: value.bytes?,
                    is_eof: value.is_eof?,
                    old_path: value.old_path?,
                    path: value.path?,
                    status: value.status?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoDiffResponseFilteredFilesItem>
            for GetByOrgByRepoDiffResponseFilteredFilesItem
        {
            fn from(value: super::GetByOrgByRepoDiffResponseFilteredFilesItem) -> Self {
                Self {
                    bytes: Ok(value.bytes),
                    is_eof: Ok(value.is_eof),
                    old_path: Ok(value.old_path),
                    path: Ok(value.path),
                    status: Ok(value.status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoDiffResponseStats {
            additions: ::std::result::Result<f64, ::std::string::String>,
            changes: ::std::result::Result<f64, ::std::string::String>,
            deletions: ::std::result::Result<f64, ::std::string::String>,
            files: ::std::result::Result<f64, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoDiffResponseStats {
            fn default() -> Self {
                Self {
                    additions: Err("no value supplied for additions".to_string()),
                    changes: Err("no value supplied for changes".to_string()),
                    deletions: Err("no value supplied for deletions".to_string()),
                    files: Err("no value supplied for files".to_string()),
                }
            }
        }

        impl GetByOrgByRepoDiffResponseStats {
            pub fn additions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.additions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for additions: {e}"));
                self
            }
            pub fn changes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.changes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for changes: {e}"));
                self
            }
            pub fn deletions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.deletions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deletions: {e}"));
                self
            }
            pub fn files<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.files = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for files: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoDiffResponseStats>
            for super::GetByOrgByRepoDiffResponseStats
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoDiffResponseStats,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    additions: value.additions?,
                    changes: value.changes?,
                    deletions: value.deletions?,
                    files: value.files?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoDiffResponseStats>
            for GetByOrgByRepoDiffResponseStats
        {
            fn from(value: super::GetByOrgByRepoDiffResponseStats) -> Self {
                Self {
                    additions: Ok(value.additions),
                    changes: Ok(value.changes),
                    deletions: Ok(value.deletions),
                    files: Ok(value.files),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoResponse {
            created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            default_branch: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<::std::string::String, ::std::string::String>,
            last_push_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            org: ::std::result::Result<::std::string::String, ::std::string::String>,
            size_bytes: ::std::result::Result<f64, ::std::string::String>,
            upstream: ::std::result::Result<
                ::std::option::Option<super::GetByOrgByRepoResponseUpstream>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    default_branch: Err("no value supplied for default_branch".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    last_push_at: Err("no value supplied for last_push_at".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    org: Err("no value supplied for org".to_string()),
                    size_bytes: Err("no value supplied for size_bytes".to_string()),
                    upstream: Err("no value supplied for upstream".to_string()),
                }
            }
        }

        impl GetByOrgByRepoResponse {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn default_branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.default_branch = value.try_into().map_err(|e| {
                    format!("error converting supplied value for default_branch: {e}")
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn last_push_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_push_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_push_at: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn org<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.org = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for org: {e}"));
                self
            }
            pub fn size_bytes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.size_bytes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size_bytes: {e}"));
                self
            }
            pub fn upstream<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::GetByOrgByRepoResponseUpstream>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.upstream = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for upstream: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoResponse> for super::GetByOrgByRepoResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    default_branch: value.default_branch?,
                    id: value.id?,
                    last_push_at: value.last_push_at?,
                    name: value.name?,
                    org: value.org?,
                    size_bytes: value.size_bytes?,
                    upstream: value.upstream?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoResponse> for GetByOrgByRepoResponse {
            fn from(value: super::GetByOrgByRepoResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    default_branch: Ok(value.default_branch),
                    id: Ok(value.id),
                    last_push_at: Ok(value.last_push_at),
                    name: Ok(value.name),
                    org: Ok(value.org),
                    size_bytes: Ok(value.size_bytes),
                    upstream: Ok(value.upstream),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgByRepoResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoResponseError> for super::GetByOrgByRepoResponseError {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoResponseError> for GetByOrgByRepoResponseError {
            fn from(value: super::GetByOrgByRepoResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoResponseUpstream {
            autosync: ::std::result::Result<
                ::std::option::Option<super::GetByOrgByRepoResponseUpstreamAutosync>,
                ::std::string::String,
            >,
            last_sync_attempt: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            last_sync_error: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            last_sync_success: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoResponseUpstream {
            fn default() -> Self {
                Self {
                    autosync: Err("no value supplied for autosync".to_string()),
                    last_sync_attempt: Err("no value supplied for last_sync_attempt".to_string()),
                    last_sync_error: Err("no value supplied for last_sync_error".to_string()),
                    last_sync_success: Err("no value supplied for last_sync_success".to_string()),
                    uri: Err("no value supplied for uri".to_string()),
                }
            }
        }

        impl GetByOrgByRepoResponseUpstream {
            pub fn autosync<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::GetByOrgByRepoResponseUpstreamAutosync>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.autosync = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for autosync: {e}"));
                self
            }
            pub fn last_sync_attempt<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_attempt = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_attempt: {e}")
                });
                self
            }
            pub fn last_sync_error<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_error = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_error: {e}")
                });
                self
            }
            pub fn last_sync_success<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_success = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_success: {e}")
                });
                self
            }
            pub fn uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for uri: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoResponseUpstream>
            for super::GetByOrgByRepoResponseUpstream
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoResponseUpstream,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    autosync: value.autosync?,
                    last_sync_attempt: value.last_sync_attempt?,
                    last_sync_error: value.last_sync_error?,
                    last_sync_success: value.last_sync_success?,
                    uri: value.uri?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoResponseUpstream>
            for GetByOrgByRepoResponseUpstream
        {
            fn from(value: super::GetByOrgByRepoResponseUpstream) -> Self {
                Self {
                    autosync: Ok(value.autosync),
                    last_sync_attempt: Ok(value.last_sync_attempt),
                    last_sync_error: Ok(value.last_sync_error),
                    last_sync_success: Ok(value.last_sync_success),
                    uri: Ok(value.uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoResponseUpstreamAutosync {
            period: ::std::result::Result<f64, ::std::string::String>,
            resolution_strategy: ::std::result::Result<
                super::GetByOrgByRepoResponseUpstreamAutosyncResolutionStrategy,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                super::GetByOrgByRepoResponseUpstreamAutosyncType,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoResponseUpstreamAutosync {
            fn default() -> Self {
                Self {
                    period: Err("no value supplied for period".to_string()),
                    resolution_strategy: Err(
                        "no value supplied for resolution_strategy".to_string()
                    ),
                    type_: Err("no value supplied for type_".to_string()),
                }
            }
        }

        impl GetByOrgByRepoResponseUpstreamAutosync {
            pub fn period<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.period = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for period: {e}"));
                self
            }
            pub fn resolution_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::GetByOrgByRepoResponseUpstreamAutosyncResolutionStrategy,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.resolution_strategy = value.try_into().map_err(|e| {
                    format!("error converting supplied value for resolution_strategy: {e}")
                });
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoResponseUpstreamAutosyncType>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoResponseUpstreamAutosync>
            for super::GetByOrgByRepoResponseUpstreamAutosync
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoResponseUpstreamAutosync,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    period: value.period?,
                    resolution_strategy: value.resolution_strategy?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoResponseUpstreamAutosync>
            for GetByOrgByRepoResponseUpstreamAutosync
        {
            fn from(value: super::GetByOrgByRepoResponseUpstreamAutosync) -> Self {
                Self {
                    period: Ok(value.period),
                    resolution_strategy: Ok(value.resolution_strategy),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoSyncResponse {
            last_sync_attempt: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            last_sync_success: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            status: ::std::result::Result<
                super::GetByOrgByRepoSyncResponseStatus,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoSyncResponse {
            fn default() -> Self {
                Self {
                    last_sync_attempt: Err("no value supplied for last_sync_attempt".to_string()),
                    last_sync_success: Err("no value supplied for last_sync_success".to_string()),
                    status: Err("no value supplied for status".to_string()),
                }
            }
        }

        impl GetByOrgByRepoSyncResponse {
            pub fn last_sync_attempt<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_attempt = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_attempt: {e}")
                });
                self
            }
            pub fn last_sync_success<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_success = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_success: {e}")
                });
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GetByOrgByRepoSyncResponseStatus>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoSyncResponse> for super::GetByOrgByRepoSyncResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoSyncResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    last_sync_attempt: value.last_sync_attempt?,
                    last_sync_success: value.last_sync_success?,
                    status: value.status?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoSyncResponse> for GetByOrgByRepoSyncResponse {
            fn from(value: super::GetByOrgByRepoSyncResponse) -> Self {
                Self {
                    last_sync_attempt: Ok(value.last_sync_attempt),
                    last_sync_success: Ok(value.last_sync_success),
                    status: Ok(value.status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoSyncResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoSyncResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgByRepoSyncResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoSyncResponseError>
            for super::GetByOrgByRepoSyncResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoSyncResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoSyncResponseError>
            for GetByOrgByRepoSyncResponseError
        {
            fn from(value: super::GetByOrgByRepoSyncResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoWebhooksResponse {
            webhooks: ::std::result::Result<
                ::std::vec::Vec<super::GetByOrgByRepoWebhooksResponseWebhooksItem>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgByRepoWebhooksResponse {
            fn default() -> Self {
                Self {
                    webhooks: Err("no value supplied for webhooks".to_string()),
                }
            }
        }

        impl GetByOrgByRepoWebhooksResponse {
            pub fn webhooks<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::GetByOrgByRepoWebhooksResponseWebhooksItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.webhooks = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for webhooks: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoWebhooksResponse>
            for super::GetByOrgByRepoWebhooksResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoWebhooksResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    webhooks: value.webhooks?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoWebhooksResponse>
            for GetByOrgByRepoWebhooksResponse
        {
            fn from(value: super::GetByOrgByRepoWebhooksResponse) -> Self {
                Self {
                    webhooks: Ok(value.webhooks),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoWebhooksResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoWebhooksResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgByRepoWebhooksResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoWebhooksResponseError>
            for super::GetByOrgByRepoWebhooksResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoWebhooksResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoWebhooksResponseError>
            for GetByOrgByRepoWebhooksResponseError
        {
            fn from(value: super::GetByOrgByRepoWebhooksResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgByRepoWebhooksResponseWebhooksItem {
            branches: ::std::result::Result<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            events: ::std::result::Result<
                ::std::vec::Vec<super::GetByOrgByRepoWebhooksResponseWebhooksItemEventsItem>,
                ::std::string::String,
            >,
            globs: ::std::result::Result<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::string::String, ::std::string::String>,
            updated_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            url: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgByRepoWebhooksResponseWebhooksItem {
            fn default() -> Self {
                Self {
                    branches: Err("no value supplied for branches".to_string()),
                    created_at: Err("no value supplied for created_at".to_string()),
                    events: Err("no value supplied for events".to_string()),
                    globs: Err("no value supplied for globs".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                    url: Err("no value supplied for url".to_string()),
                }
            }
        }

        impl GetByOrgByRepoWebhooksResponseWebhooksItem {
            pub fn branches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.branches = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for branches: {e}"));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn events<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::GetByOrgByRepoWebhooksResponseWebhooksItemEventsItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.events = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for events: {e}"));
                self
            }
            pub fn globs<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.globs = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for globs: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
                self
            }
            pub fn url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for url: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgByRepoWebhooksResponseWebhooksItem>
            for super::GetByOrgByRepoWebhooksResponseWebhooksItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgByRepoWebhooksResponseWebhooksItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    branches: value.branches?,
                    created_at: value.created_at?,
                    events: value.events?,
                    globs: value.globs?,
                    id: value.id?,
                    updated_at: value.updated_at?,
                    url: value.url?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgByRepoWebhooksResponseWebhooksItem>
            for GetByOrgByRepoWebhooksResponseWebhooksItem
        {
            fn from(value: super::GetByOrgByRepoWebhooksResponseWebhooksItem) -> Self {
                Self {
                    branches: Ok(value.branches),
                    created_at: Ok(value.created_at),
                    events: Ok(value.events),
                    globs: Ok(value.globs),
                    id: Ok(value.id),
                    updated_at: Ok(value.updated_at),
                    url: Ok(value.url),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgReposResponse {
            has_more: ::std::result::Result<bool, ::std::string::String>,
            next_cursor: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            repos: ::std::result::Result<
                ::std::vec::Vec<super::GetByOrgReposResponseReposItem>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgReposResponse {
            fn default() -> Self {
                Self {
                    has_more: Err("no value supplied for has_more".to_string()),
                    next_cursor: Err("no value supplied for next_cursor".to_string()),
                    repos: Err("no value supplied for repos".to_string()),
                }
            }
        }

        impl GetByOrgReposResponse {
            pub fn has_more<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.has_more = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for has_more: {e}"));
                self
            }
            pub fn next_cursor<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.next_cursor = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
                self
            }
            pub fn repos<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::GetByOrgReposResponseReposItem>>,
                T::Error: ::std::fmt::Display,
            {
                self.repos = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for repos: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgReposResponse> for super::GetByOrgReposResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgReposResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    has_more: value.has_more?,
                    next_cursor: value.next_cursor?,
                    repos: value.repos?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgReposResponse> for GetByOrgReposResponse {
            fn from(value: super::GetByOrgReposResponse) -> Self {
                Self {
                    has_more: Ok(value.has_more),
                    next_cursor: Ok(value.next_cursor),
                    repos: Ok(value.repos),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgReposResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgReposResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgReposResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgReposResponseError> for super::GetByOrgReposResponseError {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgReposResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgReposResponseError> for GetByOrgReposResponseError {
            fn from(value: super::GetByOrgReposResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgReposResponseReposItem {
            created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            default_branch: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<::std::string::String, ::std::string::String>,
            last_push_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            org: ::std::result::Result<::std::string::String, ::std::string::String>,
            size_bytes: ::std::result::Result<f64, ::std::string::String>,
            upstream: ::std::result::Result<
                ::std::option::Option<super::GetByOrgReposResponseReposItemUpstream>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgReposResponseReposItem {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    default_branch: Err("no value supplied for default_branch".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    last_push_at: Err("no value supplied for last_push_at".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    org: Err("no value supplied for org".to_string()),
                    size_bytes: Err("no value supplied for size_bytes".to_string()),
                    upstream: Err("no value supplied for upstream".to_string()),
                }
            }
        }

        impl GetByOrgReposResponseReposItem {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn default_branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.default_branch = value.try_into().map_err(|e| {
                    format!("error converting supplied value for default_branch: {e}")
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn last_push_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_push_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_push_at: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn org<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.org = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for org: {e}"));
                self
            }
            pub fn size_bytes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.size_bytes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size_bytes: {e}"));
                self
            }
            pub fn upstream<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::GetByOrgReposResponseReposItemUpstream>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.upstream = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for upstream: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgReposResponseReposItem>
            for super::GetByOrgReposResponseReposItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgReposResponseReposItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    default_branch: value.default_branch?,
                    id: value.id?,
                    last_push_at: value.last_push_at?,
                    name: value.name?,
                    org: value.org?,
                    size_bytes: value.size_bytes?,
                    upstream: value.upstream?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgReposResponseReposItem>
            for GetByOrgReposResponseReposItem
        {
            fn from(value: super::GetByOrgReposResponseReposItem) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    default_branch: Ok(value.default_branch),
                    id: Ok(value.id),
                    last_push_at: Ok(value.last_push_at),
                    name: Ok(value.name),
                    org: Ok(value.org),
                    size_bytes: Ok(value.size_bytes),
                    upstream: Ok(value.upstream),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgReposResponseReposItemUpstream {
            autosync: ::std::result::Result<
                ::std::option::Option<super::GetByOrgReposResponseReposItemUpstreamAutosync>,
                ::std::string::String,
            >,
            last_sync_attempt: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            last_sync_error: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            last_sync_success: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgReposResponseReposItemUpstream {
            fn default() -> Self {
                Self {
                    autosync: Err("no value supplied for autosync".to_string()),
                    last_sync_attempt: Err("no value supplied for last_sync_attempt".to_string()),
                    last_sync_error: Err("no value supplied for last_sync_error".to_string()),
                    last_sync_success: Err("no value supplied for last_sync_success".to_string()),
                    uri: Err("no value supplied for uri".to_string()),
                }
            }
        }

        impl GetByOrgReposResponseReposItemUpstream {
            pub fn autosync<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::GetByOrgReposResponseReposItemUpstreamAutosync>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.autosync = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for autosync: {e}"));
                self
            }
            pub fn last_sync_attempt<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_attempt = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_attempt: {e}")
                });
                self
            }
            pub fn last_sync_error<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_error = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_error: {e}")
                });
                self
            }
            pub fn last_sync_success<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_success = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_success: {e}")
                });
                self
            }
            pub fn uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for uri: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgReposResponseReposItemUpstream>
            for super::GetByOrgReposResponseReposItemUpstream
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgReposResponseReposItemUpstream,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    autosync: value.autosync?,
                    last_sync_attempt: value.last_sync_attempt?,
                    last_sync_error: value.last_sync_error?,
                    last_sync_success: value.last_sync_success?,
                    uri: value.uri?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgReposResponseReposItemUpstream>
            for GetByOrgReposResponseReposItemUpstream
        {
            fn from(value: super::GetByOrgReposResponseReposItemUpstream) -> Self {
                Self {
                    autosync: Ok(value.autosync),
                    last_sync_attempt: Ok(value.last_sync_attempt),
                    last_sync_error: Ok(value.last_sync_error),
                    last_sync_success: Ok(value.last_sync_success),
                    uri: Ok(value.uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgReposResponseReposItemUpstreamAutosync {
            period: ::std::result::Result<f64, ::std::string::String>,
            resolution_strategy: ::std::result::Result<
                super::GetByOrgReposResponseReposItemUpstreamAutosyncResolutionStrategy,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                super::GetByOrgReposResponseReposItemUpstreamAutosyncType,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetByOrgReposResponseReposItemUpstreamAutosync {
            fn default() -> Self {
                Self {
                    period: Err("no value supplied for period".to_string()),
                    resolution_strategy: Err(
                        "no value supplied for resolution_strategy".to_string()
                    ),
                    type_: Err("no value supplied for type_".to_string()),
                }
            }
        }

        impl GetByOrgReposResponseReposItemUpstreamAutosync {
            pub fn period<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.period = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for period: {e}"));
                self
            }
            pub fn resolution_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::GetByOrgReposResponseReposItemUpstreamAutosyncResolutionStrategy,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.resolution_strategy = value.try_into().map_err(|e| {
                    format!("error converting supplied value for resolution_strategy: {e}")
                });
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::GetByOrgReposResponseReposItemUpstreamAutosyncType,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgReposResponseReposItemUpstreamAutosync>
            for super::GetByOrgReposResponseReposItemUpstreamAutosync
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgReposResponseReposItemUpstreamAutosync,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    period: value.period?,
                    resolution_strategy: value.resolution_strategy?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgReposResponseReposItemUpstreamAutosync>
            for GetByOrgReposResponseReposItemUpstreamAutosync
        {
            fn from(value: super::GetByOrgReposResponseReposItemUpstreamAutosync) -> Self {
                Self {
                    period: Ok(value.period),
                    resolution_strategy: Ok(value.resolution_strategy),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgResponse {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            last_commit_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            num_repos: ::std::result::Result<i64, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    last_commit_at: Err("no value supplied for last_commit_at".to_string()),
                    num_repos: Err("no value supplied for num_repos".to_string()),
                }
            }
        }

        impl GetByOrgResponse {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn last_commit_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_commit_at = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_commit_at: {e}")
                });
                self
            }
            pub fn num_repos<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i64>,
                T::Error: ::std::fmt::Display,
            {
                self.num_repos = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for num_repos: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgResponse> for super::GetByOrgResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    last_commit_at: value.last_commit_at?,
                    num_repos: value.num_repos?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgResponse> for GetByOrgResponse {
            fn from(value: super::GetByOrgResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    last_commit_at: Ok(value.last_commit_at),
                    num_repos: Ok(value.num_repos),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetByOrgResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GetByOrgResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl GetByOrgResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetByOrgResponseError> for super::GetByOrgResponseError {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetByOrgResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::GetByOrgResponseError> for GetByOrgResponseError {
            fn from(value: super::GetByOrgResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PatchByOrgByRepoBody {
            name: ::std::result::Result<
                ::std::option::Option<super::PatchByOrgByRepoBodyName>,
                ::std::string::String,
            >,
            upstream: ::std::result::Result<
                ::std::option::Option<super::PatchByOrgByRepoBodyUpstream>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PatchByOrgByRepoBody {
            fn default() -> Self {
                Self {
                    name: Ok(Default::default()),
                    upstream: Ok(Default::default()),
                }
            }
        }

        impl PatchByOrgByRepoBody {
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::PatchByOrgByRepoBodyName>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn upstream<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PatchByOrgByRepoBodyUpstream>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.upstream = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for upstream: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PatchByOrgByRepoBody> for super::PatchByOrgByRepoBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PatchByOrgByRepoBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    name: value.name?,
                    upstream: value.upstream?,
                })
            }
        }

        impl ::std::convert::From<super::PatchByOrgByRepoBody> for PatchByOrgByRepoBody {
            fn from(value: super::PatchByOrgByRepoBody) -> Self {
                Self {
                    name: Ok(value.name),
                    upstream: Ok(value.upstream),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PatchByOrgByRepoBodyUpstream {
            autosync: ::std::result::Result<
                ::std::option::Option<super::PatchByOrgByRepoBodyUpstreamAutosync>,
                ::std::string::String,
            >,
            uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PatchByOrgByRepoBodyUpstream {
            fn default() -> Self {
                Self {
                    autosync: Ok(Default::default()),
                    uri: Err("no value supplied for uri".to_string()),
                }
            }
        }

        impl PatchByOrgByRepoBodyUpstream {
            pub fn autosync<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PatchByOrgByRepoBodyUpstreamAutosync>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.autosync = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for autosync: {e}"));
                self
            }
            pub fn uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for uri: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PatchByOrgByRepoBodyUpstream> for super::PatchByOrgByRepoBodyUpstream {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PatchByOrgByRepoBodyUpstream,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    autosync: value.autosync?,
                    uri: value.uri?,
                })
            }
        }

        impl ::std::convert::From<super::PatchByOrgByRepoBodyUpstream> for PatchByOrgByRepoBodyUpstream {
            fn from(value: super::PatchByOrgByRepoBodyUpstream) -> Self {
                Self {
                    autosync: Ok(value.autosync),
                    uri: Ok(value.uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PatchByOrgByRepoResponse {
            created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            default_branch: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<::std::string::String, ::std::string::String>,
            last_push_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            org: ::std::result::Result<::std::string::String, ::std::string::String>,
            size_bytes: ::std::result::Result<f64, ::std::string::String>,
            upstream: ::std::result::Result<
                ::std::option::Option<super::PatchByOrgByRepoResponseUpstream>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PatchByOrgByRepoResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    default_branch: Err("no value supplied for default_branch".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    last_push_at: Err("no value supplied for last_push_at".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    org: Err("no value supplied for org".to_string()),
                    size_bytes: Err("no value supplied for size_bytes".to_string()),
                    upstream: Err("no value supplied for upstream".to_string()),
                }
            }
        }

        impl PatchByOrgByRepoResponse {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn default_branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.default_branch = value.try_into().map_err(|e| {
                    format!("error converting supplied value for default_branch: {e}")
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn last_push_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_push_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_push_at: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn org<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.org = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for org: {e}"));
                self
            }
            pub fn size_bytes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.size_bytes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size_bytes: {e}"));
                self
            }
            pub fn upstream<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PatchByOrgByRepoResponseUpstream>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.upstream = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for upstream: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PatchByOrgByRepoResponse> for super::PatchByOrgByRepoResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PatchByOrgByRepoResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    default_branch: value.default_branch?,
                    id: value.id?,
                    last_push_at: value.last_push_at?,
                    name: value.name?,
                    org: value.org?,
                    size_bytes: value.size_bytes?,
                    upstream: value.upstream?,
                })
            }
        }

        impl ::std::convert::From<super::PatchByOrgByRepoResponse> for PatchByOrgByRepoResponse {
            fn from(value: super::PatchByOrgByRepoResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    default_branch: Ok(value.default_branch),
                    id: Ok(value.id),
                    last_push_at: Ok(value.last_push_at),
                    name: Ok(value.name),
                    org: Ok(value.org),
                    size_bytes: Ok(value.size_bytes),
                    upstream: Ok(value.upstream),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PatchByOrgByRepoResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PatchByOrgByRepoResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl PatchByOrgByRepoResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PatchByOrgByRepoResponseError>
            for super::PatchByOrgByRepoResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PatchByOrgByRepoResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::PatchByOrgByRepoResponseError> for PatchByOrgByRepoResponseError {
            fn from(value: super::PatchByOrgByRepoResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PatchByOrgByRepoResponseUpstream {
            autosync: ::std::result::Result<
                ::std::option::Option<super::PatchByOrgByRepoResponseUpstreamAutosync>,
                ::std::string::String,
            >,
            last_sync_attempt: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            last_sync_error: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            last_sync_success: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PatchByOrgByRepoResponseUpstream {
            fn default() -> Self {
                Self {
                    autosync: Err("no value supplied for autosync".to_string()),
                    last_sync_attempt: Err("no value supplied for last_sync_attempt".to_string()),
                    last_sync_error: Err("no value supplied for last_sync_error".to_string()),
                    last_sync_success: Err("no value supplied for last_sync_success".to_string()),
                    uri: Err("no value supplied for uri".to_string()),
                }
            }
        }

        impl PatchByOrgByRepoResponseUpstream {
            pub fn autosync<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PatchByOrgByRepoResponseUpstreamAutosync>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.autosync = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for autosync: {e}"));
                self
            }
            pub fn last_sync_attempt<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_attempt = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_attempt: {e}")
                });
                self
            }
            pub fn last_sync_error<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_error = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_error: {e}")
                });
                self
            }
            pub fn last_sync_success<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_success = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_success: {e}")
                });
                self
            }
            pub fn uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for uri: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PatchByOrgByRepoResponseUpstream>
            for super::PatchByOrgByRepoResponseUpstream
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PatchByOrgByRepoResponseUpstream,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    autosync: value.autosync?,
                    last_sync_attempt: value.last_sync_attempt?,
                    last_sync_error: value.last_sync_error?,
                    last_sync_success: value.last_sync_success?,
                    uri: value.uri?,
                })
            }
        }

        impl ::std::convert::From<super::PatchByOrgByRepoResponseUpstream>
            for PatchByOrgByRepoResponseUpstream
        {
            fn from(value: super::PatchByOrgByRepoResponseUpstream) -> Self {
                Self {
                    autosync: Ok(value.autosync),
                    last_sync_attempt: Ok(value.last_sync_attempt),
                    last_sync_error: Ok(value.last_sync_error),
                    last_sync_success: Ok(value.last_sync_success),
                    uri: Ok(value.uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PatchByOrgByRepoResponseUpstreamAutosync {
            period: ::std::result::Result<f64, ::std::string::String>,
            resolution_strategy: ::std::result::Result<
                super::PatchByOrgByRepoResponseUpstreamAutosyncResolutionStrategy,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                super::PatchByOrgByRepoResponseUpstreamAutosyncType,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PatchByOrgByRepoResponseUpstreamAutosync {
            fn default() -> Self {
                Self {
                    period: Err("no value supplied for period".to_string()),
                    resolution_strategy: Err(
                        "no value supplied for resolution_strategy".to_string()
                    ),
                    type_: Err("no value supplied for type_".to_string()),
                }
            }
        }

        impl PatchByOrgByRepoResponseUpstreamAutosync {
            pub fn period<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.period = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for period: {e}"));
                self
            }
            pub fn resolution_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::PatchByOrgByRepoResponseUpstreamAutosyncResolutionStrategy,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.resolution_strategy = value.try_into().map_err(|e| {
                    format!("error converting supplied value for resolution_strategy: {e}")
                });
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PatchByOrgByRepoResponseUpstreamAutosyncType>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PatchByOrgByRepoResponseUpstreamAutosync>
            for super::PatchByOrgByRepoResponseUpstreamAutosync
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PatchByOrgByRepoResponseUpstreamAutosync,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    period: value.period?,
                    resolution_strategy: value.resolution_strategy?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::PatchByOrgByRepoResponseUpstreamAutosync>
            for PatchByOrgByRepoResponseUpstreamAutosync
        {
            fn from(value: super::PatchByOrgByRepoResponseUpstreamAutosync) -> Self {
                Self {
                    period: Ok(value.period),
                    resolution_strategy: Ok(value.resolution_strategy),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgApiKeysBody {
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            scopes: ::std::result::Result<
                ::std::vec::Vec<super::PostByOrgApiKeysBodyScopesItem>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PostByOrgApiKeysBody {
            fn default() -> Self {
                Self {
                    name: Ok(Default::default()),
                    scopes: Ok(Default::default()),
                }
            }
        }

        impl PostByOrgApiKeysBody {
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn scopes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::PostByOrgApiKeysBodyScopesItem>>,
                T::Error: ::std::fmt::Display,
            {
                self.scopes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for scopes: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgApiKeysBody> for super::PostByOrgApiKeysBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgApiKeysBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    name: value.name?,
                    scopes: value.scopes?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgApiKeysBody> for PostByOrgApiKeysBody {
            fn from(value: super::PostByOrgApiKeysBody) -> Self {
                Self {
                    name: Ok(value.name),
                    scopes: Ok(value.scopes),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgApiKeysResponse {
            created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<::std::string::String, ::std::string::String>,
            key: ::std::result::Result<::std::string::String, ::std::string::String>,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            scopes: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PostByOrgApiKeysResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    key: Err("no value supplied for key".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    scopes: Err("no value supplied for scopes".to_string()),
                }
            }
        }

        impl PostByOrgApiKeysResponse {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn scopes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.scopes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for scopes: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgApiKeysResponse> for super::PostByOrgApiKeysResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgApiKeysResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    key: value.key?,
                    name: value.name?,
                    scopes: value.scopes?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgApiKeysResponse> for PostByOrgApiKeysResponse {
            fn from(value: super::PostByOrgApiKeysResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    key: Ok(value.key),
                    name: Ok(value.name),
                    scopes: Ok(value.scopes),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgApiKeysResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgApiKeysResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl PostByOrgApiKeysResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgApiKeysResponseError>
            for super::PostByOrgApiKeysResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgApiKeysResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgApiKeysResponseError> for PostByOrgApiKeysResponseError {
            fn from(value: super::PostByOrgApiKeysResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoAnalyticsRefreshResponse {
            analytics: ::std::result::Result<
                ::std::option::Option<super::PostByOrgByRepoAnalyticsRefreshResponseAnalytics>,
                ::std::string::String,
            >,
            status: ::std::result::Result<
                super::PostByOrgByRepoAnalyticsRefreshResponseStatus,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PostByOrgByRepoAnalyticsRefreshResponse {
            fn default() -> Self {
                Self {
                    analytics: Err("no value supplied for analytics".to_string()),
                    status: Err("no value supplied for status".to_string()),
                }
            }
        }

        impl PostByOrgByRepoAnalyticsRefreshResponse {
            pub fn analytics<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PostByOrgByRepoAnalyticsRefreshResponseAnalytics>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.analytics = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for analytics: {e}"));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PostByOrgByRepoAnalyticsRefreshResponseStatus>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoAnalyticsRefreshResponse>
            for super::PostByOrgByRepoAnalyticsRefreshResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoAnalyticsRefreshResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    analytics: value.analytics?,
                    status: value.status?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoAnalyticsRefreshResponse>
            for PostByOrgByRepoAnalyticsRefreshResponse
        {
            fn from(value: super::PostByOrgByRepoAnalyticsRefreshResponse) -> Self {
                Self {
                    analytics: Ok(value.analytics),
                    status: Ok(value.status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoAnalyticsRefreshResponseAnalytics {
            contributors: ::std::result::Result<
                ::std::collections::HashMap<
                    ::std::string::String,
                    super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue,
                >,
                ::std::string::String,
            >,
            history: ::std::result::Result<
                ::std::vec::Vec<super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem>,
                ::std::string::String,
            >,
            meta: ::std::result::Result<
                ::std::option::Option<super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta>,
                ::std::string::String,
            >,
            summary: ::std::result::Result<
                super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary,
                ::std::string::String,
            >,
            version: ::std::result::Result<
                super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsVersion,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PostByOrgByRepoAnalyticsRefreshResponseAnalytics {
            fn default() -> Self {
                Self {
                    contributors: Err("no value supplied for contributors".to_string()),
                    history: Err("no value supplied for history".to_string()),
                    meta: Ok(Default::default()),
                    summary: Err("no value supplied for summary".to_string()),
                    version: Err("no value supplied for version".to_string()),
                }
            }
        }

        impl PostByOrgByRepoAnalyticsRefreshResponseAnalytics {
            pub fn contributors<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.contributors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for contributors: {e}"));
                self
            }
            pub fn history<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<
                        super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.history = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for history: {e}"));
                self
            }
            pub fn meta<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<
                        super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.meta = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for meta: {e}"));
                self
            }
            pub fn summary<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.summary = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for summary: {e}"));
                self
            }
            pub fn version<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsVersion,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for version: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoAnalyticsRefreshResponseAnalytics>
            for super::PostByOrgByRepoAnalyticsRefreshResponseAnalytics
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoAnalyticsRefreshResponseAnalytics,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    contributors: value.contributors?,
                    history: value.history?,
                    meta: value.meta?,
                    summary: value.summary?,
                    version: value.version?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoAnalyticsRefreshResponseAnalytics>
            for PostByOrgByRepoAnalyticsRefreshResponseAnalytics
        {
            fn from(value: super::PostByOrgByRepoAnalyticsRefreshResponseAnalytics) -> Self {
                Self {
                    contributors: Ok(value.contributors),
                    history: Ok(value.history),
                    meta: Ok(value.meta),
                    summary: Ok(value.summary),
                    version: Ok(value.version),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue {
            ai_lines: ::std::result::Result<f64, ::std::string::String>,
            commit_count: ::std::result::Result<f64, ::std::string::String>,
            models: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, f64>,
                ::std::string::String,
            >,
            providers: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, f64>,
                ::std::string::String,
            >,
            total_lines: ::std::result::Result<f64, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue {
            fn default() -> Self {
                Self {
                    ai_lines: Err("no value supplied for ai_lines".to_string()),
                    commit_count: Err("no value supplied for commit_count".to_string()),
                    models: Err("no value supplied for models".to_string()),
                    providers: Err("no value supplied for providers".to_string()),
                    total_lines: Err("no value supplied for total_lines".to_string()),
                }
            }
        }

        impl PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue {
            pub fn ai_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.ai_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ai_lines: {e}"));
                self
            }
            pub fn commit_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.commit_count = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commit_count: {e}"));
                self
            }
            pub fn models<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.models = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for models: {e}"));
                self
            }
            pub fn providers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.providers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for providers: {e}"));
                self
            }
            pub fn total_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.total_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_lines: {e}"));
                self
            }
        }

        impl
            ::std::convert::TryFrom<
                PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue,
            > for super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ai_lines: value.ai_lines?,
                    commit_count: value.commit_count?,
                    models: value.models?,
                    providers: value.providers?,
                    total_lines: value.total_lines?,
                })
            }
        }

        impl
            ::std::convert::From<
                super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue,
            > for PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue
        {
            fn from(
                value: super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsContributorsValue,
            ) -> Self {
                Self {
                    ai_lines: Ok(value.ai_lines),
                    commit_count: Ok(value.commit_count),
                    models: Ok(value.models),
                    providers: Ok(value.providers),
                    total_lines: Ok(value.total_lines),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem {
            added: ::std::result::Result<f64, ::std::string::String>,
            ai_lines: ::std::result::Result<f64, ::std::string::String>,
            author: ::std::result::Result<::std::string::String, ::std::string::String>,
            date: ::std::result::Result<::std::string::String, ::std::string::String>,
            message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            models: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, f64>,
                ::std::string::String,
            >,
            providers: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, f64>,
                ::std::string::String,
            >,
            removed: ::std::result::Result<f64, ::std::string::String>,
            sha: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem {
            fn default() -> Self {
                Self {
                    added: Err("no value supplied for added".to_string()),
                    ai_lines: Err("no value supplied for ai_lines".to_string()),
                    author: Err("no value supplied for author".to_string()),
                    date: Err("no value supplied for date".to_string()),
                    message: Ok(Default::default()),
                    models: Ok(Default::default()),
                    providers: Ok(Default::default()),
                    removed: Err("no value supplied for removed".to_string()),
                    sha: Err("no value supplied for sha".to_string()),
                }
            }
        }

        impl PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem {
            pub fn added<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.added = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for added: {e}"));
                self
            }
            pub fn ai_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.ai_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ai_lines: {e}"));
                self
            }
            pub fn author<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.author = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for author: {e}"));
                self
            }
            pub fn date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for date: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
            pub fn models<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.models = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for models: {e}"));
                self
            }
            pub fn providers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.providers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for providers: {e}"));
                self
            }
            pub fn removed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.removed = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for removed: {e}"));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem>
            for super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    added: value.added?,
                    ai_lines: value.ai_lines?,
                    author: value.author?,
                    date: value.date?,
                    message: value.message?,
                    models: value.models?,
                    providers: value.providers?,
                    removed: value.removed?,
                    sha: value.sha?,
                })
            }
        }

        impl
            ::std::convert::From<super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem>
            for PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem
        {
            fn from(
                value: super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsHistoryItem,
            ) -> Self {
                Self {
                    added: Ok(value.added),
                    ai_lines: Ok(value.ai_lines),
                    author: Ok(value.author),
                    date: Ok(value.date),
                    message: Ok(value.message),
                    models: Ok(value.models),
                    providers: Ok(value.providers),
                    removed: Ok(value.removed),
                    sha: Ok(value.sha),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta {
            last_attempt: ::std::result::Result<::std::string::String, ::std::string::String>,
            last_error: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            last_success: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta {
            fn default() -> Self {
                Self {
                    last_attempt: Err("no value supplied for last_attempt".to_string()),
                    last_error: Err("no value supplied for last_error".to_string()),
                    last_success: Err("no value supplied for last_success".to_string()),
                }
            }
        }

        impl PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta {
            pub fn last_attempt<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.last_attempt = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_attempt: {e}"));
                self
            }
            pub fn last_error<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_error = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_error: {e}"));
                self
            }
            pub fn last_success<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_success = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_success: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta>
            for super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    last_attempt: value.last_attempt?,
                    last_error: value.last_error?,
                    last_success: value.last_success?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta>
            for PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta
        {
            fn from(value: super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsMeta) -> Self {
                Self {
                    last_attempt: Ok(value.last_attempt),
                    last_error: Ok(value.last_error),
                    last_success: Ok(value.last_success),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary {
            ai_lines: ::std::result::Result<f64, ::std::string::String>,
            human_lines: ::std::result::Result<f64, ::std::string::String>,
            models: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, f64>,
                ::std::string::String,
            >,
            providers: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, f64>,
                ::std::string::String,
            >,
            total_lines: ::std::result::Result<f64, ::std::string::String>,
            updated: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary {
            fn default() -> Self {
                Self {
                    ai_lines: Err("no value supplied for ai_lines".to_string()),
                    human_lines: Err("no value supplied for human_lines".to_string()),
                    models: Err("no value supplied for models".to_string()),
                    providers: Err("no value supplied for providers".to_string()),
                    total_lines: Err("no value supplied for total_lines".to_string()),
                    updated: Err("no value supplied for updated".to_string()),
                }
            }
        }

        impl PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary {
            pub fn ai_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.ai_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ai_lines: {e}"));
                self
            }
            pub fn human_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.human_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for human_lines: {e}"));
                self
            }
            pub fn models<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.models = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for models: {e}"));
                self
            }
            pub fn providers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.providers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for providers: {e}"));
                self
            }
            pub fn total_lines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.total_lines = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_lines: {e}"));
                self
            }
            pub fn updated<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.updated = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary>
            for super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ai_lines: value.ai_lines?,
                    human_lines: value.human_lines?,
                    models: value.models?,
                    providers: value.providers?,
                    total_lines: value.total_lines?,
                    updated: value.updated?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary>
            for PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary
        {
            fn from(value: super::PostByOrgByRepoAnalyticsRefreshResponseAnalyticsSummary) -> Self {
                Self {
                    ai_lines: Ok(value.ai_lines),
                    human_lines: Ok(value.human_lines),
                    models: Ok(value.models),
                    providers: Ok(value.providers),
                    total_lines: Ok(value.total_lines),
                    updated: Ok(value.updated),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoAnalyticsRefreshResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoAnalyticsRefreshResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl PostByOrgByRepoAnalyticsRefreshResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoAnalyticsRefreshResponseError>
            for super::PostByOrgByRepoAnalyticsRefreshResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoAnalyticsRefreshResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoAnalyticsRefreshResponseError>
            for PostByOrgByRepoAnalyticsRefreshResponseError
        {
            fn from(value: super::PostByOrgByRepoAnalyticsRefreshResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoBranchesBody {
            from: ::std::result::Result<
                super::PostByOrgByRepoBranchesBodyFrom,
                ::std::string::String,
            >,
            name: ::std::result::Result<
                super::PostByOrgByRepoBranchesBodyName,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PostByOrgByRepoBranchesBody {
            fn default() -> Self {
                Self {
                    from: Err("no value supplied for from".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl PostByOrgByRepoBranchesBody {
            pub fn from<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PostByOrgByRepoBranchesBodyFrom>,
                T::Error: ::std::fmt::Display,
            {
                self.from = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for from: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PostByOrgByRepoBranchesBodyName>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoBranchesBody> for super::PostByOrgByRepoBranchesBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoBranchesBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    from: value.from?,
                    name: value.name?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoBranchesBody> for PostByOrgByRepoBranchesBody {
            fn from(value: super::PostByOrgByRepoBranchesBody) -> Self {
                Self {
                    from: Ok(value.from),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoBranchesResponse {
            head_sha: ::std::result::Result<::std::string::String, ::std::string::String>,
            is_default: ::std::result::Result<bool, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoBranchesResponse {
            fn default() -> Self {
                Self {
                    head_sha: Err("no value supplied for head_sha".to_string()),
                    is_default: Err("no value supplied for is_default".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl PostByOrgByRepoBranchesResponse {
            pub fn head_sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.head_sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for head_sha: {e}"));
                self
            }
            pub fn is_default<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.is_default = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_default: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoBranchesResponse>
            for super::PostByOrgByRepoBranchesResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoBranchesResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    head_sha: value.head_sha?,
                    is_default: value.is_default?,
                    name: value.name?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoBranchesResponse>
            for PostByOrgByRepoBranchesResponse
        {
            fn from(value: super::PostByOrgByRepoBranchesResponse) -> Self {
                Self {
                    head_sha: Ok(value.head_sha),
                    is_default: Ok(value.is_default),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoBranchesResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoBranchesResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl PostByOrgByRepoBranchesResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoBranchesResponseError>
            for super::PostByOrgByRepoBranchesResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoBranchesResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoBranchesResponseError>
            for PostByOrgByRepoBranchesResponseError
        {
            fn from(value: super::PostByOrgByRepoBranchesResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoCommitsBody {
            author: ::std::result::Result<
                super::PostByOrgByRepoCommitsBodyAuthor,
                ::std::string::String,
            >,
            base_sha: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            branch: ::std::result::Result<
                super::PostByOrgByRepoCommitsBodyBranch,
                ::std::string::String,
            >,
            files: ::std::result::Result<
                ::std::vec::Vec<super::PostByOrgByRepoCommitsBodyFilesItem>,
                ::std::string::String,
            >,
            message: ::std::result::Result<
                super::PostByOrgByRepoCommitsBodyMessage,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PostByOrgByRepoCommitsBody {
            fn default() -> Self {
                Self {
                    author: Err("no value supplied for author".to_string()),
                    base_sha: Ok(Default::default()),
                    branch: Err("no value supplied for branch".to_string()),
                    files: Err("no value supplied for files".to_string()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl PostByOrgByRepoCommitsBody {
            pub fn author<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PostByOrgByRepoCommitsBodyAuthor>,
                T::Error: ::std::fmt::Display,
            {
                self.author = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for author: {e}"));
                self
            }
            pub fn base_sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.base_sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for base_sha: {e}"));
                self
            }
            pub fn branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PostByOrgByRepoCommitsBodyBranch>,
                T::Error: ::std::fmt::Display,
            {
                self.branch = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for branch: {e}"));
                self
            }
            pub fn files<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::PostByOrgByRepoCommitsBodyFilesItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.files = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for files: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PostByOrgByRepoCommitsBodyMessage>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoCommitsBody> for super::PostByOrgByRepoCommitsBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoCommitsBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    author: value.author?,
                    base_sha: value.base_sha?,
                    branch: value.branch?,
                    files: value.files?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoCommitsBody> for PostByOrgByRepoCommitsBody {
            fn from(value: super::PostByOrgByRepoCommitsBody) -> Self {
                Self {
                    author: Ok(value.author),
                    base_sha: Ok(value.base_sha),
                    branch: Ok(value.branch),
                    files: Ok(value.files),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoCommitsBodyAuthor {
            date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            email: ::std::result::Result<::std::string::String, ::std::string::String>,
            name: ::std::result::Result<
                super::PostByOrgByRepoCommitsBodyAuthorName,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PostByOrgByRepoCommitsBodyAuthor {
            fn default() -> Self {
                Self {
                    date: Ok(Default::default()),
                    email: Err("no value supplied for email".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl PostByOrgByRepoCommitsBodyAuthor {
            pub fn date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for date: {e}"));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PostByOrgByRepoCommitsBodyAuthorName>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoCommitsBodyAuthor>
            for super::PostByOrgByRepoCommitsBodyAuthor
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoCommitsBodyAuthor,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    date: value.date?,
                    email: value.email?,
                    name: value.name?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoCommitsBodyAuthor>
            for PostByOrgByRepoCommitsBodyAuthor
        {
            fn from(value: super::PostByOrgByRepoCommitsBodyAuthor) -> Self {
                Self {
                    date: Ok(value.date),
                    email: Ok(value.email),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoCommitsResponse {
            branch: ::std::result::Result<::std::string::String, ::std::string::String>,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
            sha: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoCommitsResponse {
            fn default() -> Self {
                Self {
                    branch: Err("no value supplied for branch".to_string()),
                    message: Err("no value supplied for message".to_string()),
                    sha: Err("no value supplied for sha".to_string()),
                }
            }
        }

        impl PostByOrgByRepoCommitsResponse {
            pub fn branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.branch = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for branch: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoCommitsResponse>
            for super::PostByOrgByRepoCommitsResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoCommitsResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    branch: value.branch?,
                    message: value.message?,
                    sha: value.sha?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoCommitsResponse>
            for PostByOrgByRepoCommitsResponse
        {
            fn from(value: super::PostByOrgByRepoCommitsResponse) -> Self {
                Self {
                    branch: Ok(value.branch),
                    message: Ok(value.message),
                    sha: Ok(value.sha),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoCommitsResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoCommitsResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl PostByOrgByRepoCommitsResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoCommitsResponseError>
            for super::PostByOrgByRepoCommitsResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoCommitsResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoCommitsResponseError>
            for PostByOrgByRepoCommitsResponseError
        {
            fn from(value: super::PostByOrgByRepoCommitsResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoSyncResponse {
            success: ::std::result::Result<bool, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoSyncResponse {
            fn default() -> Self {
                Self {
                    success: Err("no value supplied for success".to_string()),
                }
            }
        }

        impl PostByOrgByRepoSyncResponse {
            pub fn success<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.success = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for success: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoSyncResponse> for super::PostByOrgByRepoSyncResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoSyncResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    success: value.success?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoSyncResponse> for PostByOrgByRepoSyncResponse {
            fn from(value: super::PostByOrgByRepoSyncResponse) -> Self {
                Self {
                    success: Ok(value.success),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoSyncResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoSyncResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl PostByOrgByRepoSyncResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoSyncResponseError>
            for super::PostByOrgByRepoSyncResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoSyncResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoSyncResponseError>
            for PostByOrgByRepoSyncResponseError
        {
            fn from(value: super::PostByOrgByRepoSyncResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoWebhooksBody {
            branches: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            events: ::std::result::Result<
                ::std::vec::Vec<super::PostByOrgByRepoWebhooksBodyEventsItem>,
                ::std::string::String,
            >,
            globs: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            secret: ::std::result::Result<
                ::std::option::Option<super::PostByOrgByRepoWebhooksBodySecret>,
                ::std::string::String,
            >,
            url: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoWebhooksBody {
            fn default() -> Self {
                Self {
                    branches: Ok(Default::default()),
                    events: Ok(super::defaults::post_by_org_by_repo_webhooks_body_events()),
                    globs: Ok(Default::default()),
                    secret: Ok(Default::default()),
                    url: Err("no value supplied for url".to_string()),
                }
            }
        }

        impl PostByOrgByRepoWebhooksBody {
            pub fn branches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.branches = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for branches: {e}"));
                self
            }
            pub fn events<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::PostByOrgByRepoWebhooksBodyEventsItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.events = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for events: {e}"));
                self
            }
            pub fn globs<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.globs = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for globs: {e}"));
                self
            }
            pub fn secret<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PostByOrgByRepoWebhooksBodySecret>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.secret = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secret: {e}"));
                self
            }
            pub fn url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for url: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoWebhooksBody> for super::PostByOrgByRepoWebhooksBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoWebhooksBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    branches: value.branches?,
                    events: value.events?,
                    globs: value.globs?,
                    secret: value.secret?,
                    url: value.url?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoWebhooksBody> for PostByOrgByRepoWebhooksBody {
            fn from(value: super::PostByOrgByRepoWebhooksBody) -> Self {
                Self {
                    branches: Ok(value.branches),
                    events: Ok(value.events),
                    globs: Ok(value.globs),
                    secret: Ok(value.secret),
                    url: Ok(value.url),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoWebhooksResponse {
            branches: ::std::result::Result<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            events: ::std::result::Result<
                ::std::vec::Vec<super::PostByOrgByRepoWebhooksResponseEventsItem>,
                ::std::string::String,
            >,
            globs: ::std::result::Result<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::string::String, ::std::string::String>,
            secret: ::std::result::Result<::std::string::String, ::std::string::String>,
            updated_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            url: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoWebhooksResponse {
            fn default() -> Self {
                Self {
                    branches: Err("no value supplied for branches".to_string()),
                    created_at: Err("no value supplied for created_at".to_string()),
                    events: Err("no value supplied for events".to_string()),
                    globs: Err("no value supplied for globs".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    secret: Err("no value supplied for secret".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                    url: Err("no value supplied for url".to_string()),
                }
            }
        }

        impl PostByOrgByRepoWebhooksResponse {
            pub fn branches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.branches = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for branches: {e}"));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn events<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::PostByOrgByRepoWebhooksResponseEventsItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.events = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for events: {e}"));
                self
            }
            pub fn globs<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.globs = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for globs: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn secret<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.secret = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secret: {e}"));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
                self
            }
            pub fn url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for url: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoWebhooksResponse>
            for super::PostByOrgByRepoWebhooksResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoWebhooksResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    branches: value.branches?,
                    created_at: value.created_at?,
                    events: value.events?,
                    globs: value.globs?,
                    id: value.id?,
                    secret: value.secret?,
                    updated_at: value.updated_at?,
                    url: value.url?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoWebhooksResponse>
            for PostByOrgByRepoWebhooksResponse
        {
            fn from(value: super::PostByOrgByRepoWebhooksResponse) -> Self {
                Self {
                    branches: Ok(value.branches),
                    created_at: Ok(value.created_at),
                    events: Ok(value.events),
                    globs: Ok(value.globs),
                    id: Ok(value.id),
                    secret: Ok(value.secret),
                    updated_at: Ok(value.updated_at),
                    url: Ok(value.url),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgByRepoWebhooksResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgByRepoWebhooksResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl PostByOrgByRepoWebhooksResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgByRepoWebhooksResponseError>
            for super::PostByOrgByRepoWebhooksResponseError
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgByRepoWebhooksResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgByRepoWebhooksResponseError>
            for PostByOrgByRepoWebhooksResponseError
        {
            fn from(value: super::PostByOrgByRepoWebhooksResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgReposBody {
            default_branch: ::std::result::Result<::std::string::String, ::std::string::String>,
            name: ::std::result::Result<super::PostByOrgReposBodyName, ::std::string::String>,
            upstream: ::std::result::Result<
                ::std::option::Option<super::PostByOrgReposBodyUpstream>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PostByOrgReposBody {
            fn default() -> Self {
                Self {
                    default_branch: Ok(super::defaults::post_by_org_repos_body_default_branch()),
                    name: Err("no value supplied for name".to_string()),
                    upstream: Ok(Default::default()),
                }
            }
        }

        impl PostByOrgReposBody {
            pub fn default_branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.default_branch = value.try_into().map_err(|e| {
                    format!("error converting supplied value for default_branch: {e}")
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PostByOrgReposBodyName>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn upstream<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PostByOrgReposBodyUpstream>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.upstream = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for upstream: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgReposBody> for super::PostByOrgReposBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgReposBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    default_branch: value.default_branch?,
                    name: value.name?,
                    upstream: value.upstream?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgReposBody> for PostByOrgReposBody {
            fn from(value: super::PostByOrgReposBody) -> Self {
                Self {
                    default_branch: Ok(value.default_branch),
                    name: Ok(value.name),
                    upstream: Ok(value.upstream),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgReposBodyUpstream {
            autosync: ::std::result::Result<
                ::std::option::Option<super::PostByOrgReposBodyUpstreamAutosync>,
                ::std::string::String,
            >,
            uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgReposBodyUpstream {
            fn default() -> Self {
                Self {
                    autosync: Ok(Default::default()),
                    uri: Err("no value supplied for uri".to_string()),
                }
            }
        }

        impl PostByOrgReposBodyUpstream {
            pub fn autosync<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PostByOrgReposBodyUpstreamAutosync>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.autosync = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for autosync: {e}"));
                self
            }
            pub fn uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for uri: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgReposBodyUpstream> for super::PostByOrgReposBodyUpstream {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgReposBodyUpstream,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    autosync: value.autosync?,
                    uri: value.uri?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgReposBodyUpstream> for PostByOrgReposBodyUpstream {
            fn from(value: super::PostByOrgReposBodyUpstream) -> Self {
                Self {
                    autosync: Ok(value.autosync),
                    uri: Ok(value.uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgReposResponse {
            created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
            default_branch: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<::std::string::String, ::std::string::String>,
            last_push_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            org: ::std::result::Result<::std::string::String, ::std::string::String>,
            size_bytes: ::std::result::Result<f64, ::std::string::String>,
            upstream: ::std::result::Result<
                ::std::option::Option<super::PostByOrgReposResponseUpstream>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PostByOrgReposResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    default_branch: Err("no value supplied for default_branch".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    last_push_at: Err("no value supplied for last_push_at".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    org: Err("no value supplied for org".to_string()),
                    size_bytes: Err("no value supplied for size_bytes".to_string()),
                    upstream: Err("no value supplied for upstream".to_string()),
                }
            }
        }

        impl PostByOrgReposResponse {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn default_branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.default_branch = value.try_into().map_err(|e| {
                    format!("error converting supplied value for default_branch: {e}")
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn last_push_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_push_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_push_at: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn org<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.org = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for org: {e}"));
                self
            }
            pub fn size_bytes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.size_bytes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size_bytes: {e}"));
                self
            }
            pub fn upstream<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PostByOrgReposResponseUpstream>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.upstream = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for upstream: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgReposResponse> for super::PostByOrgReposResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgReposResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    default_branch: value.default_branch?,
                    id: value.id?,
                    last_push_at: value.last_push_at?,
                    name: value.name?,
                    org: value.org?,
                    size_bytes: value.size_bytes?,
                    upstream: value.upstream?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgReposResponse> for PostByOrgReposResponse {
            fn from(value: super::PostByOrgReposResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    default_branch: Ok(value.default_branch),
                    id: Ok(value.id),
                    last_push_at: Ok(value.last_push_at),
                    name: Ok(value.name),
                    org: Ok(value.org),
                    size_bytes: Ok(value.size_bytes),
                    upstream: Ok(value.upstream),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgReposResponseError {
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            details: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgReposResponseError {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    details: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl PostByOrgReposResponseError {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgReposResponseError> for super::PostByOrgReposResponseError {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgReposResponseError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    details: value.details?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgReposResponseError> for PostByOrgReposResponseError {
            fn from(value: super::PostByOrgReposResponseError) -> Self {
                Self {
                    code: Ok(value.code),
                    details: Ok(value.details),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgReposResponseUpstream {
            autosync: ::std::result::Result<
                ::std::option::Option<super::PostByOrgReposResponseUpstreamAutosync>,
                ::std::string::String,
            >,
            last_sync_attempt: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            last_sync_error: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            last_sync_success: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for PostByOrgReposResponseUpstream {
            fn default() -> Self {
                Self {
                    autosync: Err("no value supplied for autosync".to_string()),
                    last_sync_attempt: Err("no value supplied for last_sync_attempt".to_string()),
                    last_sync_error: Err("no value supplied for last_sync_error".to_string()),
                    last_sync_success: Err("no value supplied for last_sync_success".to_string()),
                    uri: Err("no value supplied for uri".to_string()),
                }
            }
        }

        impl PostByOrgReposResponseUpstream {
            pub fn autosync<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PostByOrgReposResponseUpstreamAutosync>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.autosync = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for autosync: {e}"));
                self
            }
            pub fn last_sync_attempt<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_attempt = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_attempt: {e}")
                });
                self
            }
            pub fn last_sync_error<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_error = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_error: {e}")
                });
                self
            }
            pub fn last_sync_success<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_sync_success = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_sync_success: {e}")
                });
                self
            }
            pub fn uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for uri: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgReposResponseUpstream>
            for super::PostByOrgReposResponseUpstream
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgReposResponseUpstream,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    autosync: value.autosync?,
                    last_sync_attempt: value.last_sync_attempt?,
                    last_sync_error: value.last_sync_error?,
                    last_sync_success: value.last_sync_success?,
                    uri: value.uri?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgReposResponseUpstream>
            for PostByOrgReposResponseUpstream
        {
            fn from(value: super::PostByOrgReposResponseUpstream) -> Self {
                Self {
                    autosync: Ok(value.autosync),
                    last_sync_attempt: Ok(value.last_sync_attempt),
                    last_sync_error: Ok(value.last_sync_error),
                    last_sync_success: Ok(value.last_sync_success),
                    uri: Ok(value.uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PostByOrgReposResponseUpstreamAutosync {
            period: ::std::result::Result<f64, ::std::string::String>,
            resolution_strategy: ::std::result::Result<
                super::PostByOrgReposResponseUpstreamAutosyncResolutionStrategy,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                super::PostByOrgReposResponseUpstreamAutosyncType,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PostByOrgReposResponseUpstreamAutosync {
            fn default() -> Self {
                Self {
                    period: Err("no value supplied for period".to_string()),
                    resolution_strategy: Err(
                        "no value supplied for resolution_strategy".to_string()
                    ),
                    type_: Err("no value supplied for type_".to_string()),
                }
            }
        }

        impl PostByOrgReposResponseUpstreamAutosync {
            pub fn period<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.period = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for period: {e}"));
                self
            }
            pub fn resolution_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::PostByOrgReposResponseUpstreamAutosyncResolutionStrategy,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.resolution_strategy = value.try_into().map_err(|e| {
                    format!("error converting supplied value for resolution_strategy: {e}")
                });
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PostByOrgReposResponseUpstreamAutosyncType>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<PostByOrgReposResponseUpstreamAutosync>
            for super::PostByOrgReposResponseUpstreamAutosync
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostByOrgReposResponseUpstreamAutosync,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    period: value.period?,
                    resolution_strategy: value.resolution_strategy?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::PostByOrgReposResponseUpstreamAutosync>
            for PostByOrgReposResponseUpstreamAutosync
        {
            fn from(value: super::PostByOrgReposResponseUpstreamAutosync) -> Self {
                Self {
                    period: Ok(value.period),
                    resolution_strategy: Ok(value.resolution_strategy),
                    type_: Ok(value.type_),
                }
            }
        }
    }

    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn patch_by_org_by_repo_body_upstream_autosync_poll_resolution_strategy(
        ) -> super::PatchByOrgByRepoBodyUpstreamAutosyncResolutionStrategy {
            super::PatchByOrgByRepoBodyUpstreamAutosyncResolutionStrategy::None
        }

        pub(super) fn post_by_org_by_repo_commits_body_files_item_variant0_encoding(
        ) -> super::PostByOrgByRepoCommitsBodyFilesItemVariant0Encoding {
            super::PostByOrgByRepoCommitsBodyFilesItemVariant0Encoding::Utf8
        }

        pub(super) fn post_by_org_by_repo_webhooks_body_events(
        ) -> ::std::vec::Vec<super::PostByOrgByRepoWebhooksBodyEventsItem> {
            vec![super::PostByOrgByRepoWebhooksBodyEventsItem::Push]
        }

        pub(super) fn post_by_org_repos_body_default_branch() -> ::std::string::String {
            "main".to_string()
        }

        pub(super) fn post_by_org_repos_body_upstream_autosync_poll_resolution_strategy(
        ) -> super::PostByOrgReposBodyUpstreamAutosyncResolutionStrategy {
            super::PostByOrgReposBodyUpstreamAutosyncResolutionStrategy::None
        }
    }
}

#[derive(Clone, Debug)]
///Client for Depot API
///
///Depot HTTP API v1
///
///Version: 1.0.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "1.0.0"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
impl Client {
    ///List API keys
    ///
    ///List all API keys for the organization (key values are not returned)
    ///
    ///Sends a `GET` request to `/{org}/api-keys`
    ///
    ///```ignore
    /// let response = client.get_by_org_api_keys()
    ///    .org(org)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org_api_keys(&self) -> builder::GetByOrgApiKeys<'_> {
        builder::GetByOrgApiKeys::new(self)
    }

    ///Create API key
    ///
    ///Create a new API key for programmatic access
    ///
    ///Sends a `POST` request to `/{org}/api-keys`
    ///
    ///```ignore
    /// let response = client.post_by_org_api_keys()
    ///    .org(org)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_by_org_api_keys(&self) -> builder::PostByOrgApiKeys<'_> {
        builder::PostByOrgApiKeys::new(self)
    }

    ///Revoke API key
    ///
    ///Revoke an API key by its ID
    ///
    ///Sends a `DELETE` request to `/{org}/api-keys/{id}`
    ///
    ///```ignore
    /// let response = client.delete_by_org_api_keys_by_id()
    ///    .org(org)
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_by_org_api_keys_by_id(&self) -> builder::DeleteByOrgApiKeysById<'_> {
        builder::DeleteByOrgApiKeysById::new(self)
    }

    ///List repositories
    ///
    ///List all repositories in the organization
    ///
    ///Sends a `GET` request to `/{org}/repos`
    ///
    ///```ignore
    /// let response = client.get_by_org_repos()
    ///    .org(org)
    ///    .cursor(cursor)
    ///    .limit(limit)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org_repos(&self) -> builder::GetByOrgRepos<'_> {
        builder::GetByOrgRepos::new(self)
    }

    ///Create repository
    ///
    ///Create a new repository in the organization
    ///
    ///Sends a `POST` request to `/{org}/repos`
    ///
    ///```ignore
    /// let response = client.post_by_org_repos()
    ///    .org(org)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_by_org_repos(&self) -> builder::PostByOrgRepos<'_> {
        builder::PostByOrgRepos::new(self)
    }

    ///Get repository
    ///
    ///Get metadata for a specific repository
    ///
    ///Sends a `GET` request to `/{org}/{repo}`
    ///
    ///```ignore
    /// let response = client.get_by_org_by_repo()
    ///    .org(org)
    ///    .repo(repo)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org_by_repo(&self) -> builder::GetByOrgByRepo<'_> {
        builder::GetByOrgByRepo::new(self)
    }

    ///Delete repository
    ///
    ///Permanently delete a repository and all its data
    ///
    ///Sends a `DELETE` request to `/{org}/{repo}`
    ///
    ///```ignore
    /// let response = client.delete_by_org_by_repo()
    ///    .org(org)
    ///    .repo(repo)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_by_org_by_repo(&self) -> builder::DeleteByOrgByRepo<'_> {
        builder::DeleteByOrgByRepo::new(self)
    }

    ///Update repository
    ///
    ///Update repository name or upstream configuration
    ///
    ///Sends a `PATCH` request to `/{org}/{repo}`
    ///
    ///```ignore
    /// let response = client.patch_by_org_by_repo()
    ///    .org(org)
    ///    .repo(repo)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn patch_by_org_by_repo(&self) -> builder::PatchByOrgByRepo<'_> {
        builder::PatchByOrgByRepo::new(self)
    }

    ///Get sync status
    ///
    ///Get the sync status for a repository with upstream configured
    ///
    ///Sends a `GET` request to `/{org}/{repo}/sync`
    ///
    ///```ignore
    /// let response = client.get_by_org_by_repo_sync()
    ///    .org(org)
    ///    .repo(repo)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org_by_repo_sync(&self) -> builder::GetByOrgByRepoSync<'_> {
        builder::GetByOrgByRepoSync::new(self)
    }

    ///Sync repository
    ///
    ///Trigger a sync from the upstream repository. Waits for sync to complete.
    ///
    ///Sends a `POST` request to `/{org}/{repo}/sync`
    ///
    ///```ignore
    /// let response = client.post_by_org_by_repo_sync()
    ///    .org(org)
    ///    .repo(repo)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_by_org_by_repo_sync(&self) -> builder::PostByOrgByRepoSync<'_> {
        builder::PostByOrgByRepoSync::new(self)
    }

    ///Get content
    ///
    ///Get file content or directory listing at a path. Use Accept:
    /// application/json for the JSON union response, or Accept:
    /// application/octet-stream for raw file bytes. Directory + octet-stream
    /// requests return 406 Not Acceptable.
    ///
    ///Sends a `GET` request to `/{org}/{repo}/content`
    ///
    ///```ignore
    /// let response = client.get_by_org_by_repo_content()
    ///    .org(org)
    ///    .repo(repo)
    ///    .depth(depth)
    ///    .path(path)
    ///    .ref_(ref_)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org_by_repo_content(&self) -> builder::GetByOrgByRepoContent<'_> {
        builder::GetByOrgByRepoContent::new(self)
    }

    ///List branches
    ///
    ///List all branches in a repository
    ///
    ///Sends a `GET` request to `/{org}/{repo}/branches`
    ///
    ///```ignore
    /// let response = client.get_by_org_by_repo_branches()
    ///    .org(org)
    ///    .repo(repo)
    ///    .cursor(cursor)
    ///    .limit(limit)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org_by_repo_branches(&self) -> builder::GetByOrgByRepoBranches<'_> {
        builder::GetByOrgByRepoBranches::new(self)
    }

    ///Create branch
    ///
    ///Create a new branch from an existing ref
    ///
    ///Sends a `POST` request to `/{org}/{repo}/branches`
    ///
    ///```ignore
    /// let response = client.post_by_org_by_repo_branches()
    ///    .org(org)
    ///    .repo(repo)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_by_org_by_repo_branches(&self) -> builder::PostByOrgByRepoBranches<'_> {
        builder::PostByOrgByRepoBranches::new(self)
    }

    ///Delete branch
    ///
    ///Delete a branch from a repository
    ///
    ///Sends a `DELETE` request to `/{org}/{repo}/branches/{branch}`
    ///
    ///```ignore
    /// let response = client.delete_by_org_by_repo_branches_by_branch()
    ///    .org(org)
    ///    .repo(repo)
    ///    .branch(branch)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_by_org_by_repo_branches_by_branch(
        &self,
    ) -> builder::DeleteByOrgByRepoBranchesByBranch<'_> {
        builder::DeleteByOrgByRepoBranchesByBranch::new(self)
    }

    ///List commits
    ///
    ///List commits for a repository from a specific ref
    ///
    ///Sends a `GET` request to `/{org}/{repo}/commits`
    ///
    ///```ignore
    /// let response = client.get_by_org_by_repo_commits()
    ///    .org(org)
    ///    .repo(repo)
    ///    .cursor(cursor)
    ///    .limit(limit)
    ///    .ref_(ref_)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org_by_repo_commits(&self) -> builder::GetByOrgByRepoCommits<'_> {
        builder::GetByOrgByRepoCommits::new(self)
    }

    ///Create commit
    ///
    ///Programmatically create a commit with file edits
    ///
    ///Sends a `POST` request to `/{org}/{repo}/commits`
    ///
    ///```ignore
    /// let response = client.post_by_org_by_repo_commits()
    ///    .org(org)
    ///    .repo(repo)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_by_org_by_repo_commits(&self) -> builder::PostByOrgByRepoCommits<'_> {
        builder::PostByOrgByRepoCommits::new(self)
    }

    ///Get commit
    ///
    ///Retrieve a specific commit by its SHA
    ///
    ///Sends a `GET` request to `/{org}/{repo}/commits/{sha}`
    ///
    ///```ignore
    /// let response = client.get_by_org_by_repo_commits_by_sha()
    ///    .org(org)
    ///    .repo(repo)
    ///    .sha(sha)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org_by_repo_commits_by_sha(&self) -> builder::GetByOrgByRepoCommitsBySha<'_> {
        builder::GetByOrgByRepoCommitsBySha::new(self)
    }

    ///Get diff
    ///
    ///Retrieve the diff between two refs
    ///
    ///Sends a `GET` request to `/{org}/{repo}/diff`
    ///
    ///```ignore
    /// let response = client.get_by_org_by_repo_diff()
    ///    .org(org)
    ///    .repo(repo)
    ///    .base(base)
    ///    .head(head)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org_by_repo_diff(&self) -> builder::GetByOrgByRepoDiff<'_> {
        builder::GetByOrgByRepoDiff::new(self)
    }

    ///Get AI attribution data
    ///
    ///Retrieve agentblame attribution data for commits between two refs
    ///
    ///Sends a `GET` request to `/{org}/{repo}/agentblame`
    ///
    ///```ignore
    /// let response = client.get_by_org_by_repo_agentblame()
    ///    .org(org)
    ///    .repo(repo)
    ///    .base(base)
    ///    .head(head)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org_by_repo_agentblame(&self) -> builder::GetByOrgByRepoAgentblame<'_> {
        builder::GetByOrgByRepoAgentblame::new(self)
    }

    ///Get repository analytics
    ///
    ///Retrieve repository-wide AI attribution analytics from agentblame
    ///
    ///Sends a `GET` request to `/{org}/{repo}/analytics`
    ///
    ///```ignore
    /// let response = client.get_by_org_by_repo_analytics()
    ///    .org(org)
    ///    .repo(repo)
    ///    .period(period)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org_by_repo_analytics(&self) -> builder::GetByOrgByRepoAnalytics<'_> {
        builder::GetByOrgByRepoAnalytics::new(self)
    }

    ///Refresh repository analytics
    ///
    ///Trigger a full re-aggregation of repository analytics
    ///
    ///Sends a `POST` request to `/{org}/{repo}/analytics/refresh`
    ///
    ///```ignore
    /// let response = client.post_by_org_by_repo_analytics_refresh()
    ///    .org(org)
    ///    .repo(repo)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_by_org_by_repo_analytics_refresh(
        &self,
    ) -> builder::PostByOrgByRepoAnalyticsRefresh<'_> {
        builder::PostByOrgByRepoAnalyticsRefresh::new(self)
    }

    ///List webhooks
    ///
    ///List webhooks for a repository
    ///
    ///Sends a `GET` request to `/{org}/{repo}/webhooks`
    ///
    ///```ignore
    /// let response = client.get_by_org_by_repo_webhooks()
    ///    .org(org)
    ///    .repo(repo)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org_by_repo_webhooks(&self) -> builder::GetByOrgByRepoWebhooks<'_> {
        builder::GetByOrgByRepoWebhooks::new(self)
    }

    ///Create webhook
    ///
    ///Create a webhook for a repository
    ///
    ///Sends a `POST` request to `/{org}/{repo}/webhooks`
    ///
    ///```ignore
    /// let response = client.post_by_org_by_repo_webhooks()
    ///    .org(org)
    ///    .repo(repo)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_by_org_by_repo_webhooks(&self) -> builder::PostByOrgByRepoWebhooks<'_> {
        builder::PostByOrgByRepoWebhooks::new(self)
    }

    ///Delete webhook
    ///
    ///Delete a webhook from a repository
    ///
    ///Sends a `DELETE` request to `/{org}/{repo}/webhooks/{webhookId}`
    ///
    ///```ignore
    /// let response = client.delete_by_org_by_repo_webhooks_by_webhook_id()
    ///    .org(org)
    ///    .repo(repo)
    ///    .webhook_id(webhook_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_by_org_by_repo_webhooks_by_webhook_id(
        &self,
    ) -> builder::DeleteByOrgByRepoWebhooksByWebhookId<'_> {
        builder::DeleteByOrgByRepoWebhooksByWebhookId::new(self)
    }

    ///Get organization
    ///
    ///Get organization metadata and repository counts
    ///
    ///Sends a `GET` request to `/{org}`
    ///
    ///```ignore
    /// let response = client.get_by_org()
    ///    .org(org)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_by_org(&self) -> builder::GetByOrg<'_> {
        builder::GetByOrg::new(self)
    }
}

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, ClientHooks, ClientInfo, Error, OperationInfo, RequestBuilderExt,
        ResponseValue,
    };
    ///Builder for [`Client::get_by_org_api_keys`]
    ///
    ///[`Client::get_by_org_api_keys`]: super::Client::get_by_org_api_keys
    #[derive(Debug, Clone)]
    pub struct GetByOrgApiKeys<'a> {
        client: &'a super::Client,
        org: Result<::std::string::String, String>,
    }

    impl<'a> GetByOrgApiKeys<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for org failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/{org}/api-keys`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetByOrgApiKeysResponse>,
            Error<types::GetByOrgApiKeysResponse>,
        > {
            let Self { client, org } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/api-keys",
                client.baseurl,
                encode_path(&org.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org_api_keys",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_by_org_api_keys`]
    ///
    ///[`Client::post_by_org_api_keys`]: super::Client::post_by_org_api_keys
    #[derive(Debug, Clone)]
    pub struct PostByOrgApiKeys<'a> {
        client: &'a super::Client,
        org: Result<::std::string::String, String>,
        body: Result<types::builder::PostByOrgApiKeysBody, String>,
    }

    impl<'a> PostByOrgApiKeys<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for org failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgApiKeysBody>,
            <V as std::convert::TryInto<types::PostByOrgApiKeysBody>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `PostByOrgApiKeysBody` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::PostByOrgApiKeysBody,
            ) -> types::builder::PostByOrgApiKeysBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/{org}/api-keys`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::PostByOrgApiKeysResponse>,
            Error<types::PostByOrgApiKeysResponse>,
        > {
            let Self { client, org, body } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::PostByOrgApiKeysBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/api-keys",
                client.baseurl,
                encode_path(&org.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "post_by_org_api_keys",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::delete_by_org_api_keys_by_id`]
    ///
    ///[`Client::delete_by_org_api_keys_by_id`]: super::Client::delete_by_org_api_keys_by_id
    #[derive(Debug, Clone)]
    pub struct DeleteByOrgApiKeysById<'a> {
        client: &'a super::Client,
        org: Result<::std::string::String, String>,
        id: Result<types::DeleteByOrgApiKeysByIdId, String>,
    }

    impl<'a> DeleteByOrgApiKeysById<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                id: Err("id was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for org failed".to_string()
            });
            self
        }

        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeleteByOrgApiKeysByIdId>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `DeleteByOrgApiKeysByIdId` for id failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/{org}/api-keys/{id}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::DeleteByOrgApiKeysByIdResponse>,
            Error<types::DeleteByOrgApiKeysByIdResponse>,
        > {
            let Self { client, org, id } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/api-keys/{}",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_by_org_api_keys_by_id",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_by_org_repos`]
    ///
    ///[`Client::get_by_org_repos`]: super::Client::get_by_org_repos
    #[derive(Debug, Clone)]
    pub struct GetByOrgRepos<'a> {
        client: &'a super::Client,
        org: Result<types::GetByOrgReposOrg, String>,
        cursor: Result<Option<::std::string::String>, String>,
        limit: Result<Option<::std::num::NonZeroU64>, String>,
    }

    impl<'a> GetByOrgRepos<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                cursor: Ok(None),
                limit: Ok(None),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgReposOrg>,
        {
            self.org = value
                .try_into()
                .map_err(|_| "conversion to `GetByOrgReposOrg` for org failed".to_string());
            self
        }

        pub fn cursor<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.cursor = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for cursor failed".to_string()
            });
            self
        }

        pub fn limit<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.limit = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for limit failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/{org}/repos`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetByOrgReposResponse>, Error<types::GetByOrgReposResponse>>
        {
            let Self {
                client,
                org,
                cursor,
                limit,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let cursor = cursor.map_err(Error::InvalidRequest)?;
            let limit = limit.map_err(Error::InvalidRequest)?;
            let url = format!("{}/{}/repos", client.baseurl, encode_path(&org.to_string()),);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("cursor", &cursor))
                .query(&progenitor_client::QueryParam::new("limit", &limit))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org_repos",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_by_org_repos`]
    ///
    ///[`Client::post_by_org_repos`]: super::Client::post_by_org_repos
    #[derive(Debug, Clone)]
    pub struct PostByOrgRepos<'a> {
        client: &'a super::Client,
        org: Result<types::PostByOrgReposOrg, String>,
        body: Result<types::builder::PostByOrgReposBody, String>,
    }

    impl<'a> PostByOrgRepos<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgReposOrg>,
        {
            self.org = value
                .try_into()
                .map_err(|_| "conversion to `PostByOrgReposOrg` for org failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgReposBody>,
            <V as std::convert::TryInto<types::PostByOrgReposBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `PostByOrgReposBody` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::PostByOrgReposBody,
            ) -> types::builder::PostByOrgReposBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/{org}/repos`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::PostByOrgReposResponse>,
            Error<types::PostByOrgReposResponse>,
        > {
            let Self { client, org, body } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::PostByOrgReposBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/{}/repos", client.baseurl, encode_path(&org.to_string()),);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "post_by_org_repos",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_by_org_by_repo`]
    ///
    ///[`Client::get_by_org_by_repo`]: super::Client::get_by_org_by_repo
    #[derive(Debug, Clone)]
    pub struct GetByOrgByRepo<'a> {
        client: &'a super::Client,
        org: Result<types::GetByOrgByRepoOrg, String>,
        repo: Result<types::GetByOrgByRepoRepo, String>,
    }

    impl<'a> GetByOrgByRepo<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoOrg>,
        {
            self.org = value
                .try_into()
                .map_err(|_| "conversion to `GetByOrgByRepoOrg` for org failed".to_string());
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoRepo>,
        {
            self.repo = value
                .try_into()
                .map_err(|_| "conversion to `GetByOrgByRepoRepo` for repo failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{org}/{repo}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetByOrgByRepoResponse>,
            Error<types::GetByOrgByRepoResponse>,
        > {
            let Self { client, org, repo } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org_by_repo",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::delete_by_org_by_repo`]
    ///
    ///[`Client::delete_by_org_by_repo`]: super::Client::delete_by_org_by_repo
    #[derive(Debug, Clone)]
    pub struct DeleteByOrgByRepo<'a> {
        client: &'a super::Client,
        org: Result<types::DeleteByOrgByRepoOrg, String>,
        repo: Result<types::DeleteByOrgByRepoRepo, String>,
    }

    impl<'a> DeleteByOrgByRepo<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeleteByOrgByRepoOrg>,
        {
            self.org = value
                .try_into()
                .map_err(|_| "conversion to `DeleteByOrgByRepoOrg` for org failed".to_string());
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeleteByOrgByRepoRepo>,
        {
            self.repo = value
                .try_into()
                .map_err(|_| "conversion to `DeleteByOrgByRepoRepo` for repo failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/{org}/{repo}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::DeleteByOrgByRepoResponse>,
            Error<types::DeleteByOrgByRepoResponse>,
        > {
            let Self { client, org, repo } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_by_org_by_repo",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::patch_by_org_by_repo`]
    ///
    ///[`Client::patch_by_org_by_repo`]: super::Client::patch_by_org_by_repo
    #[derive(Debug, Clone)]
    pub struct PatchByOrgByRepo<'a> {
        client: &'a super::Client,
        org: Result<types::PatchByOrgByRepoOrg, String>,
        repo: Result<types::PatchByOrgByRepoRepo, String>,
        body: Result<types::builder::PatchByOrgByRepoBody, String>,
    }

    impl<'a> PatchByOrgByRepo<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PatchByOrgByRepoOrg>,
        {
            self.org = value
                .try_into()
                .map_err(|_| "conversion to `PatchByOrgByRepoOrg` for org failed".to_string());
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PatchByOrgByRepoRepo>,
        {
            self.repo = value
                .try_into()
                .map_err(|_| "conversion to `PatchByOrgByRepoRepo` for repo failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PatchByOrgByRepoBody>,
            <V as std::convert::TryInto<types::PatchByOrgByRepoBody>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `PatchByOrgByRepoBody` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::PatchByOrgByRepoBody,
            ) -> types::builder::PatchByOrgByRepoBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PATCH` request to `/{org}/{repo}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::PatchByOrgByRepoResponse>,
            Error<types::PatchByOrgByRepoResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                body,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::PatchByOrgByRepoBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .patch(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "patch_by_org_by_repo",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_by_org_by_repo_sync`]
    ///
    ///[`Client::get_by_org_by_repo_sync`]: super::Client::get_by_org_by_repo_sync
    #[derive(Debug, Clone)]
    pub struct GetByOrgByRepoSync<'a> {
        client: &'a super::Client,
        org: Result<types::GetByOrgByRepoSyncOrg, String>,
        repo: Result<types::GetByOrgByRepoSyncRepo, String>,
    }

    impl<'a> GetByOrgByRepoSync<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoSyncOrg>,
        {
            self.org = value
                .try_into()
                .map_err(|_| "conversion to `GetByOrgByRepoSyncOrg` for org failed".to_string());
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoSyncRepo>,
        {
            self.repo = value
                .try_into()
                .map_err(|_| "conversion to `GetByOrgByRepoSyncRepo` for repo failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{org}/{repo}/sync`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetByOrgByRepoSyncResponse>,
            Error<types::GetByOrgByRepoSyncResponse>,
        > {
            let Self { client, org, repo } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/sync",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org_by_repo_sync",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_by_org_by_repo_sync`]
    ///
    ///[`Client::post_by_org_by_repo_sync`]: super::Client::post_by_org_by_repo_sync
    #[derive(Debug, Clone)]
    pub struct PostByOrgByRepoSync<'a> {
        client: &'a super::Client,
        org: Result<types::PostByOrgByRepoSyncOrg, String>,
        repo: Result<types::PostByOrgByRepoSyncRepo, String>,
    }

    impl<'a> PostByOrgByRepoSync<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoSyncOrg>,
        {
            self.org = value
                .try_into()
                .map_err(|_| "conversion to `PostByOrgByRepoSyncOrg` for org failed".to_string());
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoSyncRepo>,
        {
            self.repo = value
                .try_into()
                .map_err(|_| "conversion to `PostByOrgByRepoSyncRepo` for repo failed".to_string());
            self
        }

        ///Sends a `POST` request to `/{org}/{repo}/sync`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::PostByOrgByRepoSyncResponse>,
            Error<types::PostByOrgByRepoSyncResponse>,
        > {
            let Self { client, org, repo } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/sync",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "post_by_org_by_repo_sync",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_by_org_by_repo_content`]
    ///
    ///[`Client::get_by_org_by_repo_content`]: super::Client::get_by_org_by_repo_content
    #[derive(Debug, Clone)]
    pub struct GetByOrgByRepoContent<'a> {
        client: &'a super::Client,
        org: Result<types::GetByOrgByRepoContentOrg, String>,
        repo: Result<types::GetByOrgByRepoContentRepo, String>,
        depth: Result<Option<::std::num::NonZeroU64>, String>,
        path: Result<Option<::std::string::String>, String>,
        ref_: Result<Option<::std::string::String>, String>,
    }

    impl<'a> GetByOrgByRepoContent<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                depth: Ok(None),
                path: Ok(None),
                ref_: Ok(None),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoContentOrg>,
        {
            self.org = value
                .try_into()
                .map_err(|_| "conversion to `GetByOrgByRepoContentOrg` for org failed".to_string());
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoContentRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoContentRepo` for repo failed".to_string()
            });
            self
        }

        pub fn depth<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.depth = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for depth failed".to_string()
            });
            self
        }

        pub fn path<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.path = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for path failed".to_string()
            });
            self
        }

        pub fn ref_<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.ref_ = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for ref_ failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/{org}/{repo}/content`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetByOrgByRepoContentResponse>,
            Error<types::GetByOrgByRepoContentResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                depth,
                path,
                ref_,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let depth = depth.map_err(Error::InvalidRequest)?;
            let path = path.map_err(Error::InvalidRequest)?;
            let ref_ = ref_.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/content",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("depth", &depth))
                .query(&progenitor_client::QueryParam::new("path", &path))
                .query(&progenitor_client::QueryParam::new("ref", &ref_))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org_by_repo_content",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_by_org_by_repo_branches`]
    ///
    ///[`Client::get_by_org_by_repo_branches`]: super::Client::get_by_org_by_repo_branches
    #[derive(Debug, Clone)]
    pub struct GetByOrgByRepoBranches<'a> {
        client: &'a super::Client,
        org: Result<types::GetByOrgByRepoBranchesOrg, String>,
        repo: Result<types::GetByOrgByRepoBranchesRepo, String>,
        cursor: Result<Option<::std::string::String>, String>,
        limit: Result<Option<::std::num::NonZeroU64>, String>,
    }

    impl<'a> GetByOrgByRepoBranches<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                cursor: Ok(None),
                limit: Ok(None),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoBranchesOrg>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoBranchesOrg` for org failed".to_string()
            });
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoBranchesRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoBranchesRepo` for repo failed".to_string()
            });
            self
        }

        pub fn cursor<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.cursor = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for cursor failed".to_string()
            });
            self
        }

        pub fn limit<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.limit = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for limit failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/{org}/{repo}/branches`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetByOrgByRepoBranchesResponse>,
            Error<types::GetByOrgByRepoBranchesResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                cursor,
                limit,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let cursor = cursor.map_err(Error::InvalidRequest)?;
            let limit = limit.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/branches",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("cursor", &cursor))
                .query(&progenitor_client::QueryParam::new("limit", &limit))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org_by_repo_branches",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_by_org_by_repo_branches`]
    ///
    ///[`Client::post_by_org_by_repo_branches`]: super::Client::post_by_org_by_repo_branches
    #[derive(Debug, Clone)]
    pub struct PostByOrgByRepoBranches<'a> {
        client: &'a super::Client,
        org: Result<types::PostByOrgByRepoBranchesOrg, String>,
        repo: Result<types::PostByOrgByRepoBranchesRepo, String>,
        body: Result<types::builder::PostByOrgByRepoBranchesBody, String>,
    }

    impl<'a> PostByOrgByRepoBranches<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoBranchesOrg>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `PostByOrgByRepoBranchesOrg` for org failed".to_string()
            });
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoBranchesRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `PostByOrgByRepoBranchesRepo` for repo failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoBranchesBody>,
            <V as std::convert::TryInto<types::PostByOrgByRepoBranchesBody>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `PostByOrgByRepoBranchesBody` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::PostByOrgByRepoBranchesBody,
            ) -> types::builder::PostByOrgByRepoBranchesBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/{org}/{repo}/branches`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::PostByOrgByRepoBranchesResponse>,
            Error<types::PostByOrgByRepoBranchesResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                body,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::PostByOrgByRepoBranchesBody::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/branches",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "post_by_org_by_repo_branches",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::delete_by_org_by_repo_branches_by_branch`]
    ///
    ///[`Client::delete_by_org_by_repo_branches_by_branch`]: super::Client::delete_by_org_by_repo_branches_by_branch
    #[derive(Debug, Clone)]
    pub struct DeleteByOrgByRepoBranchesByBranch<'a> {
        client: &'a super::Client,
        org: Result<types::DeleteByOrgByRepoBranchesByBranchOrg, String>,
        repo: Result<types::DeleteByOrgByRepoBranchesByBranchRepo, String>,
        branch: Result<types::DeleteByOrgByRepoBranchesByBranchBranch, String>,
    }

    impl<'a> DeleteByOrgByRepoBranchesByBranch<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                branch: Err("branch was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeleteByOrgByRepoBranchesByBranchOrg>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `DeleteByOrgByRepoBranchesByBranchOrg` for org failed".to_string()
            });
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeleteByOrgByRepoBranchesByBranchRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `DeleteByOrgByRepoBranchesByBranchRepo` for repo failed".to_string()
            });
            self
        }

        pub fn branch<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeleteByOrgByRepoBranchesByBranchBranch>,
        {
            self.branch = value.try_into().map_err(|_| {
                "conversion to `DeleteByOrgByRepoBranchesByBranchBranch` for branch failed"
                    .to_string()
            });
            self
        }

        ///Sends a `DELETE` request to `/{org}/{repo}/branches/{branch}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::DeleteByOrgByRepoBranchesByBranchResponse>,
            Error<types::DeleteByOrgByRepoBranchesByBranchResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                branch,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let branch = branch.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/branches/{}",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
                encode_path(&branch.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_by_org_by_repo_branches_by_branch",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_by_org_by_repo_commits`]
    ///
    ///[`Client::get_by_org_by_repo_commits`]: super::Client::get_by_org_by_repo_commits
    #[derive(Debug, Clone)]
    pub struct GetByOrgByRepoCommits<'a> {
        client: &'a super::Client,
        org: Result<types::GetByOrgByRepoCommitsOrg, String>,
        repo: Result<types::GetByOrgByRepoCommitsRepo, String>,
        cursor: Result<Option<::std::string::String>, String>,
        limit: Result<Option<::std::num::NonZeroU64>, String>,
        ref_: Result<Option<::std::string::String>, String>,
    }

    impl<'a> GetByOrgByRepoCommits<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                cursor: Ok(None),
                limit: Ok(None),
                ref_: Ok(None),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoCommitsOrg>,
        {
            self.org = value
                .try_into()
                .map_err(|_| "conversion to `GetByOrgByRepoCommitsOrg` for org failed".to_string());
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoCommitsRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoCommitsRepo` for repo failed".to_string()
            });
            self
        }

        pub fn cursor<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.cursor = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for cursor failed".to_string()
            });
            self
        }

        pub fn limit<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.limit = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for limit failed".to_string()
            });
            self
        }

        pub fn ref_<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.ref_ = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for ref_ failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/{org}/{repo}/commits`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetByOrgByRepoCommitsResponse>,
            Error<types::GetByOrgByRepoCommitsResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                cursor,
                limit,
                ref_,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let cursor = cursor.map_err(Error::InvalidRequest)?;
            let limit = limit.map_err(Error::InvalidRequest)?;
            let ref_ = ref_.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/commits",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("cursor", &cursor))
                .query(&progenitor_client::QueryParam::new("limit", &limit))
                .query(&progenitor_client::QueryParam::new("ref", &ref_))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org_by_repo_commits",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_by_org_by_repo_commits`]
    ///
    ///[`Client::post_by_org_by_repo_commits`]: super::Client::post_by_org_by_repo_commits
    #[derive(Debug, Clone)]
    pub struct PostByOrgByRepoCommits<'a> {
        client: &'a super::Client,
        org: Result<types::PostByOrgByRepoCommitsOrg, String>,
        repo: Result<types::PostByOrgByRepoCommitsRepo, String>,
        body: Result<types::builder::PostByOrgByRepoCommitsBody, String>,
    }

    impl<'a> PostByOrgByRepoCommits<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoCommitsOrg>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `PostByOrgByRepoCommitsOrg` for org failed".to_string()
            });
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoCommitsRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `PostByOrgByRepoCommitsRepo` for repo failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoCommitsBody>,
            <V as std::convert::TryInto<types::PostByOrgByRepoCommitsBody>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `PostByOrgByRepoCommitsBody` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::PostByOrgByRepoCommitsBody,
            ) -> types::builder::PostByOrgByRepoCommitsBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/{org}/{repo}/commits`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::PostByOrgByRepoCommitsResponse>,
            Error<types::PostByOrgByRepoCommitsResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                body,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::PostByOrgByRepoCommitsBody::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/commits",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "post_by_org_by_repo_commits",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_by_org_by_repo_commits_by_sha`]
    ///
    ///[`Client::get_by_org_by_repo_commits_by_sha`]: super::Client::get_by_org_by_repo_commits_by_sha
    #[derive(Debug, Clone)]
    pub struct GetByOrgByRepoCommitsBySha<'a> {
        client: &'a super::Client,
        org: Result<types::GetByOrgByRepoCommitsByShaOrg, String>,
        repo: Result<types::GetByOrgByRepoCommitsByShaRepo, String>,
        sha: Result<types::GetByOrgByRepoCommitsByShaSha, String>,
    }

    impl<'a> GetByOrgByRepoCommitsBySha<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                sha: Err("sha was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoCommitsByShaOrg>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoCommitsByShaOrg` for org failed".to_string()
            });
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoCommitsByShaRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoCommitsByShaRepo` for repo failed".to_string()
            });
            self
        }

        pub fn sha<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoCommitsByShaSha>,
        {
            self.sha = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoCommitsByShaSha` for sha failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/{org}/{repo}/commits/{sha}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetByOrgByRepoCommitsByShaResponse>,
            Error<types::GetByOrgByRepoCommitsByShaResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                sha,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let sha = sha.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/commits/{}",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
                encode_path(&sha.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org_by_repo_commits_by_sha",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_by_org_by_repo_diff`]
    ///
    ///[`Client::get_by_org_by_repo_diff`]: super::Client::get_by_org_by_repo_diff
    #[derive(Debug, Clone)]
    pub struct GetByOrgByRepoDiff<'a> {
        client: &'a super::Client,
        org: Result<types::GetByOrgByRepoDiffOrg, String>,
        repo: Result<types::GetByOrgByRepoDiffRepo, String>,
        base: Result<types::GetByOrgByRepoDiffBase, String>,
        head: Result<types::GetByOrgByRepoDiffHead, String>,
    }

    impl<'a> GetByOrgByRepoDiff<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                base: Err("base was not initialized".to_string()),
                head: Err("head was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoDiffOrg>,
        {
            self.org = value
                .try_into()
                .map_err(|_| "conversion to `GetByOrgByRepoDiffOrg` for org failed".to_string());
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoDiffRepo>,
        {
            self.repo = value
                .try_into()
                .map_err(|_| "conversion to `GetByOrgByRepoDiffRepo` for repo failed".to_string());
            self
        }

        pub fn base<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoDiffBase>,
        {
            self.base = value
                .try_into()
                .map_err(|_| "conversion to `GetByOrgByRepoDiffBase` for base failed".to_string());
            self
        }

        pub fn head<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoDiffHead>,
        {
            self.head = value
                .try_into()
                .map_err(|_| "conversion to `GetByOrgByRepoDiffHead` for head failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{org}/{repo}/diff`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetByOrgByRepoDiffResponse>,
            Error<types::GetByOrgByRepoDiffResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                base,
                head,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let base = base.map_err(Error::InvalidRequest)?;
            let head = head.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/diff",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("base", &base))
                .query(&progenitor_client::QueryParam::new("head", &head))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org_by_repo_diff",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_by_org_by_repo_agentblame`]
    ///
    ///[`Client::get_by_org_by_repo_agentblame`]: super::Client::get_by_org_by_repo_agentblame
    #[derive(Debug, Clone)]
    pub struct GetByOrgByRepoAgentblame<'a> {
        client: &'a super::Client,
        org: Result<types::GetByOrgByRepoAgentblameOrg, String>,
        repo: Result<types::GetByOrgByRepoAgentblameRepo, String>,
        base: Result<types::GetByOrgByRepoAgentblameBase, String>,
        head: Result<types::GetByOrgByRepoAgentblameHead, String>,
    }

    impl<'a> GetByOrgByRepoAgentblame<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                base: Err("base was not initialized".to_string()),
                head: Err("head was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoAgentblameOrg>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoAgentblameOrg` for org failed".to_string()
            });
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoAgentblameRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoAgentblameRepo` for repo failed".to_string()
            });
            self
        }

        pub fn base<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoAgentblameBase>,
        {
            self.base = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoAgentblameBase` for base failed".to_string()
            });
            self
        }

        pub fn head<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoAgentblameHead>,
        {
            self.head = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoAgentblameHead` for head failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/{org}/{repo}/agentblame`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetByOrgByRepoAgentblameResponse>,
            Error<types::GetByOrgByRepoAgentblameResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                base,
                head,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let base = base.map_err(Error::InvalidRequest)?;
            let head = head.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/agentblame",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("base", &base))
                .query(&progenitor_client::QueryParam::new("head", &head))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org_by_repo_agentblame",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_by_org_by_repo_analytics`]
    ///
    ///[`Client::get_by_org_by_repo_analytics`]: super::Client::get_by_org_by_repo_analytics
    #[derive(Debug, Clone)]
    pub struct GetByOrgByRepoAnalytics<'a> {
        client: &'a super::Client,
        org: Result<types::GetByOrgByRepoAnalyticsOrg, String>,
        repo: Result<types::GetByOrgByRepoAnalyticsRepo, String>,
        period: Result<Option<types::GetByOrgByRepoAnalyticsPeriod>, String>,
    }

    impl<'a> GetByOrgByRepoAnalytics<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                period: Ok(None),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoAnalyticsOrg>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoAnalyticsOrg` for org failed".to_string()
            });
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoAnalyticsRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoAnalyticsRepo` for repo failed".to_string()
            });
            self
        }

        pub fn period<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoAnalyticsPeriod>,
        {
            self.period = value.try_into().map(Some).map_err(|_| {
                "conversion to `GetByOrgByRepoAnalyticsPeriod` for period failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/{org}/{repo}/analytics`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetByOrgByRepoAnalyticsResponse>,
            Error<types::GetByOrgByRepoAnalyticsResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                period,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let period = period.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/analytics",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("period", &period))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org_by_repo_analytics",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_by_org_by_repo_analytics_refresh`]
    ///
    ///[`Client::post_by_org_by_repo_analytics_refresh`]: super::Client::post_by_org_by_repo_analytics_refresh
    #[derive(Debug, Clone)]
    pub struct PostByOrgByRepoAnalyticsRefresh<'a> {
        client: &'a super::Client,
        org: Result<types::PostByOrgByRepoAnalyticsRefreshOrg, String>,
        repo: Result<types::PostByOrgByRepoAnalyticsRefreshRepo, String>,
    }

    impl<'a> PostByOrgByRepoAnalyticsRefresh<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoAnalyticsRefreshOrg>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `PostByOrgByRepoAnalyticsRefreshOrg` for org failed".to_string()
            });
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoAnalyticsRefreshRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `PostByOrgByRepoAnalyticsRefreshRepo` for repo failed".to_string()
            });
            self
        }

        ///Sends a `POST` request to `/{org}/{repo}/analytics/refresh`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::PostByOrgByRepoAnalyticsRefreshResponse>,
            Error<types::PostByOrgByRepoAnalyticsRefreshResponse>,
        > {
            let Self { client, org, repo } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/analytics/refresh",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "post_by_org_by_repo_analytics_refresh",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_by_org_by_repo_webhooks`]
    ///
    ///[`Client::get_by_org_by_repo_webhooks`]: super::Client::get_by_org_by_repo_webhooks
    #[derive(Debug, Clone)]
    pub struct GetByOrgByRepoWebhooks<'a> {
        client: &'a super::Client,
        org: Result<types::GetByOrgByRepoWebhooksOrg, String>,
        repo: Result<types::GetByOrgByRepoWebhooksRepo, String>,
    }

    impl<'a> GetByOrgByRepoWebhooks<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoWebhooksOrg>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoWebhooksOrg` for org failed".to_string()
            });
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetByOrgByRepoWebhooksRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `GetByOrgByRepoWebhooksRepo` for repo failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/{org}/{repo}/webhooks`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetByOrgByRepoWebhooksResponse>,
            Error<types::GetByOrgByRepoWebhooksResponse>,
        > {
            let Self { client, org, repo } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/webhooks",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org_by_repo_webhooks",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_by_org_by_repo_webhooks`]
    ///
    ///[`Client::post_by_org_by_repo_webhooks`]: super::Client::post_by_org_by_repo_webhooks
    #[derive(Debug, Clone)]
    pub struct PostByOrgByRepoWebhooks<'a> {
        client: &'a super::Client,
        org: Result<types::PostByOrgByRepoWebhooksOrg, String>,
        repo: Result<types::PostByOrgByRepoWebhooksRepo, String>,
        body: Result<types::builder::PostByOrgByRepoWebhooksBody, String>,
    }

    impl<'a> PostByOrgByRepoWebhooks<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoWebhooksOrg>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `PostByOrgByRepoWebhooksOrg` for org failed".to_string()
            });
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoWebhooksRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `PostByOrgByRepoWebhooksRepo` for repo failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostByOrgByRepoWebhooksBody>,
            <V as std::convert::TryInto<types::PostByOrgByRepoWebhooksBody>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `PostByOrgByRepoWebhooksBody` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::PostByOrgByRepoWebhooksBody,
            ) -> types::builder::PostByOrgByRepoWebhooksBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/{org}/{repo}/webhooks`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::PostByOrgByRepoWebhooksResponse>,
            Error<types::PostByOrgByRepoWebhooksResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                body,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::PostByOrgByRepoWebhooksBody::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/webhooks",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "post_by_org_by_repo_webhooks",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::delete_by_org_by_repo_webhooks_by_webhook_id`]
    ///
    ///[`Client::delete_by_org_by_repo_webhooks_by_webhook_id`]: super::Client::delete_by_org_by_repo_webhooks_by_webhook_id
    #[derive(Debug, Clone)]
    pub struct DeleteByOrgByRepoWebhooksByWebhookId<'a> {
        client: &'a super::Client,
        org: Result<types::DeleteByOrgByRepoWebhooksByWebhookIdOrg, String>,
        repo: Result<types::DeleteByOrgByRepoWebhooksByWebhookIdRepo, String>,
        webhook_id: Result<types::DeleteByOrgByRepoWebhooksByWebhookIdWebhookId, String>,
    }

    impl<'a> DeleteByOrgByRepoWebhooksByWebhookId<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
                repo: Err("repo was not initialized".to_string()),
                webhook_id: Err("webhook_id was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeleteByOrgByRepoWebhooksByWebhookIdOrg>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `DeleteByOrgByRepoWebhooksByWebhookIdOrg` for org failed".to_string()
            });
            self
        }

        pub fn repo<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeleteByOrgByRepoWebhooksByWebhookIdRepo>,
        {
            self.repo = value.try_into().map_err(|_| {
                "conversion to `DeleteByOrgByRepoWebhooksByWebhookIdRepo` for repo failed"
                    .to_string()
            });
            self
        }

        pub fn webhook_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeleteByOrgByRepoWebhooksByWebhookIdWebhookId>,
        {
            self . webhook_id = value . try_into () . map_err (| _ | "conversion to `DeleteByOrgByRepoWebhooksByWebhookIdWebhookId` for webhook_id failed" . to_string ()) ;
            self
        }

        ///Sends a `DELETE` request to `/{org}/{repo}/webhooks/{webhookId}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::DeleteByOrgByRepoWebhooksByWebhookIdResponse>,
            Error<types::DeleteByOrgByRepoWebhooksByWebhookIdResponse>,
        > {
            let Self {
                client,
                org,
                repo,
                webhook_id,
            } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let repo = repo.map_err(Error::InvalidRequest)?;
            let webhook_id = webhook_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/{}/webhooks/{}",
                client.baseurl,
                encode_path(&org.to_string()),
                encode_path(&repo.to_string()),
                encode_path(&webhook_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_by_org_by_repo_webhooks_by_webhook_id",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_by_org`]
    ///
    ///[`Client::get_by_org`]: super::Client::get_by_org
    #[derive(Debug, Clone)]
    pub struct GetByOrg<'a> {
        client: &'a super::Client,
        org: Result<::std::string::String, String>,
    }

    impl<'a> GetByOrg<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org: Err("org was not initialized".to_string()),
            }
        }

        pub fn org<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.org = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for org failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/{org}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetByOrgResponse>, Error<types::GetByOrgResponse>>
        {
            let Self { client, org } = self;
            let org = org.map_err(Error::InvalidRequest)?;
            let url = format!("{}/{}", client.baseurl, encode_path(&org.to_string()),);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_by_org",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                406u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
