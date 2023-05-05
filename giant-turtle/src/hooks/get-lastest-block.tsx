import { Block, GetLatestBlockRequest } from "@giant-turtle/proto/pb_pb";
import { useState, useEffect, useMemo } from "react";
import useRpc from "./rpc";

export default function useLastestBlock() {
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
  }, [rpc]);

  return useMemo(() => block, [block]);
}
