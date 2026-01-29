import { SubscribeButton } from '@/(landing)/ui/subscribe-button';
import { Metadata } from 'next';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import { getAllWeeks, getPostByWeek } from "../../lib/posts";
import MarkdownContent from '../ui/markdown-content';

export async function generateStaticParams() {
  const weeks = getAllWeeks();
  return weeks.map(num => ({ number: num.toString() }));
}

export async function generateMetadata({
  params,
}: {
  params: Promise<{ number: string }>;
}): Promise<Metadata> {
  const { number } = await params;
  const post = getPostByWeek(parseInt(number, 10));
  if (!post) {
    return { title: 'Post Not Found' };
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
    <div className="min-h-screen grid place-items-center overflow-x-hidden">
      <article className="w-full max-w-160 px-4 sm:px-8 py-8 overflow-hidden md:overflow-visible">
        <h1 className="text-2xl">{post.metadata.title}</h1>
        <Link
          href="/week"
          className="text-sm hover:underline">Week {post.metadata.week}: {post.metadata.date}</Link>
        <div className="pb-4"/>

        <MarkdownContent content={post.content} />

        <div className="mt-8 flex justify-end">
          <SubscribeButton />
        </div>
      </article>
    </div>
  );
}
