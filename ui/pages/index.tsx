import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { getKeplrFromWindow } from "@keplr-wallet/stores";
import type { NextPage } from "next";
import Head from "next/head";
import { PlusIcon } from "../public/icons/PlusIcon";

const stories: Storyline[] = [
  {
    title: "How Floob came to power",
    description:
      "A tale of mischief, cunning, and evil of the greatest villain in the Cosmos who-ever lived.",
    blocks: [
      {
        text: `In the year 2142, Floob was 
        a mere youngling of the Goober race in the Andromeda Galaxy,
        Planet 0x23df943 in the Boober system. Before he took the 
        terrifying name Floob and spread terror to the intergalaxies,
        he was known as Floobydoo of the family Floobyooby.`,
      },
      {
        text: `Floobyooby was an aristocratic family that fell
        from Imperial favor after the Oooomber Coup. Before Floob 
        came to power and banned the song, the Goobers used to 
        sing:`,
      },
    ],
  },
];

type Thread = {
  id: string;
  title: string;
  description: string;
  content: string[];
};

const THREADS: Thread[] = [
  {
    id: "0",
    title: "How Floob came to power",
    description:
      "A tale of mischief, cunning, and evil of the greatest villain in the Cosmos who-ever lived.",
    content: [
      `In the year 2142, Floob was 
      `,
    ],
  },
];

type HeaderProps = {
  threads: { title: string; id: string }[];
};

const HeaderView = ({ threads }: HeaderProps) => {
  return (
    <div className="flex">
      <div className="absolute -mx-16">
        <div className="h-10 w-10 bg-cta rounded-full cursor-pointer flex items-center justify-center hover:opacity-80">
          <PlusIcon width={24} height={24} strokeWidth={2} />
        </div>
      </div>
      {threads.map((t, i) => (
        <div
          key={i}
          className="text-xl font-bold text-primary cursor-pointer hover:opacity-80 max-w-[140px] hover:underline"
        >
          {t.title}
        </div>
      ))}
    </div>
  );
};

type Storyline = {
  title: string;
  description: string;
  blocks: Block[];
};

// Would be cool to do rich text but that's going
// to be harder
type Block = {
  text: string;
};

const StorylineView = ({ storyline }: { storyline: Storyline }) => {
  const { title, description, blocks } = storyline;
  return (
    <div className="max-w-md">
      <div className="py-4">
        <p className="text-primary text-2xl font-bold">{title}</p>
        <p className="text-secondary text-xl">{description}</p>
      </div>
      <div>
        {blocks.map((block, i) => (
          <div key={i} className="relative py-2">
            <div className="pb-1 leading-relaxed">{block.text}</div>
          </div>
        ))}
      </div>
    </div>
  );
};

const CHAIN_ID = "juno-1";

const onConnectWalletClick = async () => {
  const keplr = await getKeplrFromWindow();
  if (!keplr)
    return alert("Connect wallet failed. Download Keplr wallet to continue");

  await keplr.enable(CHAIN_ID);
};

const CONTRACT_ADDR = "";

const getThreadData = async ({
  id,
  client,
}: {
  id: string;
  client: CosmWasmClient;
}) => {
  const thread = await client.queryContractSmart(CONTRACT_ADDR, {
    get_thread: { id },
  });
  return thread as Thread;
};

const Home: NextPage = () => {
  return (
    <div>
      <Head>
        <title>Floob DAO</title>
        <meta name="description" content="Generated by create next app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div className="py-2">
        <div className="w-full flex items-center justify-center">
          <div className="flex flex-col">
            <div className="flex py-12">
              <div>
                <HeaderView threads={THREADS} />
              </div>
            </div>
            <StorylineView storyline={stories[0]} />
            {/* Add new sub-thread */}
            <div className="py-4 relative">
              {/* Input component. Invisible for now */}
              <div className="bg-[#1E1E1E] rounded-md invisible">
                <textarea
                  cols={40}
                  rows={2}
                  placeholder="New subthread story..."
                  className="w-full bg-[#1E1E1E] rounded-sm px-6 py-4 outline-none"
                />
                <div className="px-6 py-4">
                  <input
                    type="button"
                    value="Post"
                    className="text-cta text-sm font-semibold rounded-sm outline-none cursor-pointer hover:opacity-80"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div className="py-24"></div>
    </div>
  );
};

export default Home;
