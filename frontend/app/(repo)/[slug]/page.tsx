import { notFound } from "next/navigation";
import validateRepoSlug from "@/util/validate-repo-slug";
import { FileHeader } from "./ui/file-header";
import { FileViewer } from "./ui/file-viewer";

// generateStaticParams: https://nextjs.org/docs/app/api-reference/file-conventions/dynamic-routes#with-generatestaticparams
// if we provide a list of things here at build-time, we'll pre-generate static pages at build time.
export default async function RepoPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;

  if (!validateRepoSlug(slug)) {
    return notFound();
  }

  return (
    <div className="flex flex-col w-full h-screen">
      <FileHeader />
      <div className="flex-1 overflow-hidden">
        <FileViewer />
      </div>
    </div>
  );
}
