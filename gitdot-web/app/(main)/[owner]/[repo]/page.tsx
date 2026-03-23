import type { RepositoryBlobResource } from "gitdot-api";
import {
  fetchResources,
  type ResourcePromisesType,
  type ResourceRequestsType,
} from "@/provider/server";
import { PageClient } from "./page.client";

type Resources = {
  readme: RepositoryBlobResource | null;
};
export type ResourcePromises = ResourcePromisesType<Resources>;
export type ResourceRequests = ResourceRequestsType<Resources>;

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    readme: (p) => p.getBlob("README.md"),
  });

  return (
    <PageClient
      owner={owner}
      repo={repo}
      requests={requests}
      promises={promises}
    />
  );
}
