import { NextPage } from "next";
import { useRouter } from "next/router";
import { StoryPage } from ".";

const Page: NextPage = () => {
  const router = useRouter();
  const id = parseInt(router.query.id as string) || 0;

  return <StoryPage id={id} />;
};

export default Page;
