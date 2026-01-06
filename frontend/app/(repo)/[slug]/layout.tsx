import { getRepositoryTree } from "@/lib/dal";
import { RepoSidebar } from "./ui/repo-sidebar";

export default async function Layout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: Promise<{ slug: string }>;
}>) {
  const { slug } = await params;
  const tree = await getRepositoryTree("bkdevs", slug);

  if (!tree) {
    return null;
  }
  return (
    <div className="flex min-h-svh w-full">
      <RepoSidebar repo={slug} tree={tree} />
      <main className="flex-1 w-full">{children}</main>
    </div>
  );
}
