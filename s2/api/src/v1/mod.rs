pub mod basin;
pub mod config;
pub mod error;
pub mod stream;

use s2_common::types::{self, resources::RequestToken};

#[rustfmt::skip]
#[derive(Debug)]
pub struct S2RequestTokenHeader {
    /// Client-specified request token for idempotent retries.
    pub s2_request_token: RequestToken,
}

#[rustfmt::skip]
#[derive(Debug)]
pub struct BasinNamePathSegment {
    /// Basin name.
    pub basin: types::basin::BasinName,
}

#[rustfmt::skip]
#[derive(Debug)]
pub struct StreamNamePathSegment {
    /// Stream name.
    pub stream: types::stream::StreamName,
}

macro_rules! impl_list_request_conversions {
    ($name:ident, $prefix:ty, $start_after:ty) => {
        impl TryFrom<$name> for types::resources::ListItemsRequest<$prefix, $start_after> {
            type Error = types::ValidationError;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                let $name {
                    prefix,
                    start_after,
                    limit,
                } = value;

                Ok(Self::try_from(types::resources::ListItemsRequestParts {
                    prefix: prefix.unwrap_or_default(),
                    start_after: start_after.unwrap_or_default(),
                    limit: limit.map(Into::into).unwrap_or_default(),
                })?)
            }
        }
    };
}

pub(crate) use impl_list_request_conversions;
