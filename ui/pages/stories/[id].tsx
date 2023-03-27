import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { NextPage } from "next";
import { useRouter } from "next/router";
import { PageView } from "..";
import { getThreadData, RPC_HOST, RPC_PORT, useAwaited } from "../../lib/io";

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

const StoryPage: NextPage = () => {
  const router = useRouter();
  const id = parseInt(router.query.id as string) || 0;

  const displayedThread = useAwaited(async () => {
    const client = await CosmWasmClient.connect(RPC_HOST + ":" + RPC_PORT);
    const thread = await getThreadData({
      id: Math.max(id, 0),
      client,
    });
    return thread;
  }, [id]);

  return (
    <PageView>
      <StorylineView storyline={displayedThread} />
    </PageView>
  );
};

export default StoryPage;
