export default function Page() {
  const mockDiffs = [
    {
      file: "src/components/Button.tsx",
      hunks: [
        {
          oldStart: 10,
          newStart: 10,
          lines: [
            { type: "context", oldNum: 10, newNum: 10, content: "export function Button({ children, onClick }: ButtonProps) {" },
            { type: "context", oldNum: 11, newNum: 11, content: "  return (" },
            { type: "removed", oldNum: 12, newNum: null, content: "    <button className=\"px-4 py-2 bg-blue-500 text-white\" onClick={onClick}>" },
            { type: "added", oldNum: null, newNum: 12, content: "    <button className=\"px-4 py-2 bg-blue-600 text-white rounded\" onClick={onClick}>" },
            { type: "context", oldNum: 13, newNum: 13, content: "      {children}" },
            { type: "context", oldNum: 14, newNum: 14, content: "    </button>" },
          ]
        }
      ]
    },
    {
      file: "src/utils/format.ts",
      hunks: [
        {
          oldStart: 5,
          newStart: 5,
          lines: [
            { type: "context", oldNum: 5, newNum: 5, content: "export function formatDate(date: Date): string {" },
            { type: "removed", oldNum: 6, newNum: null, content: "  return date.toLocaleDateString();" },
            { type: "added", oldNum: null, newNum: 6, content: "  return date.toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });" },
            { type: "context", oldNum: 7, newNum: 7, content: "}" },
          ]
        },
        {
          oldStart: 15,
          newStart: 15,
          lines: [
            { type: "context", oldNum: 15, newNum: 15, content: "export function formatTime(date: Date): string {" },
            { type: "added", oldNum: null, newNum: 16, content: "  const hours = date.getHours();" },
            { type: "added", oldNum: null, newNum: 17, content: "  const minutes = date.getMinutes();" },
            { type: "removed", oldNum: 16, newNum: null, content: "  return date.toLocaleTimeString();" },
            { type: "added", oldNum: null, newNum: 18, content: "  return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}`;" },
            { type: "context", oldNum: 17, newNum: 19, content: "}" },
          ]
        }
      ]
    },
    {
      file: "src/api/client.ts",
      hunks: [
        {
          oldStart: 1,
          newStart: 1,
          lines: [
            { type: "context", oldNum: 1, newNum: 1, content: "import axios from 'axios';" },
            { type: "added", oldNum: null, newNum: 2, content: "import { getToken } from './auth';" },
            { type: "context", oldNum: 2, newNum: 3, content: "" },
            { type: "removed", oldNum: 3, newNum: null, content: "const BASE_URL = 'http://localhost:3000';" },
            { type: "added", oldNum: null, newNum: 4, content: "const BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';" },
            { type: "context", oldNum: 4, newNum: 5, content: "" },
            { type: "context", oldNum: 5, newNum: 6, content: "export const client = axios.create({" },
            { type: "context", oldNum: 6, newNum: 7, content: "  baseURL: BASE_URL," },
            { type: "added", oldNum: null, newNum: 8, content: "  headers: {" },
            { type: "added", oldNum: null, newNum: 9, content: "    'Content-Type': 'application/json'," },
            { type: "added", oldNum: null, newNum: 10, content: "  }," },
            { type: "context", oldNum: 7, newNum: 11, content: "});" },
          ]
        },
        {
          oldStart: 20,
          newStart: 25,
          lines: [
            { type: "context", oldNum: 20, newNum: 25, content: "export async function fetchUser(id: string) {" },
            { type: "removed", oldNum: 21, newNum: null, content: "  const response = await client.get(`/users/${id}`);" },
            { type: "added", oldNum: null, newNum: 26, content: "  const token = await getToken();" },
            { type: "added", oldNum: null, newNum: 27, content: "  const response = await client.get(`/users/${id}`, {" },
            { type: "added", oldNum: null, newNum: 28, content: "    headers: { Authorization: `Bearer ${token}` }," },
            { type: "added", oldNum: null, newNum: 29, content: "  });" },
            { type: "context", oldNum: 22, newNum: 30, content: "  return response.data;" },
            { type: "context", oldNum: 23, newNum: 31, content: "}" },
          ]
        }
      ]
    },
    {
      file: "src/hooks/useDebounce.ts",
      hunks: [
        {
          oldStart: 1,
          newStart: 1,
          lines: [
            { type: "removed", oldNum: 1, newNum: null, content: "import { useState, useEffect } from 'react';" },
            { type: "added", oldNum: null, newNum: 1, content: "import { useState, useEffect, useRef } from 'react';" },
            { type: "context", oldNum: 2, newNum: 2, content: "" },
            { type: "context", oldNum: 3, newNum: 3, content: "export function useDebounce<T>(value: T, delay: number): T {" },
            { type: "context", oldNum: 4, newNum: 4, content: "  const [debouncedValue, setDebouncedValue] = useState<T>(value);" },
            { type: "added", oldNum: null, newNum: 5, content: "  const timerRef = useRef<NodeJS.Timeout>();" },
            { type: "context", oldNum: 5, newNum: 6, content: "" },
            { type: "context", oldNum: 6, newNum: 7, content: "  useEffect(() => {" },
            { type: "removed", oldNum: 7, newNum: null, content: "    const handler = setTimeout(() => {" },
            { type: "added", oldNum: null, newNum: 8, content: "    timerRef.current = setTimeout(() => {" },
            { type: "context", oldNum: 8, newNum: 9, content: "      setDebouncedValue(value);" },
            { type: "context", oldNum: 9, newNum: 10, content: "    }, delay);" },
            { type: "context", oldNum: 10, newNum: 11, content: "" },
            { type: "removed", oldNum: 11, newNum: null, content: "    return () => clearTimeout(handler);" },
            { type: "added", oldNum: null, newNum: 12, content: "    return () => {" },
            { type: "added", oldNum: null, newNum: 13, content: "      if (timerRef.current) clearTimeout(timerRef.current);" },
            { type: "added", oldNum: null, newNum: 14, content: "    };" },
            { type: "context", oldNum: 12, newNum: 15, content: "  }, [value, delay]);" },
            { type: "context", oldNum: 13, newNum: 16, content: "" },
            { type: "context", oldNum: 14, newNum: 17, content: "  return debouncedValue;" },
            { type: "context", oldNum: 15, newNum: 18, content: "}" },
          ]
        }
      ]
    },
    {
      file: "src/components/Header.tsx",
      hunks: [
        {
          oldStart: 8,
          newStart: 8,
          lines: [
            { type: "context", oldNum: 8, newNum: 8, content: "export function Header() {" },
            { type: "added", oldNum: null, newNum: 9, content: "  const [isMenuOpen, setIsMenuOpen] = useState(false);" },
            { type: "added", oldNum: null, newNum: 10, content: "" },
            { type: "context", oldNum: 9, newNum: 11, content: "  return (" },
            { type: "removed", oldNum: 10, newNum: null, content: "    <header className=\"bg-white shadow\">" },
            { type: "added", oldNum: null, newNum: 12, content: "    <header className=\"bg-white shadow-sm\">" },
            { type: "context", oldNum: 11, newNum: 13, content: "      <div className=\"container mx-auto px-4 py-3\">" },
            { type: "removed", oldNum: 12, newNum: null, content: "        <h1 className=\"text-xl font-bold\">My App</h1>" },
            { type: "added", oldNum: null, newNum: 14, content: "        <div className=\"flex items-center justify-between\">" },
            { type: "added", oldNum: null, newNum: 15, content: "          <h1 className=\"text-xl font-bold\">My App</h1>" },
            { type: "added", oldNum: null, newNum: 16, content: "          <button" },
            { type: "added", oldNum: null, newNum: 17, content: "            onClick={() => setIsMenuOpen(!isMenuOpen)}" },
            { type: "added", oldNum: null, newNum: 18, content: "            className=\"md:hidden p-2\"" },
            { type: "added", oldNum: null, newNum: 19, content: "          >" },
            { type: "added", oldNum: null, newNum: 20, content: "            Menu" },
            { type: "added", oldNum: null, newNum: 21, content: "          </button>" },
            { type: "added", oldNum: null, newNum: 22, content: "        </div>" },
            { type: "context", oldNum: 13, newNum: 23, content: "      </div>" },
            { type: "context", oldNum: 14, newNum: 24, content: "    </header>" },
          ]
        }
      ]
    },
    {
      file: "README.md",
      hunks: [
        {
          oldStart: 12,
          newStart: 12,
          lines: [
            { type: "context", oldNum: 12, newNum: 12, content: "## Installation" },
            { type: "context", oldNum: 13, newNum: 13, content: "" },
            { type: "context", oldNum: 14, newNum: 14, content: "```bash" },
            { type: "removed", oldNum: 15, newNum: null, content: "npm install" },
            { type: "added", oldNum: null, newNum: 15, content: "pnpm install" },
            { type: "context", oldNum: 16, newNum: 16, content: "```" },
            { type: "context", oldNum: 17, newNum: 17, content: "" },
            { type: "context", oldNum: 18, newNum: 18, content: "## Development" },
            { type: "context", oldNum: 19, newNum: 19, content: "" },
            { type: "context", oldNum: 20, newNum: 20, content: "```bash" },
            { type: "removed", oldNum: 21, newNum: null, content: "npm run dev" },
            { type: "added", oldNum: null, newNum: 21, content: "pnpm dev" },
            { type: "context", oldNum: 22, newNum: 22, content: "```" },
            { type: "context", oldNum: 23, newNum: 23, content: "" },
            { type: "added", oldNum: null, newNum: 24, content: "## Testing" },
            { type: "added", oldNum: null, newNum: 25, content: "" },
            { type: "added", oldNum: null, newNum: 26, content: "```bash" },
            { type: "added", oldNum: null, newNum: 27, content: "pnpm test" },
            { type: "added", oldNum: null, newNum: 28, content: "```" },
            { type: "added", oldNum: null, newNum: 29, content: "" },
            { type: "context", oldNum: 24, newNum: 30, content: "## Building" },
          ]
        }
      ]
    },
    {
      file: "package.json",
      hunks: [
        {
          oldStart: 5,
          newStart: 5,
          lines: [
            { type: "context", oldNum: 5, newNum: 5, content: "  \"scripts\": {" },
            { type: "context", oldNum: 6, newNum: 6, content: "    \"dev\": \"next dev\"," },
            { type: "context", oldNum: 7, newNum: 7, content: "    \"build\": \"next build\"," },
            { type: "removed", oldNum: 8, newNum: null, content: "    \"start\": \"next start\"" },
            { type: "added", oldNum: null, newNum: 8, content: "    \"start\": \"next start\"," },
            { type: "added", oldNum: null, newNum: 9, content: "    \"test\": \"jest\"," },
            { type: "added", oldNum: null, newNum: 10, content: "    \"lint\": \"eslint . --ext .ts,.tsx\"" },
            { type: "context", oldNum: 9, newNum: 11, content: "  }," },
            { type: "context", oldNum: 10, newNum: 12, content: "  \"dependencies\": {" },
            { type: "removed", oldNum: 11, newNum: null, content: "    \"next\": \"^13.0.0\"," },
            { type: "removed", oldNum: 12, newNum: null, content: "    \"react\": \"^18.2.0\"," },
            { type: "added", oldNum: null, newNum: 13, content: "    \"next\": \"^14.0.0\"," },
            { type: "added", oldNum: null, newNum: 14, content: "    \"react\": \"^18.3.0\"," },
            { type: "context", oldNum: 13, newNum: 15, content: "    \"react-dom\": \"^18.2.0\"" },
            { type: "context", oldNum: 14, newNum: 16, content: "  }," },
            { type: "added", oldNum: null, newNum: 17, content: "  \"devDependencies\": {" },
            { type: "added", oldNum: null, newNum: 18, content: "    \"@types/jest\": \"^29.5.0\"," },
            { type: "added", oldNum: null, newNum: 19, content: "    \"jest\": \"^29.5.0\"," },
            { type: "added", oldNum: null, newNum: 20, content: "    \"eslint\": \"^8.50.0\"" },
            { type: "added", oldNum: null, newNum: 21, content: "  }," },
            { type: "context", oldNum: 15, newNum: 22, content: "  \"license\": \"MIT\"" },
          ]
        }
      ]
    },
    {
      file: "src/lib/validators.ts",
      hunks: [
        {
          oldStart: 1,
          newStart: 1,
          lines: [
            { type: "added", oldNum: null, newNum: 1, content: "import { z } from 'zod';" },
            { type: "added", oldNum: null, newNum: 2, content: "" },
            { type: "context", oldNum: 1, newNum: 3, content: "export function isValidEmail(email: string): boolean {" },
            { type: "removed", oldNum: 2, newNum: null, content: "  const regex = /^[^\\s@]+@[^\\s@]+\\.[^\\s@]+$/;" },
            { type: "removed", oldNum: 3, newNum: null, content: "  return regex.test(email);" },
            { type: "added", oldNum: null, newNum: 4, content: "  try {" },
            { type: "added", oldNum: null, newNum: 5, content: "    z.string().email().parse(email);" },
            { type: "added", oldNum: null, newNum: 6, content: "    return true;" },
            { type: "added", oldNum: null, newNum: 7, content: "  } catch {" },
            { type: "added", oldNum: null, newNum: 8, content: "    return false;" },
            { type: "added", oldNum: null, newNum: 9, content: "  }" },
            { type: "context", oldNum: 4, newNum: 10, content: "}" },
            { type: "context", oldNum: 5, newNum: 11, content: "" },
            { type: "context", oldNum: 6, newNum: 12, content: "export function isValidUrl(url: string): boolean {" },
            { type: "context", oldNum: 7, newNum: 13, content: "  try {" },
            { type: "removed", oldNum: 8, newNum: null, content: "    new URL(url);" },
            { type: "added", oldNum: null, newNum: 14, content: "    const parsed = new URL(url);" },
            { type: "added", oldNum: null, newNum: 15, content: "    return ['http:', 'https:'].includes(parsed.protocol);" },
            { type: "removed", oldNum: 9, newNum: null, content: "    return true;" },
            { type: "context", oldNum: 10, newNum: 16, content: "  } catch {" },
            { type: "context", oldNum: 11, newNum: 17, content: "    return false;" },
            { type: "context", oldNum: 12, newNum: 18, content: "  }" },
            { type: "context", oldNum: 13, newNum: 19, content: "}" },
          ]
        }
      ]
    }
  ];

  return (
    <div className="flex flex-col w-full h-screen">
      <div className="px-2 py-3">
        <div className="flex items-center gap-3 text-xs text-gray-600 mb-2">
          <span className="font-mono">a7f2e9c</span>
          <span>•</span>
          <span>Jane Developer</span>
          <span>•</span>
          <span>2 hours ago</span>
        </div>
        <div className="text-sm font-semibold mb-1">feat: improve button styling and date formatting</div>
        <div className="text-sm text-gray-700 leading-relaxed">
          Updated the Button component to use a darker blue shade and added rounded corners for better visual consistency.
          Also refactored the date formatting utilities to provide more control over the output format, replacing simple
          toLocaleDateString calls with explicit formatting options. This improves consistency across different locales
          and gives us better control over how dates and times are displayed throughout the application.
        </div>
      </div>

      <div className="flex-1 overflow-auto">
        {mockDiffs.map((diff, idx) => (
          <div key={idx} className="mb-8">
            <div className="bg-gray-100 px-2 py-0.5 text-sm font-mono sticky top-0 z-10">
              {diff.file}
            </div>
            {diff.hunks.map((hunk, hunkIdx) => (
              <div key={hunkIdx} className="font-mono text-sm">
                {hunk.lines.map((line, lineIdx) => (
                  <div key={lineIdx} className="grid grid-cols-[auto_1fr_auto_1fr] hover:bg-gray-50">
                    {line.type === "removed" && (
                      <>
                        <div className="bg-red-50 text-red-600 px-2 text-right select-none">{line.oldNum}</div>
                        <div className="bg-red-50 px-1">
                          <span className="text-red-700">-</span>
                          <span className="text-red-900">{line.content}</span>
                        </div>
                        <div className="px-2 select-none"></div>
                        <div className=""></div>
                      </>
                    )}
                    {line.type === "added" && (
                      <>
                        <div className="px-2 select-none"></div>
                        <div className=""></div>
                        <div className="bg-green-50 text-green-600 px-2 text-right select-none">{line.newNum}</div>
                        <div className="bg-green-50 px-1">
                          <span className="text-green-700">+</span>
                          <span className="text-green-900">{line.content}</span>
                        </div>
                      </>
                    )}
                    {line.type === "context" && (
                      <>
                        <div className="text-gray-400 px-2 text-right select-none">{line.oldNum}</div>
                        <div className="px-1 text-gray-600">{line.content}</div>
                        <div className="text-gray-400 px-2 text-right select-none">{line.newNum}</div>
                        <div className="px-1 text-gray-600">{line.content}</div>
                      </>
                    )}
                  </div>
                ))}
              </div>
            ))}
          </div>
        ))}
      </div>
    </div>
  );
}
