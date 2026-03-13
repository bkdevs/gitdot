import { getRepositoryBlob, getRepositoryFileCommits } from "@/dal";
import { FileViewer } from "./ui/file-viewer";
import { FolderViewer } from "./ui/folder-viewer";
import { parseLineSelection } from "./util";

// TODO: think about how and whether we should do generateStaticParams
// i think? this causes prefetch behavior to fail as next.js isn't aware that the layouts are shared
// causing the initial html with file previews, commits, and the trees to be prefetched.
export default async function Page({
  params,
  searchParams,
}: {
  params: Promise<{ owner: string; repo: string; filePath: string[] }>;
  searchParams: Promise<{
    lines?: string | string[];
    ref?: string;
  }>;
}) {
  const { owner, repo, filePath } = await params;
  const { lines, ref } = await searchParams;

  const filePathString = decodeURIComponent(filePath.join("/"));
  const blob = await getRepositoryBlob(owner, repo, {
    path: filePathString,
    ref_name: ref,
  });

  if (!blob) return <div>File not found.</div>;

  if (blob.type === "folder") {
    return <FolderViewer owner={owner} repo={repo} entries={blob.entries} />;
  } else {
    // TODO: parallel thing, probably make this API generic yeah makes sense for a tree history too.
    const commits = await getRepositoryFileCommits(owner, repo, {
      path: filePathString,
      ref_name: ref,
    });
    return (
      <FileViewer
        file={blob}
        commits={commits}
        selectedLines={parseLineSelection(lines)}
        selectedCommit={ref}
      />
    );
  }
}
