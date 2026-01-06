import { FileHeader } from "./ui/file-header";
import { FileViewer } from "./ui/file-viewer";

// generateStaticParams: https://nextjs.org/docs/app/api-reference/file-conventions/dynamic-routes#with-generatestaticparams
// if we provide a list of things here at build-time, we'll pre-generate static pages at build time.
export default async function Page({
  params,
}: {
  params: Promise<{ slug: string; filePath: Array<string> }>;
}) {
  const { slug, filePath: filePathArray } = await params;
  const filePath = filePathArray.join("/");

  return (
    <div className="flex flex-col w-full h-screen">
      <FileHeader filePath={filePath} />
      <div className="flex-1 overflow-hidden">
        <FileViewer repo={slug} filePath={filePath} />
      </div>
    </div>
  );
}
