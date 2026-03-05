import CommitsClient from "./ui/commits-client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  return <CommitsClient owner={owner} repo={repo} />;
}
