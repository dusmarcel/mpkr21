mod utils;

use wasm_bindgen::prelude::*;

enum Thema {
    AsylZulaessigkeit,
    AsylAnerkennung,
    AsylWiderruf,
    AsylUntaetigkeit,
    Aufenthaltstitel,
    Ausweisung,
    Pass,
    Duldung,
    Einbuergerung,
}

enum Verfahren {
    Hauptsache,
    Vorlaeufig,
    Beides,
}

fn gkg_geb(streitwert: f64) -> u32 {
    let mut tmp_wert = 500.0;
    let mut gkg_geb = 38;
    while streitwert > tmp_wert {
        if tmp_wert < 2000.0 {
            gkg_geb += 20;
            tmp_wert += 500.0;
        } else if tmp_wert < 10000.0 {
            gkg_geb += 21;
            tmp_wert += 1000.0;
        } else if tmp_wert < 25000.0 {
            gkg_geb += 29;
            tmp_wert += 3000.0;
        } else if tmp_wert < 50000.0 {
            gkg_geb += 38;
            tmp_wert += 5000.0;
        } else if tmp_wert <= 200000.0 {
            gkg_geb += 132;
            tmp_wert += 15000.0;
        } else if tmp_wert < 500000.0 {
            gkg_geb += 198;
            tmp_wert += 30000.0;
        } else {
            gkg_geb += 198;
            tmp_wert += 50000.0;
        }
    };
    gkg_geb
}

fn rvg13_geb(streitwert: f64) -> u32 {
    let mut tmp_wert = 500.0;
    let mut rvg13_geb = 49;
    while streitwert > tmp_wert {
        if tmp_wert < 2000.0 {
            rvg13_geb += 39;
            tmp_wert += 500.0;
        } else if tmp_wert < 10000.0 {
            rvg13_geb += 56;
            tmp_wert += 1000.0;
        } else if tmp_wert < 25000.0 {
            rvg13_geb += 52;
            tmp_wert += 3000.0;
        } else if tmp_wert < 50000.0 {
            rvg13_geb += 81;
            tmp_wert += 5000.0;
        } else if tmp_wert <= 200000.0 {
            rvg13_geb += 94;
            tmp_wert += 15000.0;
        } else if tmp_wert < 500000.0 {
            rvg13_geb += 132;
            tmp_wert += 30000.0;
        } else {
            rvg13_geb += 165;
            tmp_wert += 50000.0;
        }
    }
    rvg13_geb
}

fn rvg49_geb(streitwert: f64) -> u32 {
    if streitwert <= 4000.0 {
        rvg13_geb(streitwert)
    } else if streitwert <= 5000.0 {
        284
    } else if  streitwert <= 6000.0 {
        295
    } else if streitwert <= 7000.0 {
        306
    } else if streitwert <= 8000.0 {
        317
    } else if streitwert <= 9000.0 {
        328
    } else if streitwert <= 10000.0 {
        339
    } else if streitwert <= 13000.0 {
        354
    } else if streitwert <= 16000.0 {
        369
    } else if streitwert <= 19000.0 {
        384
    } else if streitwert <= 22000.0 {
        399
    } else if streitwert <= 25000.0 {
        414
    } else if streitwert <= 30000.0 {
        453
    } else if streitwert <= 35000.0 {
        492
    } else if streitwert <= 40000.0 {
        531
    } else if streitwert <= 45000.0 {
        570
    } else if streitwert <= 50000.0 {
        609
    } else {
        659
    }
}

#[wasm_bindgen]
pub fn auffangstreitwert() -> u32 {
    5000
}

#[wasm_bindgen]
pub struct Mpkr {
    thema: Thema,
    verfahren: Verfahren,
    anzahl: u32,
    streitwert: f64,
    streitwert_v: f64,
    rvg13_geb_h: u32,
    rvg49_geb_h: u32,
    gkg_geb_h: u32,
    rvg13_geb_v: u32,
    rvg49_geb_v: u32,
    gkg_geb_v: u32,
    aussergerichtlich: bool,
    gebuehrensatz: f64,
    geschaeftsgebuehr: f64,
    pauschale_aussergerichtlich: f64,
    auslagen_aussergerichtlich: f64,
    summe_aussergerichtlich: f64,
    instanz_h1: bool,
    instanz_h2: bool,
    instanz_h3: bool,
    h1_3100: bool,
    h1_3100_13: f64,
    h1_3100_49: f64,
    h1_3101: bool,
    h1_anrechnung: bool,
    h1_3104: bool,
    h1_termgeb13: f64,
    h1_termgeb49: f64,
    h1_7002: bool,
    h1_pauschale: f64,
    h1_7000ua: bool,
    h1_auslagen: f64,
    summe_rvg13_h1: f64,
    summe_rvg49_h1: f64,
    h1_5110: bool,
    h1_5111: bool,
    summe_gkg_h1: f64,
    summe_rvg13_h2: f64,
    summe_rvg49_h2: f64,
    summe_gkg_h2: f64,
    summe_rvg13_h3: f64,
    summe_rvg49_h3: f64,
    summe_gkg_h3: f64,    
    instanz_v1: bool,
    instanz_v2: bool,
    summe_gkg_v1: f64,
    summe_rvg13_v2: f64,
    summe_rvg49_v2: f64,
    summe_gkg_v2: f64,
    summe_rvg13_h: f64,
    summe_rvg49_h: f64,
    summe_gkg_h: f64,
    summe_rvg13_v: f64,
    summe_rvg49_v: f64,
    summe_gkg_v: f64,
    summe_netto: f64,
    //summe49: f64,
    steuersatz: u32,
    umsatzsteuer: f64,
    summe_brutto: f64,
    summe_gkg: f64,
    summe_total: f64,
}

impl Mpkr {

    fn anzahl(&self) -> u32 {
        self.anzahl
    }

    fn default_streitwert(&mut self) {
        match self.thema {
            Thema::AsylZulaessigkeit => self.set_streitwert((5000+((self.anzahl()-1)*1000)) as f64),
            Thema::AsylAnerkennung => self.set_streitwert((5000+((self.anzahl()-1)*1000))as f64),
            Thema::AsylWiderruf => self.set_streitwert((5000+((self.anzahl()-1)*1000)) as f64),
            Thema::AsylUntaetigkeit => self.set_streitwert((2500+((self.anzahl()-1)*500)) as f64),
            Thema::Aufenthaltstitel => self.set_streitwert((5000*self.anzahl()) as f64),
            Thema::Ausweisung => self.set_streitwert((5000*self.anzahl()) as f64),
            Thema::Pass =>  self.set_streitwert((5000*self.anzahl()) as f64),
            Thema::Duldung => self.set_streitwert((2500*self.anzahl()) as f64),
            Thema::Einbuergerung => self.set_streitwert((10000*self.anzahl()) as f64),
        };
        self.set_streitwert_v(self.streitwert() / 2.0);
    }

    fn set_summe_netto_auto(&mut self) {
        let mut summe = 0.0;
        if self.aussergerichtlich { summe += self.summe_aussergerichtlich; }
        //if self.instanz_h1 { summe += self.summe_rvg13_h1; }
        summe += self.summe_rvg13_h;
        self.set_summe_netto(summe);
    }

    fn set_summen_h1_auto(&mut self) {
        let mut summe_rvg13 = 0.0;
        let mut summe_rvg49 = 0.0;
        let mut summe_gkg = 0.0;
        if self.h1_3101 {
            summe_rvg13 += 0.8 * self.rvg13_geb_h() as f64;
            summe_rvg49 += 0.8 * self.rvg49_geb_h() as f64;
        } else if self.h1_3100 {
            summe_rvg13 += 1.3 * self.rvg13_geb_h() as f64;
            summe_rvg49 += 1.3 * self.rvg49_geb_h() as f64;
        }
        if self.h1_5111 {
            summe_gkg += self.gkg_geb_h() as f64;
        } else if self.h1_5110 {
            summe_gkg += 3.0 * self.gkg_geb_h() as f64;
        }
        self.set_summe_rvg13_h1(summe_rvg13);
        self.set_summe_rvg49_h1(summe_rvg49);
        self.set_summe_gkg_h1(summe_gkg);
        self.set_summe_rvg13_h_auto();
        self.set_summe_rvg49_h_auto();
        self.set_summe_gkg_h_auto();
    }

    fn set_summe_rvg13_h_auto(&mut self) {
        let mut summe = 0.0;
        if self.instanz_h1 { summe += self.summe_rvg13_h1; }
        if self.instanz_h2 { summe += self.summe_rvg13_h2; }
        if self.instanz_h3 { summe += self.summe_rvg13_h3; }
        self.set_summe_rvg13_h(summe);
    }

    fn set_summe_rvg49_h_auto(&mut self) {
        let mut summe = 0.0;
        if self.instanz_h1 { summe += self.summe_rvg49_h1; }
        if self.instanz_h2 { summe += self.summe_rvg49_h2; }
        if self.instanz_h3 { summe += self.summe_rvg49_h3; }
        self.set_summe_rvg49_h(summe);
    }

    fn set_summe_gkg_h_auto(&mut self) {
        let mut summe = 0.0;
        if self.instanz_h1 { summe += self.summe_gkg_h1; }
        if self.instanz_h2 { summe += self.summe_gkg_h2; }
        if self.instanz_h3 { summe += self.summe_gkg_h3; }
        self.set_summe_gkg_h(summe);
    }
}

#[wasm_bindgen]
impl Mpkr {
    pub fn new() -> Mpkr {
        utils::set_panic_hook();

        let thema = Thema::Aufenthaltstitel;
        let verfahren = Verfahren::Hauptsache;
        let anzahl = 1;
        let streitwert = auffangstreitwert() as f64;
        let streitwert_v = streitwert / 2.0;
        let rvg13_geb_h = rvg13_geb(streitwert);
        let rvg49_geb_h = rvg49_geb(streitwert);
        let gkg_geb_h = gkg_geb(streitwert);
        let rvg13_geb_v = rvg13_geb(streitwert_v);
        let rvg49_geb_v = rvg49_geb(streitwert_v);
        let gkg_geb_v = gkg_geb(streitwert_v);
        let aussergerichtlich = false;
        let gebuehrensatz = 1.3;
        let geschaeftsgebuehr = gebuehrensatz * (rvg13_geb_h as f64);
        let pauschale_aussergerichtlich = 20.0;
        let auslagen_aussergerichtlich = 0.0;
        let summe_aussergerichtlich = geschaeftsgebuehr + pauschale_aussergerichtlich + auslagen_aussergerichtlich;
        let instanz_h1 = true;
        let instanz_h2 = false;
        let instanz_h3 = false;
        let h1_3100 = true;
        let h1_3100_13 = 1.3 * rvg13_geb_h as f64;
        let h1_3100_49 = 1.3 * rvg49_geb_h as f64;
        let h1_3101 = false;
        let h1_anrechnung = false;
        let h1_3104 = true;
        let h1_termgeb13 = 1.2 * rvg13_geb_h as f64;
        let h1_termgeb49 = 1.2 * rvg49_geb_h as f64;
        let h1_7002 = true;
        let h1_pauschale = 20.0;
        let h1_7000ua = false;
        let h1_auslagen = 0.0;
        let summe_rvg13_h1 = (1.3 * rvg13_geb_h as f64) + h1_termgeb13 + h1_pauschale + h1_auslagen;
        let summe_rvg49_h1 = (1.3 * rvg49_geb_h as f64) + h1_termgeb49 + h1_pauschale + h1_pauschale;
        let h1_5110 = true;
        let h1_5111 = false;
        let summe_gkg_h1 = 3.0 * gkg_geb_h as f64;
        let summe_rvg13_h2 = 0.0;
        let summe_rvg49_h2 = 0.0;
        let summe_gkg_h2 = 0.0;
        let summe_rvg13_h3 = 0.0;
        let summe_rvg49_h3 = 0.0;
        let summe_gkg_h3 = 0.0;
        let instanz_v1 = true;
        let instanz_v2 = false;
        let summe_rvg13_v1 = 0.0;
        let summe_rvg49_v1 = 0.0;
        let summe_gkg_v1 = 0.0;
        let summe_rvg13_v2 = 0.0;
        let summe_rvg49_v2 = 0.0;
        let summe_gkg_v2 = 0.0;
        let summe_rvg13_h = summe_rvg13_h1 + summe_rvg13_h2 + summe_rvg13_h3;
        let summe_rvg49_h = summe_rvg49_h1 + summe_rvg49_h2 + summe_rvg49_h3;
        let summe_gkg_h = summe_gkg_h1 + summe_gkg_h2 + summe_gkg_h3;
        let summe_rvg13_v = summe_rvg13_v1 + summe_rvg13_v2;
        let summe_rvg49_v = summe_rvg49_v1 + summe_rvg49_v2;
        let summe_gkg_v = summe_gkg_v1 + summe_gkg_v2;
        let summe_netto = summe_rvg13_h + summe_rvg13_v;
        //let summe49 = summe_rvg49_h + summe_rvg49_v;
        let steuersatz = 19;
        let umsatzsteuer = summe_netto / 100.0 * (steuersatz as f64);
        let summe_brutto = summe_netto + umsatzsteuer;
        let summe_gkg = summe_gkg_v + summe_gkg_h;
        let summe_total = summe_brutto + summe_gkg;
        
        Mpkr {
            thema,
            verfahren,
            anzahl,
            streitwert,
            streitwert_v,
            rvg13_geb_h,
            rvg49_geb_h,
            gkg_geb_h,
            rvg13_geb_v,
            rvg49_geb_v,
            gkg_geb_v,
            aussergerichtlich,
            gebuehrensatz,
            geschaeftsgebuehr,
            pauschale_aussergerichtlich,
            auslagen_aussergerichtlich,
            summe_aussergerichtlich,
            instanz_h1,
            instanz_h2,
            instanz_h3,
            h1_3100,
            h1_3100_13,
            h1_3100_49,
            h1_3101,
            h1_anrechnung,
            h1_3104,
            h1_termgeb13,
            h1_termgeb49,
            h1_7002,
            h1_pauschale,
            h1_7000ua,
            h1_auslagen,
            summe_rvg13_h1,
            summe_rvg49_h1,
            h1_5110,
            h1_5111,
            summe_gkg_h1,
            summe_rvg13_h2,
            summe_rvg49_h2,
            summe_gkg_h2,
            summe_rvg13_h3,
            summe_rvg49_h3,
            summe_gkg_h3,
            instanz_v1,
            instanz_v2,
            summe_gkg_v1,
            summe_rvg13_v2,
            summe_rvg49_v2,
            summe_gkg_v2,
            summe_rvg13_h,
            summe_rvg49_h,
            summe_gkg_h,
            summe_rvg13_v,
            summe_rvg49_v,
            summe_gkg_v,
            summe_netto,
            //summe49,
            steuersatz,
            umsatzsteuer,
            summe_brutto,
            summe_gkg,
            summe_total,
        }
    }
    
    //pub fn thema(self) -> u32 {
    //    self.thema as u32
    //}

    pub fn set_thema(&mut self, u_thema: u32) {
        self.thema = match u_thema {
            0 => Thema::AsylZulaessigkeit,
            1 => Thema::AsylAnerkennung,
            2 => Thema::AsylWiderruf,
            3 => Thema::AsylUntaetigkeit,
            5 => Thema::Ausweisung,
            6 => Thema::Pass,
            7 => Thema::Duldung,
            8 => Thema::Einbuergerung,
            _ => Thema::Aufenthaltstitel,
        };
        self.default_streitwert();
    }    

    pub fn set_verfahren(&mut self, u_verfahren: u32) {
        self.verfahren = match u_verfahren {
            1 => Verfahren::Vorlaeufig,
            2 => Verfahren::Beides,
            _ => Verfahren::Hauptsache,
        };
        match self.verfahren {
            Verfahren::Vorlaeufig => self.aussergerichtlich = false,
            _ => (),
        }           
        self.default_streitwert();
    }

    pub fn verfahren(&self) -> u32 {
        match self.verfahren {
            Verfahren::Hauptsache => 0,
            Verfahren::Vorlaeufig => 1,
            Verfahren::Beides => 2,
        }
    }

    pub fn set_anzahl(&mut self, anzahl: u32) {
        self.anzahl = anzahl;
        self.default_streitwert();
    }
    
    pub fn set_streitwert(&mut self, streitwert: f64) {
        self.streitwert = streitwert;
        self.rvg13_geb_h = rvg13_geb(streitwert);
        self.rvg49_geb_h = rvg49_geb(streitwert);
        self.gkg_geb_h = match self.thema {
            Thema::AsylZulaessigkeit |
            Thema::AsylAnerkennung |
            Thema::AsylWiderruf |
            Thema::AsylUntaetigkeit => 0,
            _ => gkg_geb(streitwert),
        };
        self.set_geschaeftsgebuehr(self.rvg13_geb_h as f64 * self.gebuehrensatz);
        self.set_summen_h1_auto();
    }

    pub fn streitwert(&self) -> f64 {
        self.streitwert
    }

    pub fn set_streitwert_v(&mut self, streitwert_v: f64) {
        self.streitwert_v = streitwert_v;
        self.rvg13_geb_v = rvg13_geb(streitwert_v);
        self.rvg49_geb_v = rvg49_geb(streitwert_v);
        self.gkg_geb_v = match self.thema {
            Thema::AsylZulaessigkeit |
            Thema::AsylAnerkennung |
            Thema::AsylWiderruf |
            Thema::AsylUntaetigkeit => 0,
            _ => gkg_geb(streitwert_v),
        }
    }

    pub fn streitwert_v(&self) -> f64 {
        self.streitwert_v
    }

    pub fn rvg13_geb_h(&self) -> u32 {
        self.rvg13_geb_h
    }

    pub fn rvg49_geb_h(&self) -> u32 {
        self.rvg49_geb_h
    }

    pub fn gkg_geb_h(&self) -> u32 {
        self.gkg_geb_h
    }

    pub fn rvg13_geb_v(&self) -> u32 {
        self.rvg13_geb_v
    }

    pub fn rvg49_geb_v(&self) -> u32 {
        self.rvg49_geb_v
    }

    pub fn gkg_geb_v(&self) -> u32 {
        self.gkg_geb_v
    }

    pub fn set_aussergerichtlich(&mut self, aussergerichtlich: bool) {
        match self.verfahren {
            Verfahren::Vorlaeufig => self.aussergerichtlich = false,
            _ => self.aussergerichtlich = aussergerichtlich,
        }
        self.set_summe_netto_auto();
    }

    pub fn aussergerichtlich(&self) -> bool {
        self.aussergerichtlich
    }

    pub fn set_gebuehrensatz(&mut self, gebuehrensatz: f64) {
        self.gebuehrensatz = gebuehrensatz;
        if self.gebuehrensatz < 0.5 { self.gebuehrensatz = 0.5 }
        else if self.gebuehrensatz > 2.5 { self.gebuehrensatz = 2.5 };
        self.set_geschaeftsgebuehr((self.rvg13_geb_h as f64) * self.gebuehrensatz);
    }

    pub fn gebuehrensatz(&self) -> f64 {
        self.gebuehrensatz
    }

    pub fn set_geschaeftsgebuehr(&mut self, geschaeftsgebuehr: f64) {
        let mut pauschale = geschaeftsgebuehr * 0.2;
        if pauschale > 20.0 { pauschale = 20.0 };
        self.geschaeftsgebuehr = geschaeftsgebuehr;
        self.set_pauschale_aussergerichtlich(pauschale);
        self.set_summe_aussergerichtlich(self.geschaeftsgebuehr() + self.pauschale_aussergerichtlich() + self.auslagen_aussergerichtlich());
    }

    pub fn geschaeftsgebuehr(&self) -> f64 {
        self.geschaeftsgebuehr
    }

    pub fn set_pauschale_aussergerichtlich(&mut self, pauschale: f64) {
        self.pauschale_aussergerichtlich = pauschale;
        self.set_summe_aussergerichtlich(self.geschaeftsgebuehr() + self.pauschale_aussergerichtlich() + self.auslagen_aussergerichtlich());
    }

    pub fn pauschale_aussergerichtlich(&self) -> f64 {
        self.pauschale_aussergerichtlich
    }

    pub fn set_auslagen_aussergerichtlich(&mut self, auslagen: f64) {
        self.auslagen_aussergerichtlich = auslagen;
        self.set_summe_aussergerichtlich(self.geschaeftsgebuehr() + self.pauschale_aussergerichtlich() + self.auslagen_aussergerichtlich());
    }

    pub fn auslagen_aussergerichtlich(&self) -> f64 {
        self.auslagen_aussergerichtlich
    }

    pub fn set_summe_aussergerichtlich(&mut self, summe: f64) {
        self.summe_aussergerichtlich = summe;
        self.set_summe_netto_auto();
    }

    pub fn summe_aussergerichtlich(&self) -> f64 {
        self.summe_aussergerichtlich
    }

    pub fn set_instanz_h1(&mut self, i: bool) {
        self.instanz_h1 = i;
    }

    pub fn instanz_h1(&self) -> bool {
        self.instanz_h1
    }

    pub fn set_instanz_h2(&mut self, i: bool) {
        self.instanz_h2 = i;
    }

    pub fn instanz_h2(&self) -> bool {
        self.instanz_h2
    }

        pub fn set_instanz_h3(&mut self, i: bool) {
        self.instanz_h3 = i;
    }

    pub fn instanz_h3(&self) -> bool {
        self.instanz_h3
    }

    pub fn set_h1_3100(&mut self, i: bool) {
        self.h1_3100 = i;
        self.set_summen_h1_auto();
    }

    pub fn h1_3100(&self) -> bool {
        self.h1_3100
    }

    pub fn set_h1_3100_13(&mut self, h1_3100_13 :f64) {
        self.h1_3100_13 = h1_3100_13;
    }

    pub fn h1_3100_13(&self) -> f64 {
        self.h1_3100_13
    }

    pub fn h1_3100_49(&self) -> f64 {
        self.h1_3100_49
    }

    pub fn set_h1_3100_49(&mut self, h1_3100_49 :f64) {
        self.h1_3100_49 = h1_3100_49;
    }

    pub fn set_h1_3101(&mut self, i: bool) {
        if i { self.set_h1_3100(true); }
        self.h1_3101 = i;
        self.set_summen_h1_auto();
    }

    pub fn h1_3101(&self) -> bool {
        self.h1_3101
    }

    pub fn set_summe_rvg13_h1(&mut self, summe_rvg13_h1: f64) {
        self.summe_rvg13_h1 = summe_rvg13_h1;
    }

    pub fn set_summe_rvg49_h1 (&mut self, summe_rvg49_h1: f64) {
        self.summe_rvg49_h1 = summe_rvg49_h1;
    }

    pub fn set_summe_gkg_h1(&mut self, summe_gkg_h1: f64) {
        self.summe_gkg_h1 = summe_gkg_h1;
    }

    pub fn set_instanz_v1(&mut self, i: bool) {
        self.instanz_v1 = i;
    }

    pub fn instanz_v1(&self) -> bool {
        self.instanz_v1
    }

    pub fn set_instanz_v2(&mut self, i: bool) {
        self.instanz_v2 = i;
    }

    pub fn instanz_v2(&self) -> bool {
        self.instanz_v2
    }

    pub fn set_summe_rvg13_h(&mut self, summe: f64) {
        self.summe_rvg13_h = summe;
    }

    pub fn summe_rvg13_h(&self) -> f64 {
        self.summe_rvg13_h
    }

    pub fn set_summe_rvg49_h(&mut self, summe: f64) {
        self.summe_rvg49_h = summe;
    }
    
    pub fn summe_rvg49_h(&self) -> f64 {
        self.summe_rvg49_h
    }

    pub fn set_summe_gkg_h(&mut self, summe: f64) {
        self.summe_gkg_h = summe;
    }
    
    pub fn summe_gkg_h(&self) -> f64 {
        self.summe_gkg_h
    }

    pub fn summe_rvg13_v(&self) -> f64 {
        self.summe_rvg13_v
    }

    pub fn summe_rvg49_v(&self) -> f64 {
        self.summe_rvg49_v
    }

    pub fn summe_gkg_v(&self) -> f64 {
        self.summe_gkg_v
    }

    pub fn summe_gkg(&self) -> f64 {
        self.summe_gkg
    }

    pub fn set_summe_netto(&mut self, summe: f64) {
        self.summe_netto = summe;
        self.set_umsatzsteuer(self.summe_netto() / 100.0 * (self.steuersatz() as f64));
    }

    pub fn summe_netto(&self) -> f64 {
        self.summe_netto
    }

    pub fn set_steuersatz(&mut self, steuersatz: u32) {
        self.steuersatz = steuersatz;
        self.set_umsatzsteuer(self.summe_netto() / 100.0 * (self.steuersatz() as f64));
    }

    pub fn steuersatz(&self) -> u32 {
        self.steuersatz
    }

    pub fn set_umsatzsteuer(&mut self, umsatzsteuer: f64) {
        self.umsatzsteuer = umsatzsteuer;
        self.set_summe_brutto(self.summe_netto() + self.umsatzsteuer());
    }

    pub fn umsatzsteuer(&self) -> f64 {
        self.umsatzsteuer
    }

    pub fn set_summe_brutto(&mut self, summe: f64) {
        self.summe_brutto = summe;
        self.summe_total = self.summe_brutto + self.summe_gkg;
    }

    pub fn summe_brutto(&self) -> f64 {
        self.summe_brutto
    }

    pub fn summe_total(&self) -> f64 {
        self.summe_total
    }
}