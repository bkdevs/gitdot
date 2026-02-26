import type { Metadata } from "next";
import Link from "@/ui/link";
import { getAllPosts } from "../lib/posts";

export const metadata: Metadata = {
  title: "gitdot | dev log",
  description: "weekly updates on building gitdot",
};

export default function Page() {
  const posts = getAllPosts();

  return (
    <div className="min-h-screen grid place-items-center">
      <div className="w-full max-w-160 px-4 sm:px-8 py-8">
        {posts.length === 0 ? (
          <p>No posts yet.</p>
        ) : (
          <div className="space-y-2">
            {posts.map((post) => (
              <article key={post.metadata.week}>
                <div className="flex flex-row w-full items-baseline justify-between">
                  <Link
                    href={`/week/${post.metadata.week}`}
                    className="text-lg font-medium hover:underline"
                  >
                    Week {post.metadata.week}: {post.metadata.title}
                  </Link>
                  <p className="text-sm">{post.metadata.date}</p>
                </div>
              </article>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
