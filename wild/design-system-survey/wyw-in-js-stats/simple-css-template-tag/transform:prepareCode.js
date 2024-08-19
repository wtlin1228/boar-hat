// prepareCode do preeval then evaluators

// Current loadedAndParsed.code
import { css } from "@wtlin1228/processor";
import { getColor } from "./get-color";
export { foo1, foo2, foo3, foo4 } from "./foo";
export { bar1, bar2, bar3, bar4 } from "./bar";
const cls1 = css`
  background-color: ${getColor("hawk")};
  color: ${({ theme }) => theme.palette.primary.main};
  font-size: ${({ theme }) => theme.size.font.h1};
`;

// after prepareFn()

// prepared code:
const preparedCode = ```

("use strict");

Object.defineProperty(exports, "__esModule", {
  value: true,
});
exports.__wywPreval = void 0;
var _getColor = require("./get-color");
const _exp = () => (0, _getColor.getColor)("hawk");
const _exp2 = () => ({ theme }) => theme.palette.primary.main;
const _exp3 = () => ({ theme }) => theme.size.font.h1;
const __wywPreval = (exports.__wywPreval = {
  _exp: _exp,
  _exp2: _exp2,
  _exp3: _exp3,
});
```;

// imports:
const imports = new Map([["./get-color", ["getColor"]]]);

// metadata:
const metadata = {
  processors: [
    {
      /* My CSSProcessor */
    },
  ],
  replacements: [],
  rules: {},
  dependencies: [],
};
