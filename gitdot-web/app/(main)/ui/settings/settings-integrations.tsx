"use client";

import Image from "next/image";
import { useUserContext } from "@/(main)/context/user";
import { githubAppInstallUrl } from "@/util";

export function SettingsIntegrations() {
  const { installations } = useUserContext();

  return (
    <div className="p-3">
      <div className="flex items-center gap-2">
        <Image src="/github-logo.svg" alt="GitHub" width={14} height={14} />
        <p className="text-sm font-medium">GitHub</p>
      </div>
      <p className="text-sm text-muted-foreground">
        Connect a GitHub account or organization to import repositories — gitdot
        clones their full history. We only read what you authorize and never
        modify anything on GitHub.
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
            <a
              href={githubAppInstallUrl("migration")}
              className="text-sm text-muted-foreground hover:text-foreground underline underline-offset-2 cursor-pointer transition-colors"
            >
              Add account
            </a>
          </div>
        </>
      )}
    </div>
  );
}
