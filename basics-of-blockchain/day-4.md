# Web3 Fundamentals: Notes from Day 2

## 1. Blockchain Wallets: Your Gateway to Web3

-   **Definition:** A digital application to manage assets and interact with blockchain networks.
-   **Core Functions:**
    1.  **View Balances:** Check your holdings (e.g., 0.1 ETH).
    2.  **View History:** See a log of all past transactions.
    3.  **Send/Receive:** Transfer digital tokens.
-   **Key Concepts:**
    -   **Native Currency:** The primary token of a blockchain (e.g., ETH for Ethereum, SOL for Solana). Needed to pay for gas fees.
    -   **Wallet Address:** Your public "digital mailbox" (e.g., `0x...`). You share this to receive funds.

## 2. MetaMask: Setup & Core Security

-   **Installation:** Always download from the official site: `metamask.io`.
-   **Self-Custody:** The core principle. You, and only you, control your private keys and assets. This comes with 100% responsibility.
-   **Secret Recovery Phrase (SRP): The Master Key**
    -   A unique 12-word phrase that can restore your wallet on any device.
    -   **Rule #1: NEVER store it digitally.** No screenshots, no text files, no password managers, no cloud storage.
    -   **Rule #2: ALWAYS store it physically.** Write it on paper (or metal) and store it in a secure, offline location.
-   **Local Password vs. SRP:**
    -   **Local Password:** Encrypts your wallet on *one specific browser/computer*. It's for convenience.
    -   **SRP:** Recovers your entire wallet on *any computer in the world*. It's for security and backup.
-   **Operational Security (OpSec):**
    -   **CRITICAL:** Maintain two separate wallets (e.g., in different Chrome profiles).
    1.  **Dev Wallet:** For tutorials, testing, and connecting to unknown sites. Never hold real funds here.
    2.  **Funds Wallet:** For real, valuable assets. Never connect it to unaudited dApps.

## 3. Navigating MetaMask: Multi-Chain & Accounts

-   **Multi-Chain:** MetaMask is EVM-compatible (Ethereum, Arbitrum, Base, etc.) and is expanding to non-EVM chains (like Solana).
-   **Networks:** Always ensure you are on the correct network (e.g., Ethereum Mainnet vs. Sepolia Testnet).
-   **Enabling Testnets:** Go to `Settings -> Networks -> Show test networks` to make them visible.
-   **Block Explorer (Etherscan):** A public website (`etherscan.io`) to view any transaction or wallet address on the Ethereum blockchain. It provides full transparency.
-   **Multiple Accounts:** You can create many accounts (Account 1, Account 2, etc.) inside one MetaMask installation. **All accounts are controlled by the same single Secret Recovery Phrase.**

## 4. Testnets: Your Development Sandbox

-   **Public Testnets (e.g., Sepolia):**
    -   **Pros:** Mimic real-world mainnet conditions.
    -   **Cons:** Acquiring test ETH from faucets is now very difficult, slow, and often requires holding real ETH, making them impractical for rapid learning.
-   **Virtual Testnets (e.g., Tenderly):**
    -   **Pros:**
        -   **Instant Setup:** Create a private blockchain in seconds.
        -   **Forking:** Can create a personal copy of a mainnet at a specific block.
        -   **Unlimited Funds:** Instantly "fund" your wallet with any amount of test currency.
    -   **Conclusion:** The recommended solution for fast-paced development and learning.

## 5. Gas: The Fuel of the Blockchain

-   **Gas:** A unit measuring the computational effort required for a transaction.
-   **Gas Price:** The price you are willing to pay per unit of Gas (measured in Gwei). Higher price = faster transaction.
-   **Transaction Fee (Formula):** `Total Fee = Gas Used * Gas Price`.
-   **Payment:** All gas fees are paid in the network's **native currency** (e.g., ETH on Ethereum).

## 6. Smart Contracts: The Backend of Web3

-   **Definition:** An agreement where the terms are written in code and automatically enforced by the blockchain.
-   **Lifecycle:**
    1.  **Write:** Code is written in a language like Solidity.
    2.  **Compile:** Code is converted to machine-readable bytecode.
    3.  **Deploy:** The bytecode is sent to the blockchain in a transaction, making it live.
-   **Properties (Once Deployed):**
    -   **Immutable:** The code cannot be changed.
    -   **Transparent:** Anyone can view the code.
    -   **Always Available:** Cannot be censored or turned off.
-   **Smart Contract Address:** Each deployed contract gets its own unique address, which is used to interact with it.