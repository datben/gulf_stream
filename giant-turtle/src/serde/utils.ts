export function u64ToArray(n: number) {
  return [0, 0, 0, 0, 0, 0, 0, 0].map((val, i) => {
    return (n / 2 ** (8 * i)) & 0xff;
  });
}

export function getU64fromArray(data: Uint8Array) {
  let res = 0;
  for (let i = 0; i < 8; i++) {
    res = res + data[i] * 2 ** i;
  }
  return res;
}

export type TransactionMessage = {
  mint?: { amount: number };
  transfer?: { to: Uint8Array; amount: number };
};

export function decodeTransactionMessage(data: Uint8Array): TransactionMessage {
  const index = data[0];
  if (index === 0) {
    const amount = getU64fromArray(data.slice(1));
    return { mint: { amount: amount } };
  } else {
    const pk = data.slice(1, 33);
    const amount = getU64fromArray(data.slice(33));
    return {
      transfer: {
        to: pk,
        amount: amount,
      },
    };
  }
}
