# Simple Donation Platform

## Project Title
**Simple Donation Platform** - A blockchain-based transparent donation system built on Stellar/Soroban

## Project Description
The Simple Donation Platform is a decentralized application (dApp) that enables charities to register on the blockchain and receive XLM donations with complete transparency. Every donation is recorded immutably on the Stellar blockchain, ensuring donors can track their contributions and charities can demonstrate accountability. The platform leverages Soroban smart contracts to automate donation tracking and provide real-time insights into charity funding.

## Project Vision
Our vision is to revolutionize charitable giving by bringing transparency, trust, and efficiency to the donation ecosystem. We aim to:

- **Eliminate Middlemen**: Direct donations from donors to charities without intermediaries
- **Ensure Transparency**: Every transaction is publicly verifiable on the blockchain
- **Build Trust**: Donors can see exactly how much each charity has received
- **Reduce Costs**: Lower transaction fees compared to traditional payment processors
- **Global Access**: Enable anyone, anywhere to donate to worthy causes instantly

By leveraging blockchain technology, we're creating a future where charitable giving is more transparent, efficient, and accessible to everyone.

## Key Features

### 1. **Charity Registration**
- Charities can register on the platform with their name and wallet address
- Each charity receives a unique ID for tracking purposes
- Registration creates an immutable record on the blockchain

### 2. **Transparent Donation Tracking**
- All donations are recorded with complete details:
  - Donation amount in XLM
  - Donor's wallet address
  - Recipient charity
  - Timestamp of transaction
- Each donation receives a unique ID for reference

### 3. **Real-Time Statistics**
- Track total donations received by each charity
- View the number of donations each charity has received
- Monitor donation history with complete transparency

### 4. **Immutable Records**
- All transactions are permanently recorded on the Stellar blockchain
- Provides complete audit trail for donors and charities
- Ensures accountability and prevents fraud

## Future Scope

### Short-term Enhancements (3-6 months)
- **Charity Verification System**: Implement a multi-signature approval process for charity verification
- **Donation Categories**: Allow charities to create specific campaigns or causes
- **Recurring Donations**: Enable automated recurring donation schedules
- **Donor Dashboard**: Build a comprehensive interface for donors to track all their contributions

### Medium-term Development (6-12 months)
- **Multi-token Support**: Expand beyond XLM to support other Stellar tokens
- **Impact Reporting**: Allow charities to upload impact reports linked to donations
- **Donor Recognition**: Implement NFT-based donor badges and recognition system
- **Charity Ratings**: Community-driven rating and review system for charities
- **Tax Receipt Generation**: Automated tax-deductible receipt generation for donors

### Long-term Vision (1-2 years)
- **Cross-chain Integration**: Enable donations from other blockchain networks
- **AI-powered Fraud Detection**: Implement machine learning to detect suspicious activities
- **Milestone-based Releases**: Lock donations until charities meet predetermined milestones
- **DAO Governance**: Transition to community-governed platform with voting rights
- **Global Charity Network**: Partner with international NGOs and charitable organizations
- **Mobile Application**: Native iOS and Android apps for easier donation experience
- **Fiat On-ramps**: Integrate fiat payment options for users unfamiliar with crypto

---

## Technical Stack
- **Blockchain**: Stellar Network
- **Smart Contract**: Soroban SDK (Rust)
- **Storage**: On-chain immutable storage
- **Token**: XLM (Stellar Lumens)

## Getting Started

### Prerequisites
- Rust programming language
- Soroban CLI
- Stellar account with testnet XLM

### Installation
```bash
# Clone the repository
git clone <repository-url>

# Build the contract
soroban contract build

# Deploy to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/donation_platform.wasm \
  --network testnet
```

### Usage

#### Register a Charity
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- register_charity \
  --name "Example Charity" \
  --wallet <CHARITY_WALLET_ADDRESS>
```

#### Make a Donation
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- donate \
  --charity_id 1 \
  --donor <DONOR_WALLET_ADDRESS> \
  --amount 1000000
```

#### View Charity Details
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- view_charity \
  --charity_id 1
```

---

## Contributing
We welcome contributions from the community! Please read our contributing guidelines before submitting pull requests.

## License
This project is licensed under the MIT License.

## Contact
For questions or support, please open an issue in the repository or contact the development team.

## Contract Details
Contract ID