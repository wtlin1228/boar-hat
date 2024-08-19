import { css } from "@wtlin1228/processor";
import { getColor } from "./get-color";
export { foo1, foo2, foo3, foo4 } from "./foo";
export { bar1, bar2, bar3, bar4 } from "./bar";
const cls1 = css`
  background-color: ${getColor("hawk")};
  color: ${({ theme }) => theme.palette.primary.main};
  font-size: ${({ theme }) => theme.size.font.h1};
`;
