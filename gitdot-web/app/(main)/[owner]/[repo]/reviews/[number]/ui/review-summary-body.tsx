"use client";

import type { ReviewResource } from "gitdot-api";
import { AvatarBeam } from "@/ui/avatar-beam";
import { MarkdownBody } from "@/(main)/[owner]/[repo]/ui/markdown/markdown-body";

const FILES = [
  { path: "app/actions/diff.ts", status: "created" },
  { path: "app/actions/repository.ts", status: "modified" },
  { path: "app/dal/diff.ts", status: "created" },
];

const TESTS = [
  { label: "cargo test -p gitdot-core", result: "pass" },
  { label: "pnpm build", result: "pass" },
  { label: "biome check", result: "fail" },
];

function ReviewRationale({ description }: { description: string }) {
  return (
    <section className="flex flex-col gap-1">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Rationale
      </h2>
      <div className="[&>p:last-child]:mb-0">
        <MarkdownBody content={description} />
      </div>
    </section>
  );
}

function ReviewFiles() {
  return (
    <section className="flex flex-col gap-1">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Files Changed
      </h2>
      <div className="flex flex-col gap-2">
        {FILES.map((f) => (
          <div key={f.path} className="flex items-center gap-2">
            <span
              className={
                f.status === "created"
                  ? "text-[10px] font-sans bg-green-500/15 text-green-600 dark:text-green-400 px-1.5 rounded leading-5 shrink-0"
                  : "text-[10px] font-sans bg-muted text-muted-foreground px-1.5 rounded leading-5 shrink-0"
              }
            >
              {f.status === "created" ? "new" : "mod"}
            </span>
            <span className="font-mono text-xs text-foreground/70 truncate">
              {f.path}
            </span>
          </div>
        ))}
      </div>
    </section>
  );
}

function ReviewChecks() {
  return (
    <section className="flex flex-col gap-1">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Checks
      </h2>
      <div className="flex flex-col gap-1.5">
        {TESTS.map((t) => (
          <div key={t.label} className="flex items-center gap-2">
            <span
              className={
                t.result === "pass"
                  ? "text-green-600 dark:text-green-400 text-xs shrink-0"
                  : "text-red-500 dark:text-red-400 text-xs shrink-0"
              }
            >
              {t.result === "pass" ? "✓" : "✗"}
            </span>
            <span className="text-xs text-muted-foreground">{t.label}</span>
          </div>
        ))}
      </div>
    </section>
  );
}

function ReviewReviewers() {
  return (
    <section className="flex flex-col gap-1">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Reviewers
      </h2>
      <div className="flex flex-col gap-1.5">
        {["alice", "bob", "carol"].map((name) => (
          <div key={name} className="flex items-center gap-2">
            <AvatarBeam name={name} size={18} />
            <span className="text-sm text-foreground/70">{name}</span>
          </div>
        ))}
      </div>
    </section>
  );
}

export function ReviewSummaryBody({ review }: { review: ReviewResource }) {
  return (
    <div className="flex-1 overflow-y-auto px-6 pt-4 flex flex-col gap-8">
      <ReviewRationale description={review.description} />
      <ReviewFiles />
      <ReviewChecks />
      <ReviewReviewers />
    </div>
  );
}
