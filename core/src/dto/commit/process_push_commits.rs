/// Represents a single ref update from a git push
#[derive(Debug, Clone)]
pub struct RefUpdate {
    pub old_sha: String,
    pub new_sha: String,
    pub ref_name: String,
}

impl RefUpdate {
    pub fn new(old_sha: String, new_sha: String, ref_name: String) -> Self {
        Self {
            old_sha,
            new_sha,
            ref_name,
        }
    }

    /// Returns true if this is creating a new ref (old_sha is all zeros)
    pub fn is_create(&self) -> bool {
        self.old_sha.chars().all(|c| c == '0')
    }

    /// Returns true if this is deleting a ref (new_sha is all zeros)
    pub fn is_delete(&self) -> bool {
        self.new_sha.chars().all(|c| c == '0')
    }
}

#[derive(Debug, Clone)]
pub struct ProcessPushCommitsRequest {
    pub owner: String,
    pub repo: String,
    pub ref_updates: Vec<RefUpdate>,
}

impl ProcessPushCommitsRequest {
    pub fn new(owner: String, repo: String, ref_updates: Vec<RefUpdate>) -> Self {
        Self {
            owner,
            repo,
            ref_updates,
        }
    }
}

/// Parse ref updates from the git receive-pack request body.
/// The format is: `<old-sha> <new-sha> <ref-name>\0<capabilities>\n` for the first line,
/// then `<old-sha> <new-sha> <ref-name>\n` for subsequent lines.
pub fn parse_ref_updates(body: &[u8]) -> Vec<RefUpdate> {
    let mut updates = Vec::new();

    // Find the pack data boundary (starts with "PACK")
    let pack_start = body
        .windows(4)
        .position(|w| w == b"PACK")
        .unwrap_or(body.len());

    let header_section = &body[..pack_start];

    // Parse pkt-line format
    let mut pos = 0;
    while pos + 4 <= header_section.len() {
        // Read 4-byte hex length
        let len_hex = match std::str::from_utf8(&header_section[pos..pos + 4]) {
            Ok(s) => s,
            Err(_) => break,
        };

        let pkt_len = match usize::from_str_radix(len_hex, 16) {
            Ok(l) => l,
            Err(_) => break,
        };

        // 0000 is flush packet
        if pkt_len == 0 {
            pos += 4;
            continue;
        }

        if pos + pkt_len > header_section.len() {
            break;
        }

        // Extract the line content (excluding the 4-byte length prefix)
        let line_bytes = &header_section[pos + 4..pos + pkt_len];

        // Convert to string, stripping null bytes and newlines
        if let Ok(line) = std::str::from_utf8(line_bytes) {
            // Strip capabilities (after \0) and trailing newline
            let line = line.split('\0').next().unwrap_or("").trim();

            // Parse: <old-sha> <new-sha> <ref-name>
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() >= 3 && parts[0].len() == 40 && parts[1].len() == 40 {
                updates.push(RefUpdate::new(
                    parts[0].to_string(),
                    parts[1].to_string(),
                    parts[2].to_string(),
                ));
            }
        }

        pos += pkt_len;
    }

    updates
}
