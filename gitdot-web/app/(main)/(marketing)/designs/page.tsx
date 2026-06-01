import type { Metadata } from "next";
import Link from "@/ui/link";
import { leagueSpartan } from "../fonts";
import { getAllDesigns } from "./util";

export const metadata: Metadata = {
  title: "gitdot | designs",
  description: "gitdot design docs",
};

export default function Page() {
  const designs = getAllDesigns();

  return (
    <div
      className={`${leagueSpartan.className} px-3 pt-3.5 pb-2 h-full overflow-y-auto scrollbar-none`}
    >
      {designs.length === 0 ? (
        <p>No designs yet.</p>
      ) : (
        <div className="space-y-2">
          {designs.map((design) => (
            <article key={design.metadata.slug}>
              <div className="flex flex-row w-full items-baseline justify-between">
                <Link
                  href={`/designs/${design.metadata.slug}`}
                  data-page-item
                  className="text-lg font-medium outline-none hover:underline focus:underline"
                >
                  {design.metadata.title}
                </Link>
                <p className="text-sm">
                  {design.metadata.author} · {design.metadata.date}
                </p>
              </div>
            </article>
          ))}
        </div>
      )}
    </div>
  );
}
