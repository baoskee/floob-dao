import { getKeplrFromWindow } from "@keplr-wallet/stores";
import { selector } from "recoil";

const CHAIN_ID = "juno-1";

export const walletAddrSel = selector({
  key: "wallet_addr",
  get: async () => {
    try {
      const keplr = await getKeplrFromWindow();
      if (!keplr) return undefined;

      await keplr.enable(CHAIN_ID);
      const walletAddress = (await keplr.getKey(CHAIN_ID)).bech32Address;
      return walletAddress;
    } catch {
      return undefined;
    }
  },
});
