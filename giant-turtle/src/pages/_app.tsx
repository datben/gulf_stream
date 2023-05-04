import { ContextProvider } from "@giant-turtle/hooks/wallet-ctx";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import type { AppProps } from "next/app";

require("@solana/wallet-adapter-react-ui/styles.css");

export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <ContextProvider>
        <WalletMultiButton></WalletMultiButton>
        <Component {...pageProps} />
      </ContextProvider>
      ;
    </>
  );
}
