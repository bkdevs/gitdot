import type { Metadata } from "next";
import { DatabaseProvider } from "./context/database";
import { ShortcutsProvider } from "./context/shortcuts";
import { UserProvider } from "./context/user";
import { WorkerProvider } from "./context/worker";
import { MainFooter } from "./ui/main-footer";

export const metadata: Metadata = {
  title: "gitdot",
  description: "A better open-source GitHub",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <DatabaseProvider>
      <WorkerProvider>
        <UserProvider>
          <ShortcutsProvider>
            <div className="flex flex-col h-screen w-full max-w-screen overflow-hidden">
              <main className="flex-1 min-h-0 overflow-y-auto scrollbar-overlay">
                {children}
              </main>
              <MainFooter />
            </div>
          </ShortcutsProvider>
        </UserProvider>
      </WorkerProvider>
    </DatabaseProvider>
  );
}
