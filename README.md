[![Build Status](https://travis-ci.org/vstepchik/mold-web.svg?branch=master)](https://travis-ci.org/vstepchik/mold-web)
## My blog

TODO:
* [x] Build compressed CSS from SCSS
* [x] For SCSS enable:
  * [x] Linters
  * [x] Postprocessors
* [x] Show index page
* [x] Show articles
* [ ] Show 404 under /a/bla if bla not found
* [ ] Use PHF codegen to gather articles
* [x] Shor error page
* [ ] Ensure resources are served:
  * [x] just served
  * [ ] favicon.ico and robots.txt from root
* [ ] [Use CORS for fonts](https://caniuse.com/?search=fontface)
* [ ] Cleanup Dockerfile
* [ ] make_favicon.sh -> Makefile
* [ ] [Detect](https://stackoverflow.com/questions/56393880/how-do-i-detect-dark-mode-using-javascript) preferred theme
* [ ] helper script to check & notify [dev dependencies](#dev-deps) installed
* [ ] https://securityheaders.com/
* [ ] https://infosec.mozilla.org/guidelines/web_security#x-frame-options
* [ ] https://infosec.mozilla.org/guidelines/web_security#cross-origin-resource-sharing
* [ ] https://infosec.mozilla.org/guidelines/web_security#subresource-integrity
* [ ] CI

Obsolete:
* To be done by NGINX:
  * Redirect from HTTP to HTTPS
  * Response compression
  * HTTP/2 multiplexing, HTTP3

## Dev deps:

1. rustup
2. npm + yarn
3. dart sass (brew install sass/sass/sass)
4. imagemagick
