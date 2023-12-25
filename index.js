const popoverTriggerList = document.querySelectorAll('[data-bs-toggle="popover"]');
const popoverList = [...popoverTriggerList].map(popoverTriggerEl => new bootstrap.Popover(popoverTriggerEl));

import init, { Mpkr } from "./pkg/mpkr.js";

async function main() {
    await init();

    const mpkr = Mpkr.new();

    const thema = document.getElementById("thema");
    thema.addEventListener("change", (event) => {
        mpkr.set_thema(parseInt(event.target.value));
        update();
    });

    const verfahren = document.getElementById("verfahren");
    verfahren.addEventListener("change", (event) => {
        mpkr.set_verfahren(parseInt(event.target.value));
        update();
    });

    const anzahl = document.getElementById("anzahl");
    anzahl.addEventListener("change", (event) => {
        mpkr.set_anzahl(parseInt(event.target.value));
        update();
    });

    const l_verfahren_1 = document.getElementById("l_verfahren_1");
    const streitwert = document.getElementById("streitwert");
    streitwert.addEventListener("change", (event) => {
        if (mpkr.verfahren() == 1) // nur Verfahren zum vorläufigen Rechtsschutz
            mpkr.set_streitwert_v(parseFloat(event.target.value.replace(',', '.')));
        else // auch Hauptsacheverfahren
            mpkr.set_streitwert(parseFloat(event.target.value.replace(',', '.')));
        update();
    });

    const bs_div_l_streitwert_v = new bootstrap.Collapse("#div_l_streitwert_v", { toggle: false });
    const bs_div_streitwert_v = new bootstrap.Collapse("#div_streitwert_v", { toggle: false });
    const streitwert_v = document.getElementById("streitwert_v");
    streitwert_v.addEventListener("change", (event) => {
        mpkr.set_streitwert_v(parseFloat(event.target.value.replace(',', '.')));
        update();
    });

    const l_geb13_1 = document.getElementById("l_geb13_1");
    const l_geb49_1 = document.getElementById("l_geb49_1");
    const l_gkg_1 = document.getElementById("l_gkg_1");
    const l_geb13_2 = document.getElementById("l_geb13_2");
    const l_geb49_2 = document.getElementById("l_geb49_2");
    const l_gkg_2 = document.getElementById("l_gkg_2");

    const aussergerichtlich = document.getElementById("aussergerichtlich");
    aussergerichtlich.addEventListener("change", (event) => {
        mpkr.set_aussergerichtlich(event.target.checked);
        update();
    });

    const bs_div_aussergerichtlich = new bootstrap.Collapse("#div_aussergerichtlich", { toggle: false });

    const gebuehrensatz = document.getElementById("gebuehrensatz");
    gebuehrensatz.addEventListener("change", (event) => {
        mpkr.set_gebuehrensatz(parseFloat(event.target.value));
        update();
    });

    const geschaeftsgebuehr = document.getElementById("geschaeftsgebuehr");
    geschaeftsgebuehr.addEventListener("change", (event) => {
        mpkr.set_geschaeftsgebuehr(event.target.value.replace(',', '.'));
        update();
    });

    const pauschale_aussergerichtlich = document.getElementById("pauschale_aussergerichtlich");
    pauschale_aussergerichtlich.addEventListener("change", (event) => {
        mpkr.set_pauschale_aussergerichtlich(event.target.value.replace(',', '.'));
        update();
    });

    const auslagen_aussergerichtlich = document.getElementById("auslagen_aussergerichtlich");
    auslagen_aussergerichtlich.addEventListener("change", (event) => {
        mpkr.set_auslagen_aussergerichtlich(event.target.value.replace(',', '.'));
        update();
    });

    const summe_aussergerichtlich = document.getElementById("summe_aussergerichtlich");
    summe_aussergerichtlich.addEventListener("change", (event) => {
        mpkr.set_summe_aussergerichtlich(parseFloat(event.target.value.replace(',', '.')));
        update();
    });

    const instanz_h1 = document.getElementById("instanz_h1");
    instanz_h1.addEventListener("change", (event) => {
        mpkr.set_instanz_h1(event.target.checked);
        update();
    });
    const instanz_h2 = document.getElementById("instanz_h2");
    instanz_h2.addEventListener("change", (event) => {
        mpkr.set_instanz_h2(event.target.checked);
        update();
    });
    const instanz_h3 = document.getElementById("instanz_h3");
    instanz_h3.addEventListener("change", (event) => {
        mpkr.set_instanz_h3(event.target.checked);
        update();
    });

    const h1_3100 = document.getElementById("h1_3100");
    h1_3100.addEventListener("change", (event) => {
        mpkr.set_h1_3100(event.target.checked);
        update();
    });
    const h1_3100_13 = document.getElementById("h1_3100_13");
    const h1_3100_49 = document.getElementById("h1_3100_49");
    const h1_3101 = document.getElementById("h1_3101");
    h1_3101.addEventListener("change", (event) => {
        mpkr.set_h1_3101(event.target.checked);
        update();
    });
    const h1_anrechnung = document.getElementById("h1_anrechnung");
    h1_anrechnung.addEventListener("change", (event) => {
        mpkr.set_h1_anrechnung(event.target.checked);
        update();
    });
    const h1_anrechnung13 = document.getElementById("h1_anrechnung13");
    const h1_anrechnung49 = document.getElementById("h1_anrechnung49");
    const h1_3104 = document.getElementById("h1_3104");
    h1_3104.addEventListener("change", (event) => {
        mpkr.set_h1_3104(event.target.checked);
        update();
    });
    const h1_3104_13 = document.getElementById("h1_3104_13");
    const h1_3104_49 = document.getElementById("h1_3104_49");
    const h1_7002 = document.getElementById("h1_7002");
    h1_7002.addEventListener("change", (event) => {
        mpkr.set_h1_7002(event.target.checked);
        update();
    });
    const h1_7002_13 = document.getElementById("h1_7002_13");
    const h1_7002_49 = document.getElementById("h1_7002_49");
    const h1_7000ua = document.getElementById("h1_7000ua");
    h1_7000ua.addEventListener("change", (event) => {
        mpkr.set_h1_7000ua(event.target.checked);
        update();
    });
    const h1_auslagen = document.getElementById("h1_auslagen");
    h1_auslagen.addEventListener("change", (event) => {
        mpkr.set_h1_auslagen(parseFloat(event.target.value.replace(',', '.')));
        update();
    });
    const h1_5110 = document.getElementById("h1_5110");
    h1_5110.addEventListener("change", (event) => {
        mpkr.set_h1_5110(event.target.checked);
        update();
    });
    const l_h1_5110 = document.getElementById("l_h1_5110");
    const h1_5111 = document.getElementById("h1_5111");
    h1_5111.addEventListener("change", (event) => {
        mpkr.set_h1_5111(event.target.checked);
        update();
    });

    const instanz_v1 = document.getElementById("instanz_v1");
    instanz_v1.addEventListener("change", (event) => {
        mpkr.set_instanz_v1(event.target.checked);
        update();
    });
    const instanz_v2 = document.getElementById("instanz_v2");
    instanz_v2.addEventListener("change", (event) => {
        mpkr.set_instanz_v2(event.target.checked);
        update();
    });

    const v1_3100 = document.getElementById("v1_3100");
    v1_3100.addEventListener("change", (event) => {
        mpkr.set_v1_3100(event.target.checked);
        update();
    });
    const v1_3100_13 = document.getElementById("v1_3100_13");
    const v1_3100_49 = document.getElementById("v1_3100_49");
    const v1_3101 = document.getElementById("v1_3101");
    v1_3101.addEventListener("change", (event) => {
        mpkr.set_v1_3101(event.target.checked);
        update();
    });
    const v1_3104 = document.getElementById("v1_3104");
    v1_3104.addEventListener("change", (event) => {
        mpkr.set_v1_3104(event.target.checked);
        update();
    });
    const v1_3104_13 = document.getElementById("v1_3104_13");
    const v1_3104_49 = document.getElementById("v1_3104_49");
    const v1_7002 = document.getElementById("v1_7002");
    v1_7002.addEventListener("change", (event) => {
        mpkr.set_v1_7002(event.target.checked);
        update();
    });
    const v1_7002_13 = document.getElementById("v1_7002_13");
    const v1_7002_49 = document.getElementById("v1_7002_49");
    const v1_7000ua = document.getElementById("v1_7000ua");
    h1_7000ua.addEventListener("change", (event) => {
        mpkr.set_v1_7000ua(event.target.checked);
        update();
    });
    const v1_auslagen = document.getElementById("v1_auslagen");
    v1_auslagen.addEventListener("change", (event) => {
        mpkr.set_v1_auslagen(parseFloat(event.target.value.replace(',', '.')));
        update();
    });
    const v1_5210 = document.getElementById("v1_5210");
    v1_5210.addEventListener("change", (event) => {
        mpkr.set_v1_5210(event.target.checked);
        update();
    });
    const l_v1_5210 = document.getElementById("l_v1_5210");
    const v1_5211 = document.getElementById("v1_5211");
    v1_5211.addEventListener("change", (event) => {
        mpkr.set_v1_5211(event.target.checked);
        update();
    });

    const bs_hauptsache = new bootstrap.Collapse("#hauptsache", { toggle: false });
    const bs_instanz_h1 = new bootstrap.Collapse("#div_instanz_h1", { toggle: false });
    const bs_instanz_h2 = new bootstrap.Collapse("#div_instanz_h2", { toggle: false });
    const bs_instanz_h3 = new bootstrap.Collapse("#div_instanz_h3", { toggle: false });
    const summe_rvg13_h = document.getElementById("summe_rvg13_h");
    const summe_rvg49_h = document.getElementById("summe_rvg49_h");
    const summe_gkg_h = document.getElementById("summe_gkg_h");

    const bs_vorlaeufig = new bootstrap.Collapse("#vorlaeufig", { toggle: false });
    const bs_instanz_v1 = new bootstrap.Collapse("#div_instanz_v1", { toggle: false });
    const bs_instanz_v2 = new bootstrap.Collapse("#div_instanz_v2", { toggle: false });
    const summe_rvg13_v = document.getElementById("summe_rvg13_v");
    const summe_rvg49_v = document.getElementById("summe_rvg49_v");
    const summe_gkg_v = document.getElementById("summe_gkg_v");

    const bs_row_summe_aussergerichtlich = new bootstrap.Collapse("#row_summe_aussergerichtlich", { toggle: false });
    const bs_row_summe_rvg_h = new bootstrap.Collapse("#row_summe_rvg_h", { toggle: false });
    const bs_row_summe_rvg_v = new bootstrap.Collapse("#row_summe_rvg_v", { toggle: false });
    const bs_row_summe_gkg_h = new bootstrap.Collapse("#row_summe_gkg_h", { toggle: false });
    const bs_row_summe_gkg_v = new bootstrap.Collapse("#row_summe_gkg_v", { toggle: false });

    const l_summe_aussergerichtlich = document.getElementById("l_summe_aussergerichtlich");
    const l_summe_rvg_h = document.getElementById("l_summe_rvg_h");
    const l_summe_rvg_v = document.getElementById("l_summe_rvg_v");
    const summe_netto = document.getElementById("summe_netto");

    const steuersatz = document.getElementById("steuersatz");
    steuersatz.addEventListener("change", (event) => {
        mpkr.set_steuersatz(parseInt(event.target.value));
        update();
    })

    const umsatzsteuer = document.getElementById("umsatzsteuer");
    const summe_brutto = document.getElementById("summe_brutto");
    const l_summe_gkg_h = document.getElementById("l_summe_gkg_h");
    const l_summe_gkg_v = document.getElementById("l_summe_gkg_v");
    const l_summe_gkg = document.getElementById("l_summe_gkg");
    const l_summe_total = document.getElementById("l_summe_total");

    const formatNumber = (value) => {
        return new Intl.NumberFormat('de-DE',
            {
                minimumFractionDigits: 2,
                maximumFractionDigits: 2
            }).format(value);
    }

    const update = () => {
        if (mpkr.verfahren() != 1) { // Hauptsacheverfahren
            streitwert.value = formatNumber(mpkr.streitwert());
            l_verfahren_1.innerHTML = "<label>Hauptsache</label>";
            l_geb13_1.innerHTML = "<label>" + formatNumber(mpkr.rvg13_geb_h()) + " EUR</label>";
            l_geb49_1.innerHTML = "<label>" + formatNumber(mpkr.rvg49_geb_h()) + " EUR</label>";
            l_gkg_1.innerHTML = "<label>" + formatNumber(mpkr.gkg_geb_h()) + " EUR</label>";
            bs_hauptsache.show();
            instanz_h1.checked = mpkr.instanz_h1();
            instanz_h2.checked = mpkr.instanz_h2();
            instanz_h3.checked = mpkr.instanz_h3();
            if (mpkr.instanz_h1()) {
                h1_3100.checked = mpkr.h1_3100();
                h1_3100_13.innerHTML = "<label>" + formatNumber(mpkr.h1_3100_13()) + " EUR</label>";
                h1_3100_49.innerHTML = "<label>" + formatNumber(mpkr.h1_3100_49()) + " EUR</label>";
                h1_3101.checked = mpkr.h1_3101();
                h1_anrechnung.checked = mpkr.h1_anrechnung();
                h1_anrechnung13.innerHTML = "<label> - " + formatNumber(mpkr.h1_anrechnung13()) + " EUR</label>";
                h1_anrechnung49.innerHTML = "<label> - " + formatNumber(mpkr.h1_anrechnung49()) + " EUR</label>";
                h1_3104.checked = mpkr.h1_3104();
                h1_3104_13.innerHTML = "<label>" + formatNumber(mpkr.h1_3104_13()) + " EUR</label>";
                h1_3104_49.innerHTML = "<label>" + formatNumber(mpkr.h1_3104_49()) + " EUR</label>";
                h1_7002.checked = mpkr.h1_7002();
                h1_7002_13.innerHTML = "<label>" + formatNumber(mpkr.h1_pauschale()) + " EUR</label>";
                h1_7002_49.innerHTML = "<label>" + formatNumber(mpkr.h1_pauschale()) + " EUR</label>";  
                h1_7000ua.checked = mpkr.h1_7000ua();
                h1_auslagen.value = formatNumber(mpkr.h1_auslagen());
                h1_5110.checked = mpkr.h1_5110();
                l_h1_5110.innerHTML = "<label>" + formatNumber(mpkr.summe_gkg_h1()) + " EUR</label>";
                h1_5111.checked = mpkr.h1_5111();
                bs_instanz_h1.show();
            } else {
                bs_instanz_h1.hide();
            }
            if (mpkr.instanz_h2()) {
                bs_instanz_h2.show();
            } else {
                bs_instanz_h2.hide();
            }
            if (mpkr.instanz_h3()) {
                bs_instanz_h3.show();
            } else {
                bs_instanz_h3.hide();
            }
            summe_rvg13_h.innerHTML = "<label>" + formatNumber(mpkr.summe_rvg13_h()) + " EUR</label>";
            summe_rvg49_h.innerHTML = "<label>" + formatNumber(mpkr.summe_rvg49_h()) + " EUR</label>";
            summe_gkg_h.innerHTML = "<label>" + formatNumber(mpkr.summe_gkg_h()) + " EUR</label>";                       
            l_summe_rvg_h.innerHTML = "<label>" + formatNumber(mpkr.summe_rvg13_h()) + " EUR</label>"
            bs_row_summe_rvg_h.show();
            l_summe_gkg_h.innerHTML = "<label>" + formatNumber(mpkr.summe_gkg_h()) + " EUR</label>"
            bs_row_summe_gkg_h.show();
        } else { // nur Verfahren zum vorläufigen Rechtsschutz
            bs_div_l_streitwert_v.hide();
            bs_div_streitwert_v.hide();
            l_verfahren_1.innerHTML = "<label>vorläufiger Rechtsschutz</label>"
            streitwert.value = formatNumber(mpkr.streitwert_v());
            l_geb13_1.innerHTML = "<label>" + formatNumber(mpkr.rvg13_geb_v()) + " EUR</label>";
            l_geb49_1.innerHTML = "<label>" + formatNumber(mpkr.rvg49_geb_v()) + " EUR</label>";
            l_gkg_1.innerHTML = "<label>" + formatNumber(mpkr.gkg_geb_v()) + " EUR</label>";
            bs_hauptsache.hide();
            aussergerichtlich.checked = false;
            bs_row_summe_rvg_h.hide();
            bs_row_summe_gkg_h.hide();
        }

        if (mpkr.verfahren() != 0) { // Verfahren zum vorläufigen Rechtsschutz
            if (mpkr.verfahren() == 2) { // beide Verfahren
                bs_div_l_streitwert_v.show();
                bs_div_streitwert_v.show();
                streitwert_v.value = formatNumber(mpkr.streitwert_v());
                l_geb13_2.innerHTML = "<label>" + formatNumber(mpkr.rvg13_geb_v()) + " EUR</label>";
                l_geb49_2.innerHTML = "<label>" + formatNumber(mpkr.rvg49_geb_v()) + " EUR</label>";
                l_gkg_2.innerHTML = "<label>" + formatNumber(mpkr.gkg_geb_v()) + " EUR</label>";
            }
            bs_vorlaeufig.show();
            instanz_v1.checked = mpkr.instanz_v1();
            instanz_v2.checked = mpkr.instanz_v2();
            if (mpkr.instanz_v1()) {
                v1_3100.checked = mpkr.v1_3100();
                v1_3100_13.innerHTML = "<label>" + formatNumber(mpkr.v1_3100_13()) + " EUR</label>";
                v1_3100_49.innerHTML = "<label>" + formatNumber(mpkr.v1_3100_49()) + " EUR</label>";
                v1_3101.checked = mpkr.v1_3101();
                v1_3104.checked = mpkr.v1_3104();
                v1_3104_13.innerHTML = "<label>" + formatNumber(mpkr.v1_3104_13()) + " EUR</label>";
                v1_3104_49.innerHTML = "<label>" + formatNumber(mpkr.v1_3104_49()) + " EUR</label>";
                v1_7002.checked = mpkr.v1_7002();
                v1_7002_13.innerHTML = "<label>" + formatNumber(mpkr.v1_pauschale()) + " EUR</label>";
                v1_7002_49.innerHTML = "<label>" + formatNumber(mpkr.v1_pauschale()) + " EUR</label>";  
                v1_7000ua.checked = mpkr.v1_7000ua();
                v1_auslagen.value = formatNumber(mpkr.v1_auslagen());
                v1_5210.checked = mpkr.v1_5210();
                l_v1_5210.innerHTML = "<label>" + formatNumber(mpkr.summe_gkg_v1()) + " EUR</label>";
                v1_5211.checked = mpkr.v1_5211();
                bs_instanz_v1.show();
            } else {
                bs_instanz_v1.hide();
            }
            if (mpkr.instanz_v2()) {
                bs_instanz_v2.show();
            } else {
                bs_instanz_v2.hide();
            }
            summe_rvg13_v.innerHTML = "<label>" + formatNumber(mpkr.summe_rvg13_v()) + " EUR</label>";
            summe_rvg49_v.innerHTML = "<label>" + formatNumber(mpkr.summe_rvg49_v()) + " EUR</label>";
            summe_gkg_v.innerHTML = "<label>" + formatNumber(mpkr.summe_gkg_v()) + " EUR</label>";            
            l_summe_rvg_v.innerHTML = "<label>" + formatNumber(mpkr.summe_rvg13_v()) + " EUR</label>";
            bs_row_summe_rvg_v.show();
            l_summe_gkg_v.innerHTML = "<label>" + formatNumber(mpkr.summe_gkg_v()) + " EUR</label>"
            bs_row_summe_gkg_v.show();
        } else { // nur Hauptsacheverfahren
            bs_div_l_streitwert_v.hide();
            bs_div_streitwert_v.hide();
            bs_vorlaeufig.hide();
            bs_row_summe_rvg_v.hide();
            bs_row_summe_gkg_v.hide();
        }

        if (mpkr.aussergerichtlich()) {
            bs_div_aussergerichtlich.show();
            gebuehrensatz.value = mpkr.gebuehrensatz();
            geschaeftsgebuehr.value = formatNumber(mpkr.geschaeftsgebuehr());
            pauschale_aussergerichtlich.value = formatNumber(mpkr.pauschale_aussergerichtlich());
            auslagen_aussergerichtlich.value = formatNumber(mpkr.auslagen_aussergerichtlich());
            summe_aussergerichtlich.value = formatNumber(mpkr.summe_aussergerichtlich());
            l_summe_aussergerichtlich.innerHTML = "<label>" + formatNumber(mpkr.summe_aussergerichtlich()) + " EUR</label>";
            bs_row_summe_aussergerichtlich.show();
        } else {
            bs_div_aussergerichtlich.hide();
            bs_row_summe_aussergerichtlich.hide();
        }

        summe_netto.innerHTML = "<label>" + formatNumber(mpkr.summe_netto()) + " EUR</label>";
        umsatzsteuer.innerHTML = "<label>" + formatNumber(mpkr.umsatzsteuer()) + " EUR</label>";
        summe_brutto.innerHTML = "<label>" + formatNumber(mpkr.summe_brutto()) + " EUR</label>";
        l_summe_gkg.innerHTML = "<label>" + formatNumber(mpkr.summe_gkg()) + " EUR</label>";
        l_summe_total.innerHTML = "<h3>" + formatNumber(mpkr.summe_total()) + " EUR</h3>";
    }

    update();
}

main();