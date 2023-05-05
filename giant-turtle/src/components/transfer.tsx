import { Transaction } from "@giant-turtle/proto/pb_pb";
import { useWallet } from "@solana/wallet-adapter-react";
import { useState } from "react";
import { base58 } from "@scure/base";
import useRpc from "@giant-turtle/hooks/rpc";
import { SendTransactionRequest } from "@giant-turtle/proto/pb_pb";
import { u64ToArray } from "@giant-turtle/serde/utils";
import useLastestBlock from "@giant-turtle/hooks/get-lastest-block";

export default function TransferTx() {
  const [amount, setAmount] = useState<number>(0);
  const [tx, setTx] = useState<string>("");
  const [pk, setPk] = useState<string>("");

  const wallet = useWallet();
  const rpc = useRpc();
  const block = useLastestBlock();

  const handleChangeAmount = (event: any) => {
    setAmount(event.target.value);
  };
  const handleChangePk = (event: any) => {
    setPk(event.target.value);
  };
  const handleSubmit = async (event: any) => {
    event.preventDefault();

    if (wallet.signMessage && wallet.publicKey && block) {
      const tx = new Transaction();
      const blockheight = block.getIndex() + 1;
      const gas = 5;
      const msg = [1]
        .concat(Array.from(base58.decode(pk)))
        .concat(u64ToArray(amount));

      tx.setBlockheight(blockheight);
      tx.setGas(gas);
      tx.setMsg(new Uint8Array(msg));
      tx.setPayer(wallet.publicKey.toBytes());

      const toSign = new TextEncoder().encode(
        base58.encode(
          new Uint8Array(
            u64ToArray(blockheight).concat(u64ToArray(gas)).concat(msg)
          )
        )
      );

      console.log(toSign);
      const sign = await wallet.signMessage(toSign);

      tx.setSignature(sign);
      setTx(base58.encode(sign));

      const txRequest = new SendTransactionRequest();
      txRequest.setTx(tx);
      rpc.sendTransaction(txRequest, (err, msg) => {
        console.log(msg, err);
      });
    }
  };

  return (
    <>
      <form onSubmit={handleSubmit}>
        <label>
          Amount :
          <textarea
            value={amount}
            onChange={handleChangeAmount}
            rows={1}
            cols={20}
          />{" "}
        </label>
        <label>
          To Address :
          <textarea
            value={pk}
            onChange={handleChangePk}
            rows={1}
            cols={50}
          />{" "}
        </label>
        <input type="submit" value="Sign and send" />
      </form>
      Tx {tx}
    </>
  );
}
