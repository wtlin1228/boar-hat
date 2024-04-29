Those latency are added intensionally, see the `backend/src/main.rs`.

| resource   | latency | description |
| ---------- | ------- | ----------- |
| index.html | 0.5s    |             |
| fonts/\*   | 1s      |             |

# v0

LCP: 1.5s

```
1.5s =
    index.html(0.5s)
    + fonts/*(1s)
```

![v0 network](./v0_network.avif)

# v1 - Save-Data and prefers-reduced-data

LCP: 0.5s (expected)

```
0.5s =
    index.html(0.5s)
```

Both `Save-Data` and `prefers-reduced-data` are experimental features now. So I don't want to put too much effort in this approach.

See https://web.dev/articles/optimizing-content-efficiency-save-data

# v2 - @font-face "font-display: optional"

LCP: 0.725s

```
0.725s =
    index.html(0.5s)
    extremely small block period from "font-display: optional"(~0.25s)
```

[`font-display: optional`](https://developer.mozilla.org/en-US/docs/Web/CSS/@font-face/font-display#optional) gives the font face an extremely small block period and no swap period.

![v2 network](./v2_network.avif)

Actually, Smashing Magazine's current approach is download then swap, which behaviors the same as setting `font-display: swap` and without the block period.

![smashing_magazine_swap_like_approach](./smashing_magazine_swap_like_approach.avif)
