import { Transition } from "./transition";

export const createCollapseTransition = (
  element: HTMLElement,
  timeout: number,
  onMount: (el: HTMLElement) => void,
  onUnmount: (el: HTMLElement) => void
): Transition => {
  element.style.transition = `opacity ${timeout}ms ease-in-out, height ${timeout}ms ease-in-out`;
  element.style.opacity = "0";
  element.style.height = "0";

  return new Transition(
    /* element= */ element,
    /* timeout= */ timeout,
    /* onEnter= */ (el) => {
      console.log("Collapse, onEnter", "add element to DOM");
      onMount(el);
    },
    /* onEntering= */ (el) => {
      console.log("Collapse, onEntering");
      element.style.opacity = "1";
      el.style.height = "56px"; // should measure el's height in real usage
    },
    /* onEntered= */ (el) => {
      console.log("Collapse, onEntered");
      el.style.height = "auto";
    },
    /* onExit= */ (el) => {
      console.log("Collapse, onExit");
      el.style.height = "56px"; // should measure el's height in real usage
    },
    /* onExiting= */ (el) => {
      console.log("Collapse, onExiting");
      element.style.opacity = "0";
      el.style.height = "0";
    },
    /* onExited= */ (el) => {
      console.log("Collapse, onExited", "remove element from DOM");
      onUnmount(el);
    }
  );
};
