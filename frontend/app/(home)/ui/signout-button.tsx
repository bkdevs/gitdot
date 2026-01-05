import { signout } from "@/actions";

export default function SignoutButton() {
  return (
    <button type="button" onClick={signout}>
      Sign Out
    </button>
  );
}
