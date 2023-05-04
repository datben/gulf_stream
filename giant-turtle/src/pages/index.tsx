import GetBalance from "@giant-turtle/components/get-balance";
import TxHistory from "@giant-turtle/components/tx-history";

import Image from "next/image";
import mypic from "../assets/giant-turtle.png";
import GetTx from "@giant-turtle/components/get-tx";
import GetLatestBlock from "@giant-turtle/components/get-lastest-block";
import MintTx from "@giant-turtle/components/mint";
import TransferTx from "@giant-turtle/components/transfer";
const GiantTurtle = () => {
  return (
    <Image src={mypic} alt="Picture of the author" width="150" height="100" />
  );
};

export default function Home() {
  return (
    <>
      <h1>
        <GiantTurtle></GiantTurtle> Giant Turtle, a gulf-stream explorer !
      </h1>
      <div>
        <h3>Latest Block : </h3>
        <GetLatestBlock></GetLatestBlock>
        <h3>Tx History : </h3>
        <TxHistory></TxHistory>
        <h3>Send Tx : </h3>
        <h4>Mint</h4>
        <MintTx></MintTx>
        <h4>Transfer</h4>
        <TransferTx></TransferTx>
        <h3>Get Balance : </h3>
        <GetBalance></GetBalance>
        <h3>Get Tx : </h3>
        <GetTx></GetTx>
      </div>
    </>
  );
}
