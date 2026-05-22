import type { Metadata } from "next";
import { League_Spartan } from "next/font/google";
import Link from "@/ui/link";
import { getAllPosts } from "./lib/posts";

const league_spartan = League_Spartan({
  subsets: ["latin"],
  weight: ["400", "700"],
});

export const metadata: Metadata = {
  title: "gitdot | dev log",
  description: "weekly updates on building gitdot",
};

export default function Page() {
  const posts = getAllPosts();

  return (
    <div className={`${league_spartan.className} blog-root`}>
      {posts.length === 0 ? (
        <p>No posts yet.</p>
      ) : (
        <div className="space-y-2">
          {posts.map((post) => (
            <article key={post.metadata.week}>
              <div className="flex flex-row w-full items-baseline justify-between">
                <Link
                  href={`/weeks/${post.metadata.week}`}
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
  );
}
