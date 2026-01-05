import { RepoSidebar } from "./ui/repo-sidebar";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <div className="flex min-h-svh w-full">
      <RepoSidebar />
      <main className="flex-1 w-full">{children}</main>
    </div>
  );
}
