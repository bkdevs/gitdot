import type { Metadata } from "next";
import { League_Spartan } from "next/font/google";
import { notFound } from "next/navigation";
import { SubscribeButton } from "@/(main)/ui/subscribe-button";
import Link from "@/ui/link";
import { getAllWeeks, getPostByWeek } from "../lib/posts";
import MarkdownContent from "../ui/markdown-content";

const league_spartan = League_Spartan({
  subsets: ["latin"],
  weight: ["400", "700"],
});

export async function generateStaticParams() {
  const weeks = getAllWeeks();
  return weeks.map((num) => ({ number: num.toString() }));
}

export async function generateMetadata({
  params,
}: {
  params: Promise<{ number: string }>;
}): Promise<Metadata> {
  const { number } = await params;
  const post = getPostByWeek(parseInt(number, 10));
  if (!post) {
    return { title: "Post Not Found" };
  }

  return {
    title: `gitdot | ${post.metadata.title}`,
    description: post.content.slice(0, 160),
  };
}

export default async function Page({
  params,
}: {
  params: Promise<{ number: string }>;
}) {
  const { number } = await params;
  const post = getPostByWeek(parseInt(number, 10));

  if (!post) {
    notFound();
  }

  return (
    <div className="px-3 pt-4 pb-2 h-full overflow-y-auto scrollbar-none">
      <article
        className={`${league_spartan.className} blog-root w-full overflow-hidden md:overflow-visible`}
      >
        <h1 className="text-2xl">{post.metadata.title}</h1>
        <Link href="/weeks" className="text-sm hover:underline">
          Week {post.metadata.week}: {post.metadata.date}
        </Link>
        <div className="pb-4" />

        <MarkdownContent content={post.content} />

        <div className="mt-8 flex justify-end">
          <SubscribeButton />
        </div>
      </article>
    </div>
  );
}
