import { getUserMetadata } from "@/lib/auth";
import { LayoutClient } from "./layout.client";

export default async function Layout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const { username, orgs } = await getUserMetadata();
  const isAdmin = username === owner || orgs.includes(`${owner}:admin`);

  return (
    <LayoutClient owner={owner} repo={repo} showSettings={isAdmin}>
      {children}
    </LayoutClient>
  );
}
