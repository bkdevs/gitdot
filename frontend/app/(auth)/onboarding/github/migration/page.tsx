export default async function Page({
  searchParams,
}: {
  searchParams: Promise<Record<string, string | string[] | undefined>>;
}) {
  const params = await searchParams;

  return (
    <div className="max-w-3xl mx-auto flex gap-4 items-center justify-center h-screen">
      <div className="flex flex-col text-sm w-sm">
        <p className="pb-2">Migration.</p>
        <pre className="text-xs text-primary/60 whitespace-pre-wrap break-all">
          {JSON.stringify(params, null, 2)}
        </pre>
      </div>
    </div>
  );
}
