use anyhow::Result;
use s2_sdk::types::{BasinName, StreamName};

pub fn parse_s2_uri(uri: &str) -> Result<(BasinName, StreamName)> {
    let rest = uri
        .strip_prefix("s2://")
        .ok_or_else(|| anyhow::anyhow!("invalid s2 URI: {uri}"))?;
    let (basin_str, stream_str) = rest
        .split_once('/')
        .ok_or_else(|| anyhow::anyhow!("invalid s2 URI: {uri}"))?;
    let basin_str = basin_str.replace('.', "-");

    let basin_name = basin_str
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid basin name: {basin_str}"))?;
    let stream_name = stream_str
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid stream name: {stream_str}"))?;
    Ok((basin_name, stream_name))
}
