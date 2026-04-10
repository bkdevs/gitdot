import path from "node:path";
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "images.gitdot.io",
      },
    ],
  },
  reactCompiler: true,
  turbopack: {
    root: path.join(__dirname, ".."),
  },
  experimental: {
    proxyClientMaxBodySize: "2gb",
    serverActions: {
      bodySizeLimit: "6mb",
    },
  },
  async redirects() {
    return [
      {
        source: "/beta",
        destination: "/bkdevs/gitdot",
        permanent: true,
      },
      {
        source: "/signup",
        destination: "/login",
        permanent: false,
      },
    ];
  },
  async rewrites() {
    const serverUrl = process.env.GITDOT_SERVER_URL || "http://localhost:8080";
    return [
      {
        source: "/:owner/:repo/info/refs",
        destination: `${serverUrl}/:owner/:repo/info/refs`,
      },
      {
        source: "/:owner/:repo/git-upload-pack",
        destination: `${serverUrl}/:owner/:repo/git-upload-pack`,
      },
      {
        source: "/:owner/:repo/git-receive-pack",
        destination: `${serverUrl}/:owner/:repo/git-receive-pack`,
      },
      {
        source: "/settings",
        destination: "/settings/profile",
      },
    ];
  },
};

export default nextConfig;
