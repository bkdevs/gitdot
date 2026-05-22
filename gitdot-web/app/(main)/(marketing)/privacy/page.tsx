import { League_Spartan } from "next/font/google";

const league_spartan = League_Spartan({
  subsets: ["latin"],
  weight: ["400", "700"],
});

export default function Privacy() {
  return (
    <div
      className={`${league_spartan.className} blog-root flex flex-col gap-4`}
    >
      <h1 className="font-semibold text-lg">Privacy</h1>
      <p>
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.
      </p>
      <p>
        On the other hand, we denounce with righteous indignation and dislike
        men who are so beguiled and demoralized by the charms of pleasure of the
        moment, so blinded by desire, that they cannot foresee the pain and
        trouble that are bound to ensue.
      </p>
      <p>
        Nor again is there anyone who loves or pursues or desires to obtain pain
        of itself, because it is pain, but because occasionally circumstances
        occur in which toil and pain can procure him some great pleasure.
      </p>
    </div>
  );
}
