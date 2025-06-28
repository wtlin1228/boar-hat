import "./index.css";
import _ from "lodash";

const rootEl = document.querySelector("#root");
if (rootEl) {
  rootEl.innerHTML = `
  <div class="content">
    <h1>${_.join(["Vanilla", "Rsbuild"], " ")}</h1>
    <p>Start building amazing things with Rsbuild.</p>
  </div>
`;
}
