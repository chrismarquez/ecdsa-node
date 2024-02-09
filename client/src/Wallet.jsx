import server from "./server";
import {BrowserProvider} from "ethers";
import {useEffect} from "react";

const provider = new BrowserProvider(window.ethereum);

async function getSigner() {
    await connectWallet();
    return provider.getSigner();
}

async function connectWallet() {
    try {
        await provider.send('eth_requestAccounts', []);
    } catch (e) {
        console.log('user rejected request');
    }
}

function Wallet({address, setAddress, balance, setBalance, setSigner}) {

    useEffect(() => {
        async function fetch() {
            if (address) {
                const {
                    data: {balance},
                } = await server.get(`balance/${address}`);
                setBalance(balance);
            } else {
                setBalance(0);
            }
        }
        fetch();
    }, [address]);

    useEffect(() => {
        if (!window.ethereum) {
            alert("This app requires an Ethereum Wallet to work. We suggest you install Metamask");
            return;
        }
        getSigner().then(signer => {
            setSigner(signer);
            setAddress(signer.address);
        });
    }, []);

    return (
        <div className="container wallet">
            <h1>Your Wallet</h1>
            <label>
                <div className="balance">Your Wallet Address: {address}</div>
            </label>
            <div className="balance">Balance: {balance}</div>
        </div>
    );
}

export default Wallet;
