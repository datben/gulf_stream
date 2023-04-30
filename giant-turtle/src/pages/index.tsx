import GetBalance from "@giant-turtle/components/get-balance";
import TxHistory from "@giant-turtle/components/tx-history";

import Image from "next/image";
import mypic from "../assets/giant-turtle.png";
import GetTx from "@giant-turtle/components/get-tx";
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
        <h3>Tx History : </h3>
        <TxHistory></TxHistory>
        <h3>Get Balance : </h3>
        <GetBalance></GetBalance>
        <h3>Get Tx : </h3>
        <GetTx></GetTx>
      </div>
    </>
  );
}
