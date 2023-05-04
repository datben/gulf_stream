import useLastestBlock from "@giant-turtle/hooks/get-lastest-block";
import { base58 } from "@scure/base";

export default function GetLatestBlock() {
  const block = useLastestBlock();
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
