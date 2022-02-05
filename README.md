[![Build Status](https://travis-ci.org/vstepchik/mold-web.svg?branch=master)](https://travis-ci.org/vstepchik/mold-web)
## My blog

TODO:
* [x] Build compressed CSS from SCSS
* [x] For SCSS enable:
  * [x] Linters
  * [x] Postprocessors
* [x] Redirect from HTTP to HTTPS
* [ ] Show index page
* [ ] Show articles
* [x] Shor error page
* [ ] Ensure resources are served:
  * [x] just served
  * [x] compressed
  * [ ] in parallel with HTTP/2
* [ ] [Use CORS for fonts](https://caniuse.com/?search=fontface)
* [ ] Cleanup Dockerfile
* [ ] make_favicon.sh -> Makefile
* [ ] helper script to check & notify dev dependencies installed
* [ ] https://securityheaders.com/
* [ ] https://infosec.mozilla.org/guidelines/web_security#x-frame-options
* [ ] https://infosec.mozilla.org/guidelines/web_security#cross-origin-resource-sharing
* [ ] https://infosec.mozilla.org/guidelines/web_security#subresource-integrity
* [ ] CI


## Dev deps:

1. rustup
2. npm + yarn
3. dart sass (brew install sass/sass/sass)
4. imagemagick
