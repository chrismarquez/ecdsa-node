import { useState } from "react";
import server from "./server";


const domain = window.location.host;
const origin = window.location.origin;

async function signMessage(signer, message) {
  const rawMessage = JSON.stringify(message);
  const signature = await signer.signMessage(rawMessage);
  return { rawMessage, signature };
}

function Transfer({ setBalance, signer }) {
  const [sendAmount, setSendAmount] = useState("");
  const [recipient, setRecipient] = useState("");

  const setValue = (setter) => (evt) => setter(evt.target.value);

  async function transfer(evt) {
    evt.preventDefault();
    const request = {
      amount: parseInt(sendAmount),
      recipient,
    };
    const { rawMessage, signature } = await signMessage(signer, request);
    console.log(signature);
    try {
      const {
        data: { balance },
      } = await server.post(`send`, { rawMessage, signature });
      setBalance(balance);
    } catch (ex) {
      alert(ex.response.data.message);
    }
  }

  return (
    <form className="container transfer" onSubmit={transfer}>
      <h1>Send Transaction</h1>

      <label>
        Send Amount
        <input
          placeholder="1, 2, 3..."
          value={sendAmount}
          onChange={setValue(setSendAmount)}
        ></input>
      </label>

      <label>
        Recipient
        <input
          placeholder="Type an address, for example: 0x2"
          value={recipient}
          onChange={setValue(setRecipient)}
        ></input>
      </label>

      <input type="submit" className="button" value="Transfer" />
    </form>
  );
}

export default Transfer;
