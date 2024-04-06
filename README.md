# Battleboosters Solana Smart Contract

This repository contains the Solana smart contract code powering the Battleboosters platform.

**Key Features**

* **Booster Types and Availability:** There are two main types of boosters available: Shield Boosters and Points Boosters. Shield Boosters are used to enhance a fighter's lifespan, providing them with additional durability in battles. Points Boosters, on the other hand, offer fighters a bonus to the points they earn in matches. They are obtained through gameplay achievements, special events, or purchases within the game's ecosystem.
* **Fight Card Integration:**  Boosters can be associated with fight cards, providing enhanced attributes or special abilities to fighters during matches. This integration is handled seamlessly through our smart contract, ensuring that the correct boosters are applied to the right fight cards to affect gameplay as intended.
* **Booster Consumption/Closure:** Our platform treats boosters as single-use items. Once a booster is applied to a fight card and used in a match, it is automatically marked as consumed and closed, preventing any further use. This mechanism is crucial for the game's economic health and competitive balance.
* **Fighter Consumption/Closure:** Fighters in Battleboosters have a limited lifespan that diminishes after each battle. To prolong a fighter's career, players can use Shield Boosters to enhance their lifespan. Once a fighter's lifespan is depleted, they are retired (burned) from the roster.
* **Security & Fairness:** We prioritize security and fairness in all aspects of the Battleboosters platform. Our smart contract includes mechanisms to ensure random booster distribution, prevent cheating, and mitigate exploits. Continuous audits and community feedback are part of our commitment to maintaining a secure and fair gaming environment.

**Project Structure**

* **src/lib.rs:** Contains the core logic of the Solana program.
* **tests/**: Unit and integration tests for the smart contract.

**Prerequisites**

* Solana Tool Suite ([https://docs.solana.com/cli/install-solana-cli-tools](https://docs.solana.com/cli/install-solana-cli-tools))
* Anchor Framework ([https://project-serum.github.io/anchor/getting-started/installation.html](https://project-serum.github.io/anchor/getting-started/installation.html))
* Node.js and Yarn/NPM (for testing and frontend interaction)

**Getting Started**

1.  Clone this repository: `git clone https://github.com/BattleBoosters/battleboosters-solana-contract.git`
2.  Install dependencies: `yarn install` (or `npm install`)
3.  Build the program: `anchor build`
4.  Deploy to a Solana cluster (Devnet/Testnet/Mainnet): `anchor deploy`

**Testing**

1.  Run tests: `anchor test`

**Community and Support**

* Join our Discord [[Discord invitation]](https://discord.gg/9kwzRhff7Z) for questions, discussions, and updates.
* File issues on this repository for bug reports and feature requests.

**Additional Notes**

* **[Replace with the link to deployed program on Solana Explorer]**
* **[If applicable, add instructions for interacting with the smart contract from a frontend]**
* **[Expand on any unique selling points of your smart contract]**
