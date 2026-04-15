"use client";

export function ReviewDiffMessage({ message }: { message: string }) {
  const [title, ...rest] = message.split("\n");
  const body = rest.join("\n").trim();

  return (
    <div className="max-w-3xl mx-auto px-1 pt-6">
      <p className="text-sm font-semibold mb-2">{title}</p>
      {body && (
        <p className="text-xs text-muted-foreground leading-relaxed">{body}</p>
      )}
    </div>
  );
}
