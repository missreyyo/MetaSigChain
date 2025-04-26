# MetaSigChain

<img src="resim.png" alt="MetaSigChain Architecture" width="400" height="300">

MetaSigChain is a multi-signature-enabled token management smart contract deployed on Stellar Blockchain. This project implements a secure and transparent token management system with multi-signature capabilities, ensuring that critical operations require multiple approvals.

- 🔒 Multi-signature security for token operations
- 💎 Built on Stellar Blockchain using Soroban SDK
- 🛠️ Developed in Rust for maximum performance
- 🔄 Real-time transaction monitoring
- 🎯 Focus on security and transparency

## Who Am I?
- **Name:** Reyhan Duygu
- **Role:** Blockchain & Web3 Developer
- **Location:** Ankara, Turkey
- **Email:** reyhanduygu123@gmail.com
- **GitHub:** [missreyyo](https://github.com/missreyyo)
- **LinkedIn:** [reyhan-duygu-1a85661a3](https://www.linkedin.com/in/reyhan-duygu-1a85661a3/)
- **Interests:** Blockchain, ZKP, Web3, Metaverse, Decentralized Systems

## The Tech I Use
- **Blockchain Platform:** Stellar Blockchain
- **Smart Contract Language:** Rust
- **Development Framework:** Soroban SDK
- **Testing Tools:** Soroban Test Suite
- **Deployment Tools:** Soroban CLI
- **Version Control:** Git
- **IDE:** Cursor

## Smart Contract Address
- **Testnet Contract Address:** CCQWMCHX6GPZDCKZRACJM35FRSA5M6BNG5Q23GRT4IPGG7I6P5ZAURV4
- **Explorer Link:** [Stellar Expert](https://stellar.expert/explorer/testnet/contract/CCQWMCHX6GPZDCKZRACJM35FRSA5M6BNG5Q23GRT4IPGG7I6P5ZAURV4)


## Project Description
MetaSigChain is a comprehensive token management system built on the Stellar blockchain, designed to enhance security and governance in digital asset management. The project implements a multi-signature mechanism that requires multiple approvals for critical operations, significantly reducing the risk of unauthorized transactions. It features standard token functionalities like minting, burning, and transferring, along with advanced capabilities such as account freezing and multi-signature transaction management. The system is built using Soroban SDK and Rust, ensuring high performance and security. MetaSigChain aims to provide a robust solution for organizations and DAOs that require secure and transparent token management with distributed control.

## Vision Statement
MetaSigChain envisions a future where digital asset management is secure, transparent, and accessible to all. By implementing multi-signature governance, we eliminate single points of failure in token management, making blockchain technology more resilient and trustworthy. Our solution empowers organizations, DAOs, and individual users to manage their assets with confidence, knowing that critical operations require consensus. This approach not only enhances security but also promotes decentralized governance, setting a new standard for token management in the blockchain ecosystem. Through MetaSigChain, we're building the foundation for a more secure and inclusive digital economy.

## Software Development Plan

1. **Smart Contract Core Development**
   - Implement basic token functions (mint, burn, transfer)
   - Set up multi-signature transaction system
   - Develop account freezing mechanism
   - Create admin controls and permissions

2. **Advanced Features Implementation**
   - Build transaction proposal system
   - Implement approval workflow
   - Add threshold management
   - Develop emergency controls

3. **Storage and State Management**
   - Design data structures for balances
   - Implement allowance tracking
   - Set up transaction history
   - Configure metadata storage

4. **Testing and Security**
   - Write comprehensive unit tests
   - Perform integration testing
   - Conduct security audits
   - Test edge cases and failure scenarios

5. **Deployment and Monitoring**
   - Deploy to Stellar testnet
   - Monitor contract performance
   - Gather user feedback
   - Deploy to mainnet

## How It Works

### Multi-Signature Transaction System
MetaSigChain introduces a robust multi-signature system that requires multiple approvals for critical operations. Here's how it works:

1. **Transaction Proposal**
   - Any authorized user can propose a transaction
   - Transaction details include: operation type, target address, amount, and expiration time
   - Proposals are stored in the contract's state

2. **Approval Process**
   - Multiple owners must approve the transaction
   - Each approval is recorded and verified
   - Transaction executes only when threshold is met
   - Time-limited approvals prevent stale transactions

3. **Security Features**
   - Account freezing capability for security incidents
   - Emergency controls for critical situations
   - Transaction expiration to prevent stale proposals
   - Threshold-based execution requirements

### Key Functions

#### Multi-Signature Operations
- `setup_multisig`: Configure multi-signature parameters
  - Set owners and threshold
  - Configure approval requirements
  - Define emergency controls

- `propose_multisig_transaction`: Create new transaction proposal
  - Specify operation type
  - Set target and amount
  - Define expiration time

- `approve_multisig_transaction`: Approve pending transactions
  - Verify owner status
  - Record approval
  - Check threshold requirements

- `execute_multisig_transaction`: Execute approved transactions
  - Verify all requirements
  - Perform the operation
  - Update state accordingly

#### Standard Token Operations
- `mint`: Create new tokens (requires multi-signature approval)
- `burn`: Remove tokens from circulation
- `transfer`: Move tokens between accounts
- `freeze_account`: Temporarily restrict account operations
- `unfreeze_account`: Remove account restrictions

### Storage Structure
The contract maintains several key data structures:

1. **Multi-Signature Configuration**
   - Owner addresses
   - Approval threshold
   - Active proposals
   - Approval records

2. **Transaction State**
   - Pending proposals
   - Approval counts
   - Execution status
   - Expiration times

3. **Account Management**
   - Token balances
   - Frozen status
   - Allowances
   - Transaction history

## Personal Story
I'm Reyhan Duygu, a blockchain analyst developer passionate about building secure and decentralized systems. After witnessing vulnerabilities in single-admin smart contracts, I created MetaSigChain to provide a safer way of managing digital assets. With my background in blockchain, ZKP, and Web3, I aim to contribute to a more secure and trustworthy decentralized world.

## Setup Environment
1. **Prerequisites**
   - Install Rust (latest stable version)
   - Install Soroban CLI
   - Install Git

2. **Installation Steps**
   ```bash
   # Clone the repository
   git clone https://github.com/missreyyo/MetaSigChain.git
   cd soroban-token-contract

   # Install dependencies
   cargo build

   # Run tests
   cargo test

   # Deploy to testnet
   soroban contract deploy --wasm target/wasm32-unknown-unknown/release/soroban_token_contract.wasm
   ```

3. **Environment Variables**
   - Set up your Stellar testnet account
   - Configure Soroban CLI with your account details
   - Set up your network preferences
