# ChronoVault

ChronoVault is a Solana smart contract (program) built with Anchor that enables users to lock SPL tokens in a secure vault for a specified period. After the lock duration elapses, the designated recipient can withdraw the tokens. This contract is ideal for time-locked token vesting, delayed payments, or trustless escrow scenarios.

---

## Features

- **Time-Locked Vaults:** Lock SPL tokens for a custom duration.
- **Designated Recipient:** Specify who can withdraw the tokens after the lock period.
- **Secure & Trustless:** Only the intended recipient can withdraw, and only after the unlock time.
- **Anchor & SPL Token Support:** Built with Anchor and supports all SPL tokens.

---

## How It Works

1. **Deposit:**  
   - The depositor specifies the recipient, token mint, amount, and lock duration.
   - Tokens are transferred from the depositor to a vault PDA controlled by the program.
   - The vault details (depositor, recipient, mint, deposit time, unlock time) are stored on-chain.

2. **Withdraw:**  
   - After the unlock time, the recipient can withdraw the tokens to their associated token account.
   - The vault account is closed and rent is returned to the depositor.

---

## Program Details

- **Program ID (Devnet):** `AM37vnJ3mXiMSNSfaeTV1ZVicK7zoyxWqVrrBdnikUb`
- **Anchor Version:** 0.31.1
- **Solana Cluster:** Devnet (default)

---

## Usage

### Deposit Tokens

Call the `deposite` instruction with:
- `seed`: Unique identifier for the vault (u64)
- `amount`: Amount of tokens to lock (u64)
- `lock_duration`: Lock period in seconds (u64)

**Accounts required:**
- Signer (depositor)
- Mint (SPL token)
- ChronoVault PDA (auto-derived)
- Vault (auto-derived)
- User's associated token account
- Recipient's public key
- Associated Token Program, Token Program, System Program

### Withdraw Tokens

Call the `withdraw` instruction after the unlock time.

**Accounts required:**
- Recipient (must match the one set at deposit)
- Depositor
- Mint
- ChronoVault PDA
- Vault
- Recipient's associated token account
- Associated Token Program, Token Program, System Program

---

## Example (TypeScript, using Anchor)

```typescript
// Deposit
await program.methods
  .deposite(seed, depositAmount, lockDuration)
  .accounts({
    signer: user.publicKey,
    mint: mint,
    chronoAccount: chronoAccount,
    vault: vault,
    userAta: userAta,
    recipientKey: recipient.publicKey,
    associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .signers([user])
  .rpc();

// Withdraw (after unlock time)
await program.methods
  .withdraw()
  .accounts({
    recipient: recipient.publicKey,
    depositer: user.publicKey,
    mint: mint,
    chronoAccount: chronoAccount,
    vault: vault,
    recipientAta: recipientAta,
    associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .signers([recipient])
  .rpc();
```

---

## Error Codes

- `TokensStillLocked`: Attempted withdrawal before unlock time.
- `InvalidDepsiter`: Only the original depositor can close the vault.
- `InvalidRecipient`: Only the designated recipient can withdraw.
- `InvalidMint`: Token mint mismatch.

---

## Development

- **Build:** `anchor build`
- **Test:** `anchor test` (see `tests/chronovault.ts` for examples)
- **Deploy:** Update `Anchor.toml` and use `anchor deploy`

---

## License

MIT 