// collect trigger transformFromAst with the collector plugin
// 1. processor.build(values); 👈 the valueCache from evalFile
// 2. processor.doRuntimeReplacement();
// 3. processors.push(processor);

const callParam = [
  "template",
  [
    {
      type: "TemplateElement",
      value: {
        raw: "\n  background-color: ",
        cooked: "\n  background-color: ",
      },
      tail: false,
    },
    {
      kind: 0,
      ex: {
        type: "Identifier",
        name: "_exp",
      },
      importedFrom: [],
      source: './bar";\nconst cl',
    },
    {
      type: "TemplateElement",
      start: 189,
      end: 200,
      value: {
        raw: ";\n  color: ",
        cooked: ";\n  color: ",
      },
      tail: false,
    },
    {
      kind: 1,
      ex: {
        type: "Identifier",
        name: "_exp2",
      },
      importedFrom: [],
      source: "ckground-color: ${getColor('hawk')};\n  co",
    },
    {
      type: "TemplateElement",
      value: {
        raw: ";\n  font-size: ",
        cooked: ";\n  font-size: ",
      },
      tail: false,
    },
    {
      kind: 1,
      ex: {
        type: "Identifier",
        name: "_exp3",
      },
      importedFrom: [],
      source: "}) => theme.palette.primary.main}",
    },
    {
      type: "TemplateElement",
      value: {
        raw: ";\n",
        cooked: ";\n",
      },
      tail: true,
    },
  ],
];

// After loop over the callParam[1], the collected cssText:
const cssText = ```
    background-color: pink;
    color: red;
    font-size: 3rem;
```;

// after step1 processor.build(values)
const artifacts = [
  [
    "css",
    [
      {
        ".c19s9572": {
          className: "c19s9572",
          cssText:
            "\n  background-color: pink;\n  color: red;\n  font-size: 3rem;\n",
          displayName: "cls1",
          start: {
            line: 7,
            column: 13,
            index: 145,
          },
        },
      },
    ],
  ],
];

// result of the transformFromAst

const result_code = ```
export { foo1, foo2, foo3, foo4 } from "./foo";
export { bar1, bar2, bar3, bar4 } from "./bar";
const cls1 = "c19s9572";
```;

// And the CSS of `.c19s9572` is stored in the
// result.metadata.wywInJS.processors[0].artifacts
// 👆 the CSSProcessor
