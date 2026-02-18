import Link from "@/ui/link";

export default function Page() {
  return (
    <div className="p-4">
      <div className="flex items-center gap-4 mb-4">
        <h1 className="text-lg font-medium">Runners</h1>
        <Link
          href="/settings/runners/new"
          className="text-sm text-primary hover:underline"
          prefetch={true}
        >
          Register new runner
        </Link>
      </div>
      <p className="text-sm text-muted-foreground">To come.</p>
    </div>
  );
}
