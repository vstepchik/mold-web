[![Build Status](https://travis-ci.org/vstepchik/mold-web.svg?branch=master)](https://travis-ci.org/vstepchik/mold-web)
## My blog

TODO:
* [x] Build compressed CSS from SCSS
* [x] For SCSS enable:
  * [x] Linters
  * [x] Postprocessors
  * [ ] Use [SVGO](https://github.com/svg/svgo) in FE build step
* [x] Show index page
* [x] Show articles
* [x] Show 404 under /a/bla if bla not found
* [x] Shor error page
* [x] Ensure resources are served:
  * [x] just served
  * [x] favicon.ico and robots.txt from root
* [x] Cleanup Dockerfile
* [x] make_favicon.sh -> justfile
* [ ] [Detect](https://stackoverflow.com/questions/56393880/how-do-i-detect-dark-mode-using-javascript) preferred theme
* [x] helper script to check & notify [dev dependencies](#dev-deps) installed
* [ ] https://securityheaders.com/
* [x] https://infosec.mozilla.org/guidelines/web_security#x-frame-options
* [x] https://infosec.mozilla.org/guidelines/web_security#cross-origin-resource-sharing
* [x] https://infosec.mozilla.org/guidelines/web_security#subresource-integrity
* [ ] CI
* [ ] [Setup favicon properly](https://dev.to/masakudamatsu/favicon-nightmare-how-to-maintain-sanity-3al7)

Obsolete:
* To be done by NGINX:
  * Redirect from HTTP to HTTPS
  * Response compression
  * HTTP/2 multiplexing, HTTP3
