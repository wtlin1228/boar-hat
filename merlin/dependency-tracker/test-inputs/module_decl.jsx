// import A, { B, C_1 as C } from "../../../foo";

// export const D = "1234";

// // export default E;

// export default E = () => {};

// export { F };

// export * from "../../../bar";

// import * as G from "../../../boo";

// Import
import A1, { A2, A_3 as A3 } from "../../../foo";

// ExportDecl
export class B1 {}
export function B2() {}
export const B3 = "";

// ExportNamed
export { C1, C_2 as C2 };
export { C3, C_4 as C4 } from "../../..foo";

// ExportDefaultDecl
// export default class D {}
// export default class {}
// export default function D() {}
// export default function () {}

// ExportDefaultExpr
// export default D;
// export default () => {};
