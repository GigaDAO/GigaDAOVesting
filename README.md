# GigaDAOVesting

Mainnet Program ID: `CA2CNT3eyyie4dKVos5ZoaePSAB2jCZaZRi7Zbdx1RNh`

This Solana on-chain program is implemented in anchorlang framework and serves to vest GigaDAO's 
governance token (GIGS) to pre-sale investors at a linear rate, automatically starting at the 
IDO date of June 1, 2023.

### Endpoints

The program implements two key instructions:
1) Initialize
2) Claim

The `initialize` instruction creates an immutable on-chain data structure containing:
- The investors wallet address (only this address can claim)
- The vesting start date
- The total allocation
- The vesting rate (tokens per second, decimal agnostic)
- The token vault

The `claim` instruction computes the amount available to claim based upon initialized parameters, 
and permits the specified investor to withdraw tokens accordingly, recording each claim amount aptly.

### Upgrade Authority

The UA for this contract is held by a GigaDAO governance contract which itself is not upgradeable.
The UA address is `CKk2EQ6ybz6qMxMAVgDxdRksjLAQgLTfW47t9LwERW3z`, which is a PDA owned by `GzMvD8AGSiRhHapNsJzUMoYR3pkbCg6vPnnopaeFZE7E`.
The latter is a non-upgradeable voting contract gated by GIGS voting. 

### Verification

This program was deployed using `anchor build --verifiable`, meaning that anyone can download this repo,
build it locally (using anchor's docker verified build routine), and confirm it matches the on-chain binary.

Prerequisites:
- solana-cli v1.11.10
- anchor-cli v0.25.0
- node v18.8.0
- yarn
- docker

After installing the prerequisites, clone this repo, install with `yarn install`, followed by:

`anchor verify -p gdvesting CA2CNT3eyyie4dKVos5ZoaePSAB2jCZaZRi7Zbdx1RNh`


