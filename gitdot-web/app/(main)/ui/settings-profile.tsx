export function SettingsProfile() {
  return (
    <section className="space-y-3">
      <h2 className="text-xs uppercase tracking-widest text-muted-foreground">Profile</h2>
      <p className="text-sm leading-relaxed text-muted-foreground">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.
      </p>
      <div className="space-y-2">
        {["Username", "Display name", "Email", "Bio", "Website"].map((label) => (
          <div key={label} className="flex items-center justify-between py-2 border-b border-border">
            <span className="text-sm">{label}</span>
            <span className="text-xs text-muted-foreground">configure</span>
          </div>
        ))}
      </div>
    </section>
  );
}
