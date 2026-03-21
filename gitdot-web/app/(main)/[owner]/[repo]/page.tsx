import {
  fetchResources,
  type ResourceDefinition,
  type ResourcePromisesType,
  type ResourceRequestsType,
} from "@/provider/server";
import { PageClient } from "./page.client";

const resources = {
  readme: (p) => p.getBlob("README.md"),
} satisfies ResourceDefinition;

export type ResourcePromises = ResourcePromisesType<typeof resources>;
export type ResourceRequests = ResourceRequestsType<typeof resources>;

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, resources);

  return (
    <PageClient
      owner={owner}
      repo={repo}
      requests={requests}
      promises={promises}
    />
  );
}
