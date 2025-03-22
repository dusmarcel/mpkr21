<div align="center">

  <h1><code>mpkr (Stand 2021)</code></h1>

  <strong>Migrationsrechtlicher Prozesskostenrechner</strong>
</div>

## About

Dieser Prozesskostenrechner berechnet gesetzliche Gebühren auf der Grundlage des Rechtsanwaltsvergütungsgesetzes (RVG), des Gerichtskostengesetzes (GKG), des Streitwertkatalogs des Bundesverwaltungsgerichts und meiner Erfahrung mit der Interpretation dieser Vorgaben durch die Verwaltungsgerichte vornehmlich in NRW. Der Rechner dient nur einer unverbindlichen Orientierung und kann eine fachkundige Beratung nicht ersetzen. Seine Nutzung erfolgt insofern auf eigene Gefahr. Es kann eine Vielzahl von Gründen geben, warum ein Gericht höhere oder niedrigere Kosten festsetzt, als von diesem Rechner ermittelt. Auch möchten viele Rechtsanwält*innen Vergütungsvereinbarungen schließen, die zum Teil deutlich von den gesetzlich vorgesehenen Gebühren abweichen. Der Rechner geht zudem auch davon aus, dass sich die Behördenseite nicht anwaltlich vertreten lässt. Tatsächlich lassen sich Behörden in migrationsrechtlichen Streitigkeiten erfahungsgemäß selten anwaltlich vertreten. Völlig ausgeschlossen ist es aber auch nicht. Ggf. würden hierdurch weitere Kosten entstehen, die dieser Rechner nicht berücksichtigt. Außerdem berücksichtigt dieser Rechner keine Gebühren, die durch Behörden im Verwaltungsverfahren erhoben werden. Widerspruchs- und Remonstrationsverfahren werden ebenfalls (noch?) nicht abgebildet, was vor allem daran liegt, dass ich hauptsächlich in NRW tätig bin, wo es kaum noch Widerspruchsverfahren gibt. Die Anrechnung der Geschäftsgebühr auf die Verfahrensgebühr (Vorbemerkung 4 zu Teil 3 VV RVG) erfolgt immer auf die 1. Instanz des Hauptsacheverfahrens, da dies auch in der Praxis nahezu immer der Fall sein wird. Soweit zumindest theoretisch auch Fälle konstruiert werden können, in denen die Anrechnung auf die Verfahrensgebühr in einer höheren Insatz erfolgt, bleiben diese Fälle hier um der Einfachheit willen unberücksichtigt.

## Installation

Da der Rechner vollständig clientseitig ausgeführt wird, sind serverseitig keine besonderen Vorbereitungen erforderlich. Die Installation sollte auf jedem üblichen Webserver möglich sein. Für die Installation wird [Rust](https://www.rust-lang.org/tools/install) benötigt, außerdem das wasm-pack:

```console
$ cargo install wasm-pack
```

Benötigt wird auch das Target wasm32-unknown-unknown, wobei mir scheint, dass wasm-pack das im Bedarfsfalle auch selber nachlädt, sodass dieser Schritt möglicherweise auch übersprungen werden kann:

```console
$ rustup target add wasm32-unknown-unknown
```

Der Kompiliervorgang wird angestoßen mit

```console
$ wasm-pack build --target web
```

Anschließend müssen nur noch die Dateien index.html, index.js und das Verzeichnis pkg mitsamt Inhalt in das gewünschte Verzeichnis des Websververs verschoben werden.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
