import { Suspense } from "react";
import { FilesClient } from "./ui/files-client";

export default async function FilesPage({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  return (
    // TODO: i want to get rid of this but we run into some infinite loop if i do?
    <Suspense>
      <FilesClient owner={owner} repo={repo} />
    </Suspense>
  );
}
