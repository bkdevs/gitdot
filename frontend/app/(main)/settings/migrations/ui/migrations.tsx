import { Plus } from "lucide-react";

import Link from "@/ui/link";

export function Migrations() {
  return (
    <div className="flex flex-col w-full">
      <MigrationsHeader />
      <p className="px-2 py-3 text-sm text-muted-foreground">No migrations.</p>
    </div>
  );
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
