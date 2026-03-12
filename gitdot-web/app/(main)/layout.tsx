import type { Metadata } from "next";
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
    <WorkerProvider>
      <UserProvider>
        <div className="flex flex-col h-screen w-full max-w-screen overflow-hidden">
          <main className="flex-1 min-h-0 overflow-auto">{children}</main>
          <MainFooter />
        </div>
      </UserProvider>
    </WorkerProvider>
  );
}
