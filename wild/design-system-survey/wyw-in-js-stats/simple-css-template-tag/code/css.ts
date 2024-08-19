import {
  BaseProcessor,
  CallParam,
  Expression,
  Params,
  TailProcessorParams,
  TemplateParam,
  validateParams,
  ValueCache,
} from '@wyw-in-js/processor-utils';
import { ValueType } from '@wyw-in-js/shared';

const theme = {
  palette: {
    primary: {
      main: 'red',
    },
  },
  size: {
    font: {
      h1: '3rem',
    },
  },
  components: {
    MuiSlider: {
      styleOverrides: {
        rail: {
          fontSize: '3rem',
        },
      },
    },
  },
};

export default class CssProcessor extends BaseProcessor {
  callParam: CallParam | TemplateParam;

  constructor(params: Params, ...args: TailProcessorParams) {
    if (params.length < 2) {
      throw BaseProcessor.SKIP;
    }
    super([params[0]], ...args);

    validateParams(
      params,
      ['callee', ['call', 'template']],
      `Invalid use of ${this.tagSource.imported} tag.`
    );

    const [, callParams] = params;
    this.callParam = callParams;

    if (callParams[0] === 'template') {
      callParams[1].forEach((element) => {
        if ('kind' in element && element.kind !== ValueType.CONST) {
          this.dependencies.push(element);
        }
      });
    }
  }

  public build(values: ValueCache): void {
    const [callType] = this.callParam;

    if (callType === 'template') {
      this.handleTemplate(this.callParam, values);
    } else {
      this.handleCall(this.callParam, values);
    }
  }

  private handleTemplate([, callArgs]: TemplateParam, values: ValueCache) {
    let cssText = '';
    const props = { theme };

    callArgs.forEach((item) => {
      if ('kind' in item) {
        switch (item.kind) {
          case ValueType.FUNCTION: {
            const templateCallback = values.get(item.ex.name);
            // @ts-ignore
            cssText += templateCallback(props);
            break;
          }
          case ValueType.CONST:
            throw new Error('unimplemented!');
            break;
          case ValueType.LAZY: {
            const evaluatedValue = values.get(item.ex.name);
            if (typeof evaluatedValue === 'function') {
              cssText += evaluatedValue(props);
            } else {
              cssText += evaluatedValue;
            }
            break;
          }
          default:
            break;
        }
      } else if (item.type === 'TemplateElement') {
        cssText += item.value.cooked;
      }
    });

    this.artifacts.push([
      'css',
      [
        // Rules
        {
          [this.asSelector]: {
            className: this.className,
            cssText,
            displayName: this.displayName,
            start: this.location?.start ?? null,
          },
        },
        // Replacements
        [
          {
            length: cssText.length,
            original: {
              start: {
                column: this.location?.start.column ?? 0,
                line: this.location?.start.line ?? 0,
              },
              end: {
                column: this.location?.end.column ?? 0,
                line: this.location?.end.line ?? 0,
              },
            },
          },
        ],
      ],
    ]);
  }

  private handleCall([, ...callArgs]: CallParam, values: ValueCache) {
    throw new Error('unimplemented!');
  }

  doEvaltimeReplacement() {
    this.replacer(this.value, false);
  }

  doRuntimeReplacement() {
    this.doEvaltimeReplacement();
  }

  get asSelector() {
    return `.${this.className}`;
  }

  get value(): Expression {
    return this.astService.stringLiteral(this.className);
  }
}
