import type { RepositoryCommitResource } from "gitdot-api";
import { fetchResources } from "@/provider/server";
import { LayoutClient } from "./layout.client";

export type Resources = {
  commits: RepositoryCommitResource[] | null;
};

export default async function Layout({
  params,
  children,
}: {
  params: Promise<{ owner: string; repo: string; sha: string }>;
  children: React.ReactNode;
}) {
  // TODO: use sha to influence what commits we fetch before and after
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    commits: (p) => p.getCommits(),
  });

  return (
    <LayoutClient
      owner={owner}
      repo={repo}
      requests={requests}
      promises={promises}
    >
      {children}
    </LayoutClient>
  );
}
