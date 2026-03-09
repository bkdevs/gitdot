import type { Metadata } from "next";
import { MainProvider } from "./provider";
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
    <MainProvider>
      <div className="flex flex-col h-screen w-full max-w-screen overflow-hidden">
        <main className="flex-1 min-h-0 overflow-auto">{children}</main>
        <MainFooter />
      </div>
    </MainProvider>
  );
}
