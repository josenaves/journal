# journal

This code is based on the tutorial presented by brimigs on [Solana Youtube Channel](https://www.youtube.com/watch?v=cW-3jGWq1IE&)*.

This project is generated with the [create-solana-dapp](https://github.com/solana-developers/create-solana-dapp) generator.

## Getting Started

### Prerequisites

- Node v18.18.0 or higher

- Rust v1.70.0 or higher
- Anchor CLI 0.29.0 or higher
- Solana CLI 1.17.0 or higher

### Installation

#### Clone the repo

```shell
git clone <repo-url>
cd <repo-name>
```

#### Install Dependencies

```shell
npm install
```

#### Start the web app

```
npm run dev
```

## Apps

### anchor

This is a Solana program written in Rust using the Anchor framework.

#### Commands

You can use any normal anchor commands. Either move to the `anchor` directory and run the `anchor` command or prefix the command with `npm run`, eg: `npm run anchor`.

#### Sync the program id:

Running this command will create a new keypair in the `anchor/target/deploy` directory and save the address to the Anchor config file and update the `declare_id!` macro in the `./src/lib.rs` file of the program.

You will manually need to update the constant in `anchor/lib/counter-exports.ts` to match the new program id.

```shell
npm run anchor keys sync
```

#### Build the program:

```shell
npm run anchor-build
```

#### Start the test validator with the program deployed:

```shell
npm run anchor-localnet
```

#### Run the tests

```shell
npm run anchor-test
```

#### Deploy to Devnet

```shell
npm run anchor deploy --provider.cluster devnet
```

### web

This is a React app that uses the Anchor generated client to interact with the Solana program.

#### Commands

Start the web app

```shell
npm run dev
```

Build the web app

```shell
npm run build
```


## Notes

I have used bun for running the commands instead of npm.

To fix the deploy issue, I had to add some SOL to the wallet.

```shell
josenaves@skymac anchor % solana airdrop 2 --url devnet
Requesting airdrop of 2 SOL

Signature: 3c67tuq6ow3Nk3AVPBvty4my3bqp1T2mNyL7FAMHbsYNU7Lgi3gmrDSksmvWGcFKXmMNtyoUMxUXZ3DAwZevBxrn

2.49443484 SOL
```

```shell
josenaves@skymac anchor % solana balance --url devnet
2.49443484 SOL
```

I have also used Bun instead of npm / npx. 😉
