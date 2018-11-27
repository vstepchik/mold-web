use maud::{html, Markup};

use crate::markup::article::{Article, Date};
use crate::markup::icons::{icon, KOTLIN, PYTHON, RUST};

pub const ABOUT_MOLD_WEB: Article = Article {
    title: "About this site",
    date: Date { year: 2018, month: 11, day: 12 },
    summary: &summary,
    body: &body,
};

fn summary() -> Markup {
    html! {
        "The reason I started this project and some details about its implementation."
    }
}

fn body() -> Markup {
    html! {
        p {
"The idea to start the project came into my head while I was on a vacation "
"in Amsterdam. What a wonderful city! That magical and unreal combination of beautiful "
"old architecture, high-tech and nature! \u{1f498}"
        }
        p {
"I’ve been long interested in procedural content generation in general, and map "
"generation in particular, and all kinds of process simulation. I usually acted "
"as a theorist, but when it came to implement something I took the tool I was "
"the most proficient with – JVM, and with a high degree of perfectionism I was "
"designing the system architecture, polish to perfection, until my enthusiasm was gone. "
"So nothing was coming out and I had to find my way, fight perfectionism, find tools "
"that make it faster to get results, and thank you " strong { "crew4ok" }
" for keeping shouting on me “GET SHIT DONE!!!” :D. Now, I’ve stopped upon Python "
(icon(PYTHON)) " as it suits me because there are so many scientific libraries "
"in its ecosystem implementing all kinds of algorithms and data structures "
"(often in C/C++ making them really fast). The only thing that I am missing is strong "
"typing, but still you can attach it to Python if you want. Python greatly shortens "
"the time period from the idea to implementation and reserve of enthusiasm is "
"spent more efficiently."
        }
        p {
"So, on my way to Amsterdam and back I was reading some articles and blog posts about landscape "
"and world map generation. As it turned out, not as few people are interested in the subject "
"as I expected. They were writing about different approaches and ideas they’ve tried and "
"what they’ve got out of them. But what I’ve noticed is that in the Summary section some of the "
"authors were saying that one of the most useful ideas in their experiments was blogging. "
"It forced them getting deeper into the subject, filling the gaps in their knowledge and "
"organizing it in their heads so they can publish that knowledge in text form. Also it was "
"motivating not to give up the project. So I’ve decided to try blogging myself despite "
"my introversion \u{1f605}."
        }
        p {
"Just as always I was not looking for easy ways and haven’t registered an account on Blogspot "
"or somewhere else, but inspired by "
a href="http://motherfuckingwebsite.com/" target="_blank" { "motherfuckingwebsite.com" } ", "
a href="http://bettermotherfuckingwebsite.com/" target="_blank" { "bettermotherfuckingwebsite.com" }  " and "
a href="https://thebestmotherfucking.website/" target="_blank" { "thebestmotherfucking.website" }
" I decided to set up my own server with light and fast back-end and same light and fast "
"front-end. Herewith I didn’t feel like supporting all browsers altogether, so I’ve limited "
"support to "
a href="https://www.mozilla.org/firefox/" target="_blank" { "Firefox" } " and "
a href="https://www.google.com/chrome/" target="_blank" { "Chrome" } " not to support "
"ancient revolting mammoth poop like IE."
        }
        p {
"My main work tools in the recent time are Kotlin " (icon(KOTLIN)) " and Python " (icon(PYTHON))
", and my favorite language, undoubtedly, is Rust " (icon(RUST)) ". Rust is a systems "
"programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. "
"I didn’t like using JVM-based languages because I’ve simply get tired of them, and there are not "
"that many variants of the technology: that’s either bare netty, or relatively "
"heavy servers like vertx or Tomcat/jetty+Spring MVC. And I wouldn’t sleep well if my blog that "
"by design should efficiently use computing resources was written in interpreted language "
"such as Python " (icon(PYTHON)) ". In this way, I was happy to exclude all options to make "
"Rust win \u{1f606}. According to the "
a href="https://www.arewewebyet.org/" target="_blank" { "arewewebyet.org" } " at the "
"moment there are a few web frameworks in Rust’s ecosystem. At first I was "
"looking at " a href="https://crates.io/crates/rocket" target="_blank" { "Rocket" } " just "
"because I like how routes are specified with directives above functions and negligible "
"amount of boilerplate to write. But while I was writing this post I’ve stumbled upon a "
a href="https://www.techempower.com/benchmarks/#section=data-r17&hw=ph&test=plaintext" target="_blank" { "benchmark" }
" that showed Rocket sucks at performance. Instead there was a leader in rust called "
a href="https://crates.io/crates/actix-web" target="_blank" { "actix-web" } " (see actix-raw)."
        }
        p {
"At first I almost did everything as is customary: posts are in database, registration, comments, etc. "
"But roughly weighing how much effort it will take, and recent GDPR issues which I didn’t "
"want do deal with… I even was lazy to set up and support a database. So I decided to "
"put posts under version control and leave everything that is needed to run the blog here: "
a href="https://github.com/vstepchik/mold-web" target="_blank" {"github.com/vstepchik/mold-web"} "."
        }
        p {
"Of course, I had thoughts that if I have no registration, comments, but only "
"static blog posts, I don't need any dynamic server. Just take Nginx, a bunch of HTML and CSS "
"files and go. But no, it’s boring! Besides that, I’d have to implement one of the conceived "
"features with JavaScript: the day/night theme switcher. "
"There is too much entropy in this world to write even more JS. "
"I confess, I still had to write one line of JavaScript to make that feature work: switching "
"the day/night color theme. This line contains single expression that toggles the "
code { "night" } "CSS class on the " code { "html" } " element and sets the corresponding cookie "
"so the server would know if it needs to render pages with the night theme when you visit "
"another page. I like it more than the JS solution: page loads always light, and then when JS is "
"loaded, it checks the cookie (or even worse - the URL path) and sets the corresponding class. "
"It also may cause some irritating blinking on some devices when navigating the website."
        }
        p {
"As i wrote above, I’ve chosen " a href="https://actix.rs" target="_blank" { "actix-web" }
" as a web server, the pages are stored as templates, and are rendered on the server side, "
"they contain single JS line of code, and a tiny bit of CSS so your eyes won’t bleed. All the "
"UI graphics is SVG and " code { "favicon.ico" } " is generated in several sizes out of SVG too. "
"I’ve chosen " a href="https://crates.io/crates/maud" target="_blank" { "Maud" } " as a "
"template engine - I like how it accepts macros as Rust templates and bakes them into "
"static strings at compile time. It all sounds good and well, like it will run fast and "
"take little space, but I was not satisfied yet and thought that if templates are baked "
"into binary file, it would be cool to also include all the static resources into it! "
"All of them: styles, graphics, fonts, whatever. And it turned out there is a library for "
"that: " a href="https://crates.io/crates/includedir" target="_blank" { "includedir" } ". "
"No sooner said then done. The " code { "includedir" } "crate takes contents of specified "
"directory and puts it into a binary as byte arrays which are mapped by their relative "
"paths with the " a href="https://crates.io/crates/phf" target="_blank" { "phf" } " crate. "
"So I’ve used the " code { "phf" } " crate to map articles by their path-id as well."
        }
        p {
"The compiled binary of the Rocket version took around 4Mb disk space because Rust by "
"default does static linking with the all dependencies required to run, which is exactly "
"what I wanted. But I thought I could make the file even smaller. Among all optimization "
"options tried, the most helpful was enabling "
a href = "https://llvm.org/docs/LinkTimeOptimization.html" target ="_blank" { "link time optimization" }
", as it decreased the file size to 1.4Mb, so I’ve turned it on for release builds. "
"There’s another tool called " a href="https://upx.github.io" target="_blank" { "UPX" }
" that compressed the binary to only around 900Kb. But how I was surprised when the debug "
"build in actix-web variant took 70Mb! Fortunately, the release build took only 5.8Mb "
"and UPX-packed variant just 2Mb."
        }
        p {
"When I was done with the blog server, I got curious about how good is the actix-web+maud "
"bundle in terms of performance. To test the latency I took the "
a href="https://github.com/wg/wrk" target="_blank" { "WRK Toolkit" }
" and ran the server on the following setup:"
        }
        ul {
            li { a href="https://ark.intel.com/products/75044/" target="_blank" { "Intel Core i5-4570S" } }
            li { a href="https://www.kingston.com/datasheets/KHX16LC10K2_16X.pdf" target="_blank" { "Kingston 16Gb (2x8Gb) DDR3 1600MHz HyperX LoVo" } }
        }
        p {
"The test lasted for 5 minutes from my laptop connected via LAN, utilizing 8 cores "
"and 10 000 concurrent connections, and shown the following results:"
        }
        p { code { pre {
"""# wrk -c 10000 -t 8 -d 300 --latency http://192.168.1.5:80
Running 5m test @ http://192.168.1.5:80
  8 threads and 10000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    45.93ms   28.09ms 703.36ms   97.10%
    Req/Sec    10.60k     3.13k   20.04k    74.83%
  Latency Distribution
     50%   41.55ms
     75%   46.64ms
     90%   50.17ms
     99%  248.66ms
  25305739 requests in 5.00m, 23.90GB read
  Socket errors: connect 0, read 189, write 12017, timeout 0
Requests/sec:  84333.95
Transfer/sec:   81.55MB"""
         } } }
        p {
"The hecking 84K req/sec! And 90% of the requests finished in under 50ms! There are only 12K "
"unsuccessful requests out of 25M total (0.048%) under such a load. Under the same conditions "
"Rocket was serving 21K req/sec while 90% of the requests finished in under 368ms "
"and much higher error rate."
        }
        p {
"The server job was to parse the HTTP request, properly route it, read the theme cookie, "
"render the page template with dynamic article list and theme considered, and return the "
"response. Actix-web, in contrast to Rocket, did the gzip compression of the responses as well."
        }
        p {
"As a summary, I was satisfied with this experience. I love how the blog server performs, "
"love the design, and thoughts about this blog pushed me to finish this job to the end \u{1f642}."
        }
    }
}
