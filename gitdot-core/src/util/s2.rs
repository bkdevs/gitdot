use s2_sdk::types::{BasinName, StreamName};

pub fn parse_s2_uri(s2_uri: &str) -> Result<(BasinName, StreamName), String> {
    let path = s2_uri
        .strip_prefix("s2://")
        .ok_or_else(|| format!("invalid s2_uri: {s2_uri}"))?;
    let (basin_str, stream_str) = path
        .split_once('/')
        .ok_or_else(|| format!("invalid s2_uri: {s2_uri}"))?;
    let basin = basin_str
        .parse()
        .map_err(|_| format!("invalid basin name: {basin_str}"))?;
    let stream = stream_str
        .parse()
        .map_err(|_| format!("invalid stream name: {stream_str}"))?;
    Ok((basin, stream))
}
