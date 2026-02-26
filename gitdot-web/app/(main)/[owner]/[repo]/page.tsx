import { CACHED_REPOS } from "@/config";
import { getRepositoryFile, NotFound } from "@/lib/dal";
import { MarkdownBody } from "./ui/markdown/markdown-body";

export async function generateStaticParams() {
  return CACHED_REPOS;
}

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
    return <div className="p-2 text-sm">Failed to fetch README.md</div>;
  } else if (readme === NotFound) {
    return <div className="p-2 text-sm">README.md not found</div>;
  }
  return (
    <div className="p-4 max-w-4xl">
      <MarkdownBody content={readme.content} />
    </div>
  );
}
