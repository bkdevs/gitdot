export function githubAppInstallUrl(state: string): string {
  return `https://github.com/apps/gitdot-app/installations/new?state=${encodeURIComponent(state)}`;
}
