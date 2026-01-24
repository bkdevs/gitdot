"use client";

import { ChevronDown, ChevronUp } from "lucide-react";
import { useState } from "react";
import { QuestionHeader } from "./ui/question-header";

export default function Page({ params }: { params: { id: string } }) {
  const [showCommentInput, setShowCommentInput] = useState(false);
  return (
    <div className="flex flex-col w-full">
      <QuestionHeader />
      <div className="flex pl-8 py-4 max-w-4xl">
        <div className="flex flex-col items-center gap-0 text-muted-foreground text-xl">
          <button className="text-muted-foreground hover:text-foreground">
            <ChevronUp />
          </button>
          <span>42</span>
          <button className="text-muted-foreground hover:text-foreground">
            <ChevronDown />
          </button>
        </div>

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

          {/* Comments */}
          <div className="flex flex-col gap-2 text-xs">
            <div className="flex items-start gap-1">
              <span className="w-4 text-right text-muted-foreground">5</span>
              <button className="text-muted-foreground hover:text-foreground">
                <ChevronUp className="w-4 h-4" />
              </button>
              <p className="flex-1">
                Have you checked if the OAuth provider changed their API
                recently? I remember seeing a similar issue last month when they
                updated their token validation endpoint and changed the response
                format.
              </p>
              <span className="text-muted-foreground shrink-0">
                <span className="underline">devuser123</span> 2 hours ago
              </span>
            </div>
            <div className="flex items-start gap-1">
              <span className="w-4 text-right text-muted-foreground">3</span>
              <button className="text-muted-foreground hover:text-foreground">
                <ChevronUp className="w-4 h-4" />
              </button>
              <p className="flex-1">
                @devuser123 yes, that was my first thought too. The changelog
                doesn't mention any breaking changes though.
              </p>
              <span className="text-muted-foreground shrink-0">
                <span className="underline">johndoe</span> 1 hour ago
              </span>
            </div>
            <div className="flex items-start gap-1">
              <span className="w-4 text-right text-muted-foreground">1</span>
              <button className="text-muted-foreground hover:text-foreground">
                <ChevronUp className="w-4 h-4" />
              </button>
              <p className="flex-1">
                Can you share your environment variables (redacted)? Might be a
                config issue.
              </p>
              <span className="text-muted-foreground shrink-0">
                <span className="underline">helpfuldev</span> 45 min ago
              </span>
            </div>
            {showCommentInput ? (
              <input
                type="text"
                placeholder="Write a comment..."
                className="w-full mt-2 px-2 py-1 text-xs border border-border rounded bg-transparent"
                autoFocus
                onBlur={() => setShowCommentInput(false)}
              />
            ) : (
              <button
                onClick={() => setShowCommentInput(true)}
                className="text-xs text-muted-foreground hover:text-foreground mt-2 block text-left"
              >
                Add a comment...
              </button>
            )}
          </div>
        </div>
      </div>

      <div className="flex justify-between items-center pl-4 py-8 max-w-4xl">
        <span className="text-lg font-medium">2 Answers</span>
        <div className="flex items-center gap-2 text-sm">
          <span className="text-muted-foreground">Sorted by:</span>
          <select className="bg-transparent border border-border rounded px-2 py-1">
            <option>Highest score</option>
            <option>Date modified</option>
            <option>Date created</option>
          </select>
        </div>
      </div>

      <div className="flex flex-col gap-16">
        {/* Answer 1 */}
        <div className="flex pl-8 max-w-4xl">
          <div className="flex flex-col items-center gap-0 text-muted-foreground text-xl">
            <button className="text-muted-foreground hover:text-foreground">
              <ChevronUp />
            </button>
            <span>28</span>
            <button className="text-muted-foreground hover:text-foreground">
              <ChevronDown />
            </button>
          </div>

          <div className="flex-1 ml-8">
            <div className="text-sm">
              <p className="mb-4">
                This is a known issue with the latest OAuth library update. The
                token validation endpoint changed their response format.
              </p>
              <p className="mb-4">
                You need to update your validateToken function to handle the new
                response structure:
              </p>
              <pre className="bg-muted p-3 rounded text-sm overflow-x-auto mb-4">{`export async function validateToken(token: string) {
  const response = await fetch('/api/validate', {
    headers: { Authorization: \`Bearer \${token}\` }
  });
  const data = await response.json();
  // New format wraps user in a 'data' object
  return data.data.user;
}`}</pre>
              <p>
                This fixed the issue for me in production. Make sure to also
                clear your Redis cache if you're caching tokens.
              </p>
            </div>
            <div className="flex justify-between text-sm mt-8">
              <div className="flex gap-4">
                <span>
                  <span className="text-muted-foreground">Answered</span> 2
                  hours ago
                </span>
              </div>
              <div className="flex gap-2 text-muted-foreground">
                <span className="underline">janedoe</span>
                <span>5,678</span>
              </div>
            </div>

            <div className="mt-2 mb-2 border border-gray-100" />

            {/* Comments */}
            <div className="flex flex-col gap-2 text-xs">
              <div className="flex items-start gap-1">
                <span className="w-4 text-right text-muted-foreground">8</span>
                <button className="text-muted-foreground hover:text-foreground">
                  <ChevronUp className="w-4 h-4" />
                </button>
                <p className="flex-1">
                  This worked perfectly! The data.data.user change was exactly
                  what I needed.
                </p>
                <span className="text-muted-foreground shrink-0">
                  <span className="underline">johndoe</span> 1 hour ago
                </span>
              </div>
              <div className="flex items-start gap-1">
                <span className="w-4 text-right text-muted-foreground">2</span>
                <button className="text-muted-foreground hover:text-foreground">
                  <ChevronUp className="w-4 h-4" />
                </button>
                <p className="flex-1">
                  Don't forget to also update the types if you're using
                  TypeScript!
                </p>
                <span className="text-muted-foreground shrink-0">
                  <span className="underline">tsdev</span> 30 min ago
                </span>
              </div>
            </div>
          </div>
        </div>

        {/* Answer 2 */}
        <div className="flex pl-8 py-4 max-w-4xl">
          <div className="flex flex-col items-center gap-0 text-muted-foreground text-xl">
            <button className="text-muted-foreground hover:text-foreground">
              <ChevronUp />
            </button>
            <span>12</span>
            <button className="text-muted-foreground hover:text-foreground">
              <ChevronDown />
            </button>
          </div>

          <div className="flex-1 ml-8">
            <div className="text-sm">
              <p className="mb-4">
                I had the same problem. In my case, it was a CORS issue in
                production. The OAuth callback URL was being blocked.
              </p>
              <p>
                Check your CORS configuration and make sure your production
                domain is whitelisted in both your OAuth provider settings and
                your server configuration.
              </p>
            </div>
            <div className="flex justify-between text-sm mt-8">
              <div className="flex gap-4">
                <span>
                  <span className="text-muted-foreground">Answered</span> 1 hour
                  ago
                </span>
              </div>
              <div className="flex gap-2 text-muted-foreground">
                <span className="underline">bobsmith</span>
                <span>892</span>
              </div>
            </div>

            <div className="mt-2 mb-2 border border-gray-100" />
          </div>
        </div>
      </div>
    </div>
  );
}
