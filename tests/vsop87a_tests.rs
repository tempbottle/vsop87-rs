extern crate vsop87;
use vsop87::*;

#[test]
fn it_mercury() {
    let (x, y, z) = vsop87a::mercury(2451545.0);

    assert!(x > -0.1300934116 && x < -0.1300934114);
    assert!(y > -0.4472876717 && y < -0.4472876715);
    assert!(z > -0.0246009 && z < -0.0245959);

    let (x, y, z) = vsop87a::mercury(2415020.0);

    assert!(x > -0.3897246932 && x < -0.3897246930);
    assert!(y > -0.1502242200 && y < -0.1502242198);
    assert!(z > 0.0236174 && z < 0.0236224);

    let (x, y, z) = vsop87a::mercury(2378495.0);

    assert!(x > -0.1683565238 && x < -0.1683565236);
    assert!(y > 0.2735108156 && y < 0.2735108158);
    assert!(z > 0.0378079 && z < 0.0378129);

    let (x, y, z) = vsop87a::mercury(2341970.0);

    assert!(x > 0.3256720359 && x < 0.3256720361);
    assert!(y > 0.0880865801 && y < 0.0880865803);
    assert!(z > -0.0229845 && z < -0.0229795);

    let (x, y, z) = vsop87a::mercury(2305445.0);

    assert!(x > 0.2314047966 && x < 0.2314047968);
    assert!(y > -0.3620120987 && y < -0.3620120985);
    assert!(z > -0.0508613 && z < -0.0508563);

    let (x, y, z) = vsop87a::mercury(2268920.0);

    assert!(x > -0.1495554399 && x < -0.1495554397);
    assert!(y > -0.4409710105 && y < -0.4409710103);
    assert!(z > -0.0218152 && z < -0.0218102);

    let (x, y, z) = vsop87a::mercury(2232395.0);

    assert!(x > -0.3938651888 && x < -0.3938651886);
    assert!(y > -0.1288399756 && y < -0.1288399754);
    assert!(z > 0.0263319 && z < 0.0263369);

    let (x, y, z) = vsop87a::mercury(2195870.0);

    assert!(x > -0.1454241101 && x < -0.1454241099);
    assert!(y > 0.2837569446 && y < 0.2837569448);
    assert!(z > 0.0365234 && z < 0.0365284);

    let (x, y, z) = vsop87a::mercury(2159345.0);

    assert!(x > 0.3340760580 && x < 0.3340760582);
    assert!(y > 0.0655125083 && y < 0.0655125085);
    assert!(z > -0.0260687 && z < -0.0260637);

    let (x, y, z) = vsop87a::mercury(2122820.0);

    assert!(x > 0.2146329138 && x < 0.2146329140);
    assert!(y > -0.3752296251 && y < -0.3752296249);
    assert!(z > -0.0504008 && z < -0.0503958);
}

#[test]
fn it_venus() {
    let (x, y, z) = vsop87a::venus(2451545.0);

    assert!(x > -0.7183022798 && x < -0.7183022796);
    assert!(y > -0.0326546018 && y < -0.0326546016);
    assert!(z > 0.0410118 && z < 0.0410168);

    let (x, y, z) = vsop87a::venus(2415020.0);

    assert!(x > 0.6971428330 && x < 0.6971428332);
    assert!(y > -0.2033631152 && y < -0.2033631150);
    assert!(z > -0.0430226 && z < -0.0430176);

    let (x, y, z) = vsop87a::venus(2378495.0);

    assert!(x > -0.5983535209 && x < -0.5983535207);
    assert!(y > 0.3958502155 && y < 0.3958502157);
    assert!(z > 0.0398213 && z < 0.0398263);

    let (x, y, z) = vsop87a::venus(2341970.0);

    assert!(x > 0.4531193264 && x < 0.4531193266);
    assert!(y > -0.5692420970 && y < -0.5692420968);
    assert!(z > -0.0335668 && z < -0.0335618);

    let (x, y, z) = vsop87a::venus(2305445.0);

    assert!(x > -0.2501974250 && x < -0.2501974248);
    assert!(y > 0.6732855398 && y < 0.6732855400);
    assert!(z > 0.0229690 && z < 0.0229740);

    let (x, y, z) = vsop87a::venus(2268920.0);

    assert!(x > 0.0428334457 && x < 0.0428334459);
    assert!(y > -0.7259844931 && y < -0.7259844929);
    assert!(z > -0.0114050 && z < -0.0114000);

    let (x, y, z) = vsop87a::venus(2232395.0);

    assert!(x > 0.1935421815 && x < 0.1935421817);
    assert!(y > 0.6940567995 && y < 0.6940567997);
    assert!(z > -0.0029328 && z < -0.0029278);

    let (x, y, z) = vsop87a::venus(2195870.0);

    assert!(x > -0.3830059586 && x < -0.3830059584);
    assert!(y > -0.6150875571 && y < -0.6150875569);
    assert!(z > 0.0150895 && z < 0.0150945);

    let (x, y, z) = vsop87a::venus(2159345.0);

    assert!(x > 0.5643550616 && x < 0.5643550618);
    assert!(y > 0.4519394441 && y < 0.4519394443);
    assert!(z > -0.0277269 && z < -0.0277219);

    let (x, y, z) = vsop87a::venus(2122820.0);

    assert!(x > -0.6660158466 && x < -0.6660158464);
    assert!(y > -0.2753592312 && y < -0.2753592310);
    assert!(z > 0.0357849 && z < 0.0357899);
}

#[test]
fn it_earth() {
    let (x, y, z) = vsop87a::earth(2451545.0);

    assert!(x > -0.1771354587 && x < -0.1771354585);
    assert!(y > 0.9672416236 && y < 0.9672416238);
    assert!(z > -0.0000064 && z < -0.0000014);

    let (x, y, z) = vsop87a::earth(2415020.0);

    assert!(x > -0.1883079650 && x < -0.1883079648);
    assert!(y > 0.9650688843 && y < 0.9650688845);
    assert!(z > 0.0002125 && z < 0.0002175);

    let (x, y, z) = vsop87a::earth(2378495.0);

    assert!(x > -0.1993918003 && x < -0.1993918001);
    assert!(y > 0.9627974367 && y < 0.9627974369);
    assert!(z > 0.0004283 && z < 0.0004333);

    let (x, y, z) = vsop87a::earth(2341970.0);

    assert!(x > -0.2104654653 && x < -0.2104654651);
    assert!(y > 0.9603579953 && y < 0.9603579955);
    assert!(z > 0.0006448 && z < 0.0006498);

    let (x, y, z) = vsop87a::earth(2305445.0);

    assert!(x > -0.2214982929 && x < -0.2214982927);
    assert!(y > 0.9578483180 && y < 0.9578483182);
    assert!(z > 0.0008543 && z < 0.0008593);

    let (x, y, z) = vsop87a::earth(2268920.0);

    assert!(x > -0.2324780154 && x < -0.2324780152);
    assert!(y > 0.9551975792 && y < 0.9551975794);
    assert!(z > 0.0010668 && z < 0.0010718);

    let (x, y, z) = vsop87a::earth(2232395.0);

    assert!(x > -0.2435134344 && x < -0.2435134342);
    assert!(y > 0.9524373310 && y < 0.9524373312);
    assert!(z > 0.0012846 && z < 0.0012896);

    let (x, y, z) = vsop87a::earth(2195870.0);

    assert!(x > -0.2544603372 && x < -0.2544603370);
    assert!(y > 0.9495904256 && y < 0.9495904258);
    assert!(z > 0.0014937 && z < 0.0014987);

    let (x, y, z) = vsop87a::earth(2159345.0);

    assert!(x > -0.2654547157 && x < -0.2654547155);
    assert!(y > 0.9465233601 && y < 0.9465233603);
    assert!(z > 0.0017013 && z < 0.0017063);

    let (x, y, z) = vsop87a::earth(2122820.0);

    assert!(x > -0.2763146785 && x < -0.2763146783);
    assert!(y > 0.9433985306 && y < 0.9433985308);
    assert!(z > 0.0019090 && z < 0.0019140);
}

#[test]
fn it_mars() {
    let (x, y, z) = vsop87a::mars(2451545.0);

    assert!(x > 1.3907159263 && x < 1.3907159265);
    assert!(y > -0.0134157044 && y < -0.0134157042);
    assert!(z > -0.0344703 && z < -0.0344653);

    let (x, y, z) = vsop87a::mars(2415020.0);

    assert!(x > 0.4284332473 && x < 0.4284332475);
    assert!(y > -1.3552354251 && y < -1.3552354249);
    assert!(z > -0.0389675 && z < -0.0389625);

    let (x, y, z) = vsop87a::mars(2378495.0);

    assert!(x > -1.1119219622 && x < -1.1119219620);
    assert!(y > -1.0963263015 && y < -1.0963263013);
    assert!(z > 0.0049184 && z < 0.0049234);

    let (x, y, z) = vsop87a::mars(2341970.0);

    assert!(x > -1.6387489525 && x < -1.6387489523);
    assert!(y > 0.2507105241 && y < 0.2507105243);
    assert!(z > 0.0465581 && z < 0.0465631);

    let (x, y, z) = vsop87a::mars(2305445.0);

    assert!(x > -0.8307668242 && x < -0.8307668240);
    assert!(y > 1.4098595096 && y < 1.4098595098);
    assert!(z > 0.0504511 && z < 0.0504561);

    let (x, y, z) = vsop87a::mars(2268920.0);

    assert!(x > 0.6495258849 && x < 0.6495258851);
    assert!(y > 1.3657302244 && y < 1.3657302246);
    assert!(z > 0.0116897 && z < 0.0116947);

    let (x, y, z) = vsop87a::mars(2232395.0);

    assert!(x > 1.3910394545 && x < 1.3910394547);
    assert!(y > -0.0543839268 && y < -0.0543839266);
    assert!(z > -0.0371038 && z < -0.0370988);

    let (x, y, z) = vsop87a::mars(2195870.0);

    assert!(x > 0.3890073908 && x < 0.3890073910);
    assert!(y > -1.3660431024 && y < -1.3660431022);
    assert!(z > -0.0383834 && z < -0.0383784);

    let (x, y, z) = vsop87a::mars(2159345.0);

    assert!(x > -1.1440917097 && x < -1.1440917095);
    assert!(y > -1.0595533317 && y < -1.0595533315);
    assert!(z > 0.0082156 && z < 0.0082206);

    let (x, y, z) = vsop87a::mars(2122820.0);

    assert!(x > -1.6278485158 && x < -1.6278485156);
    assert!(y > 0.3060194813 && y < 0.3060194815);
    assert!(z > 0.0494191 && z < 0.0494241);
}

#[test]
fn it_jupiter() {
    let (x, y, z) = vsop87a::jupiter(2451545.0);

    assert!(x > 4.0011740267 && x < 4.0011740269);
    assert!(y > 2.9385810076 && y < 2.9385810078);
    assert!(z > -0.1017863 && z < -0.1017813);

    let (x, y, z) = vsop87a::jupiter(2415020.0);

    assert!(x > -3.0191224351 && x < -3.0191224349);
    assert!(y > -4.4582563706 && y < -4.4582563704);
    assert!(z > 0.0858617 && z < 0.0858667);

    let (x, y, z) = vsop87a::jupiter(2378495.0);

    assert!(x > -0.0180390005 && x < -0.0180390003);
    assert!(y > 5.1317748838 && y < 5.1317748840);
    assert!(z > -0.0200473 && z < -0.0200423);

    let (x, y, z) = vsop87a::jupiter(2341970.0);

    assert!(x > 1.2817318352 && x < 1.2817318354);
    assert!(y > -5.0280079875 && y < -5.0280079873);
    assert!(z > -0.0091277 && z < -0.0091227);

    let (x, y, z) = vsop87a::jupiter(2305445.0);

    assert!(x > -4.0547959776 && x < -4.0547959774);
    assert!(y > 3.4799857071 && y < 3.4799857073);
    assert!(z > 0.0779935 && z < 0.0779985);

    let (x, y, z) = vsop87a::jupiter(2268920.0);

    assert!(x > 4.5891471726 && x < 4.5891471728);
    assert!(y > -1.9870837932 && y < -1.9870837930);
    assert!(z > -0.0961117 && z < -0.0961067);

    let (x, y, z) = vsop87a::jupiter(2232395.0);

    assert!(x > -5.4239396006 && x < -5.4239396004);
    assert!(y > -0.5085487292 && y < -0.5085487290);
    assert!(z > 0.1247735 && z < 0.1247785);

    let (x, y, z) = vsop87a::jupiter(2195870.0);

    assert!(x > 4.2423286339 && x < 4.2423286341);
    assert!(y > 2.5898433578 && y < 2.5898433580);
    assert!(z > -0.1060332 && z < -0.1060282);

    let (x, y, z) = vsop87a::jupiter(2159345.0);

    assert!(x > -3.3554806096 && x < -3.3554806094);
    assert!(y > -4.2166702225 && y < -4.2166702223);
    assert!(z > 0.0919393 && z < 0.0919443);

    let (x, y, z) = vsop87a::jupiter(2122820.0);

    assert!(x > 0.4207861893 && x < 0.4207861895);
    assert!(y > 5.1019591309 && y < 5.1019591311);
    assert!(z > -0.0280113 && z < -0.0280063);
}

#[test]
fn it_saturn() {
    let (x, y, z) = vsop87a::saturn(2451545.0);

    assert!(x > 6.4064068572 && x < 6.4064068574);
    assert!(y > 6.5699929448 && y < 6.5699929450);
    assert!(z > -0.3690793 && z < -0.3690743);

    let (x, y, z) = vsop87a::saturn(2415020.0);

    assert!(x > -0.3695973751 && x < -0.3695973749);
    assert!(y > -10.0582398189 && y < -10.0582398187);
    assert!(z > 0.1916829 && z < 0.1916879);

    let (x, y, z) = vsop87a::saturn(2378495.0);

    assert!(x > -5.6790910871 && x < -5.6790910869);
    assert!(y > 7.1152478119 && y < 7.1152478121);
    assert!(z > 0.0978496 && z < 0.0978546);

    let (x, y, z) = vsop87a::saturn(2341970.0);

    assert!(x > 8.9934758991 && x < 8.9934758993);
    assert!(y > -3.7883225438 && y < -3.7883225436);
    assert!(z > -0.2866414 && z < -0.2866364);

    let (x, y, z) = vsop87a::saturn(2305445.0);

    assert!(x > -8.6570276347 && x < -8.6570276345);
    assert!(y > -4.4809792499 && y < -4.4809792497);
    assert!(z > 0.4216227 && z < 0.4216277);

    let (x, y, z) = vsop87a::saturn(2268920.0);

    assert!(x > 5.0378574918 && x < 5.0378574920);
    assert!(y > 7.5310625789 && y < 7.5310625791);
    assert!(z > -0.3348906 && z < -0.3348856);

    let (x, y, z) = vsop87a::saturn(2232395.0);

    assert!(x > 1.2601620698 && x < 1.2601620700);
    assert!(y > -10.0267935694 && y < -10.0267935692);
    assert!(z > 0.1347028 && z < 0.1347078);

    let (x, y, z) = vsop87a::saturn(2195870.0);

    assert!(x > -7.1628125748 && x < -7.1628125746);
    assert!(y > 5.7482646990 && y < 5.7482646992);
    assert!(z > 0.1724937 && z < 0.1724987);

    let (x, y, z) = vsop87a::saturn(2159345.0);

    assert!(x > 9.3511669241 && x < 9.3511669243);
    assert!(y > -2.1145906250 && y < -2.1145906248);
    assert!(z > -0.3231267 && z < -0.3231217);

    let (x, y, z) = vsop87a::saturn(2122820.0);

    assert!(x > -7.9395559174 && x < -7.9395559172);
    assert!(y > -5.8435867017 && y < -5.8435867015);
    assert!(z > 0.4165577 && z < 0.4165627);
}

#[test]
fn it_uranus() {
    let (x, y, z) = vsop87a::uranus(2451545.0);

    assert!(x > 14.4318934158 && x < 14.4318934160);
    assert!(y > -13.7343162528 && y < -13.7343162526);
    assert!(z > -0.2381447 && z < -0.2381397);

    let (x, y, z) = vsop87a::uranus(2415020.0);

    assert!(x > -6.4810833338 && x < -6.4810833336);
    assert!(y > -17.8526893407 && y < -17.8526893405);
    assert!(z > 0.0177910 && z < 0.0177960);

    let (x, y, z) = vsop87a::uranus(2378495.0);

    assert!(x > -18.2708335179 && x < -18.2708335177);
    assert!(y > 0.9877655714 && y < 0.9877655716);
    assert!(z > 0.2420319 && z < 0.2420369);

    let (x, y, z) = vsop87a::uranus(2341970.0);

    assert!(x > -4.2214391937 && x < -4.2214391935);
    assert!(y > 18.3160266383 && y < 18.3160266385);
    assert!(z > 0.1247568 && z < 0.1247618);

    let (x, y, z) = vsop87a::uranus(2305445.0);

    assert!(x > 16.1020987625 && x < 16.1020987627);
    assert!(y > 11.4900726863 && y < 11.4900726865);
    assert!(z > -0.1664644 && z < -0.1664594);

    let (x, y, z) = vsop87a::uranus(2268920.0);

    assert!(x > 17.7683247786 && x < 17.7683247788);
    assert!(y > -9.2421595877 && y < -9.2421595875);
    assert!(z > -0.2680507 && z < -0.2680457);

    let (x, y, z) = vsop87a::uranus(2232395.0);

    assert!(x > -0.7868164613 && x < -0.7868164611);
    assert!(y > -19.2532559479 && y < -19.2532559477);
    assert!(z > -0.0636449 && z < -0.0636399);

    let (x, y, z) = vsop87a::uranus(2195870.0);

    assert!(x > -17.6539243376 && x < -17.6539243374);
    assert!(y > -5.1636568777 && y < -5.1636568775);
    assert!(z > 0.2124644 && z < 0.2124694);

    let (x, y, z) = vsop87a::uranus(2159345.0);

    assert!(x > -9.8287104598 && x < -9.8287104596);
    assert!(y > 15.7711888604 && y < 15.7711888606);
    assert!(z > 0.1914792 && z < 0.1914842);

    let (x, y, z) = vsop87a::uranus(2122820.0);

    assert!(x > 11.8546461038 && x < 11.8546461040);
    assert!(y > 15.5595370552 && y < 15.5595370554);
    assert!(z > -0.0950214 && z < -0.0950164);
}

#[test]
fn it_neptune() {
    let (x, y, z) = vsop87a::neptune(2451545.0);

    assert!(x > 16.8121116575 && x < 16.8121116577);
    assert!(y > -24.9916630909 && y < -24.9916630907);
    assert!(z > 0.1272165 && z < 0.1272215);

    let (x, y, z) = vsop87a::neptune(2415020.0);

    assert!(x > 1.5164557466 && x < 1.5164557468);
    assert!(y > 29.8254538900 && y < 29.8254538902);
    assert!(z > -0.6491425 && z < -0.6491375);

    let (x, y, z) = vsop87a::neptune(2378495.0);

    assert!(x > -20.3138943579 && x < -20.3138943577);
    assert!(y > -22.4908255797 && y < -22.4908255795);
    assert!(z > 0.9309127 && z < 0.9309177);

    let (x, y, z) = vsop87a::neptune(2341970.0);

    assert!(x > 29.5022811949 && x < 29.5022811951);
    assert!(y > 4.5987701113 && y < 4.5987701115);
    assert!(z > -0.7740532 && z < -0.7740482);

    let (x, y, z) = vsop87a::neptune(2305445.0);

    assert!(x > -26.5823264273 && x < -26.5823264271);
    assert!(y > 14.1935610228 && y < 14.1935610230);
    assert!(z > 0.3196817 && z < 0.3196867);

    let (x, y, z) = vsop87a::neptune(2268920.0);

    assert!(x > 11.1160686192 && x < 11.1160686194);
    assert!(y > -28.0548309590 && y < -28.0548309588);
    assert!(z > 0.3216707 && z < 0.3216757);

    let (x, y, z) = vsop87a::neptune(2232395.0);

    assert!(x > 8.0214324005 && x < 8.0214324007);
    assert!(y > 28.7234916079 && y < 28.7234916081);
    assert!(z > -0.7759084 && z < -0.7759034);

    let (x, y, z) = vsop87a::neptune(2195870.0);

    assert!(x > -24.6234347579 && x < -24.6234347577);
    assert!(y > -17.6514428047 && y < -17.6514428045);
    assert!(z > 0.9297219 && z < 0.9297269);

    let (x, y, z) = vsop87a::neptune(2159345.0);

    assert!(x > 29.8303563035 && x < 29.8303563037);
    assert!(y > -2.0338910504 && y < -2.0338910502);
    assert!(z > -0.6441272 && z < -0.6441222);

    let (x, y, z) = vsop87a::neptune(2122820.0);

    assert!(x > -22.7985170871 && x < -22.7985170869);
    assert!(y > 19.5994768857 && y < 19.5994768859);
    assert!(z > 0.1206349 && z < 0.1206399);
}

#[test]
fn it_earth_moon() {
    let (x, y, z) = vsop87a::earth_moon(2451545.0);

    assert!(x > -0.1771591441 && x < -0.1771591439);
    assert!(y > 0.9672192890 && y < 0.9672192892);
    assert!(z > -0.0000035 && z < 0.0000015);

    let (x, y, z) = vsop87a::earth_moon(2415020.0);

    assert!(x > -0.1883097014 && x < -0.1883097012);
    assert!(y > 0.9650388427 && y < 0.9650388429);
    assert!(z > 0.0002128 && z < 0.0002178);

    let (x, y, z) = vsop87a::earth_moon(2378495.0);

    assert!(x > -0.1993643285 && x < -0.1993643283);
    assert!(y > 0.9627828194 && y < 0.9627828196);
    assert!(z > 0.0004258 && z < 0.0004308);

    let (x, y, z) = vsop87a::earth_moon(2341970.0);

    assert!(x > -0.2104343222 && x < -0.2104343220);
    assert!(y > 0.9603642781 && y < 0.9603642783);
    assert!(z > 0.0006438 && z < 0.0006488);

    let (x, y, z) = vsop87a::earth_moon(2305445.0);

    assert!(x > -0.2214911210 && x < -0.2214911208);
    assert!(y > 0.9578778166 && y < 0.9578778168);
    assert!(z > 0.0008565 && z < 0.0008615);

    let (x, y, z) = vsop87a::earth_moon(2268920.0);

    assert!(x > -0.2324953838 && x < -0.2324953836);
    assert!(y > 0.9552252050 && y < 0.9552252052);
    assert!(z > 0.0010687 && z < 0.0010737);

    let (x, y, z) = vsop87a::earth_moon(2232395.0);

    assert!(x > -0.2435434219 && x < -0.2435434217);
    assert!(y > 0.9524355202 && y < 0.9524355204);
    assert!(z > 0.0012830 && z < 0.0012880);

    let (x, y, z) = vsop87a::earth_moon(2195870.0);

    assert!(x > -0.2544800657 && x < -0.2544800655);
    assert!(y > 0.9495642256 && y < 0.9495642258);
    assert!(z > 0.0014912 && z < 0.0014962);

    let (x, y, z) = vsop87a::earth_moon(2159345.0);

    assert!(x > -0.2654471687 && x < -0.2654471685);
    assert!(y > 0.9464953235 && y < 0.9464953237);
    assert!(z > 0.0017023 && z < 0.0017073);

    let (x, y, z) = vsop87a::earth_moon(2122820.0);

    assert!(x > -0.2762837552 && x < -0.2762837550);
    assert!(y > 0.9433889918 && y < 0.9433889920);
    assert!(z > 0.0019119 && z < 0.0019169);
}
