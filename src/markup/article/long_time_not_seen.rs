use maud::{html, Markup};

use crate::markup::article::{Article, Date};

pub const LONG_TIME_NOT_SEEN: Article = Article {
    title: "Long time not seen",
    date: Date {
        year: 2024,
        month: 04,
        day: 23,
    },
    keywords: &[
        "mold",
        "mildew",
        "world",
        "personal blog",
        "blog",
        "vitalatron",
        "vstepchik",
        "vstepchyk",
        "vitalii vstepchyk",
        "akiakane",
        "design",
    ],
    summary: &summary,
    body: &body,
};

fn summary() -> Markup {
    html! {
        "Some thoughts after not posting anything for almost 6 years."
    }
}

fn body() -> Markup {
    html! {
        p {
            "Well, I did not update the website for a while, "
            "so the libraries became so outdated that I found it easier to recreate the website from scratch \u{1f642}."
        }
        p {
            "Quite a lot has happened since my first post: I changed several jobs, including writing Rust professionally, turned 30, "
            "got into a relationship, moved to the Netherlands, and met a lot of great people there, russia attacked my country, "
            "developed an addiction to YouTube, turned 33, started taking antidepressants, so much stuff!"
        }
        p {
            "Time passed, I was growing psychologically, patching my traumas, fixing life, etc, and among other things, "
            "I decided that I needed a place to share my creations with the world. "
            "Many people are doing it on Instagram or YouTube, however, I deeply despise Instagram, don’t like X (Twitter) "
            "and YouTube assumes work with video which I don’t find interesting at the moment. "
            "Then I recalled that I have such a place (" code { "this" } ") so I wanted to update the website to make blogging easier than it is now, "
            "but it turned out that the libraries I used evolved so much and made so many breaking changes that it was easier to rewrite it completely from scratch. So I did. \u{1f642}"
        }
        p {
            "In the current implementation, the website uses a single hand-written static CSS file, and articles are stored as Maud templates in the Rust sources under version control. "
            "I was proud of having a super-lightweight website, but I found creating content for it a tedious process that increased friction to start writing something."
        }
        p {
            "My new idea is to still store the content under version control but in a different format, most probably Markdown or something similar, "
            "and then turn it into HTML as a compilation step or on the client side."
        }
        p {
            "I gave up on the idea of not using JS at all as it seriously restricts the variety of tools that are available to me, "
            "for example, the said conversion of Markdown to HTML. "
            "Another reason is that I was thinking that I'd like to add a forum to the site and use WebSockets for dynamic content loading, "
            "or even load WebAssembly code."
        }
        p {
            "I already drew a new logo in SVG:"
            br;
            center { img src="/logo_2024.svg" alt="New logo SVG"; }
            br;
            "I like that it resembles a mushroom, a brain, black dripping goo, and a mold (mildew) at the same time - "
            "this gives me the vibe of a game I have always dreamed of making."
        }
        p {
            "In addition to the logo, I came up with a palette that I want to use. "
            "To create the palette I've used a bunch of artwork from one of my favorite artists " a href="http://akiakane.net/" target="_blank" rel="noopener noreferrer" { "Akiakane (秋赤音)" } " "
            "and with the power of the " a href="https://crates.io/crates/quantette" target="_blank" rel="noopener noreferrer" { code { "quantette" } } " crate "
            "converted them to the " a href="https://bottosson.github.io/posts/oklab/" target="_blank" rel="noopener noreferrer" { "OkLAB" } " color space, "
            "and derived a limited set of colors to use."
        }
        p {
            "Another piece of inspiration for the future design comes from " 
            a href="https://www.versionmuseum.com/history-of/classic-mac-os" target="_blank" rel="noopener noreferrer" { "MacOS 1-8" } ". "
            "Particularly I like the clear straight lines, use of limited palettes, dithering, and play with contrast."
        }
        p {
            "Enough talking about plans, let's focus on the process instead."
        }
        p {
            "Wish me luck \u{1f600}."
        }
    }
}
