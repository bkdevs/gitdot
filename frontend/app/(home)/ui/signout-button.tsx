import { signout } from "@/actions";
import { Button } from "@/ui/button";

export default function SignoutButton() {
  return (
    <Button variant="outline" onClick={signout}>
      Sign Out
    </Button>
  );
}
