import { notFound } from "next/navigation";
import { validateRepoSlug } from "@/util";

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
      The repo home page! should be readme
    </div>
  );
}
