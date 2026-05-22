"use client";

import { useUserContext } from "@/(main)/provider/user";
import { getGithubAppInstallUrlAction } from "@/actions/migration";

export function SettingsInstallations() {
  const { installations } = useUserContext();

  async function handleAddAccount() {
    const result = await getGithubAppInstallUrlAction("migration");
    if ("url" in result) {
      window.location.href = result.url;
    }
  }

  return (
    <div className="p-3">
      <div className="flex items-center gap-2">
        <p className="text-sm font-medium dark:font-normal">Installations</p>
      </div>
      <p className="text-sm text-muted-foreground">
        Connect a GitHub account or organization to migrate repositories —
        gitdot clones their full history. We only read what you authorize and
        never modify anything on GitHub.
      </p>
      {!installations ? (
        <div className="mt-3 text-sm text-muted-foreground">loading...</div>
      ) : (
        <>
          <div className="mt-3">
            {installations.length === 0 ? (
              <div className="border border-border rounded p-3 text-sm text-muted-foreground">
                no installations found
              </div>
            ) : (
              <ul>
                {installations.map((installation) => (
                  <li key={installation.id}>
                    <a
                      href={`https://github.com/${installation.github_login}`}
                      target="_blank"
                      rel="noreferrer"
                      className="flex items-center gap-2 text-sm border-b border-border hover:border-foreground transition-all duration-200"
                    >
                      <span className="flex-1 truncate">
                        github.com/{installation.github_login}
                      </span>
                      <span className="text-xs text-muted-foreground">
                        {installation.installation_type}
                      </span>
                    </a>
                  </li>
                ))}
              </ul>
            )}
          </div>
          <div className="flex justify-start mt-2">
            <button
              type="button"
              onClick={handleAddAccount}
              className="text-sm text-muted-foreground hover:text-foreground underline underline-offset-2 cursor-pointer transition-colors"
            >
              Add account
            </button>
          </div>
        </>
      )}
    </div>
  );
}
