import GetBalance from "@giant-turtle/components/get-balance";
import TxHistory from "@giant-turtle/components/tx-history";

import Image from "next/image";
import mypic from "../assets/giant-turtle.jpg";
const MyImage = () => {
  return (
    <Image src={mypic} alt="Picture of the author" width="100" height="100" />
  );
};
export default function Home() {
  return (
    <>
      <h1>
        <MyImage></MyImage> Giant Turtle, a gulf-stream explorer !
      </h1>
      <div>
        <p>History : </p>
        <TxHistory></TxHistory>
        <p>Get Balance : </p>
        <GetBalance></GetBalance>
      </div>
    </>
  );
}
