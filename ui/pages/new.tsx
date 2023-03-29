import { PageView } from ".";
import { getKeplrFromWindow } from "@keplr-wallet/stores";
import { CHAIN_ID, CHAIN_RPC_URL, FLOOB_ADDR, FLOOB_DAO_PROPOSAL_ADDR } from "../lib/io";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { useEffect, useState } from "react";
import { GasPrice } from "@cosmjs/stargate";
import { Loadable, useRecoilValueLoadable } from "recoil";
import { walletAddrSel } from "../lib/atom";

const onSubmit = async ({
  signer,
  addr,
}: {
  signer: SigningCosmWasmClient;
  addr: string;
}) => {
  const wasmMsg = {
    create_thread: {
      title: "Floob story",
      description: "Floob story description",
      content: ["Hello world"],
    },
  };

  signer.execute(
    addr,
    FLOOB_DAO_PROPOSAL_ADDR,
    {
      propose: {
        msg: {
          propose: {
            title: "New Floob Story",
            description: "Insert description here...",
            msgs: [
              // You need to create a wasm execute message here
              {
                wasm: {
                  execute: {
                    contract: FLOOB_ADDR,
                    funds: [],
                    msg: wasmMsg
                  },
                },
              },
            ],
          },
        },
      },
    },
    "auto",
    undefined,
    undefined
  );
};

export const useSigner = (): SigningCosmWasmClient | undefined => {
  const [signer, setSigner] = useState<SigningCosmWasmClient | undefined>();
  useEffect(() => {
    const loadSigner = async () => {
      const keplr = (await getKeplrFromWindow())!;
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

  return (
    <PageView>
      <div className="max-w-lg flex flex-col justify-start items-start">
        {/* This keeps component from resizing. */}
        <div className="transparent w-[1000px]" />
        <div
          contentEditable="true"
          className="text-xl w-full"
          data-ph="Floob story"
        ></div>
        <div
          contentEditable="true"
          className="text-secondary w-full"
          data-ph="Floob story description"
        ></div>
        <div
          contentEditable="true"
          className="py-4 w-full"
          data-ph="All things come from humble beginnings..."
        >
          {`Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
          eiusmod tempor incididunt ut labore et dolore magna aliqua. Morbi
          tincidunt ornare massa eget egestas purus viverra accumsan in. Leo
          integer malesuada nunc vel risus. Id porta nibh venenatis cras sed
          felis. Sed risus pretium quam vulputate dignissim. Viverra maecenas
          accumsan lacus vel facilisis volutpat est velit. Viverra mauris in
          aliquam sem fringilla. Vitae justo eget magna fermentum iaculis eu.
          Egestas diam in arcu cursus euismod quis viverra..`}
        </div>

        <div className="py-4">
          <button
            className="bg-primary text-black text-sm font-medium px-4 py-2 border border-transparent hover:text-white hover:bg-black hover:border-white"
            onClick={() =>
              signer && walletAddr && onSubmit({ signer, addr: walletAddr })
            }
          >
            Submit Proposal
          </button>
        </div>
      </div>
    </PageView>
  );
};

export default NewStory;
