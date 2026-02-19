import { notFound } from "next/navigation";
import { getCurrentUser, listUserOrganizations, NotFound } from "@/lib/dal";
import { CreateMigrationForm } from "../ui/create-migration-form";
import { CreateMigrationInstructions } from "../ui/create-migration-instructions";

export default async function Page() {
  const user = await getCurrentUser();
  if (!user) notFound();

  const orgs = await listUserOrganizations(user.name);
  const organizations = orgs && orgs !== NotFound ? orgs : [];

  return (
    <div className="flex p-4">
      <div className="min-w-0 flex-1 max-w-3xl">
        <CreateMigrationForm user={user} organizations={organizations} />
      </div>
      <aside className="w-72 shrink-0 hidden md:block border-l border-border ml-6 pl-6">
        <CreateMigrationInstructions />
      </aside>
    </div>
  );
}
