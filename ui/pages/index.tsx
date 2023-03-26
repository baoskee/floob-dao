import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { getKeplrFromWindow } from "@keplr-wallet/stores";
import type { NextPage } from "next";
import { DependencyList, FC, useEffect, useState } from "react";
import Head from "next/head";
import clsx from "clsx";

type Thread = {
  id: number;
  title: string;
  description: string;
  content: string[];
};

type HeaderProps = {
  titles: string[];
  onHeaderClick: (headIdx: number) => void;
  selected?: number;
};

const Link = ({ href, children }: { href: string; children: string }) => (
  <a
    href={href}
    target="_blank"
    rel="noreferrer" 
    className={clsx(
      "opacity-60 transition-opacity 100ms ease-in-out",
      "hover:opacity-100"
    )}
  >
    {children}
  </a>
);

const WhatIsFloobZone = () => (
  <div className="flex flex-col gap-2">
    <h1 className="text-primary text-2xl font-bold">What is Floob.zone?</h1>
    <p>
      Floob is a community-driven, decentralized, and{" "}
      <Link href="https://github.com/baoskee/floob-dao">open-source</Link>{" "}
      story-telling smart contract application controlled by{" "}
      <Link href="https://daodao.zone/dao/juno15aka2ufg4xg6et76q3dg95q798eusphft0yuddl764kc2wfkxaxs43r32f#proposals">
        FloobDAO
      </Link>
      .
    </p>
    <p>
      Members can propose and vote on new stories and sub-stories to be
      immortalized on-chain forever. The stories on this website are generated
      from a smart contract on Juno mainnet.
    </p>
    <p>
      The FloobDAO token holders can vote on the direction of the story,
      evolving the Floob universe to their liking.
    </p>
    <p>
      You can read the first story,{" "}
      <Link href="https://localhost:3000?id=0">{"How Floob came to power"}</Link>.
    </p>
    <p>
      This project is created and maintained by{" "}
      <Link href="https://twitter.com/baoskee">baoskee</Link>. It is intended to
      be a fun and educational project personally, and an experiment in
      decentralized story-telling for the Juno community. Never buy FLOOB, just
      ask and we will mint.
    </p>

    <p className="py-2" />

    <h1 className="text-primary text-2xl font-bold">What is FloobDAO?</h1>
    <p>
      FloobDAO is a <Link href="https://daodao.zone">Da0_Da0</Link> DAO that is
      governed by FloobDAO token holders. FloobDAO token holders can create and
      edit stories around the Galactic Floob storyline by creating a proposal
      and getting it passed.
    </p>
  </div>
);

const HeaderView: FC<HeaderProps> = ({ titles, selected, onHeaderClick }) => {
  return (
    <div className="flex flex-row px-8 py-4 bg-black items-baseline gap-2 overflow-x-scroll">
      <div className="flex flex-row gap-8 items-baseline text-sm font-medium overflow-x-scroll">
        {titles.map((t, i) => (
          <div
            key={i}
            onClick={() => onHeaderClick(i)}
            className={clsx(
              "text-sm text-primary cursor-pointer",
              "opacity-50 hover:opacity-100 transition-opacity 100ms ease-in-out",
              selected == i && "opacity-100"
            )}
          >
            {t}
          </div>
        ))}
      </div>
    </div>
  );
};

type Storyline = {
  title: string;
  description: string;
  content: Block[];
};

// Would be cool to do rich text but that's going
// to be harder
type Block = string;

const StorylineView = ({ storyline }: { storyline?: Storyline }) => {
  if (!storyline) return <div></div>;

  const { title, description, content: blocks } = storyline;
  return (
    <div className="max-w-lg">
      <div className="py-4">
        <p className="text-primary text-2xl font-bold">{title}</p>
        <p className="text-secondary text-xl">{description}</p>
      </div>
      <div>
        {blocks.map((block, i) => (
          <div key={i} className="relative">
            <div className="pb-2 leading-relaxed">{block}</div>
          </div>
        ))}
      </div>
    </div>
  );
};

const onConnectWalletClick = async () => {
  const keplr = await getKeplrFromWindow();
  if (!keplr)
    return alert("Connect wallet failed. Download Keplr wallet to continue");

  await keplr.enable(CHAIN_ID);
};

// MARK: Environmental variables

// This changes for everyone's local Docker env
const FLOOB_ADDR =
  "juno1sz6rh5av98ed9d5edejv8vgv2t64schjvwrcwclw309r6hvy6ztssy8nqk";
const CHAIN_ID = "juno-1";
const RPC_HOST = "https://rpc-juno.itastakers.com";
const RPC_PORT = "443";

// MARK: Smart contract Queries
const getThreadData = async ({
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

const getThreads = async ({ client }: { client: CosmWasmClient }) => {
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

// MARK: View
const Home: NextPage = () => {
  const [headerIdx, setHeaderIdx] = useState(0);
  const threads = useAwaited(async () => {
    const client = await CosmWasmClient.connect(RPC_HOST + ":" + RPC_PORT);
    const threads = await getThreads({ client });
    return threads as Thread[];
  }, []);

  const displayedThread = useAwaited(async () => {
    const client = await CosmWasmClient.connect(RPC_HOST + ":" + RPC_PORT);
    const thread = await getThreadData({
      id: Math.max(headerIdx - 1, 0),
      client,
    });
    return thread;
  }, [headerIdx]);

  return (
    <div>
      <Head>
        <title>Floob DAO</title>
        <meta name="description" content="Generated by create next app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <HeaderView
        titles={
          threads
            ? [
                "What is Floob.zone and FloobDAO?",
                ...threads.map((t) => t.title),
              ]
            : ["What is Floob.zone and FloobDAO?"]
        }
        selected={headerIdx}
        onHeaderClick={(i) => setHeaderIdx(i)}
      />
      <div className="py-12">
        <div className="w-full flex items-center justify-center">
          <div className="flex flex-col">
            <div className="max-w-lg">
              {headerIdx == 0 ? (
                <WhatIsFloobZone />
              ) : (
                <StorylineView storyline={displayedThread} />
              )}
            </div>
          </div>
        </div>
      </div>
      <div className="py-24"></div>
    </div>
  );
};

export default Home;
