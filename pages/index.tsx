import { FC } from "react";

import dynamic from "next/dynamic";

const GreetComponent = dynamic(() => import("@/src/greet-component"));

const Page: FC = () => {
  return <GreetComponent />;
};

export default Page;
