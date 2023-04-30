import useRpc from "@giant-turtle/hooks/rpc";
import { GetHistoryRequest, Transaction } from "@giant-turtle/proto/pb_pb";
import { useEffect, useState } from "react";
import { TransactionResumeCard } from "./tx-card";

export default function TxHistory() {
  const [history, setHistory] = useState<Transaction[]>([]);
  const rpc = useRpc();
  useEffect(() => {
    const call = setInterval(
      () =>
        rpc.getHistory(new GetHistoryRequest(), (e, v) => {
          if (v) {
            setHistory(v.getTransactionsList());
          }
        }),
      5000
    );
    return () => clearInterval(call);
  });
  return (
    <>
      {history.map((tx, i) => (
        <li key={i}>{TransactionResumeCard(tx)}</li>
      ))}
    </>
  );
}
