import { League_Spartan } from "next/font/google";

const league_spartan = League_Spartan({
  subsets: ["latin"],
  weight: ["400", "700"],
});

export default function Releases() {
  return (
    <div
      className={`${league_spartan.className} blog-root flex flex-col gap-4`}
    >
      <h1 className="font-semibold text-lg">Releases</h1>
      <p>
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.
      </p>
      <p>
        Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut
        fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem
        sequi nesciunt.
      </p>
      <p>
        Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet,
        consectetur, adipisci velit, sed quia non numquam eius modi tempora
        incidunt ut labore et dolore magnam aliquam quaerat voluptatem.
      </p>
    </div>
  );
}
