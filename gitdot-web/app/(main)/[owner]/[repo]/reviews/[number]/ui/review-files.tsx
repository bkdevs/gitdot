"use client";

const FILES = [
  { path: "app/actions/diff.ts", status: "created" },
  { path: "app/actions/repository.ts", status: "modified" },
  { path: "app/dal/diff.ts", status: "created" },
];

export function ReviewFiles() {
  return (
    <section className="flex flex-col gap-1.5">
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
