# 📖 Readable

![Screenshot](/assets/screenshot.jpg)

[Readable] is a tiny service for converting web articles to readable text.  
It's like _Reader View_ in Firefox/Safari, but for any device and with a focus on
typography.

## Features

- Removes ads, trackers, and other cruft.
- Presents the article in a clean, readable format.

## Why?

I use it for reading long-form content on my old ebook reader and it's also
great when I'm on the go and don't have a good internet connection.
Read the [announcement blog post](https://endler.dev/2022/readable) for more details.

![My kobo eBook reader](/assets/kobo.jpg)

## Deployment

You can either deploy a self-hosted instance or use [shuttle.rs](shuttle.rs).
Once you signed up for a shuttle.rs account, you can deploy Readable with a
single command:

```bash
make deploy
```

For the self-hosted version, you can just compile the project and run the binary.

```rust
cargo build --release --features local
./target/release/readable
```

This will start a server on port 8000.

## Contributions Welcome!

It's far from perfect and I built this for my own use.
I'm happy to accept pull requests and I'm looking for co-maintainers.
Reach out if you're interested. 🌈

## Credits

Built with 🚀 [shuttle](https://shuttle.rs).

[readable]: https://readable.shuttleapp.rs
