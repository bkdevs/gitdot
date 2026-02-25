import { Plus } from "lucide-react";

import type { MigrationListResponse } from "@/lib/dto/migration";
import Link from "@/ui/link";
import { formatDate } from "@/util";

export function Migrations({
  migrations,
}: {
  migrations: MigrationListResponse;
}) {
  return (
    <div className="flex flex-col w-full">
      <MigrationsHeader />
      {migrations.map((migration) => (
        <div
          key={migration.id}
          className="flex flex-row items-center px-2 py-2 border-b select-none"
        >
          <div className="flex flex-col">
            <div className="flex flex-row text-sm">
              {migration.origin} &middot; {migration.repositories.length} repo
              {migration.repositories.length !== 1 && "s"}
            </div>
            <div className="flex flex-row text-xs text-muted-foreground pt-0.5">
              <MigrationStatus status={migration.status} /> &middot;{" "}
              {formatDate(new Date(migration.created_at))}
            </div>
          </div>
        </div>
      ))}
      {migrations.length === 0 && (
        <p className="px-2 py-3 text-sm text-muted-foreground">
          No migrations.
        </p>
      )}
    </div>
  );
}

function MigrationStatus({ status }: { status: string }) {
  switch (status) {
    case "pending":
      return <span className="text-muted-foreground">Pending</span>;
    case "running":
      return <span className="text-yellow-500">Running</span>;
    case "completed":
      return <span className="text-green-500">Completed</span>;
    case "failed":
      return <span className="text-destructive">Failed</span>;
    default:
      return <span>{status}</span>;
  }
}

function MigrationsHeader() {
  return (
    <div className="flex items-center justify-between border-b pl-2 h-9">
      <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        Migrations
      </h3>
      <Link
        href="/settings/migrations/new"
        className="flex flex-row h-full items-center px-2 border-border border-l bg-primary text-xs text-primary-foreground hover:bg-primary/80 outline-0! ring-0!"
        prefetch={true}
      >
        <Plus className="size-3 mr-1.5" />
        New migration
      </Link>
    </div>
  );
}
