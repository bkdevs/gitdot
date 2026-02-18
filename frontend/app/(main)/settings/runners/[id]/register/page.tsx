import Link from "@/ui/link";

export default async function Page({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;

  return (
    <div className="p-4">
      <h1 className="text-lg font-medium border-b border-border pb-2 mb-4">
        Register runner
      </h1>
      <p className="text-sm text-muted-foreground mb-4">
        Runner {id} — registration instructions to come.
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
