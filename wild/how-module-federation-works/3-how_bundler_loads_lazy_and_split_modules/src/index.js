import "./index.css";

document.querySelector("#root").innerHTML = `
<div class="content">
  <h1>Vanilla Rsbuild</h1>
  <p>Start building amazing things with Rsbuild.</p>
  <div>
    <button id="inc-button">increment</button>
  </div>
  <h3 id="num">0</h3>
  <div id="loaded-content" style="min-height: 100px"></div>
</div>
`;

const button = document.querySelector("#inc-button");
button.addEventListener("click", () => {
  import("./m1.js").then(({ increment }) => {
    increment();
  });
});
