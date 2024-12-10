export class Transition {
  private element: HTMLElement;
  private timeout: number;
  private onEnter: (element: HTMLElement) => void;
  private onEntering: (element: HTMLElement) => void;
  private onEntered: (element: HTMLElement) => void;
  private onExit: (element: HTMLElement) => void;
  private onExiting: (element: HTMLElement) => void;
  private onExited: (element: HTMLElement) => void;

  constructor(
    element: HTMLElement,
    timeout: number,
    onEnter: (element: HTMLElement) => void,
    onEntering: (element: HTMLElement) => void,
    onEntered: (element: HTMLElement) => void,
    onExit: (element: HTMLElement) => void,
    onExiting: (element: HTMLElement) => void,
    onExited: (element: HTMLElement) => void
  ) {
    this.element = element;
    this.timeout = timeout;
    this.onEnter = onEnter;
    this.onEntering = onEntering;
    this.onEntered = onEntered;
    this.onExit = onExit;
    this.onExiting = onExiting;
    this.onExited = onExited;
  }

  public performEnter = () => {
    this.onEnter(this.element);
    setTimeout(() => {
      this.onEntering(this.element);
      setTimeout(() => {
        this.onEntered(this.element);
      }, this.timeout);
    }, 0);
  };

  public performExit = () => {
    this.onExit(this.element);
    setTimeout(() => {
      this.onExiting(this.element);
      setTimeout(() => {
        this.onExited(this.element);
      }, this.timeout);
    }, 0);
  };
}
