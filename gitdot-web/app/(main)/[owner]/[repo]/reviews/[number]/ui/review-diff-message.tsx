"use client";

export function ReviewDiffMessage({ message }: { message: string }) {
  const [title, ...rest] = message.split("\n");
  const body = rest.join("\n").trim();

  return (
    <div className="flex-1 min-w-0 border-b border-border pb-4">
      <p className="text-sm font-medium max-w-xl mb-1">{title}</p>
      <p className="text-xs text-muted-foreground leading-relaxed max-w-xl">
        {body || "no description found"}
      </p>
    </div>
  );
}
