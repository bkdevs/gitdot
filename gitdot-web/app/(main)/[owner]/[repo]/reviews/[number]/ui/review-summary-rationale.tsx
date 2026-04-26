"use client";

import { MarkdownBody } from "@/(main)/[owner]/[repo]/ui/markdown/markdown-body";

export function ReviewSummaryRationale({
  description,
}: {
  description: string;
}) {
  return (
    <section className="flex flex-col gap-1.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Overview
      </h2>
      {description ? (
        <div className="[&>p:last-child]:mb-0">
          <MarkdownBody content={description} compact={true} />
        </div>
      ) : (
        <span className="text-xs text-muted-foreground">
          no overview found
        </span>
      )}
    </section>
  );
}
