import type { Expression } from "@babel/types";
import { stringLiteral } from "@babel/types";

const toKebabCase = (str: any) =>
  str &&
  str
    .match(/[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+/g)
    .map((x: string) => x.toLowerCase())
    .join("-");

const theme = {
  palette: {
    primary: {
      main: "red",
    },
    error: {
      main: "orange",
    },
  },
  size: {
    font: {
      h1: "3rem",
      h2: "2.2rem",
    },
  },
};

let idx = 0;

export default class CSSProcessor {
  private callParam: any;
  private className: string;
  public readonly dependencies: any[] = [];
  public artifact?: { selector: string; cssText: string };

  constructor(params: any) {
    const [[, callee], callParams] = params;
    this.callParam = callParams;
    this.className = `wtlin_${idx++}`;

    if (callParams[0] === "template") {
      // export const cls1 = css`
      //   background-color: ${getColor("hawk")};
      //   color: ${({ theme }) => theme.palette.primary.main};
      //   font-size: ${({ theme }) => theme.size.font.h1};
      // `;
      callParams[1].forEach((expression: any) => {
        if ("kind" in expression) {
          this.dependencies.push(expression);
        }
      });
      return;
    }

    if (callParams[0] === "call") {
      // export const crs2 = css((theme) => ({
      //   backgroundColor: getColor("wild"),
      //   color: theme.palette.error.main,
      //   fontSize: theme.size.font.h2,
      // }));
      const [, ...callArgs] = callParams;
      this.dependencies.push(...callArgs);
      return;
    }
  }

  build(values: Map<string, unknown>) {
    let cssText = "";
    const props = { theme };

    if (this.callParam[0] === "template") {
      this.callParam[1].forEach((item: any) => {
        if ("kind" in item) {
          const evaluatedValue = values.get(item.ex.name);
          cssText +=
            typeof evaluatedValue === "function"
              ? evaluatedValue(props)
              : evaluatedValue;
        } else {
          cssText += item.value.cooked;
        }
      });
    } else if (this.callParam[0] === "call") {
      const evaluatedValue = values.get(this.callParam[1].ex.name) as Function;
      const obj = evaluatedValue(props);
      cssText += "\n";
      Object.entries(obj).forEach(([key, value]) => {
        cssText += `  ${toKebabCase(key)}: ${value};\n`;
      });
    }

    this.artifact = {
      selector: this.asSelector,
      cssText,
    };
  }

  get asSelector() {
    return `.${this.className}`;
  }

  get value(): Expression {
    return stringLiteral(this.className);
  }
}
