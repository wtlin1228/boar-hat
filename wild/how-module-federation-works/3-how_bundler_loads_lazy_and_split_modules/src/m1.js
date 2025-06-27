import "./m2";

const p = document.createElement("p");
p.innerText = "m1.js";
document.querySelector("#loaded-content").appendChild(p);

export const increment = () => {
  const num = document.querySelector("#num");
  num.innerHTML = Number(num.innerHTML) + 1;
};
