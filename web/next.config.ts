import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  webpack(config) {
    config.experiments = {
      syncWebAssembly: true,
      asyncWebAssembly: true,
      layers: true,
    };

    return config;
  },
};

export default nextConfig;
