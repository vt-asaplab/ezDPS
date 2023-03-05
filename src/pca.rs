use libspartan::{Instance, VarsAssignment, InputsAssignment};
use curve25519_dalek::scalar::Scalar;

const X: [i32; 170] = [-1412346, -634611, -767001, -655524, -101701, 333596, 692882, 1072536, 967338,
    992055, 713650, 516467, 327893, 137013, 179676, 159761, 387956, 549669, 459844, 437418, 437380,
    431344, 270103, 150449, 163878, 141648, -52116, -199918, 155510, 376098, 475893, 608054, 427150,
    330132, 154674, 234, 111382, 151365, 165045, 185773, 91428, 27917, 417179, 685120, 900861, 1130590,
    712173, 467426, -22117, -446067, -769408, -1119706, -311337, 186567, 435169, 750572, 234765, -58318,
    -310642, -573887, -581122, -656955, -544517, -482525, -758216, -943424, -1605840, -2140388, -4182982,
    -5821497, -7190166, -8631139, -6729031, -5722698, -2645726, -123580, 1734706, 3770873, 5201850,
    6794987, 5806594, 5509919, 4222003, 3199689, 3062839, 2688730, 3271176, 3597313, 3625237, 3733068,
    3091341, 2650458, 2157971, 1679311, 1976620, 2066009, 2583103, 2985593, 2965998, 3059499, 2799710,
    2634585, 1781756, 1113197, 520760, -92073, 55260, -1091, 303022, 510549, 358686, 303121, 264841,
    221929, 193015, 160350, -197083, -467494, -235973, -138944, -8123, 113642, -204412, -404617, -164905,
    -43069, 158466, 338647, -395894, -885337, -1066909, -1330974, -1187574, -1153355, -758186, -459734,
    -180050, 104662, 105390, 182212, -310281, -650226, -855724, -1097247, -929259, -870999, -453240,
    -131809, 59079, 284947, 27715, -100069, -565547, -940541, -1389733, -1819044, -1983048, -2218140,
    -2025919, -1948197, -1440086, -1047297, -418956, 146267, -98191, -125696, -921919, -1512165,
    -2101065, -2690325];
const X_CENTER: [i32; 170] = [
    -2180361, -2539923, -2388771, -2374464, -2363979, -2352731, -2343944, -2334497, -2327529, -2319897,
    -2307903, -2297078, -2291079, -2283788, -2289880, -2292386, -2294521, -2296453, -2293255, -2291513,
    -2300193, -2306081, -2341290, -2368226, -2437556, -2495594, -2581574, -2660143, -2740484, -2820350,
    -2875280, -2936910, -2966729, -3005073, -3019248, -3039677, -3049144, -3061319, -3056954, -3057166,
    -3055921, -3055184, -3052715, -3051260, -3044571, -3039771, -3030041, -3022399, -3033600, -3040449,
    -3035552, -3036462, -2958692, -2903032, -2746065, -2617413, -2563719, -2490070, -2289291, -2123605,
    -1673919, -1300481, 98936, 1222682, 3948656, 6244688, 9303968, 12132974, 14508112, 16902074, 15815418,
    15645118, 12748491, 10581959, 7498214, 4660294, 3098074, 1193755, 592091, -351446, -702737, -1212662,
    -1813716, -2390511, -3035570, -3662321, -4158439, -4689831, -5058879, -5471426, -5764522, -6089497,
    -6365001, -6653760, -6896054, -7150799, -7315291, -7503966, -7639304, -7788933, -7945263, -8099798,
    -8214075, -8339202, -8389795, -8460359, -8496253, -8541478, -8481567, -8449760, -8234016, -8067256,
    -7771726, -7510699, -7236605, -6966012, -6762776, -6541479, -6410819, -6256078, -6159038, -6046538,
    -5965919, -5876758, -5808745, -5735066, -5655340, -5577235, -5467673, -5366496, -5246581, -5131687,
    -5007122, -4885148, -4754634, -4626432, -4495763, -4365397, -4236104, -4106463, -3978116, -3849441,
    -3729899, -3607909, -3503463, -3394404, -3296026, -3194785, -3098112, -3000215, -2904873, -2809230,
    -2711219, -2613843, -2509030, -2406209, -2296040, -2187792, -2071195, -1956775, -1838890, -1721934,
    -1600531, -1480334, -1358813, -1237647, -1119384, -1000409, -929775, -846188];

const PARA: [[i32; 170]; 9] = [
    [373681, 552435, 493765, 498712, 500702, 503507, 503628, 504468, 503525, 503059, 501867, 500870, 501015, 500854, 498899, 497425, 495548, 493784, 493119, 492167, 489957, 488085, 481779, 476712, 468256, 460728, 451406, 442542, 433416, 424361, 419233, 413054, 407735, 402185, 398634, 394622, 393339, 391303, 389701, 387934, 384617, 381667, 379175, 376422, 374652, 372521, 375128, 376340, 377748, 378970, 371207, 365391, 340826, 321189, 270446, 227883, 158236, 95791, -6829, -98829, -232971, -355837, -711254, -1004333, -1857388, -2560452, -2676799, -2928988, -2279520, -1875994, -1862333, -1747809, -1896758, -1975158, -1987354, -2017274, -1903015, -1827471, -1587136, -1391107, -1132571, -890781, -646645, -403147, -168510, 68498, 312441, 554549, 796508, 1038508, 1259020, 1485312, 1674506, 1873641, 2002107, 2149508, 2209270, 2292514, 2330401, 2380441, 2459519, 2530816, 2653134, 2761773, 2872060, 2981905, 3044108, 3119074, 3079263, 3070215, 2909845, 2790051, 2580163, 2394416, 2214217, 2032532, 1906669, 1765848, 1685815, 1589470, 1534185, 1467897, 1424590, 1375125, 1337157, 1296109, 1248503, 1202655, 1135946, 1074824, 999120, 927323, 853425, 780089, 706732, 633377, 561215, 488759, 419887, 350063, 293268, 232963, 191171, 144419, 115593, 81985, 60807, 36298, 18028, -1913, -19481, -37664, -57914, -77610, -100338, -122253, -147506, -171859, -199330, -225929, -256253, -285579, -317404, -348576, -384346, -418884, -456623, -493531, -502543, -519029],
    [-817500, -860644, -854945, -862333, -869182, -876356, -883096, -889952, -899346, -908060, -910964, -915425, -919772, -924150, -931402, -937883, -942120, -946833, -949535, -952833, -963407, -972031, -983490, -994364, -1003555, -1013243, -1016052, -1020712, -1020484, -1021566, -1023815, -1025764, -1029288, -1032390, -1031514, -1031305, -1031546, -1031630, -1027183, -1024115, -1022403, -1020371, -1024141, -1026389, -1028848, -1031183, -1030910, -1031312, -1028561, -1026375, -1034215, -1040090, -1073625, -1098765, -1173512, -1234756, -1326220, -1409343, -1529486, -1639136, -1774286, -1902557, -2157494, -2378808, -2610549, -2839268, -3166963, -3474630, -3487914, -3621698, -3821735, -4006264, -3830931, -3751942, -3165999, -2715963, -2296623, -1869092, -1655329, -1388722, -1324406, -1205867, -1066424, -932702, -774626, -623064, -542763, -443555, -470117, -462979, -515999, -552849, -603663, -650735, -714492, -773777, -847219, -916868, -984886, -1053341, -1097440, -1148066, -1166167, -1192959, -1202779, -1217147, -1227865, -1239573, -1237511, -1239170, -1217059, -1201226, -1151183, -1110307, -1049782, -994523, -940907, -886842, -844496, -799095, -761332, -721522, -692659, -660863, -639152, -614739, -596137, -575978, -556081, -536084, -510712, -486779, -466702, -445592, -423461, -401600, -382407, -362551, -343329, -323952, -309656, -293806, -284181, -272889, -266356, -258707, -260061, -259003, -265509, -269988, -277191, -283872, -296052, -306758, -321562, -335268, -346228, -357941, -375609, -391654, -409905, -427565, -444709, -462064, -478436, -495071, -511752, -528430, -532893, -540629],
    [-568434, -120169, -293943, -301042, -306224, -312269, -317930, -323694, -324516, -326662, -330171, -333314, -337437, -341297, -342963, -345217, -346423, -347610, -350133, -352409, -358842, -364161, -371961, -378964, -388432, -397386, -407494, -417275, -420724, -425871, -429680, -433873, -429213, -426924, -422251, -418225, -416764, -414319, -413246, -411771, -407823, -404476, -396674, -390029, -384894, -379372, -395538, -406067, -431337, -452739, -454164, -461616, -382665, -327167, -149491, -4992, 166104, 330190, 618248, 872744, 1095809, 1327222, 1431060, 1568743, 1103617, 799667, 572448, 342294, 567389, 729905, 973475, 1199036, 992483, 901509, 749888, 614486, 394512, 196464, -73177, -325333, -545698, -774528, -986195, -1202424, -1390513, -1586119, -1625753, -1707544, -1533966, -1428814, -1179448, -968810, -739792, -515698, -328566, -131531, 44733, 226563, 383736, 547515, 699248, 854209, 1015607, 1175257, 1313296, 1457125, 1532502, 1626197, 1569130, 1552478, 1358450, 1212123, 963771, 742757, 515594, 290080, 121462, -62385, -184925, -324006, -427410, -540374, -626409, -719661, -795447, -875914, -963037, -1048377, -1150079, -1247339, -1350126, -1451432, -1551680, -1652212, -1745108, -1840058, -1924370, -2011327, -2086713, -2165176, -2222662, -2286038, -2324873, -2370284, -2397623, -2429827, -2443788, -2462638, -2464950, -2471693, -2471904, -2474327, -2471391, -2469892, -2463668, -2458710, -2448713, -2440048, -2427468, -2415840, -2406204, -2396034, -2387562, -2378574, -2365371, -2353297, -2341967, -2330359, -2247588, -2183884],
    [572088, 932794, 753730, 719297, 692128, 663157, 645262, 624398, 610312, 594410, 576843, 559722, 547312, 533639, 534710, 531829, 533990, 534704, 527011, 521616, 518329, 514477, 525123, 532024, 558929, 580542, 615129, 646201, 684725, 721253, 762108, 801815, 840856, 880075, 918599, 957135, 989473, 1023412, 1066575, 1107159, 1152677, 1196711, 1249747, 1299780, 1357021, 1412014, 1452390, 1496131, 1486978, 1491290, 1497592, 1502588, 1560578, 1603390, 1736889, 1845570, 2100853, 2316558, 2683425, 3009195, 3048123, 3163833, 2814151, 2588643, 2021488, 1545656, -250923, -1687352, -2833711, -4132827, -4758884, -5572461, -4171477, -3363864, -968687, 1001170, 1705323, 2746690, 2498339, 2578962, 1919279, 1458013, 1123522, 754874, 648481, 471807, 374604, 256259, 168619, 72752, 34133, -19556, -63676, -110360, -196258, -271649, -361659, -447751, -515806, -588693, -640377, -697743, -721346, -753979, -750430, -756576, -758507, -761589, -764795, -767942, -734434, -710583, -639610, -581263, -495963, -417886, -358722, -294497, -263983, -224403, -206858, -183409, -168581, -151443, -151875, -147600, -149756, -150190, -151998, -153461, -159383, -164110, -173441, -181539, -196015, -208803, -233447, -254874, -282858, -309035, -334984, -360613, -385505, -410594, -436944, -463052, -483280, -505083, -514276, -526848, -528090, -532200, -529963, -529427, -521694, -515891, -507086, -499098, -491150, -482831, -472541, -462779, -455009, -446771, -437354, -428253, -411446, -396787, -331019, -278946],
    [507403, 389865, 403191, 381452, 374269, 363347, 355408, 346669, 333061, 320757, 314762, 307077, 302597, 297259, 290676, 284427, 288921, 290320, 302284, 311469, 322784, 333529, 347036, 359486, 373027, 386337, 403329, 419361, 433283, 447770, 459410, 471825, 481414, 491761, 499527, 507747, 508771, 511581, 515575, 518935, 520382, 522230, 524022, 525624, 522987, 521446, 507896, 497411, 457616, 425517, 391786, 358879, 405676, 431144, 576776, 690352, 769821, 858351, 1047221, 1209311, 1271727, 1360883, 1252027, 1195550, 763850, 433008, 251550, 34398, -649475, -1198186, -456423, -49243, 326095, 710165, -163725, -700618, -1296587, -1875316, -2316575, -2789677, -3005376, -3289996, -3389134, -3537707, -3583829, -3657412, -3719110, -3783825, -3750461, -3743377, -3591016, -3478055, -3268812, -3085367, -2833421, -2599829, -2307228, -2030439, -1737713, -1449257, -1179573, -904860, -681288, -443984, -236163, -20443, 220707, 455036, 681170, 909449, 1006227, 1138297, 1089452, 1089083, 985752, 910010, 827956, 747586, 729317, 694583, 702277, 698602, 702477, 704329, 716395, 725724, 737679, 748930, 739984, 736422, 710700, 690916, 656474, 625960, 595603, 565204, 531018, 497587, 463169, 429023, 391918, 356330, 328855, 299206, 295623, 284794, 300935, 309850, 340819, 365878, 403291, 437638, 488043, 534145, 585505, 635457, 686952, 738012, 787817, 837799, 888201, 938490, 977156, 1018870, 1044287, 1074070, 1090716, 1110850, 1098776, 1095332],
    [-1542529, -1964753, -1861030, -1898233, -1913938, -1935157, -1951845, -1969748, -1977226, -1987498, -1976088, -1970487, -1967009, -1962961, -1961282, -1958968, -1957694, -1956023, -1952187, -1948854, -1926017, -1908406, -1879387, -1852850, -1822034, -1792256, -1756814, -1722977, -1686803, -1651256, -1611992, -1573706, -1542316, -1509079, -1470492, -1432640, -1395729, -1358843, -1309100, -1262764, -1225025, -1185010, -1175489, -1158036, -1157791, -1153193, -1151032, -1147873, -1144461, -1140631, -1118056, -1101637, -960965, -852430, -566453, -327770, 18299, 335382, 840559, 1294967, 1647966, 2028003, 2190714, 2410960, 2107696, 1944107, 1062033, 344864, -353580, -1089286, -1437215, -1897248, -930605, -346220, 888449, 1949139, 2251905, 2755939, 2374222, 2221596, 1756158, 1374301, 971245, 574056, 188913, -199473, -392405, -637453, -630748, -691500, -573568, -503243, -363193, -241826, -129978, -15580, 92545, 202351, 249559, 313540, 312789, 329383, 420784, 492140, 621147, 734707, 872018, 1002996, 1196074, 1372613, 1516711, 1669274, 1668482, 1708782, 1615636, 1558247, 1460239, 1373103, 1306416, 1234264, 1204794, 1163887, 1130561, 1095203, 1079004, 1057672, 1071943, 1076674, 1088039, 1097587, 1080367, 1070319, 1039037, 1013445, 994362, 973536, 941736, 913165, 872459, 835004, 789242, 746138, 693852, 644025, 612236, 575516, 560615, 539868, 525695, 509761, 502622, 493368, 497046, 497259, 499527, 501244, 512302, 520858, 529478, 537366, 530033, 526778, 539884, 548478, 566189, 581458, 590663, 601419, 571886, 553149],
    [-1145166, -1885556, -1681534, -1730566, -1754477, -1784840, -1810127, -1836774, -1862013, -1887630, -1916777, -1944979, -1972347, -1999939, -1990071, -1990240, -1935386, -1895417, -1823706, -1760412, -1642053, -1538448, -1385471, -1246025, -1036021, -844876, -607489, -382389, -199706, -5656, 105169, 238316, 355143, 476342, 571100, 671422, 732707, 804572, 853023, 907215, 952564, 999837, 1116616, 1213174, 1381464, 1529499, 1657710, 1788964, 1843372, 1914733, 1972752, 2032067, 2025498, 2030361, 2105130, 2158545, 2249742, 2329609, 2154762, 2045434, 1668991, 1363832, 749306, 218143, -396741, -989379, -334262, -3935, -573981, -842252, -24571, 492548, 1426734, 2248572, 710273, -195944, -1310486, -2366525, -2339852, -2577180, -2102314, -1817748, -1521881, -1229750, -762638, -342430, 99328, 535604, 790995, 1094853, 1194591, 1348528, 1379170, 1442849, 1421942, 1423699, 1393024, 1371039, 1349492, 1327828, 1321361, 1310821, 1209928, 1133155, 972141, 833699, 583753, 363485, -50641, -412870, -757208, -1104872, -1225632, -1407191, -1384192, -1416005, -1341405, -1295330, -1266140, -1232389, -1235979, -1229563, -1216094, -1204515, -1202427, -1197796, -1231148, -1254324, -1292195, -1326175, -1284777, -1263576, -1178990, -1111389, -1019359, -933957, -840521, -749527, -626443, -511705, -372179, -239791, -95350, 45860, 163688, 288113, 365515, 455518, 517890, 587666, 633570, 686265, 716575, 752883, 776407, 803357, 826650, 850989, 856385, 866041, 864913, 866675, 885968, 900618, 913028, 926038, 923344, 924698, 843301, 784077],
    [1663633, 1180206, 1347502, 1340437, 1329442, 1320147, 1302378, 1286880, 1265409, 1245539, 1248492, 1245330, 1226100, 1211176, 1182489, 1157490, 1093644, 1039537, 963997, 894405, 811671, 732459, 637162, 545828, 388546, 249189, 23072, -179802, -375625, -573337, -665877, -786550, -857820, -942327, -990181, -1045633, -1065206, -1095451, -1097431, -1106679, -1113500, -1121018, -1169976, -1207027, -1255756, -1300836, -1321104, -1346662, -1338174, -1336743, -1332858, -1330195, -1304010, -1277255, -1221598, -1170909, -1067407, -977784, -632185, -353252, 169897, 627777, 445799, 435559, 830414, 1117155, 852642, 738085, -1398784, -2827547, -1828174, -1460837, -700678, -45224, 772695, 1547032, 1666909, 1960269, 1097237, 530393, -13888, -564389, -728957, -998056, -1066810, -1189289, -1038442, -960156, -709944, -505800, -248273, -5456, 131338, 296542, 497115, 688211, 978606, 1242394, 1570823, 1881931, 2115597, 2370013, 2278141, 2279098, 2048055, 1879177, 1587948, 1329490, 905439, 525708, 113446, -290006, -536243, -824606, -930082, -1084562, -1098316, -1149807, -1161769, -1184115, -1242827, -1291794, -1331779, -1374170, -1416894, -1459529, -1558527, -1642422, -1731237, -1818840, -1837920, -1875362, -1840589, -1825165, -1755013, -1699484, -1609586, -1529343, -1419595, -1317885, -1145977, -992354, -802348, -622091, -444243, -265708, -84045, 96778, 278245, 459540, 629432, 803249, 969073, 1137038, 1296235, 1457782, 1589532, 1729236, 1845732, 1969018, 2062884, 2164633, 2182687, 2222738, 2219827, 2228427, 2200762, 2182329, 2089325, 2016303],
    [-1301558, -2544651, -2024737, -1977220, -1903092, -1835161, -1730075, -1634944, -1456400, -1300207, -1041594, -810424, -559633, -314099, -121859, 84659, 165327, 278962, 323013, 386004, 471045, 550178, 653644, 750425, 856458, 960518, 1028676, 1106215, 1090460, 1099703, 1055565, 1025798, 976433, 932319, 894469, 854434, 851250, 837638, 835504, 829810, 809337, 792680, 773074, 754406, 756111, 753518, 707976, 675438, 615758, 564558, 508020, 459512, 372750, 296214, 216857, 140835, -3843, -129949, -409859, -644387, -1130651, -1548495, -1728823, -1968860, -1759062, -1665158, 770826, 2565316, 3594941, 4776594, 1121224, -1256937, -3407528, -5616864, -3054168, -1769482, -396323, 958379, 1303351, 1919339, 1982526, 2193168, 1621891, 1259746, 661013, 125612, -278861, -717441, -827152, -1024983, -1055954, -1129410, -1082810, -1068379, -1017195, -975859, -859066, -762492, -634161, -514339, -346307, -191193, 106546, 366335, 650932, 928881, 1086541, 1276580, 1306172, 1378739, 1275993, 1219135, 1048377, 908139, 745252, 588433, 487020, 370718, 289061, 198115, 103172, 9300, -80797, -171906, -251478, -334142, -421044, -506811, -608566, -706190, -819432, -928490, -990211, -1064616, -1104359, -1153351, -1202437, -1251923, -1293773, -1337776, -1327547, -1331234, -1304824, -1286479, -1236604, -1195079, -1096481, -1013176, -886900, -772137, -632185, -497769, -324559, -161744, 25247, 205761, 402504, 594700, 791187, 986236, 1143986, 1311730, 1455352, 1604885, 1735498, 1871180, 1960366, 2061517, 1958259, 1909773],
];

const TEMP: [i64; 170] = [
    -10612084127220, -65345494901760, -44512095186780, -47991904075440, -62831911930608, -74617417379288, -83184361225576, -92367877462119, -85811308571379, -83420026148592, -69766960791257, -59997721707000, -51030635373364, -42764195271708, -39847723341592, -35625228098166, -37297608181659, -37083259140678, -34955704309843, -33330205379219, -30507094663331, -28007156347125, -22875643385027, -18744503234400, -15533578643440, -11956086929794, -9299972227028, -6559363326900, -7743971939826, -7700303964512, -7823832800315, -7827103263800, -6823543060176, -6110385722835, -4619944993590, -3424253027552, -1656498047646, -29816920204, 2566067665579, 4917855377659, 6366429231059, 7955220684866, 10830250502270, 13668563562060, 18495183076272, 23310791637874, 22365207251296, 22712653556250, 18478083252625, 15584896313322, 13369857557088, 11143645616580, 17609249764845, 22422057224329, 34428924052404, 45874895950905, 49087564192680, 51439832483808, 51720351407025, 47710868970426, 33670235027617, 20644999435190, -14915297163864, -29046658032665, -41506656426320, -8607649110208, -117298011700688, -222741223614198, 132903958441700, 488884518564533, 948582159352128, 1447691984999733, 1042169590017984, 879310641149710, 220797136767520, -7989031309008, -12450310660200, 53891245852802, 53784727303461, 57381740644592, 5104220301009, -36967856106117, -86008959535686, -124971772002000, -170785169547057, -217926604308021, -261699993509655, -311289898913416, -296952443471936, -297821022066230, -245297761065193, -207865976846325, -166798942847608, -128640000285306, -100871842497354, -67126104832080, -19104425034882, 32343202023389, 90812999525262, 150985005077760, 202406107973332, 256266766565733, 272426767911409, 293785419453086, 294035412516360, 296471192888842, 299783746715399, 302627366448498, 276532870630302, 257023677008901, 196221146997282, 149049863534156, 101540794450249, 58003942757832, 28807809187740, -1075510553040, -12721791807888, -26274449503975, -35100208154556, -44755760108506, -55505321830395, -65982710569880, -69786777834637, -74369745140292, -84061791098880, -92466155437177, -105738602438806, -118148688325962, -112485191031234, -108781156429252, -107610667066728, -104081822373095, -106190049857136, -106527777905172, -113220311005280, -118216318146694, -121648120100667, -125481864676465, -119363130120134, -115963065846075, -93015987501860, -76765842158580, -62990686598275, -50284608078012, -45676153212588, -39275880305580, -36494316445148, -31417649316480, -22850450852499, -14243329051110, -3780009156596, 4717015037735, 10912943579472, 13966312194516, 13135394655909, 8860243518550, 5765146441248, -660191666616, 1122828274260, 239460574370, 12079653917604, 22170292144350, 40997084143500, 59728630939703, 48157167254152, 44286196205921, 8008892890960, -21247760614164, -46303512413850, -70694532704887];
const OUT: [i64; 9] = [1140411425865537, 62326195874770, -389878768411838, 623677968079384, -291057012215111, 168218952751317, -76537370161052, 93198287374910,  16730658800076];


pub fn pca_gadget() -> (
    usize,
    usize,
    usize,
    usize,
    Instance,
    VarsAssignment,
    VarsAssignment,
    VarsAssignment,
    InputsAssignment,
) {
    //set the parameters, k is the output dimentsion, m is the input dimension
    let num_cons = 171;
    let num_vars = 2049;
    let num_inputs = 0;
    let num_non_zero_entries = 2048;

    //encode the constraints into three matrices
    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    //one
    let one = Scalar::one().to_bytes();
    let minus_one = (-Scalar::one()).to_bytes();
    let mut temp = Scalar::one();
    let mut alpha = Vec::new();
    //
    alpha.push(Scalar::one().to_bytes());
    for i in 1..9 {
        temp = temp + Scalar::one();
        alpha.push(temp.to_bytes());
    }

    //construct the constraints 0-169
    for i in 0..170 {
        for j in 0..9 {
            A.push((i, 170 * j + i, alpha[j]));
        }
        B.push((i, i + 1700, one));
        B.push((i, i + 1530, minus_one));
        C.push((i, 1870 + i, one));
    }
    //constraint 170
    for i in 0..170 {
        A.push((170, 1870 + i, one));
    }
    B.push((170, num_vars, one));
    for i in 0..9 {
        C.push((170, 2040 + i, alpha[i]));
    }

    //provide the satisfying assignments
    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();

    let mut vars_para = vec![Scalar::zero().to_bytes(); 2049];

    //将PARA的fixpoint值输入
    for i in 0..9 {
        for j in 0..170 {
            if PARA[i][j] < 0 {
                vars_para[170 * i + j] = (-Scalar::from(-PARA[i][j] as u64)).to_bytes();
            }
            else { vars_para[170 * i + j] = Scalar::from(PARA[i][j] as u64).to_bytes(); }
        }
    }
    //将X_CETER的fixpoint值输入
    for i in 0..170 {
        if X_CENTER[i] < 0 {
            vars_para[ 1530 + i] = (-Scalar::from(-X_CENTER[i] as u64)).to_bytes();
        }
        else { vars_para[1530 + i] = Scalar::from(X_CENTER[i] as u64).to_bytes(); }
    }

    let mut vars_input = vec![Scalar::zero().to_bytes(); 2049];

    //将X输入的fixpoint值输入
    for i in 0..170 {
        if X[i] < 0 {
            vars_input[i + 1700] = (-Scalar::from(-X[i] as u64)).to_bytes()
        }
        else {
            vars_input[i + 1700] = Scalar::from(X[i] as u64).to_bytes()
        }
    }


    //将中间值的fixpoint值输入
    for i in 0..170 {
        if TEMP[i] < 0 {
            vars_input[i + 1870] = (-Scalar::from(-TEMP[i] as u64)).to_bytes();
        }
        else { vars_input[i + 1870] = Scalar::from(TEMP[i] as u64).to_bytes(); }
    }

    for i in 0..9 {
        if OUT[i] < 0 {
            vars_input[i + 2040] = (-Scalar::from(- OUT[i] as u64)).to_bytes();
        }
        else { vars_input[i + 2040] = Scalar::from(OUT[i] as u64).to_bytes(); }
    }

    let assignment_vars_para = VarsAssignment::new(&vars_para).unwrap();
    let padded_vars_para = {
        let num_padded_vars = inst.inst.get_num_vars();
        let num_vars = assignment_vars_para.assignment.len();
        let padded_vars = if num_padded_vars > num_vars {
            assignment_vars_para.pad(num_padded_vars)
        } else {
            assignment_vars_para
        };
        padded_vars
    };

    let assignment_vars_input = VarsAssignment::new(&vars_input).unwrap();
    let padded_vars_input = {
        let num_padded_vars = inst.inst.get_num_vars();
        let num_vars = assignment_vars_input.assignment.len();
        let padded_vars = if num_padded_vars > num_vars {
            assignment_vars_input.pad(num_padded_vars)
        } else {
            assignment_vars_input
        };
        padded_vars
    };

    ///create the complete variable assignments
    let mut vars = vec![Scalar::zero().to_bytes(); 2049];

    //将PARA的fixpoint值输入
    for i in 0..9 {
        for j in 0..170 {
            if PARA[i][j] < 0 {
                vars[170 * i + j] = (-Scalar::from(-PARA[i][j] as u64)).to_bytes();
            }
            else { vars[170 * i + j] = Scalar::from(PARA[i][j] as u64).to_bytes(); }
        }
    }
    //将X_CETER的fixpoint值输入
    for i in 0..170 {
        if X_CENTER[i] < 0 {
            vars[1530 + i] = (-Scalar::from(-X_CENTER[i] as u64)).to_bytes();
        }
        else { vars[1530 + i] = Scalar::from(X_CENTER[i] as u64).to_bytes(); }
    }

    //将X输入的fixpoint值输入
    for i in 0..170 {
        if X[i] < 0 {
            vars[i + 1700] = (-Scalar::from(-X[i] as u64)).to_bytes()
        }
        else {
            vars[i + 1700] = Scalar::from(X[i] as u64).to_bytes()
        }
    }


    //将中间值的fixpoint值输入
    for i in 0..170 {
        if TEMP[i] < 0 {
            vars[i + 1870] = (-Scalar::from(-TEMP[i] as u64)).to_bytes();
        }
        else { vars[i + 1870] = Scalar::from(TEMP[i] as u64).to_bytes(); }
    }

    for i in 0..9 {
        if OUT[i] < 0 {
            vars[i + 2040] = (-Scalar::from(- OUT[i] as u64)).to_bytes();
        }
        else { vars[i + 2040] = Scalar::from(OUT[i] as u64).to_bytes(); }
    }

    let assignment_vars = VarsAssignment::new(&vars).unwrap();
    let padded_vars = {
        let num_padded_vars = inst.inst.get_num_vars();
        let num_vars = assignment_vars.assignment.len();
        let padded_vars = if num_padded_vars > num_vars {
            assignment_vars.pad(num_padded_vars)
        } else {
            assignment_vars.clone()
        };
        padded_vars
    };


    // create an InputAssignment
    let mut inputs = vec![Scalar::zero().to_bytes(); num_inputs];
    let assignment_inputs = InputsAssignment::new(&inputs).unwrap();

    let res = inst.is_sat(&assignment_vars, &assignment_inputs);
    assert_eq!(res.unwrap(), true);

    (
        num_cons,
        num_vars,
        num_inputs,
        num_non_zero_entries,
        inst,
        padded_vars_para,
        padded_vars_input,
        padded_vars,
        assignment_inputs
        )

}