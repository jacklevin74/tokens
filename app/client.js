const anchor = require('@coral-xyz/anchor');
const { PublicKey } = require('@solana/web3.js');

// Set up the provider and program
anchor.setProvider(anchor.AnchorProvider.env());
const provider = anchor.getProvider();

// Load the program IDL from the workspace
const program = anchor.workspace.xspl_token_program;

// Constants
const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const MINT_ACCOUNT = new PublicKey("FtzdoA48K5vwuffxmZ5pmygZZNfXkCvoBSUzuDEPzJeX");
const RECIPIENT = new PublicKey("EQmriVRcfvWphE3qnscfdTB3ytGivDhSKw4B6msaRpEV");  // This is the token account where tokens will be burned

// Function to mint tokens
async function mintTokens(amount) {
  const mintAuthority = provider.wallet.publicKey; // Default wallet address as mint authority

  try {
    await program.rpc.mintTokens(new anchor.BN(amount), {
      accounts: {
        mint: MINT_ACCOUNT,
        recipient: RECIPIENT,
        mintAuthority: mintAuthority,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [provider.wallet.payer],
    });

    console.log(`Minted ${amount} tokens to ${RECIPIENT.toString()}`);
  } catch (err) {
    console.error("Minting failed with error:", err);
  }
}

// Function to burn tokens
async function burnTokens(amount) {
  const burnAuthority = provider.wallet.publicKey; // Default wallet address as burn authority

  try {
    await program.rpc.burnTokens(new anchor.BN(amount), {
      accounts: {
        mint: MINT_ACCOUNT,
        source: RECIPIENT,
        authority: burnAuthority,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [provider.wallet.payer],
    });

    console.log(`Burned ${amount} tokens from ${RECIPIENT.toString()}`);
  } catch (err) {
    console.error("Burning failed with error:", err);
  }
}

// Main execution block
(async () => {
  const [action, amountStr] = process.argv.slice(2);

  console.log("Action:", action);
  console.log("Amount:", amountStr);

  if (!amountStr) {
    console.error("Missing required amount parameter.");
    process.exit(1);
  }

  const amount = parseInt(amountStr, 10);

  if (action === 'mint') {
    await mintTokens(amount);
  } else if (action === 'burn') {
    await burnTokens(amount);
  } else {
    console.error("Invalid action. Use 'mint' or 'burn'.");
    process.exit(1);
  }
})();

