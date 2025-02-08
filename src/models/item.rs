
#[derive(PartialEq, Clone)]
pub enum ItemStatus {
    Logrono,
    Vendido,
    Reservado,
    Disponible
}
#[derive(Clone)]
pub struct Item {
    pub image_url: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub price: f64,
    pub status: ItemStatus
}

impl Item {
    pub fn get_protections() -> Vec<Item> {
        return vec![

            // Локти 
            Item {
                image_url: "/images/protection/loktiBauerNSX.JPG",
                name: "Bauer NSX",
                description: "talla Jr S",
                price: 40.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/loktiASV.JPG",
                name: "AS-v pro",
                description: "talla señor S",
                price: 95.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/loktiASV.JPG",
                name: "AS-v pro",
                description: "talla señor S",
                price: 95.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/loktiAS580.jpeg",
                name: "As580",
                description: "tallas señor S",
                price: 80.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/loktiWarriorDX3.jpg",
                name: "Warrior DX3",
                description: "tallas señor S",
                price: 50.0,
                status: ItemStatus::Logrono,
            },

            // Колени
            Item {
                image_url: "images/protection/koleniAS580.JPG",
                name: "As 580",
                description: "",
                price: 85.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/koleniWarrior.JPG",
                name: "Warrior Alpha QX",
                description: "talla niños 9",
                price: 40.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/koleniBauerSupreme2SPro.jpg",
                name: "Bauer Supreme 2S Pro",
                description: "talla 9",
                price: 60.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/koleniBauerGS.jpg",
                name: "Bauer Supreme GS",
                description: "talla 11",
                price: 80.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/koleniTrueXC9.png",
                name: "True XC9",
                description: "talla 12",
                price: 80.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/koleniBauerX.jpg",
                name: "Bauer X",
                description: "talla 13",
                price: 60.0,
                status: ItemStatus::Vendido,
            },

            // Грудь
            Item {
                image_url: "/images/protection/grudAS580.JPG",
                name: "CCM As580",
                description: "talla jr L",
                price: 80.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/grudWarriorDX3.jpg",
                name: "Warrior DX3",
                description: "talla jr L",
                price: 70.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/grudWarriorAlpha.jpeg",
                name: "Warrior Alpha",
                description: "talla SR X-Small",
                price: 65.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/grudCCMV04.jpeg",
                name: "CCM V04",
                description: "talla JR M",
                price: 65.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/grudTrue.JPG",
                name: "True Xc9",
                description: "talla jr M",
                price: 45.0,
                status: ItemStatus::Vendido,
            },

            //Краги
            Item {
                image_url: "/images/protection/kragiBauerX.jpg",
                name: "Bauer X",
                description: "Tallas 12",
                price: 70.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/kragiBauerX.jpg",
                name: "Bauer X",
                description: "Talla 9",
                price: 45.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/kragiBauerX.jpg",
                name: "Bauer X",
                description: "Talla 13",
                price: 30.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/kragiCcmAS580.png",
                name: "CCM AS 580",
                description: "Talla señor 14",
                price: 130.0,
                status: ItemStatus::Logrono,
            },

            //Шлема
            Item {
                image_url: "/images/protection/shlemTacks70.JPG",
                name: "CCM Tacks  70",
                description: "talla sr S",
                price: 95.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/shlemReact65.JPG",
                name: "Bauer Reakt 65 con mascara",
                description: "talla señor S",
                price: 140.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/shlemReact65.JPG",
                name: "Bauer Reakt 55 con mascara",
                description: "talla señor L",
                price: 100.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/shlemIMS5.jpeg",
                name: "Bauer Ims 5.0",
                description: "talla señor M",
                price: 80.0,
                status: ItemStatus::Vendido,
            },

            // Трусы
            Item {
                image_url: "/images/protection/trusCCMVector.jpeg",
                name: "CCM Vector",
                description: "talla Jr L",
                price: 70.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/trusAS580.JPG",
                name: "CCM AS 580",
                description: "talla señor M",
                price: 95.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/trusACP.JPG",
                name: "Bauer ACP Elite (transformer)",
                description: "talla intermedium L",
                price: 95.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "/images/protection/trusN7000.JPG",
                name: "Bauer Nexus N7000",
                description: "talla junior M",
                price: 65.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/trusCCMU.jpg",
                name: "CCM U+",
                description: "talla  jr L",
                price: 60.0,
                status: ItemStatus::Vendido,
            },

            // Сумки
            Item {
                image_url: "/images/protection/sumkaBauer.JPG",
                name: "Bolso Bauer con ruedas",
                description: "niños",
                price: 60.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/sumkaKlush.jpg",
                name: "Bolso de palos para equipo con ruedas",
                description: "",
                price: 70.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/sumkaSherwoodSilver.jpg",
                name: "Bolso sherwood con ruedas",
                description: "niños",
                price: 90.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/sumkaCCM.JPG",
                name: "Bolso Ccm pro team bag",
                description: "sin ruedas",
                price: 90.0,
                status: ItemStatus::Disponible,
            },

            // Акссесуары
            Item {
                image_url: "/images/protection/butilka.png",
                name: "Botella sherwood",
                description: "2 ud",
                price: 9.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/shurki.jpg",
                name: "Cordones",
                description: "Blanco (2ud 330cm); Negro (1ud, 305cm)",
                price: 8.0,
                status: ItemStatus::Logrono,
            },

            // Коньки
            Item {
                image_url: "/images/protection/konCCM4052.png",
                name: "CCM Tacks 4052",
                description: "talla 3",
                price: 95.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/konBauerXLP.png",
                name: "Bauer XLP",
                description: "talla Y12",
                price: 95.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "/images/protection/konNexusN2700.png",
                name: "Bauer Nexus N2700",
                description: "",
                price: 105.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/konBauerM1.png",
                name: "Bauer M1",
                description: "talla 2.5 D",
                price: 120.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/konkiCCMTacks9360.png",
                name: "CCM SuperTacks 9460",
                description: "talla 2 D",
                price: 110.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "/images/protection/termoOrange.jpeg",
                name: "Bermudas",
                description: "S",
                price: 35.0,
                status: ItemStatus::Logrono,
            },
        ];
    }

    pub fn get_right_sticks() -> Vec<Item> {
        return vec![
            Item {
                image_url: "images/sticks/2sPro35.JPG",
                name: "Bauer 2s pro",
                description: "35 flex p92",
                price: 60.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/VaporProdigy.JPG",
                name: "Bauer Vapor Prodigy",
                description: "20 flex p01",
                price: 50.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/NexusN2900.JPG",
                name: "Bauer Nexus N2900",
                description: "65 flex p92",
                price: 60.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/sticks/ccmJetspeedFT1.jpeg",
                name: "CCM Jetspeed FT1",
                description: "75 flex p29",
                price: 80.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/sticks/Vapor3X.jpeg",
                name: "Bauer 3x",
                description: "77 flex p28",
                price: 100.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/sticks/2sProShadow.jpg",
                name: "Bauer 2S pro shadow line",
                description: "87 flex p28",
                price: 110.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/sticks/VaporX4.jpg",
                name: "Bauer Vapor x4",
                description: "77 flex p28",
                price: 120.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/sticks/bauerSupreme3S.png",
                name: "Bauer Supreme 3S",
                description: "77 flex p28",
                price: 120.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/Sling87.jpg",
                name: "Bauer Sling",
                description: "87 flex p92",
                price: 125.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/sticks/HyperLite55.jpg",
                name: "Bauer Vapor HiperLite",
                description: "55 flex p92",
                price: 175.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/nexus3N.png",
                name: "Bauer nexus 3N",
                description: "77 flex p92",
                price: 100.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/hiperLite2.jpg.jpeg",
                name: "Bauer Vapor HiperLite2",
                description: "87 flex p92",
                price: 185.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/sticks/Nexusn37.jpg.jpeg",
                name: "Bauer Nexus n37",
                description: "65 flex p92",
                price: 70.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "images/sticks/flyLite70.jpg.jpeg",
                name: "Bauer FlyLite",
                description: "70 flex P92",
                price: 120.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/sticks/vaporX2.7.jpeg",
                name: "Bauer Vapor x2.7",
                description: "50 flex P92",
                price: 70.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/sling55.jpg",
                name: "Bauer Sling",
                description: "55 flex p92",
                price: 130.0,
                status: ItemStatus::Logrono
            },
            Item {
                image_url: "images/sticks/ultraSonic.jpg",
                name: "Bauer Supreme Ultra Sonic",
                description: "55 flex p92",
                price: 85.0,
                status: ItemStatus::Vendido
            },
            Item {
                image_url: "images/sticks/2S70.jpeg",
                name: "Bauer Supreme 2s",
                description: "70 flex P28",
                price: 115.0,
                status: ItemStatus::Vendido
            },
            Item {
                image_url: "images/sticks/1XLite.jpg",
                name: "Bauer 1X Lite",
                description: "67 flex P88",
                price: 75.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "images/sticks/nexusGeo.jpg",
                name: "Bauer Nexus Geo",
                description: "50 flex p92, 40 flex p92",
                price: 110.0,
                status: ItemStatus::Logrono
            },
            Item {
                image_url: "images/sticks/2sProShadow.jpg",
                name: "Bauer Nexus 2n pro shadow line",
                description: "50 flex p92",
                price: 80.0,
                status: ItemStatus::Logrono
            },
            Item {
                image_url: "images/sticks/bauer2NPro.jpeg",
                name: "Bauer Nexus 2n Pro",
                description: "55 flex p92",
                price: 130.0,
                status: ItemStatus::Vendido
            },
            Item {
                image_url: "images/sticks/warriorQRE50_40flex.jpg",
                name: "Warrior QRE 50",
                description: "40 Flex P92",
                price: 40.0,
                status: ItemStatus::Vendido
            },
            Item {
                image_url: "images/sticks/warrriorDynastyX3.jpg",
                name: "Warrior Dynasty x3",
                description: "70 flex curva Kopitar",
                price: 60.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "images/sticks/warriorQRE20_55flex.jpg.jpeg",
                name: "Warrior QRE 20 pro",
                description: "55 Flex W28",
                price: 85.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "images/sticks/warriorQRE40_40flex.jpg",
                name: "Warrior QRE 40",
                description: "40 flex p92",
                price: 55.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "images/sticks/warriorAlphaDX.png",
                name: "Warrior Alpha Dx",
                description: "65 flex W28",
                price: 120.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "images/sticks/warriorAlphaQX3_55flex.jpg.jpeg",
                name: "Warrior Alpha Qx3",
                description: "55flex p92",
                price: 55.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "images/sticks/warriorQRL.jpg.webp",
                name: "Warrior QRL",
                description: "50 flex p92",
                price: 60.0,
                status: ItemStatus::Logrono
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/warriorGrey.JPG",
                name: "Warrior Gris",
                description: "p92 70flex Int",
                price: 50.0,
                status: ItemStatus::Vendido
            },
            Item {
                image_url: "images/sticks/trueXC5.jpg",
                name: "True Xc5",
                description: "58 flex p92",
                price: 55.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "images/sticks/eastonSynergy.jpg",
                name: "Easton Synergy gx",
                description: "65 flex E3",
                price: 60.0,
                status: ItemStatus::Vendido
            },
            Item {
                image_url: "images/sticks/easton450.jpg",
                name: "Easton 450",
                description: "65 flex p92",
                price: 50.0,
                status: ItemStatus::Logrono
            },
            Item {
                image_url: "images/sticks/easton450.jpg",
                name: "Easton 450",
                description: "85 flex p92",
                price: 60.0,
                status: ItemStatus::Logrono
            },
            Item {
                image_url: "images/sticks/CCMAS1.jpg",
                name: "CCM Tacks AS1",
                description: "75 flex curva custom 5",
                price: 100.0,
                status: ItemStatus::Logrono
            },
            Item {
                image_url: "images/sticks/trueXC7.jpg",
                name: "True Xc7",
                description: "75 flex TC2T(3ud),TC4T(4ud); 85 flex TC2T(ud1), TC4T(ud1)",
                price: 100.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "images/sticks/trueXC7.jpg",
                name: "True Xc7",
                description: "75 flex TC2T(2ud)",
                price: 100.0,
                status: ItemStatus::Logrono
            },
            Item {
                image_url: "images/sticks/ribcor8.jpg",
                name: "CCM Ribcor 8",
                description: "80 flex p90",
                price: 175.0,
                status: ItemStatus::Vendido
            },
            Item {
                image_url: "images/sticks/bauerCustom.jpeg",
                name: "Bauer Nexus Custom",
                description: "70 flex p92",
                price: 160.0,
                status: ItemStatus::Logrono,
            },
        ]
    }

    pub fn get_left_sticks() -> Vec<Item> {
        return vec![
            Item {
                image_url: "images/sticks/vaporX2.7.jpeg",
                name: "Bauer Vapor X2.7",
                description: "50 flex p92",
                price: 65.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/nexus3N.png",
                name: "Bauer Nexus N3",
                description: "55 flex p92 ",
                price: 95.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/nexusNSX.png",
                name: "Bauer NSX",
                description: "60 Flex p92",
                price: 60.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/hyperlite2.jpg.webp",
                name: "Bauer HyperLite 2",
                description: "55 flex p90",
                price: 190.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "images/sticks/warriorAlphaDX5.jpg",
                name: "Warrior Alpha DX5",
                description: "40 flex p92",
                price: 65.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "images/sticks/bauerProto.png",
                name: "Bauer Proto",
                description: "65 flex 90tm ",
                price: 195.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "images/sticks/bauerCustom.jpeg",
                name: "Bauer Nexus Custom",
                description: "77 flex p92, 70 flex p92",
                price: 160.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "images/sticks/bauerNexusTracer.png",
                name: "Bauer Nexus Tracer",
                description: "77 flex p92",
                price: 195.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "images/sticks/bauerNexusTracer.png",
                name: "Bauer Nexus Tracer",
                description: "70 flex p92",
                price: 195.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/HyperLite55.jpg",
                name: "Bauer HyperLite",
                description: "87 flex p28",
                price: 140.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/flyLite70.jpg.jpeg",
                name: "Bauer Fly Lite",
                description: "102 flex p28",
                price: 120.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/sticks/warriorQRE50_40flex.jpg",
                name: "Warrior QRE 50",
                description: "55 Flex p92",
                price: 60.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/warriorCovertQREdge.jpg",
                name: "Warrior QREdge",
                description: "50 Flex p92",
                price: 80.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/warriorAlpha_65flex.png",
                name: "Warrior Alpha",
                description: "65 flex w88",
                price: 75.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/sticks/warriorCovert.png",
                name: "Warrior Covert",
                description: "70 flex p92",
                price: 85.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/warriorCovertQ3.png",
                name: "Warrior covert qr3 pro t1",
                description: "75 Flex w02",
                price: 90.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/warriorGR6Pro.png",
                name: "Warrior GR6 Pro",
                description: "85 flex p92",
                price: 220.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/warriorAlphaLX2.png",
                name: "Warrior Alpha LX2 Pro",
                description: "75 flex p92",
                price: 200.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/sticks/eastonSynergy.jpg",
                name: "Easton Synergy GX",
                description: "50 flex p92",
                price: 60.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/eastonStealth.png",
                name: "Easton Stealth",
                description: "65 flex E3",
                price: 80.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/eastonSynergy.jpg",
                name: "Easton Synergy GX",
                description: "100 flex E3",
                price: 90.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/sticks/trueXC7.jpg",
                name: "True Xc7",
                description: "85 flex TC2T(3), TC4T(1); 95 flex TC4T(1)",
                price: 100.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/trueXC7.jpg",
                name: "True Xc7",
                description: "85 flex TC2T",
                price: 100.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/bauerNexusSync.jpg",
                name: "Bauer Nexus Sync",
                description: "65 flex p92",
                price: 170.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/sticks/bauerAg5nt.jpg",
                name: "Bauer AG5NT",
                description: "55 flex p92",
                price: 180.0,
                status: ItemStatus::Vendido,
            },
        ]
    }

    pub fn get_portero() -> Vec<Item> {
        return vec![
            Item {
                image_url: "images/portero/hyperLite2.jpg.webp",
                name: "Bauer HyperLite 2",
                description: "26inc P31 Left",
                price: 250.0,
                status: ItemStatus::Logrono,
            },
            Item {
                image_url: "images/portero/mach.jpeg",
                name: "Bauer Mach Shadow",
                description: "26inc P31 Left",
                price: 230.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/portero/porteroStickSupremeM5Pro.jpg",
                name: "Bauer Supreme M5 Pro Sr",
                description: "26inc P31 Left",
                price: 160.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/portero/vpor3X.jpg.webp",
                name: "Bauer Vapor 3X",
                description: "25inc P31 left",
                price: 130.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "images/portero/blinAndLovushka.jpg",
                name: "Bauer Blocker y Catcher",
                description: "Sr",
                price: 350.0,
                status: ItemStatus::Disponible,
            },
        ]
    }

    pub fn get_linea() -> Vec<Item> {
        return vec![
            Item {
                image_url: "images/linea/vapor3xPro.jpg.webp",
                name: "Bauer Vapor 3xPro",
                description: "talla 8 fit 2",
                price: 580.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "images/linea/vapor3x.jpg",
                name: "Bauer Vapor 3x",
                description: "tallas 7 fit 2; 7.5 fit 2; 8 fit 2",
                price: 390.0,
                status: ItemStatus::Disponible,
            },
        ]
    }
}