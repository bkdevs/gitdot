import { FilesClient } from "./ui/files-client";

export default async function FilesPage({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  return <FilesClient owner={owner} repo={repo} />;
}
