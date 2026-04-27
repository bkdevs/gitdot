import type { Metadata } from "next";
import { DatabaseProvider } from "./context/database";
import { HistoryProvider } from "./context/history";
import { SettingsProvider } from "./context/settings";
import { ShortcutsProvider } from "./context/shortcuts";
import { ToasterProvider } from "./context/toaster";
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
            <SettingsProvider>
              <HistoryProvider>
                <ToasterProvider>
                  <div className="flex flex-col h-screen w-full max-w-screen overflow-hidden">
                    <main className="flex-1 min-h-0 overflow-hidden">
                      {children}
                    </main>
                    <MainFooter />
                  </div>
                </ToasterProvider>
              </HistoryProvider>
            </SettingsProvider>
          </ShortcutsProvider>
        </UserProvider>
      </WorkerProvider>
    </DatabaseProvider>
  );
}
