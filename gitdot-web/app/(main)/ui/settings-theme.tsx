export function SettingsTheme() {
  return (
    <section className="space-y-3">
      <h2 className="text-xs uppercase tracking-widest text-muted-foreground">
        Theme
      </h2>
      <p className="text-sm leading-relaxed text-muted-foreground">
        Duis aute irure dolor in reprehenderit in voluptate velit esse cillum
        dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
        proident.
      </p>
      <div className="space-y-2">
        {[
          "Color scheme",
          "Font size",
          "Line height",
          "Tab width",
          "Syntax highlighting",
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
