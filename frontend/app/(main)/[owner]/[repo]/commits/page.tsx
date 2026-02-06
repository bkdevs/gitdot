import { Fragment } from "react";
import { getRepositoryCommits, NotFound } from "@/lib/dal";
import Link from "@/ui/link";
import { formatDateKey, formatTime } from "@/util";
import { groupCommitsByDate } from "../util/commit";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const commitsData = await getRepositoryCommits(owner, repo);

  if (!commitsData || commitsData === NotFound) return null;

  const commitsByDate = groupCommitsByDate(commitsData.commits);

  return (
    <div className="flex flex-col">
      {commitsByDate.map(([date, commits]) => (
        <Fragment key={date}>
          <div className="sticky top-0 bg-background flex items-center border-b px-2 h-9 z-10">
            <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
              {formatDateKey(date)}
            </h3>
          </div>
          {commits.map((commit) => (
            <Link
              key={commit.sha}
              href={`/${owner}/${repo}/commits/${commit.sha.substring(0, 7)}`}
              className="flex w-full border-b hover:bg-accent/50 select-none cursor-pointer py-2 px-2"
              prefetch={true}
            >
              <div className="flex flex-col w-full justify-start items-start min-w-0">
                <div className="text-sm truncate mb-0.5 w-full">
                  {commit.message}
                </div>
                <div className="text-xs text-muted-foreground flex items-center gap-1 w-full min-w-0">
                  <span className="truncate min-w-0">
                    {typeof commit.author === "string"
                      ? commit.author
                      : commit.author.name}
                  </span>
                  <span className="shrink-0">•</span>
                  <span className="shrink-0">
                    {formatTime(new Date(commit.date))}
                  </span>
                </div>
              </div>
            </Link>
          ))}
        </Fragment>
      ))}
    </div>
  );
}
