import { SidebarProvider } from "@/ui/sidebar";
import { RepoSidebar } from "./ui/repo-sidebar";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <SidebarProvider>
      <RepoSidebar />
      <main className="flex-1 w-full">{children}</main>
    </SidebarProvider>
  );
}
