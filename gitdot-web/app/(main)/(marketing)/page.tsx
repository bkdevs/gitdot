import {
  listLatestRepositories,
  listTrendingRepositories,
} from "gitdot-client";
import Link from "@/ui/link";
import { PageClient } from "./page.client";
import { getAllReleases } from "./releases/util";
import { getAllPosts } from "./weeks/util";

export default async function Page() {
  const [trending, latest] = await Promise.all([
    listTrendingRepositories(),
    listLatestRepositories(),
  ]);

  const thisWeek = getAllPosts()[0] ?? null;
  const nextRelease =
    getAllReleases().find((r) => r.metadata.status === "upcoming") ?? null;

  return (
    <>
      <PageClient trending={trending ?? []} latest={latest ?? []} />
      <aside className="hidden lg:flex pt-4 pl-8 pr-4 flex-col gap-8">
        {thisWeek && (
          <section className="group flex flex-col gap-0.5 cursor-pointer">
            <span className="text-xs font-mono text-muted-foreground">
              # this week
            </span>
            <Link
              href={`/weeks/${thisWeek.metadata.week}`}
              className="text-sm font-medium text-foreground underline decoration-transparent group-hover:decoration-current transition-colors duration-200"
            >
              {thisWeek.metadata.title}
            </Link>
            <span className="text-xs text-muted-foreground">
              Week {thisWeek.metadata.week}:{" "}
              {thisWeek.metadata.date.replace(" - ", " — ")}
            </span>
          </section>
        )}

        {nextRelease && (
          <section className="group flex flex-col gap-0.5 cursor-pointer">
            <span className="text-xs font-mono text-muted-foreground">
              # next release
            </span>
            <Link
              href={`/releases/${nextRelease.metadata.version}`}
              className="text-sm font-medium text-foreground underline decoration-transparent group-hover:decoration-current transition-colors duration-200"
            >
              {nextRelease.metadata.version}: {nextRelease.metadata.title}
            </Link>
            <span className="text-xs text-muted-foreground">
              ETA: {nextRelease.metadata.date}
            </span>
          </section>
        )}
      </aside>
    </>
  );
}
