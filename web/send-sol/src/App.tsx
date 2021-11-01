import React from 'react';
import { SendOneLamportToRandomAddress } from './sendLamport';
import { WalletMultiButton, WalletDisconnectButton } from '@solana/wallet-adapter-react-ui';

function App() {
  return (
    <div className="App">
      <h1>Wallet Connect Button</h1>
      <WalletMultiButton />
      <WalletDisconnectButton />
      <h1>Send One Lamport</h1>
      <SendOneLamportToRandomAddress />
    </div>
  );
}

export default App;
