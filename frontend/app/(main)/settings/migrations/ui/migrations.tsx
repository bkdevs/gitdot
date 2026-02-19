export function Migrations() {
  return (
    <div className="flex flex-col w-full">
      <MigrationsHeader />
      <p className="px-2 py-3 text-sm text-muted-foreground">
        No migrations.
      </p>
    </div>
  );
}

function MigrationsHeader() {
  return (
    <div className="flex items-center justify-between border-b pl-2 h-9">
      <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        Migrations
      </h3>
    </div>
  );
}
