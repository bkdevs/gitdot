use enumset::EnumSetType;

use super::{
    basin::{BasinName, BasinNamePrefix},
    stream::{StreamName, StreamNamePrefix},
};

#[derive(Debug, Hash, EnumSetType, strum::EnumCount)]
pub enum Operation {
    ListBasins = 1,
    CreateBasin = 2,
    DeleteBasin = 3,
    ReconfigureBasin = 4,
    GetBasinConfig = 5,
    ListStreams = 9,
    CreateStream = 10,
    DeleteStream = 11,
    GetStreamConfig = 12,
    ReconfigureStream = 13,
    CheckTail = 14,
    Append = 15,
    Read = 16,
    Trim = 17,
    Fence = 18,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
pub enum ResourceSet<E, P> {
    #[default]
    None,
    Exact(E),
    Prefix(P),
}

pub type BasinResourceSet = ResourceSet<BasinName, BasinNamePrefix>;
pub type StreamResourceSet = ResourceSet<StreamName, StreamNamePrefix>;
