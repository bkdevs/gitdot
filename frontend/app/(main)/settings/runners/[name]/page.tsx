import { notFound } from "next/navigation";

import { getCurrentUser, getRunner } from "@/lib/dal";
import Link from "@/ui/link";

export default async function Page({
  params,
}: {
  params: Promise<{ name: string }>;
}) {
  const { name } = await params;

  const user = await getCurrentUser();
  if (!user) notFound();

  const runner = await getRunner(name, {
    owner_name: user.name,
    owner_type: "user",
  });
  if (!runner) notFound();

  return (
    <div className="p-4">
      <h1 className="text-lg font-medium border-b border-border pb-2 mb-4">
        Register runner
      </h1>
      <p className="text-sm text-muted-foreground mb-4">
        Runner <span className="font-mono">{runner.name}</span> — registration
        instructions to come.
      </p>
      <Link
        href="/settings/runners"
        className="text-sm text-primary hover:underline"
        prefetch={true}
      >
        Back to runners
      </Link>
    </div>
  );
}
