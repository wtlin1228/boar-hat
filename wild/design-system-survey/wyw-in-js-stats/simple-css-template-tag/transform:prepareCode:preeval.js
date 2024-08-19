// original code:

import { css } from "@wtlin1228/processor";
import { getColor } from "./get-color";
export { foo1, foo2, foo3, foo4 } from "./foo";
export { bar1, bar2, bar3, bar4 } from "./bar";
const cls1 = css`
  background-color: ${getColor("hawk")};
  color: ${({ theme }) => theme.palette.primary.main};
  font-size: ${({ theme }) => theme.size.font.h1};
`;

// transform the original code by babel.transformFromAst

// My CSSProcessor received:

this.callParams = [
  "template",
  [
    {
      type: "TemplateElement",
      start: 149,
      end: 170,
      loc: {
        start: {
          line: 7,
          column: 17,
          index: 149,
        },
        end: {
          line: 8,
          column: 20,
          index: 170,
        },
      },
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
        loc: {
          start: {
            line: 8,
            column: 22,
            index: 172,
          },
          end: {
            line: 8,
            column: 38,
            index: 188,
          },
        },
      },
      importedFrom: [],
      source: './bar";\nconst cl',
    },
    {
      type: "TemplateElement",
      start: 189,
      end: 200,
      loc: {
        start: {
          line: 8,
          column: 39,
          index: 189,
        },
        end: {
          line: 9,
          column: 9,
          index: 200,
        },
      },
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
        loc: {
          start: {
            line: 9,
            column: 11,
            index: 202,
          },
          end: {
            line: 9,
            column: 52,
            index: 243,
          },
        },
      },
      importedFrom: [],
      source: "ckground-color: ${getColor('hawk')};\n  co",
    },
    {
      type: "TemplateElement",
      start: 244,
      end: 259,
      loc: {
        start: {
          line: 9,
          column: 53,
          index: 244,
        },
        end: {
          line: 10,
          column: 13,
          index: 259,
        },
      },
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
        loc: {
          start: {
            line: 10,
            column: 15,
            index: 261,
          },
          end: {
            line: 10,
            column: 48,
            index: 294,
          },
        },
      },
      importedFrom: [],
      source: "}) => theme.palette.primary.main}",
    },
    {
      type: "TemplateElement",
      start: 295,
      end: 297,
      loc: {
        start: {
          line: 10,
          column: 49,
          index: 295,
        },
        end: {
          line: 11,
          column: 0,
          index: 297,
        },
      },
      value: {
        raw: ";\n",
        cooked: ";\n",
      },
      tail: true,
    },
  ],
];

this.dependencies = [
  {
    kind: 0,
    ex: {
      type: "Identifier",
      name: "_exp",
      loc: {
        start: {
          line: 8,
          column: 22,
          index: 172,
        },
        end: {
          line: 8,
          column: 38,
          index: 188,
        },
      },
    },
    importedFrom: [],
    source: './bar";\nconst cl',
  },
  {
    kind: 1,
    ex: {
      type: "Identifier",
      name: "_exp2",
      loc: {
        start: {
          line: 9,
          column: 11,
          index: 202,
        },
        end: {
          line: 9,
          column: 52,
          index: 243,
        },
      },
    },
    importedFrom: [],
    source: "ckground-color: ${getColor('hawk')};\n  co",
  },
  {
    kind: 1,
    ex: {
      type: "Identifier",
      name: "_exp3",
      loc: {
        start: {
          line: 10,
          column: 15,
          index: 261,
        },
        end: {
          line: 10,
          column: 48,
          index: 294,
        },
      },
    },
    importedFrom: [],
    source: "}) => theme.palette.primary.main}",
  },
];

// result.code:

import { getColor } from "./get-color";
export { foo1, foo2, foo3, foo4 } from "./foo";
export { bar1, bar2, bar3, bar4 } from "./bar";
const _exp = () => getColor("hawk");
const _exp2 =
  () =>
  ({ theme }) =>
    theme.palette.primary.main;
const _exp3 =
  () =>
  ({ theme }) =>
    theme.size.font.h1;
const cls1 = "c19s9572";
export const __wywPreval = {
  _exp: _exp,
  _exp2: _exp2,
  _exp3: _exp3,
};

// result.metadata:

const result_metadata = {
  wywInJS: {
    processors: [
      {
        artifacts: [],
        dependencies: [
          {
            kind: 0,
            ex: {
              type: "Identifier",
              name: "_exp",
              loc: {
                start: {
                  line: 8,
                  column: 22,
                  index: 172,
                },
                end: {
                  line: 8,
                  column: 38,
                  index: 188,
                },
              },
            },
            importedFrom: [],
            source: './bar";\nconst cl',
          },
          {
            kind: 1,
            ex: {
              type: "Identifier",
              name: "_exp2",
              loc: {
                start: {
                  line: 9,
                  column: 11,
                  index: 202,
                },
                end: {
                  line: 9,
                  column: 52,
                  index: 243,
                },
              },
            },
            importedFrom: [],
            source: "ckground-color: ${getColor('hawk')};\n  co",
          },
          {
            kind: 1,
            ex: {
              type: "Identifier",
              name: "_exp3",
              loc: {
                start: {
                  line: 10,
                  column: 15,
                  index: 261,
                },
                end: {
                  line: 10,
                  column: 48,
                  index: 294,
                },
              },
            },
            importedFrom: [],
            source: "}) => theme.palette.primary.main}",
          },
        ],
        interpolations: [],
        tagSource: {
          imported: "css",
          source: "@wtlin1228/processor",
        },
        astService: {
          /* ... */
        },
        location: {
          start: {
            line: 7,
            column: 13,
            index: 145,
          },
          end: {
            line: 7,
            column: 16,
            index: 148,
          },
          identifierName: "css",
        },
        displayName: "cls1",
        isReferenced: false,
        idx: 0,
        options: {
          /* ... */
        },
        context: {
          /* ... */
        },
        className: "c19s9572",
        slug: "c19s9572",
        callee: {
          type: "Identifier",
          start: 145,
          end: 148,
          loc: {
            start: {
              line: 7,
              column: 13,
              index: 145,
            },
            end: {
              line: 7,
              column: 16,
              index: 148,
            },
            identifierName: "css",
          },
          name: "css",
        },
        callParam: [
          "template",
          [
            {
              type: "TemplateElement",
              start: 149,
              end: 170,
              loc: {
                start: {
                  line: 7,
                  column: 17,
                  index: 149,
                },
                end: {
                  line: 8,
                  column: 20,
                  index: 170,
                },
              },
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
                loc: {
                  start: {
                    line: 8,
                    column: 22,
                    index: 172,
                  },
                  end: {
                    line: 8,
                    column: 38,
                    index: 188,
                  },
                },
              },
              importedFrom: [],
              source: './bar";\nconst cl',
            },
            {
              type: "TemplateElement",
              start: 189,
              end: 200,
              loc: {
                start: {
                  line: 8,
                  column: 39,
                  index: 189,
                },
                end: {
                  line: 9,
                  column: 9,
                  index: 200,
                },
              },
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
                loc: {
                  start: {
                    line: 9,
                    column: 11,
                    index: 202,
                  },
                  end: {
                    line: 9,
                    column: 52,
                    index: 243,
                  },
                },
              },
              importedFrom: [],
              source: "ckground-color: ${getColor('hawk')};\n  co",
            },
            {
              type: "TemplateElement",
              start: 244,
              end: 259,
              loc: {
                start: {
                  line: 9,
                  column: 53,
                  index: 244,
                },
                end: {
                  line: 10,
                  column: 13,
                  index: 259,
                },
              },
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
                loc: {
                  start: {
                    line: 10,
                    column: 15,
                    index: 261,
                  },
                  end: {
                    line: 10,
                    column: 48,
                    index: 294,
                  },
                },
              },
              importedFrom: [],
              source: "}) => theme.palette.primary.main}",
            },
            {
              type: "TemplateElement",
              start: 295,
              end: 297,
              loc: {
                start: {
                  line: 10,
                  column: 49,
                  index: 295,
                },
                end: {
                  line: 11,
                  column: 0,
                  index: 297,
                },
              },
              value: {
                raw: ";\n",
                cooked: ";\n",
              },
              tail: true,
            },
          ],
        ],
      },
    ],
    replacements: [],
    rules: {},
    dependencies: [],
  },
};
