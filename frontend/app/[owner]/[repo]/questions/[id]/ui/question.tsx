import { Comments } from "./comments";
import { VoteBox } from "./vote-box";

export function Question() {
  return (
    <div className="flex pl-8 py-4 max-w-4xl">
      <VoteBox score={28} />
      <div className="flex-1 ml-8">
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
            I've tried clearing the session cookies and regenerating the client
            credentials, but the issue persists. The weird thing is that it
            works fine in development but fails in production.
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
        <div className="flex mt-8 justify-between text-sm">
          <div className="flex gap-4">
            <span>
              <span className="text-muted-foreground">Asked</span> today
            </span>
            <span>
              <span className="text-muted-foreground">Modified</span> today
            </span>
          </div>
          <div className="flex gap-2 text-muted-foreground">
            <span className="underline">johndoe</span>
            <span>1,234</span>
          </div>
        </div>

        <div className="mt-2 mb-2 border border-gray-100" />
        <Comments />
      </div>
    </div>
  );
}
