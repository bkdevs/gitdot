import type { RepositoryPathsResource } from "gitdot-api";
import { fetchResources } from "@/provider/server";
import { LayoutClient } from "./layout.client";

export type Resources = {
  paths: RepositoryPathsResource | null;
};

export default async function Layout({
  params,
  children,
}: {
  params: Promise<{ owner: string; repo: string }>;
  children: React.ReactNode;
}) {
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    paths: (p) => p.getPaths(),
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
