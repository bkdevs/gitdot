import { getRepositoryFile } from "@/lib/dal";
import { MarkdownBody } from "./ui/markdown/markdown-body";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const readme = await getRepositoryFile(owner, repo, {
    path: "README.md",
  });

  if (!readme) {
    return <div>README.md not found</div>;
  }

  return <div className="p-4 max-w-4xl">
    <MarkdownBody content={readme.content} />
  </div>;
}
