"use client";

import type { MigrationResource } from "gitdot-api";
import { ChevronRight } from "lucide-react";
import { useEffect, useState } from "react";
import { listMigrationsAction } from "@/actions";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/ui/collapsible";
import { formatDate } from "@/util";

export function SettingsMigrations() {
  const [migrations, setMigrations] = useState<
    MigrationResource[] | null | undefined
  >(undefined);

  useEffect(() => {
    let cancelled = false;
    listMigrationsAction().then((result) => {
      if (!cancelled) setMigrations(result);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <div className="p-3">
      <div className="flex items-center gap-2">
        <p className="text-sm font-medium">Migrations</p>
      </div>
      <p className="text-sm text-muted-foreground">
        Each migration brings repositories from a connected GitHub account into
        gitdot, preserving their full history. Past migrations and the
        repositories they covered are listed below.
      </p>
      {migrations === undefined ? (
        <div className="mt-3 text-sm text-muted-foreground">loading...</div>
      ) : (
        <div className="mt-3">
          {!migrations || migrations.length === 0 ? (
            <div className="border border-border rounded p-3 text-sm text-muted-foreground">
              no migrations yet
            </div>
          ) : (
            <ul>
              {migrations.map((migration) => (
                <li key={migration.id}>
                  <Collapsible>
                    <CollapsibleTrigger className="group flex items-center gap-2 w-full text-sm border-b border-border hover:border-foreground transition-all duration-200 cursor-pointer py-0.4 text-left">
                      <ChevronRight className="size-3 text-muted-foreground group-data-[state=open]:rotate-90" />
                      <span className="text-muted-foreground">
                        {formatDate(new Date(migration.created_at))}:
                      </span>
                      <span className="flex-1 truncate">
                        {migration.origin} → {migration.destination} (
                        {migration.repositories.length} repo
                        {migration.repositories.length !== 1 && "s"})
                      </span>
                      <MigrationStatusLabel status={migration.status} />
                    </CollapsibleTrigger>
                    <CollapsibleContent>
                      <ul className="pt-1 pb-2">
                        {migration.repositories.length === 0 ? (
                          <li className="pl-5 text-sm text-muted-foreground py-0.4">
                            no repositories
                          </li>
                        ) : (
                          migration.repositories.map((repo) => (
                            <li
                              key={repo.id}
                              className="pl-5 text-sm py-0.4 truncate"
                            >
                              {repo.origin_full_name} →{" "}
                              {repo.destination_full_name}
                            </li>
                          ))
                        )}
                      </ul>
                    </CollapsibleContent>
                  </Collapsible>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}
    </div>
  );
}

function MigrationStatusLabel({ status }: { status: string }) {
  switch (status) {
    case "completed":
      return (
        <span className="font-mono text-xs text-green-500">completed</span>
      );
    case "running":
      return <span className="font-mono text-xs text-yellow-500">running</span>;
    case "failed":
      return <span className="font-mono text-xs text-destructive">failed</span>;
    case "pending":
      return (
        <span className="font-mono text-xs text-muted-foreground">pending</span>
      );
    default:
      return (
        <span className="font-mono text-xs text-muted-foreground">
          {status}
        </span>
      );
  }
}
