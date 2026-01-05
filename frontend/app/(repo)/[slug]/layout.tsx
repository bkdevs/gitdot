import { RepoSidebar } from "./ui/repo-sidebar";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <div
      className="flex min-h-svh w-full"
      style={{
        "--sidebar-width": "12.5rem",
        "--sidebar-width-icon": "2.25rem",
      } as React.CSSProperties}
    >
      <RepoSidebar />
      <main className="flex-1 w-full">{children}</main>
    </div>
  );
}
