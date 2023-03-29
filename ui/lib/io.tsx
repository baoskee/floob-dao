import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { DependencyList, useEffect, useState } from "react";
import { Thread } from "../pages/index";

// MARK: Environmental variables
// This changes for everyone's local Docker env
export const FLOOB_ADDR =
  "juno1sz6rh5av98ed9d5edejv8vgv2t64schjvwrcwclw309r6hvy6ztssy8nqk";
export const FLOOB_DAO_PROPOSAL_ADDR =
  "juno182768g6lsl7fqpk8magmn4yqpjcndaeevju3uju80y2dkjnpqwtsrffpyr";
export const CHAIN_ID = "juno-1";
export const RPC_HOST = "https://rpc-juno.itastakers.com";
export const RPC_PORT = "443";
export const CHAIN_RPC_URL = "https://rpc-juno.itastakers.com:443";

// MARK: Smart contract Queries
export const getThreadData = async ({
  id,
  client,
}: {
  id: number;
  client: CosmWasmClient;
}) => {
  const thread = await client.queryContractSmart(FLOOB_ADDR, {
    get_thread: { id },
  });
  return thread as Thread;
};

export const getThreads = async ({ client }: { client: CosmWasmClient }) => {
  const threads = await client.queryContractSmart(FLOOB_ADDR, {
    get_threads_created: {},
  });
  return threads;
};
// MARK: Hooks

export const useAwaited = <T,>(f: () => Promise<T>, deps: DependencyList) => {
  const [res, setRes] = useState<T>();
  useEffect(() => {
    (async () => {
      try {
        setRes(await f());
      } catch {}
    })();
  }, deps);

  return res;
};
