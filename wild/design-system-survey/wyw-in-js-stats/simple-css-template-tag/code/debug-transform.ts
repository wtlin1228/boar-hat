import * as fs from 'node:fs';
import * as path from 'node:path';

import { syncResolve } from '@wyw-in-js/shared';
import { transformSync } from '../transform';

const inputFilePath = path.join(__dirname, '__fixtures__/css.input.js');
const inputContent = fs.readFileSync(inputFilePath, 'utf8');

const result = transformSync(
  {
    options: {
      filename: inputFilePath,
      preprocessor: 'stylis',
      pluginOptions: {
        tagResolver: (source, tag) => {
          return require.resolve('./css.ts');
        },
      },
    },
  },
  inputContent,
  syncResolve
);

console.log(result);
