const scriptCache = {};

const fetchLargeJS = async () => {
  console.log("fetch!");
  const res = await fetch("/large.js");
  scriptCache.large = await res.text();
  console.log("fetch finished!");
};

const executeLargeJS = () => {
  console.log("eval!");
  eval(scriptCache.large);
  console.log("eval finished!");
};

const fetchButton = document.createElement("button");
fetchButton.onclick = () => fetchLargeJS();
fetchButton.innerText = "fetch large JS";
document.querySelector("#fetch-eval").appendChild(fetchButton);

const evalButton = document.createElement("button");
evalButton.onclick = () => executeLargeJS();
evalButton.innerText = "eval large JS";
document.querySelector("#fetch-eval").appendChild(evalButton);

const fetchThenEvalButton = document.createElement("button");
fetchThenEvalButton.onclick = () => fetchLargeJS().then(() => executeLargeJS());
fetchThenEvalButton.innerText = "fetch then eval large JS";
document.querySelector("#fetch-eval").appendChild(fetchThenEvalButton);

const pretendToBeBusy = () => {
  const now = new Date();
  const fiveSecondsLater = new Date(now.getTime() + 5000);

  let i = 0;
  while (new Date().getTime() < fiveSecondsLater) {
    i += 1;
    if (i >= Number.MAX_SAFE_INTEGER) {
      i = 0;
    }
  }

  console.log(`stopped at ${i}`);
};

const pretendToBeBusyButton = document.createElement("button");
pretendToBeBusyButton.onclick = () => pretendToBeBusy();
pretendToBeBusyButton.innerText = "Busy for 5s";
document.querySelector("#main-thread").appendChild(pretendToBeBusyButton);
