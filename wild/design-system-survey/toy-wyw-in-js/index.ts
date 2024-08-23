import * as fs from "node:fs";
import * as path from "node:path";
import vm from "node:vm";

import babel from "@babel/core";
import generator from "@babel/generator";
import { statement } from "@babel/template";
import { cloneNode } from "@babel/types";

import type { TransformOptions } from "@babel/core";
import type { NodePath } from "@babel/traverse";
import type {
  Identifier,
  StringLiteral,
  Expression,
  TaggedTemplateExpression,
  ImportDefaultSpecifier,
  ImportNamespaceSpecifier,
  ImportSpecifier,
  Statement,
  VariableDeclaration,
  TemplateElement,
  ExportNamedDeclaration,
  ObjectExpression,
  ObjectProperty,
} from "@babel/types";

import CSSProcessor from "./processors/css.js";
import StyledProcessor from "./processors/styled.js";

/////////////////////////////////////////////
//           Create Entrypoint             //
/////////////////////////////////////////////

const root = path.resolve("./");
const filename = path.resolve("./__fixtures__/css.input.js");

const originalCode = fs.readFileSync(filename, "utf8");
console.log(originalCode);
// import { css } from "@wtlin1228/processor";
// import { getColor } from "./get-color";

// export const cls1 = css`
//   background-color: ${getColor("hawk")};
//   color: ${({ theme }) => theme.palette.primary.main};
//   font-size: ${({ theme }) => theme.size.font.h1};
// `;

// export const crs2 = css(({ theme }) => ({
//   backgroundColor: getColor("wild"),
//   color: theme.palette.error.main,
//   fontSize: theme.size.font.h2,
// }));

const parseOptions: TransformOptions = {
  assumptions: {},
  ast: true,
  babelrc: false,
  browserslistConfigFile: false,
  caller: {
    name: "wtlin-in-js",
    supportsStaticESM: true,
    supportsDynamicImport: true,
    supportsTopLevelAwait: true,
    supportsExportNamespaceFrom: true,
  },
  cloneInputAst: true,
  configFile: false,
  filename: filename,
  inputSourceMap: undefined,
  root: root,
  rootMode: "root",
  sourceFileName: filename,
  sourceMaps: true,
};

const ast = babel.parseSync(originalCode, parseOptions);

if (ast === null) {
  throw new Error("babel parse failed");
}

/////////////////////////////////////////////
//               Transform                 //
/////////////////////////////////////////////

function isImportSpecifier(
  specifier: NodePath<
    ImportDefaultSpecifier | ImportNamespaceSpecifier | ImportSpecifier
  >
): specifier is NodePath<ImportSpecifier> {
  return specifier.node.type === "ImportSpecifier";
}

function getValue({ node }: { node: Identifier | StringLiteral }): string {
  return node.type === "Identifier" ? node.name : node.value;
}

// collect imports
const imports: any[] = [];
babel.traverse(ast, {
  ImportDeclaration: function collectImports(path) {
    const source = getValue(path.get("source"));
    path
      .get("specifiers")
      .filter(isImportSpecifier)
      .forEach((specifier) => {
        const imported = getValue(specifier.get("imported"));
        const local = specifier.get("local");
        imports.push({ imported, local, source, type: "esm" });
      });
  },
});
console.table(imports);
// ┌─────────┬────────────┬────────────┬────────────────────────┬───────┐
// │ (index) │ imported   │ local      │ source                 │ type  │
// ├─────────┼────────────┼────────────┼────────────────────────┼───────┤
// │ 0       │ 'css'      │ [NodePath] │ '@wtlin1228/processor' │ 'esm' │
// │ 1       │ 'getColor' │ [NodePath] │ './get-color'          │ 'esm' │
// └─────────┴────────────┴────────────┴────────────────────────┴───────┘

// get defined processors
const definedProcessors = new Map();
imports.forEach(({ imported, source }) => {
  if (source === "@wtlin1228/processor") {
    switch (imported) {
      case "css":
        definedProcessors.set("css", [CSSProcessor, [imported, source]]);
        break;
      case "styled":
        definedProcessors.set("styled", [StyledProcessor, [imported, source]]);
        break;
      default:
        throw new Error(`${imported} is not implemented`);
    }
  }
});
console.table(definedProcessors);
// ┌───────────────────┬───────┬───────────────────────────────────┐
// │ (iteration index) │ Key   │ Values                            │
// ├───────────────────┼───────┼───────────────────────────────────┤
// │ 0                 │ 'css' │ [ [class CSSProcessor], [Array] ] │
// └───────────────────┴───────┴───────────────────────────────────┘

// collect usages
const usages: any[] = [];
babel.traverse(ast, {
  Program: function collectUsages(path) {
    definedProcessors.forEach((processor, idName) => {
      path.scope.getBinding(idName)?.referencePaths.forEach((identifier) => {
        if (identifier.isIdentifier()) {
          usages.push({
            identifier,
            processor,
          });
        }
      });
    });
  },
});
usages.sort(
  (a, b) => (a.identifier.node.start ?? 0) - (b.identifier.node.start ?? 0)
);
console.table(usages);
// ┌─────────┬────────────┬───────────────────────────────────┐
// │ (index) │ identifier │ processor                         │
// ├─────────┼────────────┼───────────────────────────────────┤
// │ 0       │ [NodePath] │ [ [class CSSProcessor], [Array] ] │
// │ 1       │ [NodePath] │ [ [class CSSProcessor], [Array] ] │
// └─────────┴────────────┴───────────────────────────────────┘

// extract expressions
type ExpressionValue = {
  kind: "LAZY" | "FUNCTION";
  ex: Identifier;
  source: string;
  importedFrom: string[];
};

function getSource(path: NodePath): string {
  const source = generator.default(path.node).code;
  return path.node.extra?.parenthesized ? `(${source})` : source;
}

const expressionDeclarationTpl = statement(
  "const %%expId%% = /*#__PURE__*/ () => %%expression%%",
  {
    preserveComments: true,
  }
);

/**
 * Only an expression that can be evaluated in the root scope can be
 * used in a WYW template. This function tries to hoist the expression.
 *
 * ```
 * export const cls1 = css`
 *   background-color: ${getColor("hawk")};
 *   color: ${({ theme }) => theme.palette.primary.main};
 *   font-size: ${({ theme }) => theme.size.font.h1};
 * `;
 * ```
 *
 * will be transformed to
 *
 * ```
 * const _exp = () => getColor("hawk");
 * const _exp2 = () => ({ theme }) => theme.palette.primary.main;
 * const _exp3 = () => ({ theme }) => theme.size.font.h1;
 * export const cls1 = css`
 *   background-color: ${_exp()};
 *   color: ${_exp2()};
 *   font-size: ${_exp3()};
 * `;
 * ```
 */
function extractExpression(
  ex: NodePath<Expression>
): Omit<ExpressionValue, "source"> {
  const rootScope = ex.scope.getProgramParent();
  const statementInRoot = ex.findParent(
    (p) => p.parentPath?.isProgram() === true
  ) as NodePath<Statement>;
  const isFunction =
    ex.isFunctionExpression() || ex.isArrowFunctionExpression();

  // Declare _expN const with the lazy expression
  const expUid = rootScope.generateUid("exp");
  const declaration = expressionDeclarationTpl({
    expId: { type: "Identifier", name: expUid },
    expression: cloneNode(ex.node),
  }) as VariableDeclaration;

  // Insert the declaration as close as possible to the original expression
  const [inserted] = statementInRoot.insertBefore(declaration);
  rootScope.registerDeclaration(inserted);

  const importedFrom: string[] = [];
  function findImportSourceOfIdentifier(idPath: NodePath<Identifier>) {
    const exBindingIdentifier = idPath.scope.getBinding(
      idPath.node.name
    )?.identifier;
    const exImport =
      imports.find((i) => i.local.node === exBindingIdentifier) ?? null;
    if (exImport) {
      importedFrom.push(exImport.source);
    }
  }

  if (ex.isIdentifier()) {
    findImportSourceOfIdentifier(ex);
  } else {
    ex.traverse({
      Identifier: findImportSourceOfIdentifier,
    });
  }

  // Replace the expression with the _expN() call
  const { loc } = ex.node;
  ex.replaceWith({
    type: "CallExpression",
    callee: { type: "Identifier", name: expUid },
    arguments: [],
  });
  ex.node.loc = loc;

  return {
    kind: isFunction ? "FUNCTION" : "LAZY",
    ex: { type: "Identifier", name: expUid, loc },
    importedFrom,
  };
}

function collectTemplateDependencies(
  path: NodePath<TaggedTemplateExpression>
): [quasis: TemplateElement[], expressionValues: ExpressionValue[]] {
  const quasi = path.get("quasi");
  const quasis = quasi.get("quasis");
  const expressions = quasi.get("expressions");

  const expressionValues: ExpressionValue[] = expressions
    .filter((ex) => ex.isExpression())
    .map((ex) => {
      const source = getSource(ex);
      const extracted = extractExpression(ex);

      return {
        ...extracted,
        source,
      };
    });

  return [quasis.map((p) => p.node), expressionValues];
}

function zip<T1, T2>(arr1: T1[], arr2: T2[]) {
  const result: (T1 | T2)[] = [];
  for (let i = 0; i < arr1.length; i++) {
    result.push(arr1[i]);
    if (arr2[i]) result.push(arr2[i]);
  }

  return result;
}

// initialize processors
const processors: any[] = [];
for (const usage of usages) {
  const id = usage.identifier;
  const parent = id.parentPath;
  const params = [["callee", id.node]];

  if (parent?.isTaggedTemplateExpression({ tag: id.node })) {
    const [quasis, expressionValues] = collectTemplateDependencies(parent);
    params.push(["template", zip(quasis, expressionValues)]);
  } else if (parent?.isCallExpression({ callee: id.node })) {
    const args = parent.get("arguments");
    const cookedArgs = args.map((arg: NodePath<Expression>) => {
      const source = getSource(arg);
      const extracted = extractExpression(arg);
      return {
        ...extracted,
        source,
      };
    });
    params.push(["call", ...cookedArgs]);
  }

  const Processor = usage.processor[0];
  const p = new Processor(params);
  processors.push(p);
}

// collect __wywPreval
babel.traverse(ast, {
  Program: function addWywPreval(path) {
    const prevalExport: ExportNamedDeclaration = {
      declaration: {
        declarations: [
          {
            id: {
              type: "Identifier",
              name: "__wywPreval",
            },
            init: {
              properties: [],
              type: "ObjectExpression",
            },
            type: "VariableDeclarator",
          },
        ],
        kind: "const",
        type: "VariableDeclaration",
      },
      specifiers: [],
      type: "ExportNamedDeclaration",
    };
    const [inserted] = path.pushContainer("body", [prevalExport]);
    const wywPrevalObject = inserted.get(
      "declaration.declarations.0.init"
    ) as NodePath<ObjectExpression>;
    path.setData("__wywPreval", wywPrevalObject);

    for (const processor of processors) {
      console.log(processor.dependencies);
      for (const dependency of processor.dependencies) {
        const newProperty: ObjectProperty = {
          type: "ObjectProperty",
          key: {
            type: "Identifier",
            name: dependency.ex.name,
          },
          value: {
            type: "Identifier",
            name: dependency.ex.name,
          },
          computed: false,
          shorthand: false,
        };
        wywPrevalObject.pushContainer("properties", [newProperty]);
      }
    }
  },
});

// console.log(generator.default(ast).code);
// ```
// import { css } from "@wtlin1228/processor";
// import { getColor } from "./get-color";
// const _exp = /*#__PURE__*/() => getColor("hawk");
// const _exp2 = /*#__PURE__*/() => ({ theme }) => theme.palette.primary.main;
// const _exp3 = /*#__PURE__*/() => ({ theme }) => theme.size.font.h1;
// export const cls1 = css`
//   background-color: ${_exp()};
//   color: ${_exp2()};
//   font-size: ${_exp3()};
// `;
// const _exp4 = /*#__PURE__*/() => ({ theme }) ({
//   backgroundColor: getColor("wild"),
//   color: theme.palette.error.main,
//   fontSize: theme.size.font.h2
// });
// export const crs2 = css(_exp4());
// export const __wywPreval = {
//   _exp: _exp,
//   _exp2: _exp2,
//   _exp3: _exp3,
//   _exp4: _exp4
// };
// ```

// do evaltime replacement
for (let i = 0; i < usages.length; i++) {
  usages[i].identifier.parentPath.replaceWith(processors[i].value);
}
// console.log(generator.default(ast).code);
// ```
// import { css } from "@wtlin1228/processor";
// import { getColor } from "./get-color";
// const _exp = /*#__PURE__*/() => getColor("hawk");
// const _exp2 = /*#__PURE__*/() => ({ theme }) => theme.palette.primary.main;
// const _exp3 = /*#__PURE__*/() => ({ theme }) => theme.size.font.h1;
// export const cls1 = "wtlin_0";
// const _exp4 = /*#__PURE__*/() => ({ theme }) => ({
//   backgroundColor: getColor("wild"),
//   color: theme.palette.error.main,
//   fontSize: theme.size.font.h2
// });
// export const crs2 = "wtlin_1";
// export const __wywPreval = {
//   _exp: _exp,
//   _exp2: _exp2,
//   _exp3: _exp3,
//   _exp4: _exp4
// };
// ```

const transformedResult = babel.transformFromAstSync(ast, originalCode, {
  plugins: ["@babel/plugin-transform-modules-commonjs"],
});
if (!transformedResult) {
  throw new Error("transform result failed");
}
console.log(transformedResult.code);
// ```
// "use strict";
//
// Object.defineProperty(exports, "__esModule", {
//   value: true
// });
// exports.crs2 = exports.cls1 = exports.__wywPreval = void 0;
// var _processor = require("@wtlin1228/processor");
// var _getColor = require("./get-color");
// const _exp = /*#__PURE__*/() => (0, _getColor.getColor)("hawk");
// const _exp2 = /*#__PURE__*/() => ({ theme }) => theme.palette.primary.main;
// const _exp3 = /*#__PURE__*/() => ({ theme }) => theme.size.font.h1;
// const cls1 = exports.cls1 = "wtlin_0";
// const _exp4 = /*#__PURE__*/() => ({ theme }) => ({
//   backgroundColor: (0, _getColor.getColor)("wild"),
//   color: theme.palette.error.main,
//   fontSize: theme.size.font.h2
// });
// const crs2 = exports.crs2 = "wtlin_1";
// const __wywPreval = exports.__wywPreval = {
//   _exp: _exp,
//   _exp2: _exp2,
//   _exp3: _exp3,
//   _exp4: _exp4
// };
// ```

/////////////////////////////////////////////
//                EvalFile                 //
/////////////////////////////////////////////

// Both processors are depending on the `'get-color.js'.getColor`
const getColorFilename = path.resolve("./__fixtures__/get-color.js");
const getColorCode = fs.readFileSync(getColorFilename, "utf8");
const getColorTransformedResult = babel.transformSync(getColorCode, {
  plugins: ["@babel/plugin-transform-modules-commonjs"],
});
if (!getColorTransformedResult) {
  throw new Error("failed to transform get-color.js");
}
const getColorScript = new vm.Script(
  `(function (exports) {\n${getColorTransformedResult.code}\n})(exports);`,
  {
    filename: getColorFilename,
  }
);
const getColorSandbox = { exports: {} };
const getColorContext = vm.createContext(getColorSandbox);
getColorScript.runInContext(getColorContext);
// console.log(getColorSandbox);
// { exports: { getColor: [Function: getColor] } }

// evaluate our entry script
const source = transformedResult.code;
const script = new vm.Script(`(function (exports) {\n${source}\n})(exports);`, {
  filename,
});
const sandbox = {
  exports: {},
  require: (id: string) => {
    switch (id) {
      case "./get-color":
        return getColorSandbox.exports;
      case "@wtlin1228/processor":
      default:
        return {};
    }
  },
};
const context = vm.createContext(sandbox);
script.runInContext(context);
// console.log(sandbox);
// {
//   exports: {
//     __wywPreval: {
//       _exp: [Function: _exp],
//       _exp2: [Function: _exp2],
//       _exp3: [Function: _exp3],
//       _exp4: [Function: _exp4]
//     },
//     cls1: 'wtlin_0',
//     crs2: 'wtlin_1'
//   },
//   require: [Function: require]
// }

const { __wywPreval } = sandbox.exports as {
  __wywPreval: Record<string, Function>;
};

const valueCache = new Map();
Object.entries(__wywPreval).forEach(([key, lazyValue]) => {
  valueCache.set(key, lazyValue());
});

console.table(valueCache);
// ┌───────────────────┬─────────┬────────────────────────┐
// │ (iteration index) │ Key     │ Values                 │
// ├───────────────────┼─────────┼────────────────────────┤
// │ 0                 │ '_exp'  │ 'pink'                 │
// │ 1                 │ '_exp2' │ [Function (anonymous)] │
// │ 2                 │ '_exp3' │ [Function (anonymous)] │
// │ 3                 │ '_exp4' │ [Function (anonymous)] │
// └───────────────────┴─────────┴────────────────────────┘

/////////////////////////////////////////////
//                Collect                  //
/////////////////////////////////////////////
processors.forEach((processor) => {
  processor.build(valueCache);

  const { selector, cssText } = processor.artifact;
  console.log(`${selector} {${cssText}}`);
});
// .wtlin_0 {
//   background-color: pink;
//   color: red;
//   font-size: 3rem;
// }
// .wtlin_1 {
//   background-color: purple;
//   color: orange;
//   font-size: 2.2rem;
// }

/////////////////////////////////////////////
//                Extract                  //
/////////////////////////////////////////////
