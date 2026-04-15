"use client";

export function ReviewDiffMessage({ message }: { message: string }) {
  const [title, ...rest] = message.split("\n");
  const body = rest.join("\n").trim();

  return (
    <div className="flex-1 min-w-0">
      <p className="text-sm font-semibold max-w-xl mb-2">{title}</p>
      {body && (
        <p className="text-xs text-muted-foreground leading-relaxed max-w-xl">{body}</p>
      )}
    </div>
  );
}
