import useRpc from "@giant-turtle/hooks/rpc";
import { GetBalanceRequest } from "@giant-turtle/proto/pb_pb";
import { base58 } from "@scure/base";
import { useState } from "react";
import { SeaShell } from "./sea-shell";

export default function GetBalance() {
  const rpc = useRpc();
  const [address, setAddress] = useState<string>("");
  const [balance, setBalance] = useState(0);

  const handleChange = (event: any) => {
    setAddress(event.target.value);
  };
  const handleSubmit = (event: any) => {
    const req = new GetBalanceRequest();
    req.setAddress(base58.decode(address));
    rpc.getBalance(req, (e, v) => {
      if (v) {
        setBalance(v.getBalance());
      } else {
        setBalance(0);
      }
    });
    event.preventDefault();
  };

  return (
    <>
      <form onSubmit={handleSubmit}>
        <label>
          Address :
          <textarea
            value={address}
            onChange={handleChange}
            rows={1}
            cols={50}
          />{" "}
        </label>
        <input type="submit" value="Fetch" />
      </form>
      <>
        Balance : {balance} {SeaShell(20, 20)}
      </>
    </>
  );
}
