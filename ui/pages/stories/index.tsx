import { getThreadData, useAwaited } from "../../lib/io";
import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { RPC_HOST, RPC_PORT } from "../../lib/io";
import { PageView } from "..";
import { NavElem } from "../../lib/components";
import { useRecoilValueLoadable } from "recoil";
import { threadsSel } from "../../lib/atom";
import { useRouter } from "next/router";

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
      {/* Some bottom padding */}
      <div className="py-12"/> 
    </div>
  );
};

export const StoryPage = ({ id }: { id: number }) => {
  const router = useRouter();
  const threads = useRecoilValueLoadable(threadsSel);
  const displayedThread = useAwaited(async () => {
    const client = await CosmWasmClient.connect(RPC_HOST + ":" + RPC_PORT);
    const thread = await getThreadData({
      id,
      client,
    });
    return thread;
  }, [id]);

  return (
    <PageView>
      <div className="absolute top-0 left-0 flex flex-row w-full px-8 py-3">
        {threads.state == "hasValue"
          ? threads.contents.map((thread, i) => (
              <NavElem
                selected={i == 0}
                onClick={() => router.push(`/stories/${i}`)}
              >
                <p className="font-medium">
                  {thread.title} 
                </p>
              </NavElem>
            ))
          : undefined}
      </div>
      <StorylineView storyline={displayedThread} />
    </PageView>
  );
};

const Stories = () => <StoryPage id={0} />;

export default Stories;
