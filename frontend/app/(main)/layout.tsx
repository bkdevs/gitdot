import type { Metadata } from "next";
import { getCurrentUser } from "@/lib/dal";
import { Providers } from "./providers";
import { MainHeader } from "./ui/main-header";
import { MainSidebar } from "./ui/main-sidebar";

export const metadata: Metadata = {
  title: "gitdot",
  description: "A better open-source GitHub",
};

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const user = await getCurrentUser(); // TODO: does not re-render, fix with auth.

  return (
    <Providers user={user}>
      <div className="flex flex-row h-screen w-full max-w-screen overflow-hidden">
        <div className="hidden md:flex h-full shrink-0">
          <MainSidebar />
        </div>
        <div className="flex flex-col flex-1 min-w-0">
          <MainHeader />
          <main className="flex-1 min-h-0 overflow-auto">{children}</main>
        </div>
      </div>
    </Providers>
  );
}
