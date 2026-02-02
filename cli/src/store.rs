use std::io::Write;
use std::process::{Command, Stdio};

use url::Url;

pub struct GitCredentialStore;

impl GitCredentialStore {
    /// Store credentials using git's credential helper system.
    /// This works with whatever credential helper the user has configured
    /// (e.g., osxkeychain, manager-core, cache, store).
    /// See https://git-scm.com/docs/git-credential for more information.
    pub fn store(url: &str, username: &str, password: &str) -> anyhow::Result<()> {
        let parsed = Url::parse(url)?;
        let protocol = parsed.scheme();
        let host = parsed
            .host_str()
            .ok_or_else(|| anyhow::anyhow!("URL missing host"))?;
        let port = parsed.port();

        let mut credential_input = format!(
            "protocol={}\nhost={}\nusername={}\npassword={}\n",
            protocol, host, username, password
        );
        if let Some(p) = port {
            credential_input = format!(
                "protocol={}\nhost={}:{}\nusername={}\npassword={}\n",
                protocol, host, p, username, password
            );
        }

        let mut child = Command::new("git")
            .args(["credential", "approve"])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(credential_input.as_bytes())?;
        }
        let output = child.wait_with_output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git credential approve failed: {}", stderr);
        }

        Ok(())
    }
}
