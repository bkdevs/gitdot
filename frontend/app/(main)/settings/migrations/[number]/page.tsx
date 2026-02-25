import { notFound } from "next/navigation";

import { getCurrentUser, getMigration } from "@/lib/dal";
import { formatDate } from "@/util";

export default async function Page({
  params,
}: {
  params: Promise<{ number: string }>;
}) {
  const user = await getCurrentUser();
  if (!user) notFound();

  const { number } = await params;
  const migration = await getMigration(Number(number));
  if (!migration) notFound();

  return (
    <div className="flex flex-col w-full">
      <div className="flex items-center border-b pl-2 h-9">
        <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
          Migration #{migration.number}
        </h3>
      </div>
      <div className="flex flex-col gap-3 p-4">
        <div className="flex flex-row gap-4 text-sm">
          <div>
            <span className="text-muted-foreground">Origin:</span>{" "}
            {migration.origin}
          </div>
          <div>
            <span className="text-muted-foreground">Destination:</span>{" "}
            {migration.destination}
          </div>
          <div>
            <span className="text-muted-foreground">Status:</span>{" "}
            <MigrationStatus status={migration.status} />
          </div>
          <div>
            <span className="text-muted-foreground">Created:</span>{" "}
            {formatDate(new Date(migration.created_at))}
          </div>
        </div>
        <div className="flex flex-col border rounded">
          <div className="flex items-center px-2 h-8 border-b bg-muted/50">
            <h4 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
              Repositories
            </h4>
          </div>
          {migration.repositories.map((repo) => (
            <div
              key={repo.id}
              className="flex flex-row items-center justify-between px-2 py-2 border-b last:border-b-0 text-sm"
            >
              <span>
                {repo.origin_full_name} &rarr; {repo.destination_full_name} ({repo.visibility})
              </span>
              <RepositoryStatus status={repo.status} error={repo.error} />
            </div>
          ))}
          {migration.repositories.length === 0 && (
            <p className="px-2 py-3 text-sm text-muted-foreground">
              No repositories.
            </p>
          )}
        </div>
      </div>
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

function RepositoryStatus({
  status,
  error,
}: {
  status: string;
  error: string | null;
}) {
  switch (status) {
    case "pending":
      return <span className="text-xs text-muted-foreground">Pending</span>;
    case "running":
      return <span className="text-xs text-yellow-500">Running</span>;
    case "completed":
      return <span className="text-xs text-green-500">Completed</span>;
    case "failed":
      return (
        <span className="text-xs text-destructive" title={error ?? undefined}>
          Failed
        </span>
      );
    default:
      return <span className="text-xs">{status}</span>;
  }
}
