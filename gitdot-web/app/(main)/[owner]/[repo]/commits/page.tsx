import { Suspense } from "react";
import { CommitsClient } from "./ui/commits-client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  return (
    <Suspense>
      <CommitsClient owner={owner} repo={repo} />
    </Suspense>
  );
}
