import { getRepositoryFile } from "@/lib/dal";
import Markdown from 'react-markdown'

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

  return <div className="px-4 py-2">
    <Markdown>
      {readme.content}
    </Markdown>
  </div>;
}
