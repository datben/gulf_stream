import { NodeClient } from "@giant-turtle/proto/pb_pb_service";
import { useMemo } from "react";

export default function useRpc() {
  const endpoint = "http://0.0.0.0:50051";
  return useMemo(() => new NodeClient(endpoint), [endpoint]);
}
