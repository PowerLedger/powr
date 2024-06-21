module.exports = {
  docs: {
    About: ["introduction", "terminology", "history"],
    Wallets: [
      "wallet-guide",
      {
        type: "category",
        label: "Command-line Wallets",
        items: [
          "wallet-guide/cli",
          "wallet-guide/paper-wallet",
          {
            type: "category",
            label: "Hardware Wallets",
            items: [
              "wallet-guide/hardware-wallets",
              "wallet-guide/hardware-wallets/ledger",
            ],
          },
          "wallet-guide/file-system-wallet",
        ],
      },
      "wallet-guide/support",
    ],
    Staking: ["staking", "staking/stake-accounts"],
    "Command Line": [
      "cli",
      "cli/install-solana-cli-tools",
      "cli/conventions",
      "cli/choose-a-cluster",
      "cli/transfer-tokens",
      "cli/deploy-a-program",
      "offline-signing",
      "offline-signing/durable-nonce",
      "cli/usage",
    ],
    Developing: [
      {
        type: "category",
        label: "Programming Model",
        items: [
          "developing/programming-model/overview",
          "developing/programming-model/transactions",
          "developing/programming-model/accounts",
          "developing/programming-model/runtime",
          "developing/programming-model/calling-between-programs",
        ],
      },
      {
        type: "category",
        label: "Clients",
        items: [
          "developing/clients/jsonrpc-api",
          "developing/clients/javascript-api",
          "developing/clients/javascript-reference",
          "developing/clients/rust-api",
        ],
      },
      {
        type: "category",
        label: "Runtime Facilities",
        items: [
          "developing/runtime-facilities/programs",
          "developing/runtime-facilities/sysvars",
        ],
      },
      {
        type: "category",
        label: "On-chain Programs",
        items: [
          "developing/on-chain-programs/overview",
          "developing/on-chain-programs/developing-rust",
          "developing/on-chain-programs/developing-c",
          "developing/on-chain-programs/deploying",
          "developing/on-chain-programs/debugging",
          "developing/on-chain-programs/examples",
          "developing/on-chain-programs/faq",
        ],
      },
      "developing/test-validator",
      "developing/backwards-compatibility",
    ],
    Integrating: [
      "integrations/exchange",
      "integrations/retrying-transactions",
    ],
    Validating: [
      "running-validator",
      "running-validator/validator-reqs",
      "running-validator/validator-start",
      "running-validator/vote-accounts",
      "running-validator/validator-stake",
      "running-validator/validator-monitor",
      "running-validator/validator-info",
      "running-validator/validator-failover",
      "running-validator/validator-troubleshoot",
    ],
    Clusters: [
      "clusters",
      "cluster/rpc-endpoints",
      "cluster/performance-metrics",
    ],
    Architecture: [
      {
        type: "category",
        label: "Cluster",
        items: [
          "cluster/overview",
          "cluster/synchronization",
          "cluster/leader-rotation",
          "cluster/fork-generation",
          "cluster/managing-forks",
          "cluster/turbine-block-propagation",
          "cluster/vote-signing",
          "cluster/stake-delegation-and-rewards",
        ],
      },
      {
        type: "category",
        label: "Validator",
        items: [
          "validator/anatomy",
          "validator/tpu",
          "validator/tvu",
          "validator/blockstore",
          "validator/gossip",
          "validator/runtime",
        ],
      },
    ],
    Economics: [
      "economics_overview",
      {
        type: "category",
        label: "Inflation Design",
        items: [
          "inflation/terminology",
          "inflation/inflation_schedule",
          "inflation/adjusted_staking_yield",
        ],
      },
      "transaction_fees",
      "storage_rent_economics",
    ],
  },
};
