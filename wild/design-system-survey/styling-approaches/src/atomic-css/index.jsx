import "./index.css";
import { cva } from "class-variance-authority";

const button = cva(
  "px-4 border border-transparent rounded h-9 cursor-pointer font-medium",
  {
    variants: {
      status: {
        primary: "bg-leo-blue text-white",
        error: "bg-red-600 text-black",
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
