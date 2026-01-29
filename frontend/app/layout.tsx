import type { Metadata } from "next";
import { IBM_Plex_Sans, Inconsolata, League_Spartan } from "next/font/google";
import "./globals.css";

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

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html
      lang="en"
      className={`${ibm_plex_sans.variable} ${inconsolata.variable} ${league_spartan.variable} overscroll-none`}
    >
      <body>{children}</body>
    </html>
  );
}
