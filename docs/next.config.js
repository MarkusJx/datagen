const withNextra = require('nextra')({
  theme: 'nextra-theme-docs',
  themeConfig: './theme.config.tsx',
  defaultShowCopyCode: true,
  titleSuffix: '-',
});

const isProduction = process.env.NODE_ENV === 'production';
const assetPrefix = isProduction ? '/datagen' : '';

module.exports = withNextra({
  images: {
    unoptimized: true,
  },
  reactStrictMode: true,
  swcMinify: true,
  trailingSlash: true,
  assetPrefix,
  basePath: assetPrefix,
  output: 'export',
});
