import { Transaction } from "@giant-turtle/proto/pb_pb";
import { base58 } from "@scure/base";

import Image from "next/image";
import mypic from "../assets/seashell.png";
const SeaShell = () => {
  return (
    <Image src={mypic} alt="Picture of the author" width="20" height="20" />
  );
};

export default function TransactionCard(tx: Transaction) {
  return (
    <>
      {" "}
      tx : {base58.encode(tx.getSignature_asU8())}, payer :{" "}
      {base58.encode(tx.getPayer_asU8())}, gas : {tx.getGas()}{" "}
      <SeaShell></SeaShell>
    </>
  );
}
