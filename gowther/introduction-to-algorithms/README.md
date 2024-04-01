# Asymptotic Complexity

| Input | Constant | Logarithmic | Linear | Log-Linear   | Quadratic | Polynomial  | Exponential     |
| ----- | -------- | ----------- | ------ | ------------ | --------- | ----------- | --------------- |
| n     | $Θ(1)$   | $Θ(log n)$  | $Θ(n)$ | $Θ(n log n)$ | $Θ(n^2)$  | $Θ(n^c)$    | $2^Θ(n^c)$      |
| 1000  | 1        | ≈ 10        | 1000   | ≈ 10,000     | 1,000,000 | 1000^c      | 2^1000 ≈ 10^301 |
| Time  | 1ns      | 10ns        | 1us    | 10us         | 1ms       | 10^(3c-9) s | 10^281 millenia |

# Sequence Interface

- Maintain a sequence of items (order is extrinsic)
- Ex: (x0, x1, x2, ..., xn-1)
- Use n to denote the number of items stored in the data structure
- Supports sequence operations:
  - Container
    - build(X): given an iterable X, build sequence from items in X
    - len(): return the number of stored items
  - Static
    - iter_seq(): return the stored items one-by-one in sequence order
    - get_at(i): return the ith item
    - set_at(i, x): replace the ith item with x
  - Dynamic
    - insert_at(i, x): add x as the ith item
    - delete_at(i): remove and return the ith item
    - insert_first(x): add x as the first item
    - delete_first(): remove and return the first item
    - insert_last(x): add x as the last item
    - delete_last(): remove and return the last item

<table>
  <caption>
    Sequence Data Structure
  </caption>
  <thead>
    <tr>
      <th scope="col" rowspan="4">Sequence Data Structure</th>
      <th scope="col" colspan="5">Operations O(·)</th>
    </tr>
    <tr>
      <th scope="col">Container</th>
      <th scope="col">Static</th>
      <th scope="col" colspan="3">Dynamic</th>
    </tr>
    <tr>
      <th scope="col" rowspan="2">build(X)</th>
      <th scope="col">get_at(i)</th>
      <th scope="col">insert_first(x)</th>
      <th scope="col">insert_last(x)</th>
      <th scope="col">insert_at(i,x)</th>
    </tr>
    <tr>
      <th scope="col">set_at(i,x)</th>
      <th scope="col">delete_first()</th>
      <th scope="col">delete_last()</th>
      <th scope="col">delete_at(i)</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <th scope="row">Array</th>
      <td>n</td>
      <td>1</td>
      <td>n</td>
      <td>n</td>
      <td>n</td>
    </tr>
    <tr>
      <th scope="row">Linked List</th>
      <td>n</td>
      <td>n</td>
      <td>1</td>
      <td>n</td>
      <td>n</td>
    </tr>
    <tr>
      <th scope="row">Dynamic Array</th>
      <td>n</td>
      <td>1</td>
      <td>n</td>
      <td>1(a)</td>
      <td>n</td>
    </tr>
    <tr>
      <th scope="row">Binary Tree</th>
      <td>n log n</td>
      <td>h</td>
      <td>h</td>
      <td>h</td>
      <td>h</td>
    </tr>
    <tr>
      <th scope="row">AVL Tree</th>
      <td>n</td>
      <td>log n</td>
      <td>log n</td>
      <td>log n</td>
      <td>log n</td>
    </tr>
  </tbody>
</table>

# Set Interface

- Sequence about extrinsic order, set is about intrinsic order
- Maintain a set of items having unique keys (e.g., item x has key x.key)
- Set or multi-set?
- Often we let key of an item be the item itself, but may want to store more info than just key
- Supports set operations:
  - Container
    - build(X): given an iterable X, build sequence from items in X
    - len(): return the number of stored items
  - Static
    - find(k): return the stored item with key k
  - Dynamic
    - insert(x): add x to set (replace item with key x.key if one already exists)
    - delete(k): remove and return the stored item with key k
  - Order
    - iter_ord(): return the stored items one-by-one in key order
    - find_min(): return the stored item with smallest key
    - find_max(): return the stored item with largest key
    - find_next(k): return the stored item with smallest key larger than k
    - find_prev(k): return the stored item with largest key smaller than k

<table>
  <caption>
    Set Data Structure
  </caption>
  <thead>
    <tr>
      <th scope="col" rowspan="4">Set Data Structure</th>
      <th scope="col" colspan="5">Operations O(·)</th>
    </tr>
    <tr>
      <th scope="col">Container</th>
      <th scope="col">Static</th>
      <th scope="col">Dynamic</th>
      <th scope="col" colspan="2">Order</th>
    </tr>
    <tr>
      <th scope="col" rowspan="2">build(X)</th>
      <th scope="col" rowspan="2">find(k)</th>
      <th scope="col">insert(x)</th>
      <th scope="col">find_min()</th>
      <th scope="col">find_prev(k)</th>
    </tr>
    <tr>
      <th scope="col">delete(k)</th>
      <th scope="col">find_max()</th>
      <th scope="col">find_next(k)</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <th scope="row">Array</th>
      <td>n</td>
      <td>n</td>
      <td>n</td>
      <td>n</td>
      <td>n</td>
    </tr>
    <tr>
      <th scope="row">Sorted Array</th>
      <td>n log n</td>
      <td>log n</td>
      <td>n</td>
      <td>1</td>
      <td>log n</td>
    </tr>
    <tr>
      <th scope="row">Direct Access Array</th>
      <td>u</td>
      <td>1</td>
      <td>1</td>
      <td>u</td>
      <td>u</td>
    </tr>
    <tr>
      <th scope="row">Hash Table</th>
      <td>n(e)</td>
      <td>1(e)</td>
      <td>1(a)(e)</td>
      <td>n</td>
      <td>n</td>
    </tr>
    <tr>
      <th scope="row">Binary Tree</th>
      <td>n</td>
      <td>h</td>
      <td>h</td>
      <td>h</td>
      <td>h</td>
    </tr>
    <tr>
      <th scope="row">AVL Tree</th>
      <td>n log n</td>
      <td>log n</td>
      <td>log n</td>
      <td>log n</td>
      <td>log n</td>
    </tr>
  </tbody>
</table>

# Mathematics

## Stirling's approximation

${\displaystyle n!\sim {\sqrt {2\pi n}}\left({\frac {n}{e}}\right)^{n}}$
