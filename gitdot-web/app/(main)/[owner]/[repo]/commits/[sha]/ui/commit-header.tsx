"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { ExternalLink } from "lucide-react";
import { UserImage } from "@/(main)/[owner]/ui/user/user-image";
import { useTimezone } from "@/(main)/provider/timezone";
import Link from "@/ui/link";
import { formatDateTime } from "@/util/date";

export function CommitHeader({
  commit,
  owner,
  repo,
  showOpenInTab = false,
}: {
  commit: RepositoryCommitResource;
  owner: string;
  repo: string;
  showOpenInTab?: boolean;
}) {
  const tz = useTimezone();
  const date = new Date(commit.date);
  const shortSha = commit.sha.substring(0, 7);
  const { author } = commit;
  const linkedName = author.id && author.name ? author.name : null;

  return (
    <div className="flex flex-col">
      <div className="flex items-center justify-between gap-2 text-sm text-muted-foreground">
        <div className="flex items-center gap-2">
          <UserImage
            userId={author.id}
            username={author.name ?? author.git_name}
            px={20}
          />
          <span>
            {linkedName ? (
              <Link
                href={`/${linkedName}`}
                className="underline hover:text-foreground transition-colors duration-200"
                prefetch={true}
              >
                {linkedName}
              </Link>
            ) : (
              <span>{author.git_name}</span>
            )}
            {" in "}
            <span className="font-mono">
              <Link
                href={`/${owner}`}
                className="underline decoration-transparent hover:decoration-current transition-colors duration-200"
                prefetch={true}
              >
                {owner}
              </Link>
              /
              <Link
                href={`/${owner}/${repo}`}
                className="font-medium text-foreground underline decoration-transparent hover:decoration-current transition-colors duration-200"
                prefetch={true}
              >
                {repo}
              </Link>
            </span>
          </span>
        </div>
        {showOpenInTab && (
          <a
            href={`/${owner}/${repo}/commits/${shortSha}`}
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-1.5 font-mono text-sm text-muted-foreground hover:text-foreground underline decoration-transparent hover:decoration-current transition-colors duration-200"
          >
            <ExternalLink className="w-3 h-3" />
            open in tab
          </a>
        )}
      </div>
      <div className="text-sm text-foreground whitespace-pre-wrap mt-1">
        {commit.message}
      </div>
      <div className="flex items-baseline gap-1 text-xs font-mono text-muted-foreground mt-1">
        <span>{formatDateTime(date, tz)}</span>
        <span>·</span>
        <span>{commit.sha.substring(0, 7)}</span>
      </div>
    </div>
  );
}
