import { useState } from "react";
import { Button as StaticCssButton } from "./static-css";
import { Button as CssModulesButton } from "./css-modules";
import { Button as BemButton } from "./bem";
import { Button as CssModulesWithBemButton } from "./css-modules-with-bem";
import { Button as CssInJsButton } from "./css-in-js";
import { Button as CssInJsWithCssVariablesButton } from "./css-in-js-with-css-variables";
import { Button as AtomicCssButton } from "./atomic-css";
import { Button as VanillaExtractButton } from "./zero-runtime-css-in-js/vanilla-extract";

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

      <div style={{ margin: 20 }}>
        <StaticCssButton isError={isError}>Static CSS</StaticCssButton>
      </div>

      <div style={{ margin: 20 }}>
        <CssModulesButton isError={isError}>CSS Modules</CssModulesButton>
      </div>

      <div style={{ margin: 20 }}>
        <BemButton isError={isError}>BEM</BemButton>
      </div>

      <div style={{ margin: 20 }}>
        <CssModulesWithBemButton isError={isError}>
          CSS Modules + BEM
        </CssModulesWithBemButton>
      </div>

      <div style={{ margin: 20 }}>
        <CssInJsButton isError={isError}>CSS in JS</CssInJsButton>
      </div>

      <div style={{ margin: 20 }}>
        <CssInJsWithCssVariablesButton isError={isError}>
          CSS in JS + CSS Variables
        </CssInJsWithCssVariablesButton>
      </div>

      <div style={{ margin: 20 }}>
        <AtomicCssButton isError={isError}>Atomic CSS</AtomicCssButton>
      </div>

      <div style={{ margin: 20 }}>
        <VanillaExtractButton isError={isError}>
          Zero run-time CSS in JS - Vanilla Extract
        </VanillaExtractButton>
      </div>
    </>
  );
}

export default App;
