This implements epub font (de)obfuscation as [described in the epub3 spec](https://www.w3.org/publishing/epub3/epub-ocf.html#sec-resource-obfuscation).

Usage:

```bash
epub-font-obfuscator --id <ID> --input <INPUT> --output <OUTPUT>
```

Where `<ID>` is specified by the epub `dc:identifier` in the `.opf` file.
