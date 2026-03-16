import { ApiProvider } from "@/provider/api";
import { Client } from "./client";
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

  return <Shell context={{readme: provider.getBlob("README.md")}}>
    <Client />
  </Shell>
}
