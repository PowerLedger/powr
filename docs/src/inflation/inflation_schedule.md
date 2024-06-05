---
title: Powerledger Blockchain's Proposed Inflation Schedule
---

As mentioned above, the network's _Inflation Schedule_ is uniquely described by three parameters: _Initial Inflation Rate_, _Dis-inflation Rate_ and _Long-term Inflation Rate_. When considering these numbers, there are many factors to take into account:

- A large portion of the POWR issued via inflation will be distributed to stake-holders in proportion to the POWR they have staked. We want to ensure that the _Inflation Schedule_ design results in reasonable _Staking Yields_ for token holders who delegate POWR and for validation service providers (via commissions taken from _Staking Yields_).
- The primary driver of _Staked Yield_ is the amount of POWR staked divided by the total amount of POWR (% of total POWR staked). Therefore the distribution and delegation of tokens across validators are important factors to understand when determining initial inflation parameters.
- Overall token issuance - i.e. what do we expect the Current Total Supply to be in 10 years, or 20 years?
- Long-term, steady-state inflation is an important consideration not only for sustainable support for the validator ecosystem and Powerledger grant programs, but also should be tuned in consideration with expected token losses and burning over time.
- The rate at which we expect network usage to grow, as a consideration to the dis-inflationary rate. Over time, we plan for inflation to drop and expect that usage will grow.

Based on these considerations the Powerledger blockchain was launched with the following Inflation Schedule parameters:

- Initial Inflation Rate: $1.25\%$
- Dis-inflation Rate: $-15\%$
- Long-term Inflation Rate: $0.75\%$

These parameters define the proposed _Inflation Schedule_. 

Setting aside validator uptime and commissions, the expected Staking Yield and Adjusted Staking Yield metrics are then primarily a function of the % of total POWR staked on the network. Therefore we can we can model _Staking Yield_, if we introduce an additional parameter _% of Staked POWR_:

$$
\%~\text{POWR Staked} = \frac{\text{Total POWR Staked}}{\text{Total Current Supply}}
$$

This parameter must be estimated because it is a dynamic property of the token holders and staking incentives.
