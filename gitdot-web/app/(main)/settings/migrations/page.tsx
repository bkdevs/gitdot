import { getCurrentUser, listMigrations } from "@/dal";
import { Migrations } from "./ui/migrations";

export default async function Page() {
  const current = await getCurrentUser();
  if (!current) return null;

  const migrations = await listMigrations();

  return (
    <div className="flex flex-col w-full">
      <Migrations migrations={migrations ?? []} />
    </div>
  );
}
