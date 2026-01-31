import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  reactCompiler: true,
  async redirects() {
    return [
      {
        source: "/beta",
        destination: "/bkdevs/gitdot",
        permanent: true,
      },
    ];
  },
};

export default nextConfig;
