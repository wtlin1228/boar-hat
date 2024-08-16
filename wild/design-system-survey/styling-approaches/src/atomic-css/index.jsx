import "./index.css";
import { cva } from "class-variance-authority";

const button = cva(
  "atomic-padding-inline atomic-border-radius atomic-border atomic-height atomic-cursor atomic-font-weight",
  {
    variants: {
      status: {
        primary: "atomic-background-primary atomic-background-primary",
        error: "atomic-background-error atomic-background-error",
      },
    },
    defaultVariants: {
      status: "primary",
    },
  }
);

export const Button = ({ isError, children }) => {
  const className = button({ status: isError ? "error" : undefined });

  return <button className={className}>{children}</button>;
};
