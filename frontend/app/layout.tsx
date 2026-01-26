import type { Metadata } from "next";
import { IBM_Plex_Sans, Inconsolata } from "next/font/google";
import { AppHeader } from "@/ui/app/app-header";
import { AppSidebar } from "@/ui/app/app-sidebar";
import "./globals.css";
import { Providers } from "./providers";

export const metadata: Metadata = {
  title: "gitdot",
  description: "A better open-source GitHub",
};

const ibm_plex_sans = IBM_Plex_Sans({
  subsets: ["latin"],
  variable: "--font-ibm-plex-sans",
});

const inconsolata = Inconsolata({
  subsets: ["latin"],
  variable: "--font-inconsolata",
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html
      lang="en"
      className={`${ibm_plex_sans.variable} ${inconsolata.variable}`}
    >
      <body>
        <Providers>
          <div className="flex flex-col h-screen w-full max-w-screen overflow-hidden">
            <AppHeader />
            <div className="flex flex-1 min-h-0">
              <div className="hidden md:flex h-full shrink-0">
                <AppSidebar />
              </div>
              <main className="flex-1 min-w-0 overflow-auto">{children}</main>
            </div>
          </div>
        </Providers>
      </body>
    </html>
  );
}
