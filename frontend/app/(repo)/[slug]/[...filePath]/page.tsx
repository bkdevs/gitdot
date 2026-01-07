import { getRepositoryCommits, getRepositoryFile } from "@/lib/dal";
import { FileHeader } from "./ui/file-header";
import { FileViewer } from "./ui/file-viewer";
import { type LineSelection, parseLineSelection } from "./util";

// generateStaticParams: https://nextjs.org/docs/app/api-reference/file-conventions/dynamic-routes#with-generatestaticparams
// if we provide a list of things here at build-time, we'll pre-generate static pages at build time.
export default async function Page({
  params,
  searchParams,
}: {
  params: Promise<{ slug: string; filePath: string[] }>;
  searchParams: Promise<{ [key: string]: string | string[] | undefined }>;
}) {
  const { slug, filePath } = await params;
  const file = await getRepositoryFile("bkdevs", slug, { path: filePath.join("/") });
  if (!file) {
    return <div>File not found.</div>
  }
  const commits = await getRepositoryCommits("bkdevs", slug);
  const mostRecentCommit = commits?.commits.find(commit => commit.sha === file.commit_sha)!;

  const { lines } = await searchParams;
  const selectedLines: LineSelection | null =
    typeof lines === "string" ? parseLineSelection(lines) : null;

  return (
    <div className="flex flex-col w-full h-screen">
      <FileHeader file={file} commit={mostRecentCommit}  />
      <div className="flex-1 overflow-hidden">
        <FileViewer
          repo={slug}
          file={file}
          selectedLines={selectedLines}
        />
      </div>
    </div>
  );
}
