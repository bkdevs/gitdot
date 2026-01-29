import type { Metadata } from "next";
import { IBM_Plex_Sans, Inconsolata, League_Spartan } from "next/font/google";
import { AppHeader } from "@/ui/app/app-header";
import { AppSidebar } from "@/ui/app/app-sidebar";
import "./globals.css";
import { getCurrentUser } from "./lib/dal";
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

const league_spartan = League_Spartan({
  subsets: ["latin"],
  weight: ["400", "700"],
  variable: "--font-league-spartan",
});

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const user = await getCurrentUser(); // TODO: does not re-render, fix with auth.

  return (
    <html
      lang="en"
      className={`${ibm_plex_sans.variable} ${inconsolata.variable} ${league_spartan.variable} overscroll-none`}
    >
      <body>
        <Providers user={user}>
          <div className="flex flex-row h-screen w-full max-w-screen overflow-hidden">
            <div className="hidden md:flex h-full shrink-0">
              <AppSidebar />
            </div>
            <div className="flex flex-col flex-1 min-w-0">
              <AppHeader />
              <main className="flex-1 min-h-0 overflow-auto">{children}</main>
            </div>
          </div>
        </Providers>
      </body>
    </html>
  );
}
