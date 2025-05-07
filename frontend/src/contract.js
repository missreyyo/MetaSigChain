import { Server, Contract, TransactionBuilder, Address, nativeToScVal } from 'soroban-client';

const CONTRACT_ADDRESS = 'CCQWMCHX6GPZDCKZRACJM35FRSA5M6BNG5Q23GRT4IPGG7I6P5ZAURV4';
const NETWORK_PASSPHRASE = 'Test SDF Network ; September 2015';
const server = new Server('https://soroban-testnet.stellar.org:443');

export const transfer = async (from, to, amount) => {
  const source = await server.getAccount(from);
  const contract = new Contract(CONTRACT_ADDRESS);
  const fromAddr = new Address(from);
  const toAddr = new Address(to);
  const amtVal = nativeToScVal(amount, { type: 'i128' });
  const tx = new TransactionBuilder(source, {
    fee: '100',
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(contract.call('transfer', fromAddr, toAddr, amtVal))
    .setTimeout(30)
    .build();
  return tx.toXDR();
};

export const approve = async (from, spender, amount, expiration_ledger) => {
  const source = await server.getAccount(from);
  const contract = new Contract(CONTRACT_ADDRESS);
  const fromAddr = new Address(from);
  const spenderAddr = new Address(spender);
  const amtVal = nativeToScVal(amount, { type: 'i128' });
  const expVal = nativeToScVal(expiration_ledger, { type: 'u32' });
  const tx = new TransactionBuilder(source, {
    fee: '100',
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(contract.call('approve', fromAddr, spenderAddr, amtVal, expVal))
    .setTimeout(30)
    .build();
  return tx.toXDR();
};

export const transferFrom = async (spender, from, to, amount) => {
  const source = await server.getAccount(spender);
  const contract = new Contract(CONTRACT_ADDRESS);
  const spenderAddr = new Address(spender);
  const fromAddr = new Address(from);
  const toAddr = new Address(to);
  const amtVal = nativeToScVal(amount, { type: 'i128' });
  const tx = new TransactionBuilder(source, {
    fee: '100',
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(contract.call('transfer_from', spenderAddr, fromAddr, toAddr, amtVal))
    .setTimeout(30)
    .build();
  return tx.toXDR();
};

export const burn = async (from, amount) => {
  const source = await server.getAccount(from);
  const contract = new Contract(CONTRACT_ADDRESS);
  const fromAddr = new Address(from);
  const amtVal = nativeToScVal(amount, { type: 'i128' });
  const tx = new TransactionBuilder(source, {
    fee: '100',
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(contract.call('burn', fromAddr, amtVal))
    .setTimeout(30)
    .build();
  return tx.toXDR();
};

export const burnFrom = async (spender, from, amount) => {
  const source = await server.getAccount(spender);
  const contract = new Contract(CONTRACT_ADDRESS);
  const spenderAddr = new Address(spender);
  const fromAddr = new Address(from);
  const amtVal = nativeToScVal(amount, { type: 'i128' });
  const tx = new TransactionBuilder(source, {
    fee: '100',
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(contract.call('burn_from', spenderAddr, fromAddr, amtVal))
    .setTimeout(30)
    .build();
  return tx.toXDR();
};

export const freezeAccount = async (admin, account) => {
  const source = await server.getAccount(admin);
  const contract = new Contract(CONTRACT_ADDRESS);
  const adminAddr = new Address(admin);
  const accountAddr = new Address(account);
  const tx = new TransactionBuilder(source, {
    fee: '100',
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(contract.call('freeze_account', adminAddr, accountAddr))
    .setTimeout(30)
    .build();
  return tx.toXDR();
};

export const unfreezeAccount = async (admin, account) => {
  const source = await server.getAccount(admin);
  const contract = new Contract(CONTRACT_ADDRESS);
  const adminAddr = new Address(admin);
  const accountAddr = new Address(account);
  const tx = new TransactionBuilder(source, {
    fee: '100',
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(contract.call('unfreeze_account', adminAddr, accountAddr))
    .setTimeout(30)
    .build();
  return tx.toXDR();
};

export const setupMultisig = async (admin, owners, threshold) => {
  const source = await server.getAccount(admin);
  const contract = new Contract(CONTRACT_ADDRESS);
  const ownersArr = Array.isArray(owners) ? owners.map(addr => new Address(addr)) : owners.split(',').map(s => new Address(s.trim()));
  const tx = new TransactionBuilder(source, {
    fee: '100',
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(contract.call('setup_multisig', ownersArr, nativeToScVal(threshold, { type: 'u32' })))
    .setTimeout(30)
    .build();
  return tx.toXDR();
};

export const proposeMultisigTransaction = async (owner, functionName, to, amount, expiration) => {
  const source = await server.getAccount(owner);
  const contract = new Contract(CONTRACT_ADDRESS);
  const toAddr = new Address(to);
  const amtVal = nativeToScVal(amount, { type: 'i128' });
  const expVal = nativeToScVal(expiration, { type: 'u64' });
  const tx = new TransactionBuilder(source, {
    fee: '100',
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(contract.call('propose_multisig_transaction', functionName, toAddr, amtVal, expVal))
    .setTimeout(30)
    .build();
  return tx.toXDR();
};

export const approveMultisigTransaction = async (owner, transactionId) => {
  const source = await server.getAccount(owner);
  const contract = new Contract(CONTRACT_ADDRESS);
  const tx = new TransactionBuilder(source, {
    fee: '100',
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(contract.call('approve_multisig_transaction', nativeToScVal(transactionId, { type: 'u64' })))
    .setTimeout(30)
    .build();
  return tx.toXDR();
}; 