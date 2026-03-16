import { ApiProvider } from "@/provider/api";
import { CommitsClient } from "./context";
import { Resources } from "./resources";

export default async function Layout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const provider = new ApiProvider(owner, repo);
  const serverPromises = provider.fetch(Resources);
  return (
    <CommitsClient owner={owner} repo={repo} serverPromises={serverPromises}>
      {children}
    </CommitsClient>
  );
}
