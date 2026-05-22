import { League_Spartan } from "next/font/google";

const league_spartan = League_Spartan({
  subsets: ["latin"],
  weight: ["400", "700"],
});

export default function Terms() {
  return (
    <div
      className={`${league_spartan.className} blog-root flex flex-col gap-4`}
    >
      <h1 className="font-semibold text-lg">Terms</h1>
      <p>
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.
      </p>
      <p>
        Temporibus autem quibusdam et aut officiis debitis aut rerum
        necessitatibus saepe eveniet ut et voluptates repudiandae sint et
        molestiae non recusandae.
      </p>
      <p>
        Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis
        voluptatibus maiores alias consequatur aut perferendis doloribus
        asperiores repellat.
      </p>
    </div>
  );
}
