export function SettingsShortcuts() {
  return (
    <section className="space-y-3">
      <h2 className="text-xs uppercase tracking-widest text-muted-foreground">
        Shortcuts
      </h2>
      <p className="text-sm leading-relaxed text-muted-foreground">
        Sunt in culpa qui officia deserunt mollit anim id est laborum. Curabitur
        pretium tincidunt lacus nulla gravida orci a odio.
      </p>
      <div className="space-y-2">
        {[
          "Open settings",
          "Open file search",
          "Toggle sidebar",
          "Go to repo",
          "Show shortcuts",
        ].map((label) => (
          <div
            key={label}
            className="flex items-center justify-between py-2 border-b border-border"
          >
            <span className="text-sm">{label}</span>
            <span className="text-xs text-muted-foreground">configure</span>
          </div>
        ))}
      </div>
    </section>
  );
}
