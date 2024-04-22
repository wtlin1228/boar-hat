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
