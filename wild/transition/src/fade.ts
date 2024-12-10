import { Transition } from "./transition";

export const createFadeTransition = (
  element: HTMLElement,
  timeout: number,
  onMount: (el: HTMLElement) => void,
  onUnmount: (el: HTMLElement) => void
): Transition => {
  element.style.transition = `opacity ${timeout}ms ease-in-out`;
  element.style.opacity = "0";

  return new Transition(
    /* element= */ element,
    /* timeout= */ timeout,
    /* onEnter= */ (el) => {
      console.log("Fade, onEnter", "add element to DOM");
      onMount(el);
    },
    /* onEntering= */ (el) => {
      console.log("Fade, onEntering");
      el.style.opacity = "1";
    },
    /* onEntered= */ (_el) => {
      console.log("Fade, onEntered");
    },
    /* onExit= */ (_el) => {
      console.log("Fade, onExit");
    },
    /* onExiting= */ (el) => {
      console.log("Fade, onExiting");
      el.style.opacity = "0";
    },
    /* onExited= */ (el) => {
      console.log("Fade, onExited", "remove element from DOM");
      onUnmount(el);
    }
  );
};
