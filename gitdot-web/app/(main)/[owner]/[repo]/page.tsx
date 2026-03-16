import { ApiProvider } from "@/provider/api";
import { Client, Resources } from "./client";

// export async function generateStaticParams() {
//   // return CACHED_REPOS;
// }

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const provider = new ApiProvider(owner, repo);

  return (
    <Client
      owner={owner}
      repo={repo}
      serverPromises={provider.fetch(Resources)}
    />
  );
}
