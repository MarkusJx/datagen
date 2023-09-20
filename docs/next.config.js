const withNextra = require('nextra')({
    theme: 'nextra-theme-docs',
    themeConfig: './theme.config.tsx',
    defaultShowCopyCode: true,
    titleSuffix: '-',
});

module.exports = withNextra({
    images: {
        unoptimized: true,
    },
    output: 'export',
});
