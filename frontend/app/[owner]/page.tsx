export default async function RepoPage({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;
  return <div>The {owner} home page!</div>;
}
