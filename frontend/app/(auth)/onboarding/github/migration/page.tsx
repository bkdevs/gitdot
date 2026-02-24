import { redirect } from "next/navigation";
import { createInstallation } from "@/lib/dal";

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

  return (
    <div className="max-w-3xl mx-auto flex gap-4 items-center justify-center h-screen">
      <div className="flex flex-col text-sm w-sm">
        <p className="pb-2">Migration.</p>
        <pre className="text-xs text-primary/60 whitespace-pre-wrap break-all">
          {JSON.stringify(installation, null, 2)}
        </pre>
      </div>
    </div>
  );
}
