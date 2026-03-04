import type { Metadata } from "next";
import { Providers } from "./providers";
import { MainHeader } from "./ui/main-header";

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
    <Providers>
      <div className="flex flex-col h-screen w-full max-w-screen overflow-hidden">
        <MainHeader />
        <main className="flex-1 min-h-0 overflow-auto">{children}</main>
      </div>
    </Providers>
  );
}
