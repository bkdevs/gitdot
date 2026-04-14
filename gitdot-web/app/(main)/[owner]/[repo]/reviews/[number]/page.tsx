import { AvatarBeam } from "@/ui/avatar-beam";
import { DiffTabs } from "./diff-tabs";
import { ReviewHeader } from "./review-header";

const FILES = [
  { path: "app/actions/diff.ts", status: "created" as const },
  { path: "app/actions/repository.ts", status: "modified" as const },
  { path: "app/actions/index.ts", status: "modified" as const },
  { path: "reviews/[number]/review-diff-body.tsx", status: "created" as const },
  { path: "reviews/[number]/review-diff-content.tsx", status: "modified" as const },
  { path: "commits/[sha]/ui/commit-body.tsx", status: "modified" as const },
  { path: "commits/[sha]/ui/commit-client.tsx", status: "modified" as const },
];

const TESTS = [
  { label: "pnpm biome check .", result: "pass" as const },
  { label: "pnpm build", result: "pass" as const },
  { label: "commit page — diff renders correctly", result: "pass" as const },
  { label: "review page — diff renders via ReviewDiffBody", result: "pass" as const },
];

export default async function Page() {
  return (
    <div className="flex h-full overflow-hidden">
      <div className="w-[30%] shrink-0 border-r border-border flex flex-col overflow-hidden">
        <ReviewHeader />
        <div className="flex-1 overflow-y-auto px-7 pt-4 pb-7 flex flex-col gap-8">

        {/* Rationale */}
        <section className="flex flex-col gap-2">
          <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
            Rationale
          </h2>
          <p className="text-sm leading-relaxed">
            Diff rendering logic was bundled inside{" "}
            <code className="text-[12px] bg-muted px-1 py-0.5 rounded">repository.ts</code>{" "}
            alongside unrelated CRUD actions. The review diff path was also doing
            redundant blob fetches — fetching{" "}
            <code className="text-[12px] bg-muted px-1 py-0.5 rounded">left_content</code> and{" "}
            <code className="text-[12px] bg-muted px-1 py-0.5 rounded">right_content</code>{" "}
            separately even though{" "}
            <code className="text-[12px] bg-muted px-1 py-0.5 rounded">getReviewDiff</code>{" "}
            already embeds both fields in the response.
          </p>
          <p className="text-sm leading-relaxed">
            This moves all diff actions into a dedicated{" "}
            <code className="text-[12px] bg-muted px-1 py-0.5 rounded">diff.ts</code> module,
            deletes <code className="text-[12px] bg-muted px-1 py-0.5 rounded">computeDiffDataFromBlobs</code>,
            and aligns the review diff path with the commit diff path — both now call{" "}
            <code className="text-[12px] bg-muted px-1 py-0.5 rounded">renderDiffs</code> directly.
          </p>
        </section>

        {/* Files */}
        <section className="flex flex-col gap-2">
          <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
            Files Changed
          </h2>
          <div className="flex flex-col gap-1">
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

        {/* Checks */}
        <section className="flex flex-col gap-2">
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

        {/* Reviewers */}
        <section className="flex flex-col gap-2">
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

        </div>
      </div>
      <div className="flex-1 flex flex-col overflow-hidden">
        <DiffTabs />
      </div>
    </div>
  );
}
