import { PageView } from ".";
import { getKeplrFromWindow } from "@keplr-wallet/stores";
import {
  CHAIN_ID,
  CHAIN_RPC_URL,
  FLOOB_ADDR,
  FLOOB_DAO_PROPOSAL_ADDR,
} from "../lib/io";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { createRef, useEffect, useRef, useState } from "react";
import { GasPrice } from "@cosmjs/stargate";
import { Loadable, useRecoilValueLoadable } from "recoil";
import { walletAddrSel } from "../lib/atom";
import { toBase64, toUtf8 } from "@cosmjs/encoding";

type ThreadData = {
  title: string;
  description: string;
  content: string[];
};

const onSubmit = async ({
  signer,
  addr,
  data,
}: {
  signer: SigningCosmWasmClient;
  addr: string;
  data: ThreadData;
}) => {
  const wasmMsg = {
    create_thread: {
      ...data,
    },
  };
  const daodaoMsg = {
    propose: {
      msg: {
        propose: {
          title: `New Floob Story - ${data.title}`,
          description: `${data.description}`,
          msgs: [
            {
              wasm: {
                execute: {
                  contract_addr: FLOOB_ADDR,
                  funds: [],
                  msg: toBase64(toUtf8(JSON.stringify(wasmMsg))),
                },
              },
            },
          ],
        },
      },
    },
  };
  console.log(daodaoMsg);

  const res = await signer.execute(
    addr,
    FLOOB_DAO_PROPOSAL_ADDR,
    daodaoMsg,
    "auto",
    "0.0025ujuno",
    undefined
  );
};

export const useSigner = (): SigningCosmWasmClient | undefined => {
  const [signer, setSigner] = useState<SigningCosmWasmClient | undefined>();
  useEffect(() => {
    const loadSigner = async () => {
      const keplr = (await getKeplrFromWindow());
      if (!keplr)
        return

      await keplr.enable(CHAIN_ID);
      const offlineSigner = keplr.getOfflineSigner(CHAIN_ID);
      const client = await SigningCosmWasmClient.connectWithSigner(
        CHAIN_RPC_URL,
        offlineSigner,
        {
          gasPrice: GasPrice.fromString("0.0025ujuno"),
        }
      );
      setSigner(client);
    };
    loadSigner();
  }, []);

  return signer;
};

export const getLoadable = <T,>(loadable: Loadable<T>): T | undefined =>
  loadable.state == "hasValue" ? loadable.contents : undefined;

// Need to have title
// Description
// And body text with no rich text
// And a submit button
// - That compiles everything and send a proposal to the DAO
const NewStory = () => {
  const signer = useSigner();
  const walletAddr = getLoadable(useRecoilValueLoadable(walletAddrSel));
  const titleRef = createRef<HTMLDivElement>();
  const desRef = createRef<HTMLDivElement>();
  const contentRef = createRef<HTMLDivElement>();

  return (
    <PageView>
      <div className="max-w-lg flex flex-col justify-start items-start">
        {/* This keeps component from resizing. */}
        <div className="transparent w-[1000px] max-w-lg" />
        <div
          contentEditable="true"
          className="text-xl w-full"
          data-ph="Floob story"
          ref={titleRef}
        />
        <div
          contentEditable="true"
          className="text-secondary w-full"
          data-ph="Floob story description"
          ref={desRef}
        />
        <div
          contentEditable="true"
          className="py-4 w-full"
          data-ph="All things come from humble beginnings..."
          ref={contentRef}
        >
          {`In a far-off corner of the universe lies a vast and sprawling empire, ruled with an iron fist 
          by the notorious intergalactic dictator, Floob. For years, he has wielded his power with impunity, 
          subjugating entire planets and civilizations to his will, crushing all those who dared to oppose him.`}
        </div>

        <div className="py-4">
          <button
            className="bg-primary text-black text-sm font-medium px-4 py-2 border border-transparent hover:text-white hover:bg-black hover:border-white"
            onClick={() => {
              const title = titleRef.current?.innerText ?? "";
              const description = desRef.current?.innerText ?? "";
              const content =
                contentRef.current?.innerText
                  .split("\n")
                  .filter((x) => x != "") || [];

              signer &&
                walletAddr &&
                onSubmit({
                  signer,
                  addr: walletAddr,
                  data: { title, description, content },
                });
            }}
          >
            Submit Proposal
          </button>
        </div>
      </div>
    </PageView>
  );
};

export default NewStory;
