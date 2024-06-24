const math = require("remark-math");
const katex = require("rehype-katex");
module.exports = {
  title: "Powerledger Blockchain Docs",
  tagline:
    "Powerledger blockchain is a high-performance, permissionless blockchain, based on the Solana blockchain project.",
  url: "https://docs.powerledger.io",
  baseUrl: "/",
  favicon: "img/favicon.ico",
  organizationName: "PowerLedger", // Usually your GitHub org/user name.
  projectName: "powr", // Usually your repo name.
  onBrokenLinks: "throw",
  stylesheets: [
    {
      href: "/katex/katex.min.css",
      type: "text/css",
      integrity:
        "sha384-AfEj0r4/OFrOo5t7NnNe46zW/tFgW6x/bCJG8FqQCEo3+Aro6EYUG4+cU+KJWu/X",
      crossorigin: "anonymous",
    },
  ],
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
    // locales: ["en", "de", "es", "ru", "ar"],
    // localesNotBuilding: ["ko", "pt", "vi", "zh", "ja"],
    localeConfigs: {
      en: {
        label: "English",
      },
    },
  },
  themeConfig: {
    navbar: {
      logo: {
        alt: "Solana Logo",
        src: "img/logo-horizontal.svg",
        srcDark: "img/logo-horizontal-dark.svg",
      },
      items: [
        {
          href: "https://docs.powerledger.io",
          label: "Powerledger Blockchain Documentation »",
          position: "left",
        },
        {
          to: "developing/programming-model/overview",
          label: "Develop",
          position: "left",
        },
        {
          to: "running-validator",
          label: "Validate",
          position: "left",
        },
        {
          to: "integrations/exchange",
          label: "Integrate",
          position: "left",
        },
        {
          to: "cluster/overview",
          label: "Learn",
          position: "left",
        },
        {
          type: "localeDropdown",
          position: "right",
        },
        {
          href: "https://t.me/powerledger",
          label: "Telegram",
          position: "right",
        },
        {
          href: "https://github.com/PowerLedger/powr",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    algolia: {
      // This API key is "search-only" and safe to be published
      appId: "DP4G9H4UW9",
      apiKey: "7ec8c359868cf4a396e832596f03897c",
      indexName: "powerledger",
      contextualSearch: true,
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Docs",
          items: [
            {
              label: "Introduction",
              to: "introduction",
            },
          ],
        },
        {
          title: "Community",
          items: [
            {
              label: "Telegram",
              href: "https://t.me/powerledger",
            },
            {
              label: "Twitter/X",
              href: "https://x.com/PowerLedger_io",
            },
          ],
        },
        {
          title: "More",
          items: [
            {
              label: "GitHub",
              href: "https://github.com/PowerLedger/powr",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Powerledger AG`,
    },
  },
  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          path: "src",
          routeBasePath: "/",
          sidebarPath: require.resolve("./sidebars.js"),
          remarkPlugins: [math],
          rehypePlugins: [katex],
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      },
    ],
  ],
};
