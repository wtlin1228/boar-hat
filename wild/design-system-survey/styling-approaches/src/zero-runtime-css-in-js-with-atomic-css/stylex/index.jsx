import "./stylex.css";
import * as stylex from "@stylexjs/stylex";

const styles = stylex.create({
  button: {
    paddingInline: "1.125rem",
    borderRadius: ".25rem",
    border: ".0625rem solid transparent",
    height: "2.25rem",
    cursor: "pointer",
    fontWeight: 500,
  },
  primary: {
    background: "#1971C2",
    color: "#FFFFFF",
  },
  error: {
    background: "#E03131",
    color: "#000000",
  },
});

export const Button = ({ isError, children }) => {
  return (
    <button
      {...stylex.props(styles.button, styles.primary, isError && styles.error)}
    >
      {children}
    </button>
  );
};
