import type { Metadata } from "next";
import { notFound } from "next/navigation";
import MarkdownContent from "@/(main)/(marketing)/ui/markdown-content";
import Link from "@/ui/link";
import { leagueSpartan } from "../../fonts";
import { getAllSlugs, getDesignBySlug } from "../util";

export async function generateStaticParams() {
  const slugs = getAllSlugs();
  return slugs.map((slug) => ({ slug }));
}

export async function generateMetadata({
  params,
}: {
  params: Promise<{ slug: string }>;
}): Promise<Metadata> {
  const { slug } = await params;
  const design = getDesignBySlug(slug);
  if (!design) {
    return { title: "Design Not Found" };
  }

  return {
    title: `gitdot | ${design.metadata.title}`,
    description: design.content.slice(0, 160),
  };
}

export default async function Page({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const design = getDesignBySlug(slug);

  if (!design) {
    notFound();
  }

  return (
    <div
      className={`${leagueSpartan.className} px-3 pt-3 pb-2 h-full overflow-y-auto scrollbar-none`}
    >
      <article className="w-full overflow-hidden md:overflow-visible">
        <h1 className="text-2xl">{design.metadata.title}</h1>
        <Link href="/designs" className="text-sm hover:underline">
          {design.metadata.author} · {design.metadata.date}
        </Link>
        <div className="pb-4" />

        <MarkdownContent content={design.content} />
      </article>
    </div>
  );
}
