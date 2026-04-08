import { MarkdownBody } from "../[repo]/ui/markdown/markdown-body";

export function UserReadme({ readme }: { readme: string | null | undefined }) {
  return (
    <div>
      <p className="text-xs text-muted-foreground font-mono mb-2">
        <span className="text-foreground/40 select-none"># </span>
        README.md
      </p>
      {readme ? (
        <MarkdownBody content={readme} />
      ) : (
        <span className="font-mono text-xs">README.md not found</span>
      )}
    </div>
  );
}
