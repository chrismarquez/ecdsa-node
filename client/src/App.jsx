import Wallet from "./Wallet";
import Transfer from "./Transfer";
import "./App.scss";
import { useState } from "react";


function App() {
  const [balance, setBalance] = useState(0);
  const [address, setAddress] = useState(null);
  const [signer, setSigner] = useState(null);

  return (
    <div className="app">
      <Wallet
        balance={balance}
        setBalance={setBalance}
        address={address}
        setAddress={setAddress}
        setSigner={setSigner}
      />
      <Transfer setBalance={setBalance} signer={signer}/>
    </div>
  );
}

export default App;
