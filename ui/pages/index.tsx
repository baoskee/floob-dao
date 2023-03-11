import type { NextPage } from "next";
import Head from "next/head";

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
  title: string;
};
const THREADS: Thread[] = [
  {
    title: "How Floob came to power",
  },
  {
    title: "How Floob grew up",
  },
  {
    title: "The Hubble Mission",
  },
];

type HeaderProps = {
  threads: Thread[];
};

const HeaderView = ({ threads }: HeaderProps) => {
  return (
    <div className="flex">
      {threads.map((t, i) => (
        <div
          key={i}
          className="text-xl font-bold text-primary cursor-pointer hover:opacity-80 max-w-[140px]"
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
        {blocks.map((block) => (
          <div className="py-2">
            {/* Decoration */}
            <div className="absolute flex gap-4 items-baseline -mx-36">
              <div className="text-cta font-semibold text-base cursor-pointer hover:opacity-80">
                juno03f...49l
              </div>
              <span className="relative flex h-2 w-2">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-white opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2 w-2 bg-white"></span>
              </span>
            </div>
            <div className="pb-1 leading-relaxed">{block.text}</div>
          </div>
        ))}
      </div>
    </div>
  );
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
          </div>
        </div>
      </div>
    </div>
  );
};

export default Home;
