import { createCollapseTransition } from "./collapse";

export class Basket {
  private basketEl: HTMLUListElement;
  private fruits: string[];

  constructor(basketEl: HTMLUListElement, fruits: string[]) {
    this.basketEl = basketEl;
    this.fruits = fruits;
  }

  public addFruit = () => {
    const fruit = this.fruits.pop();
    if (!fruit) {
      return;
    }

    const fruitEl = Basket.createFruitElement(fruit);
    const transition = createCollapseTransition(
      /* element= */ fruitEl,
      /* timeout= */ 400,
      /* onMount= */ (el) => {
        this.basketEl.prepend(el);
      },
      /* onUnmount= */ (el) => {
        this.basketEl.removeChild(el);
        this.fruits.push(fruit);
      }
    );

    const removeButton = Basket.createRemoveButton(transition.performExit);
    fruitEl.appendChild(removeButton);

    transition.performEnter();
  };

  static createFruitElement = (fruit: string): HTMLLIElement => {
    const li = document.createElement("li");

    const p = document.createElement("p");
    p.textContent = fruit;
    li.appendChild(p);

    return li;
  };

  static createRemoveButton = (onClick: () => void): HTMLButtonElement => {
    const button = document.createElement("button");
    button.textContent = "x";
    button.onclick = onClick;
    return button;
  };
}
