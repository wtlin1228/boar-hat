import './index.css';
import { a, b } from './m1';

document.querySelector('#root').innerHTML = `
<div class="content">
  <h1>Vanilla Rsbuild</h1>
  <p>Start building amazing things with Rsbuild.</p>
  <p>${a}</p>
  <p>${b}</p>
</div>
`;

