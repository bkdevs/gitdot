import { redirect } from "next/navigation";
import { createInstallation, listInstallationRepositories } from "@/lib/dal";
import Link from "@/ui/link";

export default async function Page({
  searchParams,
}: {
  searchParams: Promise<Record<string, string | string[] | undefined>>;
}) {
  const params = await searchParams;
  const installationId = params.installation_id;

  if (!installationId || typeof installationId !== "string") {
    redirect("/onboarding/github");
  }

  const installation = await createInstallation(Number(installationId));

  if (!installation) {
    redirect("/onboarding/github");
  }

  const repositories = await listInstallationRepositories(
    installation.installation_id,
  );

  return (
    <div className="max-w-3xl mx-auto flex gap-4 items-center justify-center h-screen">
      <div className="flex flex-col text-sm w-sm">
        <p className="pb-2">Repositories.</p>
        <p className="text-primary/60 pb-4">
          Select the repositories you want to migrate to gitdot.
        </p>

        {repositories && repositories.length > 0 ? (
          <ul className="flex flex-col border border-border divide-y divide-border">
            {repositories.map((repo) => (
              <li key={repo.id} className="flex items-center gap-3 px-3 py-2">
                <span className="flex-1 truncate">{repo.full_name}</span>
                {repo.private && (
                  <span className="text-xs text-primary/40">private</span>
                )}
              </li>
            ))}
          </ul>
        ) : (
          <p className="text-primary/60">No repositories found.</p>
        )}

        <div className="flex justify-end mt-4">
          <Link href="/home" className="decoration-primary/40">
            Continue.
          </Link>
        </div>
      </div>
    </div>
  );
}
