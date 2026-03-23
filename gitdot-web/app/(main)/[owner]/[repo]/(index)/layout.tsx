import { getUserMetadata } from "@/lib/supabase";
import { OverlayScroll } from "../../../../ui/scroll";
import { RepoSidebar } from "./ui/sidebar";

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
    <>
      <RepoSidebar owner={owner} repo={repo} showSettings={isAdmin} />
      <OverlayScroll>{children}</OverlayScroll>
    </>
  );
}
