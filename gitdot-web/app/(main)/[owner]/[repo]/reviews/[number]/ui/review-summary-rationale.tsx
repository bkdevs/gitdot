"use client";

import { MarkdownBody } from "@/(main)/[owner]/[repo]/ui/markdown/markdown-body";

export function ReviewSummaryRationale({
  description,
}: {
  description: string;
}) {
  return (
    <section className="flex flex-col gap-1">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Rationale
      </h2>
      <div className="[&>p:last-child]:mb-0">
        <MarkdownBody content={description} compact={true} />
      </div>
    </section>
  );
}
