import "./index.css";
import { cva } from "class-variance-authority";

const button = cva(["button"], {
  variants: {
    status: {
      primary: ["button--primary"],
      error: ["button--error"],
    },
  },
  defaultVariants: {
    status: "primary",
  },
});

export const Button = ({ isError, children }) => {
  const className = button({ status: isError ? "error" : undefined });

  return <button className={className}>{children}</button>;
};