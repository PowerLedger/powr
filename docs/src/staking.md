---
title: Staking on Powerledger chain
---

_Note before reading: All references to increases in values are in absolute
terms with regards to balance of POWR.
This document makes no suggestion as to the monetary value of POWR at any time._

By staking your POWR tokens, you help secure the network and
[earn rewards](implemented-proposals/staking-rewards.md) while doing so.

You can stake by delegating your tokens to validators who process transactions and run the network.

Delegating stake is a shared-risk shared-reward financial model that may provide
returns to holders of tokens delegated for a long period.
This is achieved by aligning the financial incentives of the token-holders
(delegators) and the validators to whom they delegate.

The more stake delegated to a validator, the more often this validator
is chosen to write new transactions to the ledger. The more transactions
the validator writes, the more rewards the validator and its delegators earn.
Validators who configure their systems to be able to process more transactions
earn proportionally more rewards and
because they keep the network running as fast and as smoothly as possible.

Validators incur costs by running and maintaining their systems, and this is
passed on to delegators in the form of a fee collected as a percentage of
rewards earned. This fee is known as a _commission_. Since validators earn more
rewards the more stake is delegated to them, they may compete with one another
to offer the lowest commission for their services.

You risk losing tokens when staking through a process known as
_slashing_. Slashing involves the removal and destruction of a portion of a
validator's delegated stake in response to intentional malicious behavior,
such as creating invalid transactions or censoring certain types of transactions
or network participants.

When a validator is slashed, all token holders who have delegated stake to that
validator lose a portion of their delegation. While this means an immediate
loss for the token holder, it also is a loss of future rewards for the validator
due to their reduced total delegation.

Note: Powerledger blockchain does not implement slashing at this stage

Rewards and slashing align validator and token holder interests which helps keep the network
secure, robust and performant.


## How do I stake my POWR tokens?

You can stake POWR through the staking portal at https://stake.powerledger.io with instructions found [here](https://powerledger.io/staking). This portal allows you to lock up your ERC-20 POWR on the Ethereum network into a smart contract, and delegate it to a chosen validator on the Powerledger Blockchain. 

Once a blockchain bridge is deployed, native staking will be available, which will have its own process.