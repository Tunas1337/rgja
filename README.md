# rgja

![](https://github.com/Tunas1337/rgja/raw/glavna/logo.png)

Aren't you _преку гла_ from writing Rust programs in English? Do you like saying
"у пизду матер" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Macedonian touch to your
programs?

**rgja** ('рѓа, Macedonian for _Rust_) is here to save the day, as it allows you to
write Rust programs in Macedonian, using Macedonian keywords, Macedonian function names,
and Macedonian idioms.

This has been designed to be used as the official programming language to
develop the future Macedonian sovereign operating system.

Don't worry!
Macedonian Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with rgja:

### trait and impl (aka osobina & implementacija_na)

```rust
rgja::rgja! {
    nadvoreshna kutija rgja;

    koristi std::collections::Rechnik kako Rech;

    osobina KlucVrednost {
        funk zapisi(&sebe, kluc: Tekst, vrednost: Tekst);
        funk zemi(&sebe, kluc: Tekst) -> Rezultat<Mozhda<&Tekst>, Tekst>;
    }

    staticno mutabilno RECHNIK: Mozhda<Rech<Tekst, Tekst>> = Nishto;

    objekt Konkretno;

    implementacija_na KlucVrednost za_sekoj Konkretno {
        funk zapisi(&sebe, kluc: Tekst, vrednost: Tekst) {
            neka rech = nesigurno {
                RECHNIK.zemi_ili_vnesi_so(Standard::standard)
            };
            rech.vnesi(kluc, vrednost);
        }
        funk zemi(&sebe, kluc: Tekst) -> Rezultat<Mozhda<&Tekst>, Tekst> {
            ako neka Neshto(rech) = nesigurno { RECHNIK.kako_ref() } {
                Taman(rech.zemi(&kluc))
            } inaku {
                Netaman("nema takvo".stavi_vo())
            }
        }
    }

    funk mozhda(i: u32) -> Mozhda<Rezultat<u32, Tekst>> {
        ako i % 2 == 1 {
            ako i == 42 {
                Neshto(Netaman(Tekst::od("merde")))
            } inaku {
                Neshto(Taman(33))
            }
        } inaku {
            Nishto
        }
    }

    funk glavna() {
        neka mutabilno x = 31;

        sovpadni x {
            42 => {
                ispecati!("omelette du fromage")
            }
            _ => ispecati!("ете работи")
        }

        za_sekoj i vo 0..10 {
            neka val = vokrug {
                stoj i;
            };

            dodeka ima x < val {
                x += 1;
            }

            x = ako neka Neshto(rezultat) = mozhda(i) {
                rezultat.odvitkaj()
            } inaku {
                12
            };
        }
        upm!("демонстрација на краш!");
    }
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Voilà, that's it.

## Other languages

- French: [rouille](https://github.com/bnjbvr/rouille.git)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [ржавчина](https://github.com/FluxIndustries/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)

## la license

[License Publique Rien à Branler](http://sam.zoy.org/lprab/),
_le_ official translation of the [WTFPL](http://www.wtfpl.net/)
by the same author.
