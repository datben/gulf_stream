import useRpc from "@giant-turtle/hooks/rpc";
import { Block, GetLatestBlockRequest } from "@giant-turtle/proto/pb_pb";
import { base58 } from "@scure/base";
import { useEffect, useState } from "react";

export default function GetLatestBlock() {
  const [block, setBlock] = useState<Block | undefined>(undefined);
  const rpc = useRpc();
  useEffect(() => {
    const call = setInterval(
      () =>
        rpc.getLatestBlock(new GetLatestBlockRequest(), (e, v) => {
          if (v) {
            setBlock(v.getBlock());
          }
        }),
      5000
    );
    return () => clearInterval(call);
  });
  if (block) {
    return (
      <>
        blockheight : {block.getIndex()} , blockhash :{" "}
        {base58.encode(block.getBlockhash_asU8())}
      </>
    );
  } else {
    return <>Loading ...</>;
  }
}
