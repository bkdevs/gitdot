const colors = [
  { name: "background", var: "--background" },
  { name: "foreground", var: "--foreground" },
  { name: "popover", var: "--popover" },
  { name: "popover-foreground", var: "--popover-foreground" },
  { name: "primary", var: "--primary" },
  { name: "primary-foreground", var: "--primary-foreground" },
  { name: "secondary", var: "--secondary" },
  { name: "secondary-foreground", var: "--secondary-foreground" },
  { name: "muted", var: "--muted" },
  { name: "muted-foreground", var: "--muted-foreground" },
  { name: "accent", var: "--accent" },
  { name: "accent-foreground", var: "--accent-foreground" },
  { name: "destructive", var: "--destructive" },
  { name: "border", var: "--border" },
  { name: "input", var: "--input" },
  { name: "ring", var: "--ring" },
  { name: "sidebar", var: "--sidebar" },
  { name: "sidebar-foreground", var: "--sidebar-foreground" },
  { name: "sidebar-primary", var: "--sidebar-primary" },
  { name: "sidebar-primary-foreground", var: "--sidebar-primary-foreground" },
  { name: "sidebar-accent", var: "--sidebar-accent" },
  { name: "sidebar-accent-foreground", var: "--sidebar-accent-foreground" },
  { name: "sidebar-border", var: "--sidebar-border" },
  { name: "diff-red", var: "--diff-red" },
  { name: "diff-green", var: "--diff-green" },
  { name: "upvote", var: "--upvote" },
  { name: "downvote", var: "--downvote" },
  { name: "vote", var: "--vote" },
];

export default function ColorsPage() {
  return (
    <div className="p-8 bg-white min-h-screen">
      <h1 className="text-2xl font-bold mb-6 font-mono">Color Palette</h1>
      <div className="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4">
        {colors.map(({ name, var: cssVar }) => (
          <div key={name} className="flex flex-col gap-1">
            <div
              className="h-24 w-full rounded-lg border border-black/10"
              style={{ backgroundColor: `var(${cssVar})` }}
            />
            <span className="font-mono text-xs text-gray-700">{name}</span>
            <span className="font-mono text-xs text-gray-400">{cssVar}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
