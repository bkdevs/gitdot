import { ChevronDown, ChevronUp } from "lucide-react";
import { QuestionHeader } from "./ui/question-header";

export default function Page({ params }: { params: { id: string } }) {
  return (
    <div className="flex flex-col w-full">
      <QuestionHeader />
      <div className="flex px-6 py-4">
        <div className="flex flex-col items-center gap-0 text-muted-foreground text-xl">
          <button className="text-muted-foreground hover:text-foreground">
            <ChevronUp />
          </button>
          <span>42</span>
          <button className="text-muted-foreground hover:text-foreground">
            <ChevronDown />
          </button>
        </div>

        <div className="flex-1 ml-6 max-w-3xl">
          <h1 className="text-xl font-medium mb-2">
            How do I fix authentication bug in login flow?
          </h1>

          <div className="text-sm">
            <p className="mb-4">
              The authentication flow is broken when users try to log in with
              social providers. Getting a 401 error on callback. This started
              happening after we upgraded to the latest version of the OAuth
              library.
            </p>
            <p className="mb-4">
              I've tried clearing the session cookies and regenerating the
              client credentials, but the issue persists. The weird thing is
              that it works fine in development but fails in production.
            </p>
            <pre className="bg-muted p-3 rounded text-sm overflow-x-auto mb-4">
              Error: Invalid token received from OAuth provider at validateToken
            </pre>
            <p className="mb-4">
              Here's the relevant code from our callback handler:
            </p>
            <pre className="bg-muted p-3 rounded text-sm overflow-x-auto mb-4">{`export async function handleOAuthCallback(code: string) {
  const tokens = await oauth.exchangeCode(code);
  const user = await validateToken(tokens.access_token);
  return createSession(user);
}`}</pre>
            <p>
              Has anyone else encountered this? Any help would be appreciated.
            </p>
          </div>
          <div className="mt-8 mb-2 border border-border" />
          <div className="flex justify-between text-sm">
            <div className="flex gap-4">
              <span>
                <span className="text-muted-foreground">Asked</span> today
              </span>
              <span>
                <span className="text-muted-foreground">Modified</span> today
              </span>
              <span>
                <span className="text-muted-foreground">Viewed</span> 3 times
              </span>
            </div>
            <div className="flex gap-2 text-muted-foreground">
              <span className="underline">johndoe</span>
              <span>1,234</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
