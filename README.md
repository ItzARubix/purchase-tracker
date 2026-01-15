# purchase-tracker

I made a purchase tracker using Rust, using [bincode v2.0.1](https://crates.io/crates/bincode/2.0.1) to encode orders to disk.

It's probably still extremely bug-ridden and ugly, but from testing it's "functional" (some strings aren't `.trim()`ed when they should be, so the CLI will look a tad bit broken at times, but I was still able to successfully save an order to a file). 

Commits are accepted if anyone decides they want to make this more functional for whatever reason.

## Regarding bincode

The fiasco with bincode occurred in the middle of the development of this product. I'm sincerely sorry for Zoey, who didn't deserve any of this. Zoey's mentioned that v1.3.3 is production-ready/feature-complete, while v2.0.1 is... not, really. But, there's some stuff in v2.0.1 that I really couldn't do without, so I chose to use it. 

In reality, if I wanted this to be truly production-ready and usable to others, I'd pull myself away from bincode altogether and use a different library. This was mostly bodged together.
