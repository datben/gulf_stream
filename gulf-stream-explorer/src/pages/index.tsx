import { GetHistoryRequest } from "@/proto/pb_pb";
import { NodeClient } from "@/proto/pb_pb_service";
import { useEffect } from "react";

export default function Home() {
  useEffect(() => {
    const rpc = new NodeClient("http://0.0.0.0:50051");
    rpc.getHistory(new GetHistoryRequest(), (e, v) => {
      console.log(e, v);
    });
  });
  return (
    <>
      <div>
        <p>Test</p>
      </div>
    </>
  );
}
