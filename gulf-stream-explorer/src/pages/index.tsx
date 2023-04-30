import { GetHistoryRequest, TransactionHistory } from "@/proto/pb_pb";
import { NodeClient } from "@/proto/pb_pb_service";
import { base58 } from "@scure/base";
import { useEffect, useState } from "react";

export default function Home() {
  const [history, setHistory] = useState<Array<String> | null>(null);
  useEffect(() => {
    const rpc = new NodeClient("http://0.0.0.0:50051");
    rpc.getHistory(new GetHistoryRequest(), (e, v) => {
      if (v) {
        setHistory(
          v.getTransactionsList().map((tx) => {
            return base58.encode(tx.getSignature() as Uint8Array);
          })
        );
      }
    });
  });
  return (
    <>
      <div>
        <p>History {history}</p>
      </div>
    </>
  );
}
