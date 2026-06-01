import type { Metadata } from "next";
import Link from "@/ui/link";
import { leagueSpartan } from "../fonts";
import { getAllReleases } from "./util";

export const metadata: Metadata = {
  title: "gitdot | releases",
  description: "gitdot releases",
};

export default function Page() {
  const releases = getAllReleases();

  return (
    <div
      className={`${leagueSpartan.className} px-3 pt-3.5 pb-2 h-full overflow-y-auto scrollbar-none`}
    >
      {releases.length === 0 ? (
        <p>No releases yet.</p>
      ) : (
        <div className="space-y-2">
          {releases.map((release) => (
            <article key={release.metadata.version}>
              <div className="flex flex-row w-full items-baseline justify-between">
                <Link
                  href={`/releases/${release.metadata.version}`}
                  data-page-item
                  className="text-lg font-medium outline-none hover:underline focus:underline"
                >
                  {release.metadata.version}: {release.metadata.title}
                </Link>
                <p className="text-sm">{release.metadata.date}</p>
              </div>
            </article>
          ))}
        </div>
      )}
    </div>
  );
}
