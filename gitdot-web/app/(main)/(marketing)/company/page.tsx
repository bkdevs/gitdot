import { League_Spartan } from "next/font/google";

const league_spartan = League_Spartan({
  subsets: ["latin"],
  weight: ["400", "700"],
});

export default function Company() {
  return (
    <div
      className={`${league_spartan.className} blog-root flex flex-col gap-4`}
    >
      <h1 className="font-semibold text-lg">Company</h1>
      <p>
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.
      </p>
      <p>
        At vero eos et accusamus et iusto odio dignissimos ducimus qui
        blanditiis praesentium voluptatum deleniti atque corrupti quos dolores
        et quas molestias excepturi sint occaecati cupiditate non provident.
      </p>
      <p>
        Similique sunt in culpa qui officia deserunt mollitia animi, id est
        laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita
        distinctio.
      </p>
    </div>
  );
}
