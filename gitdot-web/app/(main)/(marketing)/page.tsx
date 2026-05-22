"use client";

import { useState } from "react";
import Link from "@/ui/link";
import { cn } from "@/util";

type FeedTab = "trending" | "new";

type FeedRepo = {
  owner: string;
  name: string;
  description?: string;
  stars: number;
};

const TRENDING: FeedRepo[] = [
  {
    owner: "rust-lang",
    name: "rust",
    description:
      "Empowering everyone to build reliable and efficient software.",
    stars: 92410,
  },
  {
    owner: "tokio-rs",
    name: "tokio",
    description:
      "A runtime for writing reliable asynchronous applications with Rust.",
    stars: 26312,
  },
  {
    owner: "vercel",
    name: "next.js",
    description: "The React Framework for the Web.",
    stars: 121008,
  },
  {
    owner: "biomejs",
    name: "biome",
    description:
      "A toolchain for web projects, aimed to provide functionalities to maintain them.",
    stars: 15820,
  },
  {
    owner: "supabase",
    name: "supabase",
    description: "The open source Firebase alternative.",
    stars: 70234,
  },
  {
    owner: "denoland",
    name: "deno",
    description: "A modern runtime for JavaScript and TypeScript.",
    stars: 94120,
  },
  {
    owner: "tailwindlabs",
    name: "tailwindcss",
    description: "A utility-first CSS framework for rapid UI development.",
    stars: 80211,
  },
  {
    owner: "withastro",
    name: "astro",
    description: "The web framework for content-driven websites.",
    stars: 45102,
  },
];

const NEW: FeedRepo[] = [
  {
    owner: "baepaul",
    name: "tinypost",
    description: "A minimal, single-binary blog engine written in Rust.",
    stars: 12,
  },
  {
    owner: "mikkelk",
    name: "claude-cookbook",
    description: "Patterns and snippets for shipping with the Claude API.",
    stars: 38,
  },
  {
    owner: "halfwit",
    name: "weft",
    description: "A tiny terminal multiplexer for people who only need splits.",
    stars: 4,
  },
  {
    owner: "ninabit",
    name: "lila",
    description: "Visual diffing for SQL migrations.",
    stars: 21,
  },
  {
    owner: "ptr-collective",
    name: "kettle",
    description: "Postgres connection pooler written in Zig.",
    stars: 0,
  },
  {
    owner: "okta-okta",
    name: "draft",
    description: "Local-first writing tool with end-to-end encryption.",
    stars: 7,
  },
  {
    owner: "softserve",
    name: "ferment",
    description: "A pull-request bot that drafts changelogs from commits.",
    stars: 53,
  },
  {
    owner: "anonyhalibut",
    name: "kelp",
    description: "An opinionated CI runner that won't bill you for re-runs.",
    stars: 2,
  },
];

const FEEDS: Record<FeedTab, FeedRepo[]> = {
  trending: TRENDING,
  new: NEW,
};

export default function Home() {
  const [tab, setTab] = useState<FeedTab>("trending");

  return (
    <>
      <div className="flex items-baseline gap-4">
        {(Object.keys(FEEDS) as FeedTab[]).map((key) => (
          <button
            key={key}
            type="button"
            onClick={() => setTab(key)}
            className={cn(
              "text-sm font-mono cursor-pointer transition-colors",
              key === tab
                ? "font-semibold text-foreground"
                : "text-muted-foreground hover:text-foreground",
            )}
          >
            {key}
          </button>
        ))}
      </div>

      <div className="flex flex-col gap-2">
        {FEEDS[tab].map((repo) => (
          <div key={`${repo.owner}/${repo.name}`} className="flex flex-col">
            <div className="flex items-baseline justify-between gap-4">
              <Link
                href={`/${repo.owner}/${repo.name}`}
                className="text-sm font-medium dark:font-normal underline decoration-transparent hover:decoration-current transition-colors duration-200 truncate"
              >
                <span className="font-normal text-muted-foreground">
                  {repo.owner}/
                </span>
                {repo.name}
              </Link>
              {repo.stars > 0 && (
                <span className="text-xs text-muted-foreground font-mono">
                  ({repo.stars})
                </span>
              )}
            </div>
            {repo.description && (
              <div className="text-xs text-foreground truncate pb-1">
                {repo.description}
              </div>
            )}
          </div>
        ))}
      </div>
    </>
  );
}
