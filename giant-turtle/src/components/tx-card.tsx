import { Transaction } from "@giant-turtle/proto/pb_pb";
import { base58 } from "@scure/base";
import { SeaShell } from "./sea-shell";
import {
  TransactionMessage,
  decodeTransactionMessage,
} from "@giant-turtle/serde/utils";

export function TransactionResumeCard(tx: Transaction) {
  return (
    <>
      {" "}
      tx : {base58.encode(tx.getSignature_asU8())}, payer :{" "}
      {base58.encode(tx.getPayer_asU8())}, gas : {tx.getGas()}{" "}
      {SeaShell(20, 20)}
    </>
  );
}

export function TransactionCard(tx: Transaction | undefined) {
  if (tx) {
    return (
      <>
        <li>blockheight : {tx.getBlockheight()}</li>
        <li>signature : {base58.encode(tx.getSignature_asU8())}</li>
        <li>
          msg :{" "}
          {displayTransactionMessage(
            decodeTransactionMessage(tx.getMsg_asU8())
          )}
        </li>
        <li>payer : {base58.encode(tx.getPayer_asU8())}</li>
        <li>
          gas : {tx.getGas()} {SeaShell(20, 20)}
        </li>
      </>
    );
  } else {
    return <>Tx not found</>;
  }
}

function displayTransactionMessage(msg: TransactionMessage) {
  if (msg.mint) {
    return (
      <>
        Minted : {msg.mint.amount} {SeaShell(20, 20)}
      </>
    );
  } else if (msg.transfer) {
    return (
      <>
        Transfer : {msg.transfer.amount} {SeaShell(20, 20)} to{" "}
        {base58.encode(msg.transfer.to)}
      </>
    );
  } else {
    return <>None</>;
  }
}
