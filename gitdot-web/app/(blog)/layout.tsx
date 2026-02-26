import { League_Spartan } from "next/font/google";

const league_spartan = League_Spartan({
  subsets: ["latin"],
  weight: ["400", "700"],
});

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className={`${league_spartan.className} blog-root`}>{children}</div>
  );
}
