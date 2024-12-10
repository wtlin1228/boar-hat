import "./style.css";

import { Basket } from "./basket";

const FRUITS = [
  "🍏 Apple",
  "🍌 Banana",
  "🍍 Pineapple",
  "🥥 Coconut",
  "🍉 Watermelon",
];

const basket = new Basket(
  document.getElementById("basket") as HTMLUListElement,
  FRUITS
);

document.getElementById("add-fruit")?.addEventListener("click", () => {
  basket.addFruit();
});
