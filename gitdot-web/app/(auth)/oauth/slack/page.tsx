import ConnectSlackForm from "../../ui/connect-slack-form";

export default async function Page({
  searchParams,
}: {
  searchParams: Promise<{ state?: string }>;
}) {
  const { state } = await searchParams;

  return (
    <div className="max-w-3xl mx-auto flex gap-4 items-center justify-center h-screen">
      <ConnectSlackForm state={state} />
    </div>
  );
}
