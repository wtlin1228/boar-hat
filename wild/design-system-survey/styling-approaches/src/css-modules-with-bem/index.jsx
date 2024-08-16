import classes from "./index.module.css";
import { cva } from "class-variance-authority";

const button = cva([classes.button], {
  variants: {
    status: {
      primary: [classes["button--primary"]],
      error: [classes["button--error"]],
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