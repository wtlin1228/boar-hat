import { createVar, style } from "@vanilla-extract/css";

// Compiled to: --background__1cnmuie0
const background = createVar();
// Compiled to: --color__1cnmuie1
const color = createVar();

// Compiled to: .vanilla-extract_button__1cnmuie2
export const button = style({
  paddingInline: "1.125rem",
  borderRadius: ".25rem",
  border: ".0625rem solid transparent",
  height: "2.25rem",
  cursor: "pointer",
  fontWeight: 500,
  // Compiled to: var(--background__1cnmuie0)
  background: background,
  // Compiled to: var(--color__1cnmuie1)
  color: color,
});

// Compiled to: .vanilla-extract_error__1cnmuie3
export const primary = style({
  vars: {
    [background]: "#1971C2",
    [color]: "#FFFFFF",
  },
});

// Compiled to: .vanilla-extract_primary__1cnmuie4
export const error = style({
  vars: {
    [background]: "#E03131",
    [color]: "#000000",
  },
});
