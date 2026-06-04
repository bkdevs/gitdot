---
title: "gitdot: designed by developers"
slug: "gitdot-designed-by-developers"
author: "baepaul"
date: "Jun 2, 2026"
---


There is a distance our profession has with the word design.

It gives us engineers pause.
We prefer to live in the world of the concrete — to solve problems with optimal and objective answers.
Realms of subjectivity, where the “best answer” depends on one’s opinion, are something almost impure.

Yet, there are few cohorts that care as much about as design as developers do.
Developers are opinionated, picky people, and the choice of software they use is often more considered than the clothes they wear.

I’m no exception to the rule. I care about design.

The rational reason is that I think it counts: design can be and is a differentiator.
The irrational reason is that I simply can’t not.

I like beautiful things — products that make us happier each time we use them.
I want gitdot to be the same.

That is knowingly hard, but it is with sincerity that I approach it.
Here’s how gitdot was designed.

&nbsp;
## Principles

Six months ago, I wrote down the following principles.
1. Form follows function — products are designed to be used
1. Remove what doesn’t add; make every pixel count
1. Less is more, do few things and do them well

And in each feature I designed since, I attempted to adhere.
That wasn’t easy, as the temptation to start doing without thinking is always there, but I was stubborn as to its necessity. I knew that design without intention just leads to a mess of affectation.

It was my own ego that I wanted to remove.
Engineering is an emotional endeavor — and often, the decisions we make are less an outcome of rationality, but our own sensibilities. Insecurity compels us to add more, anxiety makes our identity unclear, and disregard makes us reckless.

To give a few examples:
1. I was attached to the idea of shortening org urls because I hated seeing repetition, e.g,. `github.com/django/django -> gitdot.io/django`
1. I hated file trees viscerally and convinced myself that they were both inherently ugly and unnecessary
1. I overcomplicated diffs. I spent days building an AST-based monochromatic differ blind to the fact that it barely worked

Those decisions were wrong. 
I wanted a differentiator, so I grasped for the things I knew developers liked (short urls, complicated algorithms). But at the same time, I lost confidence in intuiting customer needs so I chose my own preferences instead (i..e, I dislike file trees, so no file trees).

It’s hard to say that I won’t make the same mistake again.
But I do promise to adhere to the principles above.

&nbsp;
## Decisions

### 1. No loading animations
The absence of is an idea I think about a lot.
Rather than add, remove — and see what that makes us feel.

Many websites are terribly flawed.
They’re riddled with frustrations: loading animations, flickering UIs, and layout shifts.
These are all things we take for granted and we call them necessities, things we accept as the status quo of software.

But I see them as deficiencies.
I want to get rid of it.
I want gitdot to be instant — and for not a damn pixel on the page to flicker.

That is quite hard.
There is a reason these affordances exist and it goes beyond a lack of care.

> “0.1 second is about the limit for having the user feel that the system is reacting instantaneously, meaning that no special feedback is necessary except to display the result.” - Jakob Nielsen, Usability Engineering (1993).

Or in other words, each page must paint within 100 milliseconds to avoid the need for a spinner.
That is egregiously fast, given that a good LCP (Largest Contentful Paint) is 2.5 seconds or less, and that a plain HTML file on localhost paints within 50ms or so.

I spent a lot of time engineering a solution.
I first reached for Next.js with the idea to pre-generate all pages as HTML and serve them from a CDN. 
That, it turns out, is terribly impractical even at limited scale, so I looked elsewhere.

I ended up with a relatively novel architecture that leverages shared web workers, IndexedDB, and a few invariants about git (i.e., blobs and trees are immutable). It is a zero-cost abstraction (pages only ever load faster), maintains semantics around SEO (e.g., no content is reliant on client-side fetches), and is framework agnostic.

It merits its own post, but for now, enjoy the absence of.

<video width="100%" muted>
  <source src="/blog/designed-by-developers-fast.mp4" type="video/mp4" />
</video>

&nbsp;

### 2. No navbar
The browser is an incredible piece of software.
That wasn’t so clear to me six months ago, but I wanted to build a website that respected that.

Most of the websites we use are really “web apps.”
They define their own navigation schemes, their own information hierarchies, their own design systems and expect the user to learn them.

That felt odd to me.
The browser has back and forth buttons, a refresh button, and an auto-complete address bar.
Many of these constructs that web apps invent, the browser already has.

And there is a cost to redundancy: real estate.
For every header we add, every pixel we give to the sidebar, we shrink the canvas used for content.

I hate that.
I hate having to scroll down on a website to get to the content I care about.
I hate the feeling of claustrophobia that large navigational elements give.

The content is primary. It should be the first thing a user sees.
The layout is secondary. It should be as minimal as usably possible.

To illustrate:

<div class="grid grid-cols-2 gap-0 [&>span]:mb-1">
<img src="/blog/designed-by-developers-gitdot-1.png" alt="gitdot 1" />
<img src="/blog/designed-by-developers-github-1.png" alt="github 1" />
<img src="/blog/designed-by-developers-gitdot-2.png" alt="gitdot 2" />
<img src="/blog/designed-by-developers-github-2.png" alt="github 2" />
</div>

Two details define what you see above.

**1\) Beautiful URLs**
gitdot treats the browser’s address bar as its title.

The URL is the title of the page and we don’t see a point in repeating in it.
This forces our URLs to be beautiful, something we take particular pride in:
```
github.com/bkdevs/gitdot/blob/main/gitdot-auth/src/handler.rs
gitdot.io/bkdevs/gitdot/gitdot-auth/src/handler.rs
```

We recognize this paradigm is unusual: most users are not accustomed to looking at their address bar.
We debated it heavily, whether it is affectation or intention that is driving this decision.

But we’ve chosen to stick with it.
We do believe, that as strange as it is, it is a better design.

&nbsp;
**2\) Keyboard affordances**
gitdot assumes you have a keyboard.

We treat the keyboard the same way that CLIs and IDEs do.
The keyboard is more than a tool for text input, but a new mode of interaction entirely.

`h` is short for home, `u` takes you to your profile page, `r` opens a dialog of recent repositories, `p` fuzzy searches files within a repository, `j/k` navigates between items on a page, `tab/⇧+tab` navigates between pages, and `;/:/⌘k` open up the command bar in the footer.

That isn’t to say we neglect the mouse.
Most navigations are possible with mouse and keyboard, but there are preferred modes of interaction for each.

If a user is searching for a file, it makes little sense that he use his mouse to click a search bar and then use his keyboard to type the query. If a user is new to a repository and simply browsing around, it is more intuitive that he simply click.

&nbsp;

### 3. No fluff

gitdot has no logos, no social proof, no call-to-actions, no auth walls, and no AI copilots.

Many patterns in design are awfully effective at inducing behavior.
If you add more product recommendations to an online store, it is likely that people click.
If you gate “free” content behind a signup wall, you will receive emails while frustrating others.
If you plaster your home page with a bunch of social proof, those easily influenced by others may choose you too.

We disagree with them.
Everything should be designed for a purpose, and if that purpose is to advance our own aims at the expense of the customer, we should get rid of it. And least of all, do I wish for a developer to be forced to stare at our logo in their everyday.

&nbsp;

## Details
