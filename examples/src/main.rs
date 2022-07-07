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

    asinhrona funk primer() {
    }

    asinhrona funk primer2() {
        primer().pricekaj;
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
