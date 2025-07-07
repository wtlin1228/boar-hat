import { useState } from "react";
import "./App.css";
import { KirbyCafe } from "./KirbyCafe";
import { PikminCafe } from "./PikminCafe";

const App = () => {
  const [showKirbyCafe, setShowKirbyCafe] = useState(false);
  const [showPikminCafe, setShowPikminCafe] = useState(false);

  return (
    <>
      <button onClick={() => setShowKirbyCafe((v) => !v)}>toggle kirby</button>
      <button onClick={() => setShowPikminCafe((v) => !v)}>
        toggle pikmin
      </button>

      <div className="content">
        {showKirbyCafe && <KirbyCafe />}
        {showPikminCafe && <PikminCafe />}
      </div>
    </>
  );
};

export default App;
