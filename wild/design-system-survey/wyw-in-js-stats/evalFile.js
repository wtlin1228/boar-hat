const transformResultCode = ```
"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.__wywPreval = void 0;
var _getColor = require("./get-color");
const _exp = /*#__PURE__*/() => (0, _getColor.getColor)('hawk');
const _exp2 = /*#__PURE__*/() => ({
  theme
}) => theme.palette.primary.main;
const _exp3 = /*#__PURE__*/() => ({
  theme
}) => theme.size.font.h1;
const __wywPreval = exports.__wywPreval = {
  _exp: _exp,
  _exp2: _exp2,
  _exp3: _exp3
};
```;

// create a Module for this entrypoint, then do module.evaluate()
// module will create a vm to evaluate the module, and also its dependencies
const evaluated = {
  value: {
    __esModule: true,
    __wywPreval: {
      _exp: () => (0, _getColor.getColor)("hawk"),
      _exp2:
        () =>
        ({ theme }) =>
          theme.palette.primary.main,
      _exp3:
        () =>
        ({ theme }) =>
          theme.size.font.h1,
    },
  },
  dependencies: ["./get-color"],
};

// execuate each entry in the evaluated.value.__wywPreval
const valueCache = new Map(
  ["_exp", "pink"],
  ["_exp2", ({ theme }) => theme.palette.primary.main],
  ["_exp3", ({ theme }) => theme.size.font.h1]
);

// result of this action:
const result = [valueCache, evaluated.dependencies];
