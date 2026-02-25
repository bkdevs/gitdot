import { getCurrentUser, listMigrations } from "@/lib/dal";
import { Migrations } from "./ui/migrations";

export default async function Page() {
  const user = await getCurrentUser();
  if (!user) return null;

  const migrations = await listMigrations();

  return (
    <div className="flex flex-col w-full">
      <Migrations migrations={migrations ?? []} />
    </div>
  );
}
