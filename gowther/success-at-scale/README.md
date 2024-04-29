# Making Instagram.com Faster

1. Preload critical resources. (see [making-instagram-faster/v1](/making-instagram-faster/v1))
2. Prefetch images with correct resolution and only prefetch off-screen images when browser is idle. (see [making-instagram-faster/v2](/making-instagram-faster/v2))
3. Pushing data using Early Flushing and Progressive HTML. (see [making-instagram-faster/v3](/making-instagram-faster/v3))
4. Send API response directly from server to client by streaming. It saves one round trip since server starts preparing the API response right after it receives the first request.

# Shopify: Want to Improve UI Performance? Start by Understanding Your User

1. Prioritize resources correctly.
2. Defer non-critical resources.
3. Use singleton instead of creating new instances repeatedly.
4. [User Experience Only] Use skelton, maintain the height for users to feel the website is stable, and show static information before the data is available.

# Improving Third-Party Web Performance at the Telegraph

1. Make web performance visible to non-technical teams.
2. Defer all JavaScript. (I think that's because they're building their pages as static pages, so JavaScript resources aren't their critical resources.)
3. Regular audits of the Tag Managers. (see [Keeping third-party scripts under control](https://web.dev/articles/controlling-third-party-scripts))
4. Testing each additional tag request. They have a blank page with some dummy text on it and a single,synchronous, tag manager. They add the third-party script there and run the page through [WebPageTest](https://www.webpagetest.org/) after the initial benchmarking. (also see [HOW TO BUILD A GOOGLE TAG MANAGER MONITOR](https://www.simoahava.com/analytics/google-tag-manager-monitor/))
5. Monitor synthetically with [SpeedCurve](https://www.speedcurve.com/).
6. Monitor real user by collecting data manually through [Beacon API](https://developer.mozilla.org/en-US/docs/Web/API/Beacon_API)
7. Monitor real user with external services like Akamai’s mPulse and SpeedCurve’s lux.
8. Use server-side tagging. (see [An introduction to server-side tagging](https://developers.google.com/tag-platform/tag-manager/server-side/intro))

# Wix: Trim the Fat from Your Bundles Using Webpack Analyzer and React Lazy/Suspense

1. Know the cost by [Bundlephobia](https://bundlephobia.com/) and [Import Cost Extension](https://marketplace.visualstudio.com/items?itemName=wix.vscode-import-cost).
2. Work closely with the product manager to know which part can be deferred and be split. Then apply something like (chunk-splitting)[https://webpack.js.org/guides/code-splitting/] and (React Suspense)[https://react.dev/reference/react/Suspense].

# Improving Core Web Vitals: A Smashing Magazine Case Study

1. Investigate the LCP issue with [Google Search Console](https://search.google.com/search-console).
2. Try to inline the author image compressed with AVIF then base64, resulting 3KB weight to the HTML. It's a good approach, but LCP issue is still there. Note that inlining is a double-edged sword and must be used with caution. It beefs up the page and means subsequent page view do not benefit from the fact that data is already downloaded.
3. Move author image below the fold. Figure out what's the "right LCP element", which is the article title. It's also a good approach, but LCP issue is still there.
4. Find out the LCP issue is caused by the mobile (Chrome only) users from those countries without good connectivity by collecting RUM data with `web-vitals` then post it back to Google Analytics for analysis. Those mobile (Chrome only) traffic comes from India(31%), United States(13%), Philippines(8%), ..., so that also means countries with more iPhones (like the USA) will have a much smaller proportion of their mobile users represented in CrUX and so in core web vitals.
5. Smashing Magazine then fixed this LCP issue by `Save-Data` and `prefers-reduced-data`. (see [a-smashing-magazine-case-study/v1](/a-smashing-magazine-case-study/README.md#v1---save-data-and-prefers-reduced-data))
