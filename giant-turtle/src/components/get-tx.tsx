import useRpc from "@giant-turtle/hooks/rpc";
import { GetHistoryRequest, Transaction } from "@giant-turtle/proto/pb_pb";
import { base58 } from "@scure/base";
import { useState } from "react";
import { TransactionCard } from "./tx-card";

export default function GetTx() {
  const rpc = useRpc();
  const [sign, setSign] = useState<string>("");
  const [tx, setTx] = useState<Transaction | undefined>(undefined);

  const handleChange = (event: any) => {
    setSign(event.target.value);
  };
  const handleSubmit = (event: any) => {
    rpc.getHistory(new GetHistoryRequest(), (e, v) => {
      if (v && sign.length !== 0) {
        const transac = v.getTransactionsList().find((tx) => {
          return base58.encode(tx.getSignature_asU8()) === sign;
        });
        setTx(transac);
      }
    });

    event.preventDefault();
  };

  return (
    <>
      <form onSubmit={handleSubmit}>
        <label>
          Tx :
          <textarea
            value={sign}
            onChange={handleChange}
            rows={1}
            cols={120}
          />{" "}
        </label>
        <input type="submit" value="Fetch" />
      </form>
      Result : {TransactionCard(tx)}
    </>
  );
}
