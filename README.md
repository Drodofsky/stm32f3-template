# stm32f3-template
Ein tempflate für den STM32f3

### Installation

Folgen Sie einfach der Anleitung aus dem [Discovery Buch](https://docs.rust-embedded.org/discovery/index.html).

Zusätzlich benötigen Sie [cargo-generate](https://github.com/cargo-generate/cargo-generate).

Um ein Projekt zu generieren können Sie folgenden Befehl nutzten.

 

```bash
cargo generate --git https://github.com/Drodofsky/stm32f3-template.git --branch main
```



Nun starten Sie openocd mit folgendem Befehl:

```bash
openocd \
  -f interface/stlink-v2-1.cfg \
  -f target/stm32f3x.cfg

```

In einem weiten Fenster können Sie nun mit `cargo run` das Programm starten. Mit dem Befehl `continue` starten sie dann die Startfunktion.

 