import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Contracts } from "../target/types/contracts";
import { assert } from "chai";
const web3 = anchor.web3;

describe("Users", () => {

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const user = (provider.wallet as anchor.Wallet).payer;
  const program = anchor.workspace.contracts as Program<Contracts>;

  before(async () => {
    const balance = await provider.connection.getBalance(user.publicKey);
    const balanceInSOL = balance / web3.LAMPORTS_PER_SOL;
    const formattedBalance = new Intl.NumberFormat().format(balanceInSOL);
    console.log(`Balance: ${formattedBalance} SOL`);
  });

  // Here's what we want to write to the blockchain
  const id = new anchor.BN(2);
  const name = 'Himanshu';
  const hobbies = ['badminton', 'boxing', 'cricket'];

  it('add user', async () => {
    await program.methods.addUser(id, name, hobbies).signers([user]).rpc();
    const userPdaAndBump = web3.PublicKey.findProgramAddressSync([Buffer.from('users'), user.publicKey.toBuffer()], program.programId);
    const dataFromPda = await program.account.user.fetch(userPdaAndBump[0]);
    assert.equal(dataFromPda.name, name);
    assert.equal(dataFromPda.id.toString(), id.toString());
    assert.deepEqual(dataFromPda.hobbies, hobbies);
  });

  it('update user', async () => {
    const updateHobbies = ['badminton', 'playing', 'cricket'];
    await program.methods.updateUser(name, updateHobbies).signers([user]).rpc();
    const userPdaAndBump = web3.PublicKey.findProgramAddressSync([Buffer.from('users'), user.publicKey.toBuffer()], program.programId);
    const dataFromPda = await program.account.user.fetch(userPdaAndBump[0]);
    assert.deepEqual(dataFromPda.hobbies, updateHobbies);
    assert.equal(dataFromPda.id.toString(), id.toString());
  });

  it('Rejects transactions from unauthorized signers', async () => {
    try {
      const someRandomGuy = anchor.web3.Keypair.generate();
      await program.methods.updateUser(name, hobbies).signers([someRandomGuy]).rpc();
    } catch (error) {
      const errorMessage = (error as Error).message;
      assert.isTrue(errorMessage.includes('unknown signer'));
    }
  });

  it('delete user', async () => {
    await program.methods.deleteUser().rpc();
    const userPdaAndBump = web3.PublicKey.findProgramAddressSync([Buffer.from('users'), user.publicKey.toBuffer()], program.programId);
    // Verify the account no longer exists
    try {
      await program.account.user.fetch(userPdaAndBump[0]);
      assert.fail('Account should have been deleted');
    } catch (error) {
      const errorMessage = (error as Error).message;
      assert.isTrue(
        errorMessage.includes('Account does not exist') ||
        errorMessage.includes('AccountNotFound'),
        'Expected account to be deleted'
      );
    }
  });


});
