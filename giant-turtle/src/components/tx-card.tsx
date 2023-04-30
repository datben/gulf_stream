import { Transaction } from "@giant-turtle/proto/pb_pb";
import { base58 } from "@scure/base";

export default function TransactionCard(tx: Transaction) {
  return (
    <>
      {" "}
      tx : {base58.encode(tx.getSignature_asU8())}, payer :{" "}
      {base58.encode(tx.getPayer_asU8())}, gas : {tx.getGas()} Seashell
    </>
  );
}
