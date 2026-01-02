### Onchain Features
1. One-time Payment
a. Seller initializes a PDA.
b. All payments are done to this PDA.

2. Claim Payments
a. Seller call claim function.
b. Unnati takes 10% fees and sends rest to seller.

3. Subscriptions
a. Users pay fees upfront for 1 month 3 months, 6 months, 9 months, 1 year.
b. Solana programs mints a NFT.
c. The NFT’s metadata can include subscription details like expiration date, plan type, or access level.
d. Dapp checks the NFT before granting access (e.g., “Does this NFT exist and is its expiry still valid?”).
e. If a user doesn’t want to use their subscription, they could sell or gift it.
