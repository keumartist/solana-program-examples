import { WalletNotConnectedError } from '@solana/wallet-adapter-base';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { Keypair, SystemProgram, Transaction, TransactionError } from '@solana/web3.js';
import React, {  useCallback } from 'react';

export const SendOneLamportToRandomAddress  = () => {
    const { connection } = useConnection();
    const { publicKey, sendTransaction } = useWallet();

    console.log(publicKey)
    const onClick = useCallback(async () => {
        if (!publicKey) throw new WalletNotConnectedError();

        try { const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: publicKey,
                toPubkey: Keypair.generate().publicKey,
                lamports: 1,
            })
        );
       

        console.log("transaction")
        console.log(transaction)

        const signature = await sendTransaction(transaction, connection);
        console.log("signature")
        console.log(signature)

        const result = await connection.confirmTransaction(signature, 'processed');
        console.log(result)

        } catch(e) {
            alert("Transaction failed");
        }
    }, [publicKey, sendTransaction, connection]);


    return (
        <button onClick={onClick} disabled={!publicKey}>
            Send 1 lamport to a random address!
        </button>
    );
};