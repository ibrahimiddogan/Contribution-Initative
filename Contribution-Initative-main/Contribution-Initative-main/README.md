# contribution_initative1

Project Name: Contribution Initiative

About Us:

Name: Ibrahim Doğan & Doğu Kaan Sakallı & Cem Kandemir

Project Details: We are developing a donation system on the blockchain. This system will allow individuals and organizations to donate through a single platform. Individuals wishing to donate will be able to easily donate to the organizations they choose. State-approved organizations will also be able to collect donations through this system. One of the most important features of our system will be transparency. This way, donors will be able to track the amounts of their donations and receive feedback from the organizations they donate to. Additionally, entrepreneurs will be able to use this platform reliably when they want to raise donations for their projects. Our goal is to create a platform based on completely transparent and reliable communication.

Development Plan:

Design of Smart Contracts: We will design and develop smart contracts to manage donations, user profiles, and transparency features.
Smart Contract Development: The smart contracts will be coded in a blockchain-compatible language (e.g., Solidity for Ethereum, or Rust for other blockchains) and deployed on the blockchain.
User Interface Development: A user-friendly web-based interface will be developed to allow users to easily donate, track donations, and receive feedback.
Integration of Interface and Smart Contracts: The user interface will be integrated with the smart contracts to enable seamless interaction between users and the blockchain.
Beta Testing and Feedback: The platform will undergo thorough beta testing to ensure functionality and user experience meet expectations. Feedback will be collected and implemented.
Deployment: After successful testing and feedback implementation, the platform will be deployed to the main blockchain network for public use.
Vision: Our vision is to create a transparent and reliable donation platform that facilitates easy and secure donations for individuals, organizations, and entrepreneurs, ultimately contributing to positive social impact and community development.

Personal Story Summary: As the founder of this project, I am passionate about creating positive change through technology. I believe that transparency and reliability are key values that should be upheld in all aspects of life, including donations and charitable activities. I am excited to lead this project and make a meaningful contribution to society.
Welcome to your new contribution_initative1 project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with contribution_initative1, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd contribution_initative1/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
