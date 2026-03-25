"use client";

import { useRightSidebar } from "@/(main)/hooks/use-sidebar";
import Link from "@/ui/link";

export type TocHeader = { level: number; text: string; slug: string };

export function FolderToc({ headers }: { headers: TocHeader[] }) {
  const open = useRightSidebar();
  if (!open) return null;

  return (
    <div className="w-64 h-full border-l flex flex-col">
      <div className="flex-1 overflow-auto scrollbar-none">
        {headers.map((h) => (
          <Link
            key={h.slug}
            href={`#${h.slug}`}
            tabIndex={-1}
            className="flex w-full border-b focus:bg-accent/50 select-none cursor-default py-2 focus:outline-none"
            style={{ paddingLeft: `${(h.level - 1) * 12 + 8}px` }}
          >
            <span className="text-sm truncate">{h.text}</span>
          </Link>
        ))}
      </div>
    </div>
  );
}
