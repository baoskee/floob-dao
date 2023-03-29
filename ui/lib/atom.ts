import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { getKeplrFromWindow } from "@keplr-wallet/stores";
import { selector } from "recoil";
import { getThreads, RPC_HOST, RPC_PORT, CHAIN_ID } from "./io";

export type Thread = {
  id: number;
  title: string;
  description: string;
  content: string[];
};

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

export const threadsSel = selector({
  key: "threads",
  get: async () => {
    const client = await CosmWasmClient.connect(RPC_HOST + ":" + RPC_PORT);
    const threads = await getThreads({ client });
    return threads as Thread[];
  },
});
