import { ApiProvider } from "@/provider/api";
import { Client } from "./client";
import { repoDef } from "./def";
import { Shell } from "./shell";

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
  const serverContext = provider.define(repoDef);

  return (
    <Shell owner={owner} repo={repo} serverContext={serverContext}>
      <Client />
    </Shell>
  );
}
