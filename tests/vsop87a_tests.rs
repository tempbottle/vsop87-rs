extern crate vsop87;
use vsop87::vsop87a::*;

#[test]
fn it_mercury() {
    let (x, y, z) = mercury(2451545.0);

    assert!(x > -0.1300934116 && x < -0.1300934114);
    assert!(y > -0.4472876717 && y < -0.4472876715);
    assert!(z > -0.024600 && z < -0.024596);

    let (x, y, z) = mercury(2415020.0);

    assert!(x > -0.3897246932 && x < -0.3897246930);
    assert!(y > -0.1502242200 && y < -0.1502242198);
    assert!(z > 0.023618 && z < 0.023622);

    let (x, y, z) = mercury(2378495.0);

    assert!(x > -0.1683565238 && x < -0.1683565236);
    assert!(y > 0.2735108156 && y < 0.2735108158);
    assert!(z > 0.037808 && z < 0.037812);

    let (x, y, z) = mercury(2341970.0);

    assert!(x > 0.3256720359 && x < 0.3256720361);
    assert!(y > 0.0880865801 && y < 0.0880865803);
    assert!(z > -0.022984 && z < -0.022980);

    let (x, y, z) = mercury(2305445.0);

    assert!(x > 0.2314047966 && x < 0.2314047968);
    assert!(y > -0.3620120987 && y < -0.3620120985);
    assert!(z > -0.050861 && z < -0.050857);

    let (x, y, z) = mercury(2268920.0);

    assert!(x > -0.1495554399 && x < -0.1495554397);
    assert!(y > -0.4409710105 && y < -0.4409710103);
    assert!(z > -0.021815 && z < -0.021811);

    let (x, y, z) = mercury(2232395.0);

    assert!(x > -0.3938651888 && x < -0.3938651886);
    assert!(y > -0.1288399756 && y < -0.1288399754);
    assert!(z > 0.026332 && z < 0.026336);

    let (x, y, z) = mercury(2195870.0);

    assert!(x > -0.1454241101 && x < -0.1454241099);
    assert!(y > 0.2837569446 && y < 0.2837569448);
    assert!(z > 0.036524 && z < 0.036528);

    let (x, y, z) = mercury(2159345.0);

    assert!(x > 0.3340760580 && x < 0.3340760582);
    assert!(y > 0.0655125083 && y < 0.0655125085);
    assert!(z > -0.026068 && z < -0.026064);

    let (x, y, z) = mercury(2122820.0);

    assert!(x > 0.2146329138 && x < 0.2146329140);
    assert!(y > -0.3752296251 && y < -0.3752296249);
    assert!(z > -0.050400 && z < -0.050396);
}

#[test]
fn it_venus() {
    let (x, y, z) = venus(2451545.0);

    assert!(x > -0.7183022798 && x < -0.7183022796);
    assert!(y > -0.0326546018 && y < -0.0326546016);
    assert!(z > 0.041012 && z < 0.041016);

    let (x, y, z) = venus(2415020.0);

    assert!(x > 0.6971428330 && x < 0.6971428332);
    assert!(y > -0.2033631152 && y < -0.2033631150);
    assert!(z > -0.043022 && z < -0.043018);

    let (x, y, z) = venus(2378495.0);

    assert!(x > -0.5983535209 && x < -0.5983535207);
    assert!(y > 0.3958502155 && y < 0.3958502157);
    assert!(z > 0.039822 && z < 0.039826);

    let (x, y, z) = venus(2341970.0);

    assert!(x > 0.4531193264 && x < 0.4531193266);
    assert!(y > -0.5692420970 && y < -0.5692420968);
    assert!(z > -0.033566 && z < -0.033562);

    let (x, y, z) = venus(2305445.0);

    assert!(x > -0.2501974250 && x < -0.2501974248);
    assert!(y > 0.6732855398 && y < 0.6732855400);
    assert!(z > 0.022969 && z < 0.022973);

    let (x, y, z) = venus(2268920.0);

    assert!(x > 0.0428334457 && x < 0.0428334459);
    assert!(y > -0.7259844931 && y < -0.7259844929);
    assert!(z > -0.011405 && z < -0.011401);

    let (x, y, z) = venus(2232395.0);

    assert!(x > 0.1935421815 && x < 0.1935421817);
    assert!(y > 0.6940567995 && y < 0.6940567997);
    assert!(z > -0.002932 && z < -0.002928);

    let (x, y, z) = venus(2195870.0);

    assert!(x > -0.3830059586 && x < -0.3830059584);
    assert!(y > -0.6150875571 && y < -0.6150875569);
    assert!(z > 0.015090 && z < 0.015094);

    let (x, y, z) = venus(2159345.0);

    assert!(x > 0.5643550616 && x < 0.5643550618);
    assert!(y > 0.4519394441 && y < 0.4519394443);
    assert!(z > -0.027726 && z < -0.027722);

    let (x, y, z) = venus(2122820.0);

    assert!(x > -0.6660158466 && x < -0.6660158464);
    assert!(y > -0.2753592312 && y < -0.2753592310);
    assert!(z > 0.035785 && z < 0.035789);
}

#[test]
fn it_earth() {
    let (x, y, z) = earth(2451545.0);

    assert!(x > -0.1771354587 && x < -0.1771354585);
    assert!(y > 0.9672416236 && y < 0.9672416238);
    assert!(z > -0.000006 && z < -0.000002);

    let (x, y, z) = earth(2415020.0);

    assert!(x > -0.1883079650 && x < -0.1883079648);
    assert!(y > 0.9650688843 && y < 0.9650688845);
    assert!(z > 0.000213 && z < 0.000217);

    let (x, y, z) = earth(2378495.0);

    assert!(x > -0.1993918003 && x < -0.1993918001);
    assert!(y > 0.9627974367 && y < 0.9627974369);
    assert!(z > 0.000429 && z < 0.000433);

    let (x, y, z) = earth(2341970.0);

    assert!(x > -0.2104654653 && x < -0.2104654651);
    assert!(y > 0.9603579953 && y < 0.9603579955);
    assert!(z > 0.000645 && z < 0.000649);

    let (x, y, z) = earth(2305445.0);

    assert!(x > -0.2214982929 && x < -0.2214982927);
    assert!(y > 0.9578483180 && y < 0.9578483182);
    assert!(z > 0.000855 && z < 0.000859);

    let (x, y, z) = earth(2268920.0);

    assert!(x > -0.2324780154 && x < -0.2324780152);
    assert!(y > 0.9551975792 && y < 0.9551975794);
    assert!(z > 0.001067 && z < 0.001071);

    let (x, y, z) = earth(2232395.0);

    assert!(x > -0.2435134344 && x < -0.2435134342);
    assert!(y > 0.9524373310 && y < 0.9524373312);
    assert!(z > 0.001285 && z < 0.001289);

    let (x, y, z) = earth(2195870.0);

    assert!(x > -0.2544603372 && x < -0.2544603370);
    assert!(y > 0.9495904256 && y < 0.9495904258);
    assert!(z > 0.001494 && z < 0.001498);

    let (x, y, z) = earth(2159345.0);

    assert!(x > -0.2654547157 && x < -0.2654547155);
    assert!(y > 0.9465233601 && y < 0.9465233603);
    assert!(z > 0.001702 && z < 0.001706);

    let (x, y, z) = earth(2122820.0);

    assert!(x > -0.2763146785 && x < -0.2763146783);
    assert!(y > 0.9433985306 && y < 0.9433985308);
    assert!(z > 0.001910 && z < 0.001914);
}

#[test]
fn it_mars() {
    let (x, y, z) = mars(2451545.0);

    assert!(x > 1.3907159263 && x < 1.3907159265);
    assert!(y > -0.0134157044 && y < -0.0134157042);
    assert!(z > -0.034470 && z < -0.034466);

    let (x, y, z) = mars(2415020.0);

    assert!(x > 0.4284332473 && x < 0.4284332475);
    assert!(y > -1.3552354251 && y < -1.3552354249);
    assert!(z > -0.038967 && z < -0.038963);

    let (x, y, z) = mars(2378495.0);

    assert!(x > -1.1119219622 && x < -1.1119219620);
    assert!(y > -1.0963263015 && y < -1.0963263013);
    assert!(z > 0.004919 && z < 0.004923);

    let (x, y, z) = mars(2341970.0);

    assert!(x > -1.6387489525 && x < -1.6387489523);
    assert!(y > 0.2507105241 && y < 0.2507105243);
    assert!(z > 0.046559 && z < 0.046563);

    let (x, y, z) = mars(2305445.0);

    assert!(x > -0.8307668242 && x < -0.8307668240);
    assert!(y > 1.4098595096 && y < 1.4098595098);
    assert!(z > 0.050452 && z < 0.050456);

    let (x, y, z) = mars(2268920.0);

    assert!(x > 0.6495258849 && x < 0.6495258851);
    assert!(y > 1.3657302244 && y < 1.3657302246);
    assert!(z > 0.011690 && z < 0.011694);

    let (x, y, z) = mars(2232395.0);

    assert!(x > 1.3910394545 && x < 1.3910394547);
    assert!(y > -0.0543839268 && y < -0.0543839266);
    assert!(z > -0.037103 && z < -0.037099);

    let (x, y, z) = mars(2195870.0);

    assert!(x > 0.3890073908 && x < 0.3890073910);
    assert!(y > -1.3660431024 && y < -1.3660431022);
    assert!(z > -0.038383 && z < -0.038379);

    let (x, y, z) = mars(2159345.0);

    assert!(x > -1.1440917097 && x < -1.1440917095);
    assert!(y > -1.0595533317 && y < -1.0595533315);
    assert!(z > 0.008216 && z < 0.008220);

    let (x, y, z) = mars(2122820.0);

    assert!(x > -1.6278485158 && x < -1.6278485156);
    assert!(y > 0.3060194813 && y < 0.3060194815);
    assert!(z > 0.049420 && z < 0.049424);
}

#[test]
fn it_jupiter() {
    let (x, y, z) = jupiter(2451545.0);

    assert!(x > 4.0011740267 && x < 4.0011740269);
    assert!(y > 2.9385810076 && y < 2.9385810078);
    assert!(z > -0.101786 && z < -0.101782);

    let (x, y, z) = jupiter(2415020.0);

    assert!(x > -3.0191224351 && x < -3.0191224349);
    assert!(y > -4.4582563706 && y < -4.4582563704);
    assert!(z > 0.085862 && z < 0.085866);

    let (x, y, z) = jupiter(2378495.0);

    assert!(x > -0.0180390005 && x < -0.0180390003);
    assert!(y > 5.1317748838 && y < 5.1317748840);
    assert!(z > -0.020047 && z < -0.020043);

    let (x, y, z) = jupiter(2341970.0);

    assert!(x > 1.2817318352 && x < 1.2817318354);
    assert!(y > -5.0280079875 && y < -5.0280079873);
    assert!(z > -0.009127 && z < -0.009123);

    let (x, y, z) = jupiter(2305445.0);

    assert!(x > -4.0547959776 && x < -4.0547959774);
    assert!(y > 3.4799857071 && y < 3.4799857073);
    assert!(z > 0.077994 && z < 0.077998);

    let (x, y, z) = jupiter(2268920.0);

    assert!(x > 4.5891471726 && x < 4.5891471728);
    assert!(y > -1.9870837932 && y < -1.9870837930);
    assert!(z > -0.096111 && z < -0.096107);

    let (x, y, z) = jupiter(2232395.0);

    assert!(x > -5.4239396006 && x < -5.4239396004);
    assert!(y > -0.5085487292 && y < -0.5085487290);
    assert!(z > 0.124774 && z < 0.124778);

    let (x, y, z) = jupiter(2195870.0);

    assert!(x > 4.2423286339 && x < 4.2423286341);
    assert!(y > 2.5898433578 && y < 2.5898433580);
    assert!(z > -0.106033 && z < -0.106029);

    let (x, y, z) = jupiter(2159345.0);

    assert!(x > -3.3554806096 && x < -3.3554806094);
    assert!(y > -4.2166702225 && y < -4.2166702223);
    assert!(z > 0.091940 && z < 0.091944);

    let (x, y, z) = jupiter(2122820.0);

    assert!(x > 0.4207861893 && x < 0.4207861895);
    assert!(y > 5.1019591309 && y < 5.1019591311);
    assert!(z > -0.028011 && z < -0.028007);
}

#[test]
fn it_saturn() {
    let (x, y, z) = saturn(2451545.0);

    assert!(x > 6.4064068572 && x < 6.4064068574);
    assert!(y > 6.5699929448 && y < 6.5699929450);
    assert!(z > -0.369079 && z < -0.369075);

    let (x, y, z) = saturn(2415020.0);

    assert!(x > -0.3695973751 && x < -0.3695973749);
    assert!(y > -10.0582398189 && y < -10.0582398187);
    assert!(z > 0.191683 && z < 0.191687);

    let (x, y, z) = saturn(2378495.0);

    assert!(x > -5.6790910871 && x < -5.6790910869);
    assert!(y > 7.1152478119 && y < 7.1152478121);
    assert!(z > 0.097850 && z < 0.097854);

    let (x, y, z) = saturn(2341970.0);

    assert!(x > 8.9934758991 && x < 8.9934758993);
    assert!(y > -3.7883225438 && y < -3.7883225436);
    assert!(z > -0.286641 && z < -0.286637);

    let (x, y, z) = saturn(2305445.0);

    assert!(x > -8.6570276347 && x < -8.6570276345);
    assert!(y > -4.4809792499 && y < -4.4809792497);
    assert!(z > 0.421623 && z < 0.421627);

    let (x, y, z) = saturn(2268920.0);

    assert!(x > 5.0378574918 && x < 5.0378574920);
    assert!(y > 7.5310625789 && y < 7.5310625791);
    assert!(z > -0.334890 && z < -0.334886);

    let (x, y, z) = saturn(2232395.0);

    assert!(x > 1.2601620698 && x < 1.2601620700);
    assert!(y > -10.0267935694 && y < -10.0267935692);
    assert!(z > 0.134703 && z < 0.134707);

    let (x, y, z) = saturn(2195870.0);

    assert!(x > -7.1628125748 && x < -7.1628125746);
    assert!(y > 5.7482646990 && y < 5.7482646992);
    assert!(z > 0.172494 && z < 0.172498);

    let (x, y, z) = saturn(2159345.0);

    assert!(x > 9.3511669241 && x < 9.3511669243);
    assert!(y > -2.1145906250 && y < -2.1145906248);
    assert!(z > -0.323126 && z < -0.323122);

    let (x, y, z) = saturn(2122820.0);

    assert!(x > -7.9395559174 && x < -7.9395559172);
    assert!(y > -5.8435867017 && y < -5.8435867015);
    assert!(z > 0.416558 && z < 0.416562);
}

#[test]
fn it_uranus() {
    let (x, y, z) = uranus(2451545.0);

    assert!(x > 14.4318934158 && x < 14.4318934160);
    assert!(y > -13.7343162528 && y < -13.7343162526);
    assert!(z > -0.238144 && z < -0.238140);

    let (x, y, z) = uranus(2415020.0);

    assert!(x > -6.4810833338 && x < -6.4810833336);
    assert!(y > -17.8526893407 && y < -17.8526893405);
    assert!(z > 0.017792 && z < 0.017796);

    let (x, y, z) = uranus(2378495.0);

    assert!(x > -18.2708335179 && x < -18.2708335177);
    assert!(y > 0.9877655714 && y < 0.9877655716);
    assert!(z > 0.242032 && z < 0.242036);

    let (x, y, z) = uranus(2341970.0);

    assert!(x > -4.2214391937 && x < -4.2214391935);
    assert!(y > 18.3160266383 && y < 18.3160266385);
    assert!(z > 0.124757 && z < 0.124761);

    let (x, y, z) = uranus(2305445.0);

    assert!(x > 16.1020987625 && x < 16.1020987627);
    assert!(y > 11.4900726863 && y < 11.4900726865);
    assert!(z > -0.166464 && z < -0.166460);

    let (x, y, z) = uranus(2268920.0);

    assert!(x > 17.7683247786 && x < 17.7683247788);
    assert!(y > -9.2421595877 && y < -9.2421595875);
    assert!(z > -0.268050 && z < -0.268046);

    let (x, y, z) = uranus(2232395.0);

    assert!(x > -0.7868164613 && x < -0.7868164611);
    assert!(y > -19.2532559479 && y < -19.2532559477);
    assert!(z > -0.063644 && z < -0.063640);

    let (x, y, z) = uranus(2195870.0);

    assert!(x > -17.6539243376 && x < -17.6539243374);
    assert!(y > -5.1636568777 && y < -5.1636568775);
    assert!(z > 0.212465 && z < 0.212469);

    let (x, y, z) = uranus(2159345.0);

    assert!(x > -9.8287104598 && x < -9.8287104596);
    assert!(y > 15.7711888604 && y < 15.7711888606);
    assert!(z > 0.191480 && z < 0.191484);

    let (x, y, z) = uranus(2122820.0);

    assert!(x > 11.8546461038 && x < 11.8546461040);
    assert!(y > 15.5595370552 && y < 15.5595370554);
    assert!(z > -0.095021 && z < -0.095017);
}

#[test]
fn it_neptune() {
    let (x, y, z) = neptune(2451545.0);

    assert!(x > 16.8121116575 && x < 16.8121116577);
    assert!(y > -24.9916630909 && y < -24.9916630907);
    assert!(z > 0.127217 && z < 0.127221);

    let (x, y, z) = neptune(2415020.0);

    assert!(x > 1.5164557466 && x < 1.5164557468);
    assert!(y > 29.8254538900 && y < 29.8254538902);
    assert!(z > -0.649142 && z < -0.649138);

    let (x, y, z) = neptune(2378495.0);

    assert!(x > -20.3138943579 && x < -20.3138943577);
    assert!(y > -22.4908255797 && y < -22.4908255795);
    assert!(z > 0.930913 && z < 0.930917);

    let (x, y, z) = neptune(2341970.0);

    assert!(x > 29.5022811949 && x < 29.5022811951);
    assert!(y > 4.5987701113 && y < 4.5987701115);
    assert!(z > -0.774053 && z < -0.774049);

    let (x, y, z) = neptune(2305445.0);

    assert!(x > -26.5823264273 && x < -26.5823264271);
    assert!(y > 14.1935610228 && y < 14.1935610230);
    assert!(z > 0.319682 && z < 0.319686);

    let (x, y, z) = neptune(2268920.0);

    assert!(x > 11.1160686192 && x < 11.1160686194);
    assert!(y > -28.0548309590 && y < -28.0548309588);
    assert!(z > 0.321671 && z < 0.321675);

    let (x, y, z) = neptune(2232395.0);

    assert!(x > 8.0214324005 && x < 8.0214324007);
    assert!(y > 28.7234916079 && y < 28.7234916081);
    assert!(z > -0.775908 && z < -0.775904);

    let (x, y, z) = neptune(2195870.0);

    assert!(x > -24.6234347579 && x < -24.6234347577);
    assert!(y > -17.6514428047 && y < -17.6514428045);
    assert!(z > 0.929722 && z < 0.929726);

    let (x, y, z) = neptune(2159345.0);

    assert!(x > 29.8303563035 && x < 29.8303563037);
    assert!(y > -2.0338910504 && y < -2.0338910502);
    assert!(z > -0.644127 && z < -0.644123);

    let (x, y, z) = neptune(2122820.0);

    assert!(x > -22.7985170871 && x < -22.7985170869);
    assert!(y > 19.5994768857 && y < 19.5994768859);
    assert!(z > 0.120635 && z < 0.120639);
}

#[test]
fn it_earth_moon() {
    let (x, y, z) = earth_moon(2451545.0);

    assert!(x > -0.1771591441 && x < -0.1771591439);
    assert!(y > 0.9672192890 && y < 0.9672192892);
    assert!(z > -0.000003 && z < 0.000001);

    let (x, y, z) = earth_moon(2415020.0);

    assert!(x > -0.1883097014 && x < -0.1883097012);
    assert!(y > 0.9650388427 && y < 0.9650388429);
    assert!(z > 0.000213 && z < 0.000217);

    let (x, y, z) = earth_moon(2378495.0);

    assert!(x > -0.1993643285 && x < -0.1993643283);
    assert!(y > 0.9627828194 && y < 0.9627828196);
    assert!(z > 0.000426 && z < 0.000430);

    let (x, y, z) = earth_moon(2341970.0);

    assert!(x > -0.2104343222 && x < -0.2104343220);
    assert!(y > 0.9603642781 && y < 0.9603642783);
    assert!(z > 0.000644 && z < 0.000648);

    let (x, y, z) = earth_moon(2305445.0);

    assert!(x > -0.2214911210 && x < -0.2214911208);
    assert!(y > 0.9578778166 && y < 0.9578778168);
    assert!(z > 0.000857 && z < 0.000861);

    let (x, y, z) = earth_moon(2268920.0);

    assert!(x > -0.2324953838 && x < -0.2324953836);
    assert!(y > 0.9552252050 && y < 0.9552252052);
    assert!(z > 0.001069 && z < 0.001073);

    let (x, y, z) = earth_moon(2232395.0);

    assert!(x > -0.2435434219 && x < -0.2435434217);
    assert!(y > 0.9524355202 && y < 0.9524355204);
    assert!(z > 0.001284 && z < 0.001288);

    let (x, y, z) = earth_moon(2195870.0);

    assert!(x > -0.2544800657 && x < -0.2544800655);
    assert!(y > 0.9495642256 && y < 0.9495642258);
    assert!(z > 0.001492 && z < 0.001496);

    let (x, y, z) = earth_moon(2159345.0);

    assert!(x > -0.2654471687 && x < -0.2654471685);
    assert!(y > 0.9464953235 && y < 0.9464953237);
    assert!(z > 0.001703 && z < 0.001707);

    let (x, y, z) = earth_moon(2122820.0);

    assert!(x > -0.2762837552 && x < -0.2762837550);
    assert!(y > 0.9433889918 && y < 0.9433889920);
    assert!(z > 0.001912 && z < 0.001916);
}