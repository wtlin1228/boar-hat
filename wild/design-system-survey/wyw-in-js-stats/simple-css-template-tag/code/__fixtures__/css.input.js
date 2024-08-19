import { css } from '@wtlin1228/processor';
import { getColor } from './get-color';

export * from './foo';
export * from './bar';

const cls1 = css`
  background-color: ${getColor('hawk')};
  color: ${({ theme }) => theme.palette.primary.main};
  font-size: ${({ theme }) => theme.size.font.h1};
`;
