import { css } from "@wtlin1228/processor";
import { getColor } from "./get-color";

export const cls1 = css`
  background-color: ${getColor("hawk")};
  color: ${({ theme }) => theme.palette.primary.main};
  font-size: ${({ theme }) => theme.size.font.h1};
`;

export const crs2 = css(({ theme }) => ({
  backgroundColor: getColor("wild"),
  color: theme.palette.error.main,
  fontSize: theme.size.font.h2,
}));
