import type { Metadata } from "next";
import { notFound } from "next/navigation";
import MarkdownContent from "@/(main)/(marketing)/ui/markdown-content";
import Link from "@/ui/link";
import { leagueSpartan } from "../../fonts";
import { getAllVersions, getReleaseByVersion } from "../util";

export async function generateStaticParams() {
  const versions = getAllVersions();
  return versions.map((version) => ({ version }));
}

export async function generateMetadata({
  params,
}: {
  params: Promise<{ version: string }>;
}): Promise<Metadata> {
  const { version } = await params;
  const release = getReleaseByVersion(version);
  if (!release) {
    return { title: "Release Not Found" };
  }

  return {
    title: `gitdot | ${release.metadata.title}`,
    description: release.content.slice(0, 160),
  };
}

export default async function Page({
  params,
}: {
  params: Promise<{ version: string }>;
}) {
  const { version } = await params;
  const release = getReleaseByVersion(version);

  if (!release) {
    notFound();
  }

  return (
    <div
      className={`${leagueSpartan.className} px-3 pt-3 pb-2 h-full overflow-y-auto scrollbar-none`}
    >
      <article className="w-full overflow-hidden md:overflow-visible">
        <h1 className="text-2xl">{release.metadata.title}</h1>
        <Link href="/releases" className="text-sm hover:underline">
          {release.metadata.version}: {release.metadata.date}
        </Link>
        <div className="pb-4" />

        <MarkdownContent content={release.content} />
      </article>
    </div>
  );
}
