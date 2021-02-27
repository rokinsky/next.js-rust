import { FC, useEffect } from "react";

import * as wasm from "wasm";

const GreetComponent: FC = () => {
  useEffect(() => {
    wasm.greet("<username>");
  }, []);

  return null;
};

export default GreetComponent;
