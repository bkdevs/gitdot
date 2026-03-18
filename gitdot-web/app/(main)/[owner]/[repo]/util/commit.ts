import type { RepositoryCommitResource } from "gitdot-api";

export function sortCommits(
  commits: RepositoryCommitResource[] | null,
): RepositoryCommitResource[] | null {
  if (!commits) return null;
  return commits
    .slice()
    .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
}
