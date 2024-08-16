import { useState } from "react";
import { Button as StaticCssButton } from "./static-css";
import { Button as CssModulesButton } from "./css-modules";
import { Button as BemButton } from "./bem";
import { Button as CssModulesWithBemButton } from "./css-modules-with-bem";
import { Button as CssInJsButton } from "./css-in-js";
import { Button as CssInJsWithCssVariablesButton } from "./css-in-js-with-css-variables";
import { Button as AtomicCssButton } from "./atomic-css";
import { Button as VanillaExtractButton } from "./zero-runtime-css-in-js/vanilla-extract";
import { Button as StyleXButton } from "./zero-runtime-css-in-js-with-atomic-css/stylex";

const approaches = [
  ["Static CSS", StaticCssButton],
  ["CSS Modules", CssModulesButton],
  ["BEM", BemButton],
  ["CSS Modules + BEM", CssModulesWithBemButton],
  ["CSS in JS", CssInJsButton],
  ["CSS in JS + CSS Variables", CssInJsWithCssVariablesButton],
  ["Atomic CSS", AtomicCssButton],
  ["Zero run-time CSS in JS - Vanilla Extract", VanillaExtractButton],
  ["Zero run-time CSS in JS + Atomic CSS - StyleX", StyleXButton],
];

function App() {
  const [count, setCount] = useState(0);
  const isError = count > 3;

  return (
    <>
      <div>
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
      </div>

      {approaches.map(([description, Button]) => (
        <div style={{ margin: 20 }}>
          <Button isError={isError}>{description}</Button>
        </div>
      ))}
    </>
  );
}

export default App;
