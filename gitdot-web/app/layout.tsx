import type { Metadata } from "next";
import { IBM_Plex_Sans, Inconsolata } from "next/font/google";
import { ThemeProvider } from "next-themes";
import { MetricsProvider } from "./context/metrics";
import "./globals.css";
import { TooltipProvider } from "./ui/tooltip";

export const metadata: Metadata = {
  title: "gitdot",
  description: "A better open-source GitHub",
  icons: {
    icon: "/favicon.ico",
  },
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
      className={`${ibm_plex_sans.variable} ${inconsolata.variable} overscroll-none`}
      suppressHydrationWarning
    >
      <body>
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
        >
          <MetricsProvider>
            <TooltipProvider delayDuration={0}>{children}</TooltipProvider>
          </MetricsProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
