export function UserReadme() {
  return (
    <div>
      <p className="text-xs text-muted-foreground font-mono mb-2">
        <span className="text-foreground/40 select-none"># </span>
        README.md
      </p>
      <p className="text-sm text-muted-foreground">
        software engineer building tools for open-source maintainers. i care a
        lot about developer experience, fast feedback loops, and shipping things
        that actually work.
      </p>
      <p className="text-sm text-muted-foreground mt-3">
        currently working on gitdot — a github alternative built for maintainers
        who want more control. the stack is rust on the backend (axum + sqlx)
        and next.js on the frontend.
      </p>
      <p className="text-sm text-muted-foreground mt-3">
        when i&apos;m not coding, you can find me in brooklyn, probably thinking
        about distributed systems or eating a good sandwich.
      </p>
    </div>
  );
}
