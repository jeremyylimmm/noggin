use crate::*;

const FENS: [(&str, &[(i32, usize)]); 128] = [
    (
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        &[
            (1, 20),
            (2, 400),
            (3, 8902),
            (4, 197281),
            (5, 4865609),
            (6, 119060324),
        ],
    ),
    (
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        &[(1, 48), (2, 2039), (3, 97862), (4, 4085603), (5, 193690690)],
    ),
    (
        "4k3/8/8/8/8/8/8/4K2R w K - 0 1",
        &[
            (1, 15),
            (2, 66),
            (3, 1197),
            (4, 7059),
            (5, 133987),
            (6, 764643),
        ],
    ),
    (
        "4k3/8/8/8/8/8/8/R3K3 w Q - 0 1",
        &[
            (1, 16),
            (2, 71),
            (3, 1287),
            (4, 7626),
            (5, 145232),
            (6, 846648),
        ],
    ),
    (
        "4k2r/8/8/8/8/8/8/4K3 w k - 0 1",
        &[
            (1, 5),
            (2, 75),
            (3, 459),
            (4, 8290),
            (5, 47635),
            (6, 899442),
        ],
    ),
    (
        "r3k3/8/8/8/8/8/8/4K3 w q - 0 1",
        &[
            (1, 5),
            (2, 80),
            (3, 493),
            (4, 8897),
            (5, 52710),
            (6, 1001523),
        ],
    ),
    (
        "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1",
        &[
            (1, 26),
            (2, 112),
            (3, 3189),
            (4, 17945),
            (5, 532933),
            (6, 2788982),
        ],
    ),
    (
        "r3k2r/8/8/8/8/8/8/4K3 w kq - 0 1",
        &[
            (1, 5),
            (2, 130),
            (3, 782),
            (4, 22180),
            (5, 118882),
            (6, 3517770),
        ],
    ),
    (
        "8/8/8/8/8/8/6k1/4K2R w K - 0 1",
        &[
            (1, 12),
            (2, 38),
            (3, 564),
            (4, 2219),
            (5, 37735),
            (6, 185867),
        ],
    ),
    (
        "8/8/8/8/8/8/1k6/R3K3 w Q - 0 1",
        &[
            (1, 15),
            (2, 65),
            (3, 1018),
            (4, 4573),
            (5, 80619),
            (6, 413018),
        ],
    ),
    (
        "4k2r/6K1/8/8/8/8/8/8 w k - 0 1",
        &[
            (1, 3),
            (2, 32),
            (3, 134),
            (4, 2073),
            (5, 10485),
            (6, 179869),
        ],
    ),
    (
        "r3k3/1K6/8/8/8/8/8/8 w q - 0 1",
        &[
            (1, 4),
            (2, 49),
            (3, 243),
            (4, 3991),
            (5, 20780),
            (6, 367724),
        ],
    ),
    (
        "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
        &[
            (1, 26),
            (2, 568),
            (3, 13744),
            (4, 314346),
            (5, 7594526),
            (6, 179862938),
        ],
    ),
    (
        "r3k2r/8/8/8/8/8/8/1R2K2R w Kkq - 0 1",
        &[
            (1, 25),
            (2, 567),
            (3, 14095),
            (4, 328965),
            (5, 8153719),
            (6, 195629489),
        ],
    ),
    (
        "r3k2r/8/8/8/8/8/8/2R1K2R w Kkq - 0 1",
        &[
            (1, 25),
            (2, 548),
            (3, 13502),
            (4, 312835),
            (5, 7736373),
            (6, 184411439),
        ],
    ),
    (
        "r3k2r/8/8/8/8/8/8/R3K1R1 w Qkq - 0 1",
        &[
            (1, 25),
            (2, 547),
            (3, 13579),
            (4, 316214),
            (5, 7878456),
            (6, 189224276),
        ],
    ),
    (
        "1r2k2r/8/8/8/8/8/8/R3K2R w KQk - 0 1",
        &[
            (1, 26),
            (2, 583),
            (3, 14252),
            (4, 334705),
            (5, 8198901),
            (6, 198328929),
        ],
    ),
    (
        "2r1k2r/8/8/8/8/8/8/R3K2R w KQk - 0 1",
        &[
            (1, 25),
            (2, 560),
            (3, 13592),
            (4, 317324),
            (5, 7710115),
            (6, 185959088),
        ],
    ),
    (
        "r3k1r1/8/8/8/8/8/8/R3K2R w KQq - 0 1",
        &[
            (1, 25),
            (2, 560),
            (3, 13607),
            (4, 320792),
            (5, 7848606),
            (6, 190755813),
        ],
    ),
    (
        "4k3/8/8/8/8/8/8/4K2R b K - 0 1",
        &[
            (1, 5),
            (2, 75),
            (3, 459),
            (4, 8290),
            (5, 47635),
            (6, 899442),
        ],
    ),
    (
        "4k3/8/8/8/8/8/8/R3K3 b Q - 0 1",
        &[
            (1, 5),
            (2, 80),
            (3, 493),
            (4, 8897),
            (5, 52710),
            (6, 1001523),
        ],
    ),
    (
        "4k2r/8/8/8/8/8/8/4K3 b k - 0 1",
        &[
            (1, 15),
            (2, 66),
            (3, 1197),
            (4, 7059),
            (5, 133987),
            (6, 764643),
        ],
    ),
    (
        "r3k3/8/8/8/8/8/8/4K3 b q - 0 1",
        &[
            (1, 16),
            (2, 71),
            (3, 1287),
            (4, 7626),
            (5, 145232),
            (6, 846648),
        ],
    ),
    (
        "4k3/8/8/8/8/8/8/R3K2R b KQ - 0 1",
        &[
            (1, 5),
            (2, 130),
            (3, 782),
            (4, 22180),
            (5, 118882),
            (6, 3517770),
        ],
    ),
    (
        "r3k2r/8/8/8/8/8/8/4K3 b kq - 0 1",
        &[
            (1, 26),
            (2, 112),
            (3, 3189),
            (4, 17945),
            (5, 532933),
            (6, 2788982),
        ],
    ),
    (
        "8/8/8/8/8/8/6k1/4K2R b K - 0 1",
        &[
            (1, 3),
            (2, 32),
            (3, 134),
            (4, 2073),
            (5, 10485),
            (6, 179869),
        ],
    ),
    (
        "8/8/8/8/8/8/1k6/R3K3 b Q - 0 1",
        &[
            (1, 4),
            (2, 49),
            (3, 243),
            (4, 3991),
            (5, 20780),
            (6, 367724),
        ],
    ),
    (
        "4k2r/6K1/8/8/8/8/8/8 b k - 0 1",
        &[
            (1, 12),
            (2, 38),
            (3, 564),
            (4, 2219),
            (5, 37735),
            (6, 185867),
        ],
    ),
    (
        "r3k3/1K6/8/8/8/8/8/8 b q - 0 1",
        &[
            (1, 15),
            (2, 65),
            (3, 1018),
            (4, 4573),
            (5, 80619),
            (6, 413018),
        ],
    ),
    (
        "r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1",
        &[
            (1, 26),
            (2, 568),
            (3, 13744),
            (4, 314346),
            (5, 7594526),
            (6, 179862938),
        ],
    ),
    (
        "r3k2r/8/8/8/8/8/8/1R2K2R b Kkq - 0 1",
        &[
            (1, 26),
            (2, 583),
            (3, 14252),
            (4, 334705),
            (5, 8198901),
            (6, 198328929),
        ],
    ),
    (
        "r3k2r/8/8/8/8/8/8/2R1K2R b Kkq - 0 1",
        &[
            (1, 25),
            (2, 560),
            (3, 13592),
            (4, 317324),
            (5, 7710115),
            (6, 185959088),
        ],
    ),
    (
        "r3k2r/8/8/8/8/8/8/R3K1R1 b Qkq - 0 1",
        &[
            (1, 25),
            (2, 560),
            (3, 13607),
            (4, 320792),
            (5, 7848606),
            (6, 190755813),
        ],
    ),
    (
        "1r2k2r/8/8/8/8/8/8/R3K2R b KQk - 0 1",
        &[
            (1, 25),
            (2, 567),
            (3, 14095),
            (4, 328965),
            (5, 8153719),
            (6, 195629489),
        ],
    ),
    (
        "2r1k2r/8/8/8/8/8/8/R3K2R b KQk - 0 1",
        &[
            (1, 25),
            (2, 548),
            (3, 13502),
            (4, 312835),
            (5, 7736373),
            (6, 184411439),
        ],
    ),
    (
        "r3k1r1/8/8/8/8/8/8/R3K2R b KQq - 0 1",
        &[
            (1, 25),
            (2, 547),
            (3, 13579),
            (4, 316214),
            (5, 7878456),
            (6, 189224276),
        ],
    ),
    (
        "8/1n4N1/2k5/8/8/5K2/1N4n1/8 w - - 0 1",
        &[
            (1, 14),
            (2, 195),
            (3, 2760),
            (4, 38675),
            (5, 570726),
            (6, 8107539),
        ],
    ),
    (
        "8/1k6/8/5N2/8/4n3/8/2K5 w - - 0 1",
        &[
            (1, 11),
            (2, 156),
            (3, 1636),
            (4, 20534),
            (5, 223507),
            (6, 2594412),
        ],
    ),
    (
        "8/8/4k3/3Nn3/3nN3/4K3/8/8 w - - 0 1",
        &[
            (1, 19),
            (2, 289),
            (3, 4442),
            (4, 73584),
            (5, 1198299),
            (6, 19870403),
        ],
    ),
    (
        "K7/8/2n5/1n6/8/8/8/k6N w - - 0 1",
        &[
            (1, 3),
            (2, 51),
            (3, 345),
            (4, 5301),
            (5, 38348),
            (6, 588695),
        ],
    ),
    (
        "k7/8/2N5/1N6/8/8/8/K6n w - - 0 1",
        &[
            (1, 17),
            (2, 54),
            (3, 835),
            (4, 5910),
            (5, 92250),
            (6, 688780),
        ],
    ),
    (
        "8/1n4N1/2k5/8/8/5K2/1N4n1/8 b - - 0 1",
        &[
            (1, 15),
            (2, 193),
            (3, 2816),
            (4, 40039),
            (5, 582642),
            (6, 8503277),
        ],
    ),
    (
        "8/1k6/8/5N2/8/4n3/8/2K5 b - - 0 1",
        &[
            (1, 16),
            (2, 180),
            (3, 2290),
            (4, 24640),
            (5, 288141),
            (6, 3147566),
        ],
    ),
    (
        "8/8/3K4/3Nn3/3nN3/4k3/8/8 b - - 0 1",
        &[
            (1, 4),
            (2, 68),
            (3, 1118),
            (4, 16199),
            (5, 281190),
            (6, 4405103),
        ],
    ),
    (
        "K7/8/2n5/1n6/8/8/8/k6N b - - 0 1",
        &[
            (1, 17),
            (2, 54),
            (3, 835),
            (4, 5910),
            (5, 92250),
            (6, 688780),
        ],
    ),
    (
        "k7/8/2N5/1N6/8/8/8/K6n b - - 0 1",
        &[
            (1, 3),
            (2, 51),
            (3, 345),
            (4, 5301),
            (5, 38348),
            (6, 588695),
        ],
    ),
    (
        "B6b/8/8/8/2K5/4k3/8/b6B w - - 0 1",
        &[
            (1, 17),
            (2, 278),
            (3, 4607),
            (4, 76778),
            (5, 1320507),
            (6, 22823890),
        ],
    ),
    (
        "8/8/1B6/7b/7k/8/2B1b3/7K w - - 0 1",
        &[
            (1, 21),
            (2, 316),
            (3, 5744),
            (4, 93338),
            (5, 1713368),
            (6, 28861171),
        ],
    ),
    (
        "k7/B7/1B6/1B6/8/8/8/K6b w - - 0 1",
        &[
            (1, 21),
            (2, 144),
            (3, 3242),
            (4, 32955),
            (5, 787524),
            (6, 7881673),
        ],
    ),
    (
        "K7/b7/1b6/1b6/8/8/8/k6B w - - 0 1",
        &[
            (1, 7),
            (2, 143),
            (3, 1416),
            (4, 31787),
            (5, 310862),
            (6, 7382896),
        ],
    ),
    (
        "B6b/8/8/8/2K5/5k2/8/b6B b - - 0 1",
        &[
            (1, 6),
            (2, 106),
            (3, 1829),
            (4, 31151),
            (5, 530585),
            (6, 9250746),
        ],
    ),
    (
        "8/8/1B6/7b/7k/8/2B1b3/7K b - - 0 1",
        &[
            (1, 17),
            (2, 309),
            (3, 5133),
            (4, 93603),
            (5, 1591064),
            (6, 29027891),
        ],
    ),
    (
        "k7/B7/1B6/1B6/8/8/8/K6b b - - 0 1",
        &[
            (1, 7),
            (2, 143),
            (3, 1416),
            (4, 31787),
            (5, 310862),
            (6, 7382896),
        ],
    ),
    (
        "K7/b7/1b6/1b6/8/8/8/k6B b - - 0 1",
        &[
            (1, 21),
            (2, 144),
            (3, 3242),
            (4, 32955),
            (5, 787524),
            (6, 7881673),
        ],
    ),
    (
        "7k/RR6/8/8/8/8/rr6/7K w - - 0 1",
        &[
            (1, 19),
            (2, 275),
            (3, 5300),
            (4, 104342),
            (5, 2161211),
            (6, 44956585),
        ],
    ),
    (
        "R6r/8/8/2K5/5k2/8/8/r6R w - - 0 1",
        &[
            (1, 36),
            (2, 1027),
            (3, 29215),
            (4, 771461),
            (5, 20506480),
            (6, 525169084),
        ],
    ),
    (
        "7k/RR6/8/8/8/8/rr6/7K b - - 0 1",
        &[
            (1, 19),
            (2, 275),
            (3, 5300),
            (4, 104342),
            (5, 2161211),
            (6, 44956585),
        ],
    ),
    (
        "R6r/8/8/2K5/5k2/8/8/r6R b - - 0 1",
        &[
            (1, 36),
            (2, 1027),
            (3, 29227),
            (4, 771368),
            (5, 20521342),
            (6, 524966748),
        ],
    ),
    (
        "6kq/8/8/8/8/8/8/7K w - - 0 1",
        &[
            (1, 2),
            (2, 36),
            (3, 143),
            (4, 3637),
            (5, 14893),
            (6, 391507),
        ],
    ),
    (
        "6KQ/8/8/8/8/8/8/7k b - - 0 1",
        &[
            (1, 2),
            (2, 36),
            (3, 143),
            (4, 3637),
            (5, 14893),
            (6, 391507),
        ],
    ),
    (
        "K7/8/8/3Q4/4q3/8/8/7k w - - 0 1",
        &[
            (1, 6),
            (2, 35),
            (3, 495),
            (4, 8349),
            (5, 166741),
            (6, 3370175),
        ],
    ),
    (
        "6qk/8/8/8/8/8/8/7K b - - 0 1",
        &[
            (1, 22),
            (2, 43),
            (3, 1015),
            (4, 4167),
            (5, 105749),
            (6, 419369),
        ],
    ),
    (
        "6KQ/8/8/8/8/8/8/7k b - - 0 1",
        &[
            (1, 2),
            (2, 36),
            (3, 143),
            (4, 3637),
            (5, 14893),
            (6, 391507),
        ],
    ),
    (
        "K7/8/8/3Q4/4q3/8/8/7k b - - 0 1",
        &[
            (1, 6),
            (2, 35),
            (3, 495),
            (4, 8349),
            (5, 166741),
            (6, 3370175),
        ],
    ),
    (
        "8/8/8/8/8/K7/P7/k7 w - - 0 1",
        &[(1, 3), (2, 7), (3, 43), (4, 199), (5, 1347), (6, 6249)],
    ),
    (
        "8/8/8/8/8/7K/7P/7k w - - 0 1",
        &[(1, 3), (2, 7), (3, 43), (4, 199), (5, 1347), (6, 6249)],
    ),
    (
        "K7/p7/k7/8/8/8/8/8 w - - 0 1",
        &[(1, 1), (2, 3), (3, 12), (4, 80), (5, 342), (6, 2343)],
    ),
    (
        "7K/7p/7k/8/8/8/8/8 w - - 0 1",
        &[(1, 1), (2, 3), (3, 12), (4, 80), (5, 342), (6, 2343)],
    ),
    (
        "8/2k1p3/3pP3/3P2K1/8/8/8/8 w - - 0 1",
        &[(1, 7), (2, 35), (3, 210), (4, 1091), (5, 7028), (6, 34834)],
    ),
    (
        "8/8/8/8/8/K7/P7/k7 b - - 0 1",
        &[(1, 1), (2, 3), (3, 12), (4, 80), (5, 342), (6, 2343)],
    ),
    (
        "8/8/8/8/8/7K/7P/7k b - - 0 1",
        &[(1, 1), (2, 3), (3, 12), (4, 80), (5, 342), (6, 2343)],
    ),
    (
        "K7/p7/k7/8/8/8/8/8 b - - 0 1",
        &[(1, 3), (2, 7), (3, 43), (4, 199), (5, 1347), (6, 6249)],
    ),
    (
        "7K/7p/7k/8/8/8/8/8 b - - 0 1",
        &[(1, 3), (2, 7), (3, 43), (4, 199), (5, 1347), (6, 6249)],
    ),
    (
        "8/2k1p3/3pP3/3P2K1/8/8/8/8 b - - 0 1",
        &[(1, 5), (2, 35), (3, 182), (4, 1091), (5, 5408), (6, 34822)],
    ),
    (
        "8/8/8/8/8/4k3/4P3/4K3 w - - 0 1",
        &[(1, 2), (2, 8), (3, 44), (4, 282), (5, 1814), (6, 11848)],
    ),
    (
        "4k3/4p3/4K3/8/8/8/8/8 b - - 0 1",
        &[(1, 2), (2, 8), (3, 44), (4, 282), (5, 1814), (6, 11848)],
    ),
    (
        "8/8/7k/7p/7P/7K/8/8 w - - 0 1",
        &[(1, 3), (2, 9), (3, 57), (4, 360), (5, 1969), (6, 10724)],
    ),
    (
        "8/8/k7/p7/P7/K7/8/8 w - - 0 1",
        &[(1, 3), (2, 9), (3, 57), (4, 360), (5, 1969), (6, 10724)],
    ),
    (
        "8/8/3k4/3p4/3P4/3K4/8/8 w - - 0 1",
        &[(1, 5), (2, 25), (3, 180), (4, 1294), (5, 8296), (6, 53138)],
    ),
    (
        "8/3k4/3p4/8/3P4/3K4/8/8 w - - 0 1",
        &[
            (1, 8),
            (2, 61),
            (3, 483),
            (4, 3213),
            (5, 23599),
            (6, 157093),
        ],
    ),
    (
        "8/8/3k4/3p4/8/3P4/3K4/8 w - - 0 1",
        &[
            (1, 8),
            (2, 61),
            (3, 411),
            (4, 3213),
            (5, 21637),
            (6, 158065),
        ],
    ),
    (
        "k7/8/3p4/8/3P4/8/8/7K w - - 0 1",
        &[(1, 4), (2, 15), (3, 90), (4, 534), (5, 3450), (6, 20960)],
    ),
    (
        "8/8/7k/7p/7P/7K/8/8 b - - 0 1",
        &[(1, 3), (2, 9), (3, 57), (4, 360), (5, 1969), (6, 10724)],
    ),
    (
        "8/8/k7/p7/P7/K7/8/8 b - - 0 1",
        &[(1, 3), (2, 9), (3, 57), (4, 360), (5, 1969), (6, 10724)],
    ),
    (
        "8/8/3k4/3p4/3P4/3K4/8/8 b - - 0 1",
        &[(1, 5), (2, 25), (3, 180), (4, 1294), (5, 8296), (6, 53138)],
    ),
    (
        "8/3k4/3p4/8/3P4/3K4/8/8 b - - 0 1",
        &[
            (1, 8),
            (2, 61),
            (3, 411),
            (4, 3213),
            (5, 21637),
            (6, 158065),
        ],
    ),
    (
        "8/8/3k4/3p4/8/3P4/3K4/8 b - - 0 1",
        &[
            (1, 8),
            (2, 61),
            (3, 483),
            (4, 3213),
            (5, 23599),
            (6, 157093),
        ],
    ),
    (
        "k7/8/3p4/8/3P4/8/8/7K b - - 0 1",
        &[(1, 4), (2, 15), (3, 89), (4, 537), (5, 3309), (6, 21104)],
    ),
    (
        "7k/3p4/8/8/3P4/8/8/K7 w - - 0 1",
        &[(1, 4), (2, 19), (3, 117), (4, 720), (5, 4661), (6, 32191)],
    ),
    (
        "7k/8/8/3p4/8/8/3P4/K7 w - - 0 1",
        &[(1, 5), (2, 19), (3, 116), (4, 716), (5, 4786), (6, 30980)],
    ),
    (
        "k7/8/8/7p/6P1/8/8/K7 w - - 0 1",
        &[(1, 5), (2, 22), (3, 139), (4, 877), (5, 6112), (6, 41874)],
    ),
    (
        "k7/8/7p/8/8/6P1/8/K7 w - - 0 1",
        &[(1, 4), (2, 16), (3, 101), (4, 637), (5, 4354), (6, 29679)],
    ),
    (
        "k7/8/8/6p1/7P/8/8/K7 w - - 0 1",
        &[(1, 5), (2, 22), (3, 139), (4, 877), (5, 6112), (6, 41874)],
    ),
    (
        "k7/8/6p1/8/8/7P/8/K7 w - - 0 1",
        &[(1, 4), (2, 16), (3, 101), (4, 637), (5, 4354), (6, 29679)],
    ),
    (
        "k7/8/8/3p4/4p3/8/8/7K w - - 0 1",
        &[(1, 3), (2, 15), (3, 84), (4, 573), (5, 3013), (6, 22886)],
    ),
    (
        "k7/8/3p4/8/8/4P3/8/7K w - - 0 1",
        &[(1, 4), (2, 16), (3, 101), (4, 637), (5, 4271), (6, 28662)],
    ),
    (
        "7k/3p4/8/8/3P4/8/8/K7 b - - 0 1",
        &[(1, 5), (2, 19), (3, 117), (4, 720), (5, 5014), (6, 32167)],
    ),
    (
        "7k/8/8/3p4/8/8/3P4/K7 b - - 0 1",
        &[(1, 4), (2, 19), (3, 117), (4, 712), (5, 4658), (6, 30749)],
    ),
    (
        "k7/8/8/7p/6P1/8/8/K7 b - - 0 1",
        &[(1, 5), (2, 22), (3, 139), (4, 877), (5, 6112), (6, 41874)],
    ),
    (
        "k7/8/7p/8/8/6P1/8/K7 b - - 0 1",
        &[(1, 4), (2, 16), (3, 101), (4, 637), (5, 4354), (6, 29679)],
    ),
    (
        "k7/8/8/6p1/7P/8/8/K7 b - - 0 1",
        &[(1, 5), (2, 22), (3, 139), (4, 877), (5, 6112), (6, 41874)],
    ),
    (
        "k7/8/6p1/8/8/7P/8/K7 b - - 0 1",
        &[(1, 4), (2, 16), (3, 101), (4, 637), (5, 4354), (6, 29679)],
    ),
    (
        "k7/8/8/3p4/4p3/8/8/7K b - - 0 1",
        &[(1, 5), (2, 15), (3, 102), (4, 569), (5, 4337), (6, 22579)],
    ),
    (
        "k7/8/3p4/8/8/4P3/8/7K b - - 0 1",
        &[(1, 4), (2, 16), (3, 101), (4, 637), (5, 4271), (6, 28662)],
    ),
    (
        "7k/8/8/p7/1P6/8/8/7K w - - 0 1",
        &[(1, 5), (2, 22), (3, 139), (4, 877), (5, 6112), (6, 41874)],
    ),
    (
        "7k/8/p7/8/8/1P6/8/7K w - - 0 1",
        &[(1, 4), (2, 16), (3, 101), (4, 637), (5, 4354), (6, 29679)],
    ),
    (
        "7k/8/8/1p6/P7/8/8/7K w - - 0 1",
        &[(1, 5), (2, 22), (3, 139), (4, 877), (5, 6112), (6, 41874)],
    ),
    (
        "7k/8/1p6/8/8/P7/8/7K w - - 0 1",
        &[(1, 4), (2, 16), (3, 101), (4, 637), (5, 4354), (6, 29679)],
    ),
    (
        "k7/7p/8/8/8/8/6P1/K7 w - - 0 1",
        &[(1, 5), (2, 25), (3, 161), (4, 1035), (5, 7574), (6, 55338)],
    ),
    (
        "k7/6p1/8/8/8/8/7P/K7 w - - 0 1",
        &[(1, 5), (2, 25), (3, 161), (4, 1035), (5, 7574), (6, 55338)],
    ),
    (
        "3k4/3pp3/8/8/8/8/3PP3/3K4 w - - 0 1",
        &[
            (1, 7),
            (2, 49),
            (3, 378),
            (4, 2902),
            (5, 24122),
            (6, 199002),
        ],
    ),
    (
        "7k/8/8/p7/1P6/8/8/7K b - - 0 1",
        &[(1, 5), (2, 22), (3, 139), (4, 877), (5, 6112), (6, 41874)],
    ),
    (
        "7k/8/p7/8/8/1P6/8/7K b - - 0 1",
        &[(1, 4), (2, 16), (3, 101), (4, 637), (5, 4354), (6, 29679)],
    ),
    (
        "7k/8/8/1p6/P7/8/8/7K b - - 0 1",
        &[(1, 5), (2, 22), (3, 139), (4, 877), (5, 6112), (6, 41874)],
    ),
    (
        "7k/8/1p6/8/8/P7/8/7K b - - 0 1",
        &[(1, 4), (2, 16), (3, 101), (4, 637), (5, 4354), (6, 29679)],
    ),
    (
        "k7/7p/8/8/8/8/6P1/K7 b - - 0 1",
        &[(1, 5), (2, 25), (3, 161), (4, 1035), (5, 7574), (6, 55338)],
    ),
    (
        "k7/6p1/8/8/8/8/7P/K7 b - - 0 1",
        &[(1, 5), (2, 25), (3, 161), (4, 1035), (5, 7574), (6, 55338)],
    ),
    (
        "3k4/3pp3/8/8/8/8/3PP3/3K4 b - - 0 1",
        &[
            (1, 7),
            (2, 49),
            (3, 378),
            (4, 2902),
            (5, 24122),
            (6, 199002),
        ],
    ),
    (
        "8/Pk6/8/8/8/8/6Kp/8 w - - 0 1",
        &[
            (1, 11),
            (2, 97),
            (3, 887),
            (4, 8048),
            (5, 90606),
            (6, 1030499),
        ],
    ),
    (
        "n1n5/1Pk5/8/8/8/8/5Kp1/5N1N w - - 0 1",
        &[
            (1, 24),
            (2, 421),
            (3, 7421),
            (4, 124608),
            (5, 2193768),
            (6, 37665329),
        ],
    ),
    (
        "8/PPPk4/8/8/8/8/4Kppp/8 w - - 0 1",
        &[
            (1, 18),
            (2, 270),
            (3, 4699),
            (4, 79355),
            (5, 1533145),
            (6, 28859283),
        ],
    ),
    (
        "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N w - - 0 1",
        &[
            (1, 24),
            (2, 496),
            (3, 9483),
            (4, 182838),
            (5, 3605103),
            (6, 71179139),
        ],
    ),
    (
        "8/Pk6/8/8/8/8/6Kp/8 b - - 0 1",
        &[
            (1, 11),
            (2, 97),
            (3, 887),
            (4, 8048),
            (5, 90606),
            (6, 1030499),
        ],
    ),
    (
        "n1n5/1Pk5/8/8/8/8/5Kp1/5N1N b - - 0 1",
        &[
            (1, 24),
            (2, 421),
            (3, 7421),
            (4, 124608),
            (5, 2193768),
            (6, 37665329),
        ],
    ),
    (
        "8/PPPk4/8/8/8/8/4Kppp/8 b - - 0 1",
        &[
            (1, 18),
            (2, 270),
            (3, 4699),
            (4, 79355),
            (5, 1533145),
            (6, 28859283),
        ],
    ),
    (
        "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1",
        &[
            (1, 24),
            (2, 496),
            (3, 9483),
            (4, 182838),
            (5, 3605103),
            (6, 71179139),
        ],
    ),
    (
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        &[(4, 43238), (5, 674624), (6, 11030083)],
    ),
    (
        "rnbqkb1r/ppppp1pp/7n/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3",
        &[(5, 11139762)],
    ),
];

fn test_perft_from_position(index: usize) {
    let (fen, entries) = FENS[index];
    let pos = Position::from_fen(fen).unwrap();

    for &(depth, count) in entries {
        assert_eq!(pos.perft(depth), count);
    }
}

macro_rules! define_perft_test {
    ($name:ident, $index: expr) => {
        #[test]
        fn $name() {
            test_perft_from_position($index);
        }
    };
}

define_perft_test!(test_perft_0, 0);
define_perft_test!(test_perft_1, 1);
define_perft_test!(test_perft_2, 2);
define_perft_test!(test_perft_3, 3);
define_perft_test!(test_perft_4, 4);
define_perft_test!(test_perft_5, 5);
define_perft_test!(test_perft_6, 6);
define_perft_test!(test_perft_7, 7);
define_perft_test!(test_perft_8, 8);
define_perft_test!(test_perft_9, 9);
define_perft_test!(test_perft_10, 10);
define_perft_test!(test_perft_11, 11);
define_perft_test!(test_perft_12, 12);
define_perft_test!(test_perft_13, 13);
define_perft_test!(test_perft_14, 14);
define_perft_test!(test_perft_15, 15);
define_perft_test!(test_perft_16, 16);
define_perft_test!(test_perft_17, 17);
define_perft_test!(test_perft_18, 18);
define_perft_test!(test_perft_19, 19);
define_perft_test!(test_perft_20, 20);
define_perft_test!(test_perft_21, 21);
define_perft_test!(test_perft_22, 22);
define_perft_test!(test_perft_23, 23);
define_perft_test!(test_perft_24, 24);
define_perft_test!(test_perft_25, 25);
define_perft_test!(test_perft_26, 26);
define_perft_test!(test_perft_27, 27);
define_perft_test!(test_perft_28, 28);
define_perft_test!(test_perft_29, 29);
define_perft_test!(test_perft_30, 30);
define_perft_test!(test_perft_31, 31);
define_perft_test!(test_perft_32, 32);
define_perft_test!(test_perft_33, 33);
define_perft_test!(test_perft_34, 34);
define_perft_test!(test_perft_35, 35);
define_perft_test!(test_perft_36, 36);
define_perft_test!(test_perft_37, 37);
define_perft_test!(test_perft_38, 38);
define_perft_test!(test_perft_39, 39);
define_perft_test!(test_perft_40, 40);
define_perft_test!(test_perft_41, 41);
define_perft_test!(test_perft_42, 42);
define_perft_test!(test_perft_43, 43);
define_perft_test!(test_perft_44, 44);
define_perft_test!(test_perft_45, 45);
define_perft_test!(test_perft_46, 46);
define_perft_test!(test_perft_47, 47);
define_perft_test!(test_perft_48, 48);
define_perft_test!(test_perft_49, 49);
define_perft_test!(test_perft_50, 50);
define_perft_test!(test_perft_51, 51);
define_perft_test!(test_perft_52, 52);
define_perft_test!(test_perft_53, 53);
define_perft_test!(test_perft_54, 54);
define_perft_test!(test_perft_55, 55);
define_perft_test!(test_perft_56, 56);
define_perft_test!(test_perft_57, 57);
define_perft_test!(test_perft_58, 58);
define_perft_test!(test_perft_59, 59);
define_perft_test!(test_perft_60, 60);
define_perft_test!(test_perft_61, 61);
define_perft_test!(test_perft_62, 62);
define_perft_test!(test_perft_63, 63);
define_perft_test!(test_perft_64, 64);
define_perft_test!(test_perft_65, 65);
define_perft_test!(test_perft_66, 66);
define_perft_test!(test_perft_67, 67);
define_perft_test!(test_perft_68, 68);
define_perft_test!(test_perft_69, 69);
define_perft_test!(test_perft_70, 70);
define_perft_test!(test_perft_71, 71);
define_perft_test!(test_perft_72, 72);
define_perft_test!(test_perft_73, 73);
define_perft_test!(test_perft_74, 74);
define_perft_test!(test_perft_75, 75);
define_perft_test!(test_perft_76, 76);
define_perft_test!(test_perft_77, 77);
define_perft_test!(test_perft_78, 78);
define_perft_test!(test_perft_79, 79);
define_perft_test!(test_perft_80, 80);
define_perft_test!(test_perft_81, 81);
define_perft_test!(test_perft_82, 82);
define_perft_test!(test_perft_83, 83);
define_perft_test!(test_perft_84, 84);
define_perft_test!(test_perft_85, 85);
define_perft_test!(test_perft_86, 86);
define_perft_test!(test_perft_87, 87);
define_perft_test!(test_perft_88, 88);
define_perft_test!(test_perft_89, 89);
define_perft_test!(test_perft_90, 90);
define_perft_test!(test_perft_91, 91);
define_perft_test!(test_perft_92, 92);
define_perft_test!(test_perft_93, 93);
define_perft_test!(test_perft_94, 94);
define_perft_test!(test_perft_95, 95);
define_perft_test!(test_perft_96, 96);
define_perft_test!(test_perft_97, 97);
define_perft_test!(test_perft_98, 98);
define_perft_test!(test_perft_99, 99);
define_perft_test!(test_perft_100, 100);
define_perft_test!(test_perft_101, 101);
define_perft_test!(test_perft_102, 102);
define_perft_test!(test_perft_103, 103);
define_perft_test!(test_perft_104, 104);
define_perft_test!(test_perft_105, 105);
define_perft_test!(test_perft_106, 106);
define_perft_test!(test_perft_107, 107);
define_perft_test!(test_perft_108, 108);
define_perft_test!(test_perft_109, 109);
define_perft_test!(test_perft_110, 110);
define_perft_test!(test_perft_111, 111);
define_perft_test!(test_perft_112, 112);
define_perft_test!(test_perft_113, 113);
define_perft_test!(test_perft_114, 114);
define_perft_test!(test_perft_115, 115);
define_perft_test!(test_perft_116, 116);
define_perft_test!(test_perft_117, 117);
define_perft_test!(test_perft_118, 118);
define_perft_test!(test_perft_119, 119);
define_perft_test!(test_perft_120, 120);
define_perft_test!(test_perft_121, 121);
define_perft_test!(test_perft_122, 122);
define_perft_test!(test_perft_123, 123);
define_perft_test!(test_perft_124, 124);
define_perft_test!(test_perft_125, 125);
define_perft_test!(test_perft_126, 126);
define_perft_test!(test_perft_127, 127);

fn test_zobrist(pos: &Position, depth: i32) {
    assert_eq!(pos.hash, pos.compute_hash());

    if depth > 0 {
        let moves = pos.gen_legal_moves();

        for mv in moves {
            let child = pos.make_move(mv);
            test_zobrist(&child, depth - 1);
        }
    }
}

fn test_zobrist_from_position(index: usize) {
    let (fen, _) = FENS[index];
    let pos = Position::from_fen(fen).unwrap();
    test_zobrist(&pos, 5);
}

macro_rules! define_zobrist_test {
    ($name:ident, $index: expr) => {
        #[test]
        fn $name() {
            test_zobrist_from_position($index);
        }
    };
}

define_zobrist_test!(test_zobrist_0, 0);
define_zobrist_test!(test_zobrist_1, 1);
define_zobrist_test!(test_zobrist_2, 2);
define_zobrist_test!(test_zobrist_3, 3);
define_zobrist_test!(test_zobrist_4, 4);
define_zobrist_test!(test_zobrist_5, 5);
define_zobrist_test!(test_zobrist_6, 6);
define_zobrist_test!(test_zobrist_7, 7);
define_zobrist_test!(test_zobrist_8, 8);
define_zobrist_test!(test_zobrist_9, 9);
define_zobrist_test!(test_zobrist_10, 10);
define_zobrist_test!(test_zobrist_11, 11);
define_zobrist_test!(test_zobrist_12, 12);
define_zobrist_test!(test_zobrist_13, 13);
define_zobrist_test!(test_zobrist_14, 14);
define_zobrist_test!(test_zobrist_15, 15);
define_zobrist_test!(test_zobrist_16, 16);
define_zobrist_test!(test_zobrist_17, 17);
define_zobrist_test!(test_zobrist_18, 18);
define_zobrist_test!(test_zobrist_19, 19);
define_zobrist_test!(test_zobrist_20, 20);
define_zobrist_test!(test_zobrist_21, 21);
define_zobrist_test!(test_zobrist_22, 22);
define_zobrist_test!(test_zobrist_23, 23);
define_zobrist_test!(test_zobrist_24, 24);
define_zobrist_test!(test_zobrist_25, 25);
define_zobrist_test!(test_zobrist_26, 26);
define_zobrist_test!(test_zobrist_27, 27);
define_zobrist_test!(test_zobrist_28, 28);
define_zobrist_test!(test_zobrist_29, 29);
define_zobrist_test!(test_zobrist_30, 30);
define_zobrist_test!(test_zobrist_31, 31);
define_zobrist_test!(test_zobrist_32, 32);
define_zobrist_test!(test_zobrist_33, 33);
define_zobrist_test!(test_zobrist_34, 34);
define_zobrist_test!(test_zobrist_35, 35);
define_zobrist_test!(test_zobrist_36, 36);
define_zobrist_test!(test_zobrist_37, 37);
define_zobrist_test!(test_zobrist_38, 38);
define_zobrist_test!(test_zobrist_39, 39);
define_zobrist_test!(test_zobrist_40, 40);
define_zobrist_test!(test_zobrist_41, 41);
define_zobrist_test!(test_zobrist_42, 42);
define_zobrist_test!(test_zobrist_43, 43);
define_zobrist_test!(test_zobrist_44, 44);
define_zobrist_test!(test_zobrist_45, 45);
define_zobrist_test!(test_zobrist_46, 46);
define_zobrist_test!(test_zobrist_47, 47);
define_zobrist_test!(test_zobrist_48, 48);
define_zobrist_test!(test_zobrist_49, 49);
define_zobrist_test!(test_zobrist_50, 50);
define_zobrist_test!(test_zobrist_51, 51);
define_zobrist_test!(test_zobrist_52, 52);
define_zobrist_test!(test_zobrist_53, 53);
define_zobrist_test!(test_zobrist_54, 54);
define_zobrist_test!(test_zobrist_55, 55);
define_zobrist_test!(test_zobrist_56, 56);
define_zobrist_test!(test_zobrist_57, 57);
define_zobrist_test!(test_zobrist_58, 58);
define_zobrist_test!(test_zobrist_59, 59);
define_zobrist_test!(test_zobrist_60, 60);
define_zobrist_test!(test_zobrist_61, 61);
define_zobrist_test!(test_zobrist_62, 62);
define_zobrist_test!(test_zobrist_63, 63);
define_zobrist_test!(test_zobrist_64, 64);
define_zobrist_test!(test_zobrist_65, 65);
define_zobrist_test!(test_zobrist_66, 66);
define_zobrist_test!(test_zobrist_67, 67);
define_zobrist_test!(test_zobrist_68, 68);
define_zobrist_test!(test_zobrist_69, 69);
define_zobrist_test!(test_zobrist_70, 70);
define_zobrist_test!(test_zobrist_71, 71);
define_zobrist_test!(test_zobrist_72, 72);
define_zobrist_test!(test_zobrist_73, 73);
define_zobrist_test!(test_zobrist_74, 74);
define_zobrist_test!(test_zobrist_75, 75);
define_zobrist_test!(test_zobrist_76, 76);
define_zobrist_test!(test_zobrist_77, 77);
define_zobrist_test!(test_zobrist_78, 78);
define_zobrist_test!(test_zobrist_79, 79);
define_zobrist_test!(test_zobrist_80, 80);
define_zobrist_test!(test_zobrist_81, 81);
define_zobrist_test!(test_zobrist_82, 82);
define_zobrist_test!(test_zobrist_83, 83);
define_zobrist_test!(test_zobrist_84, 84);
define_zobrist_test!(test_zobrist_85, 85);
define_zobrist_test!(test_zobrist_86, 86);
define_zobrist_test!(test_zobrist_87, 87);
define_zobrist_test!(test_zobrist_88, 88);
define_zobrist_test!(test_zobrist_89, 89);
define_zobrist_test!(test_zobrist_90, 90);
define_zobrist_test!(test_zobrist_91, 91);
define_zobrist_test!(test_zobrist_92, 92);
define_zobrist_test!(test_zobrist_93, 93);
define_zobrist_test!(test_zobrist_94, 94);
define_zobrist_test!(test_zobrist_95, 95);
define_zobrist_test!(test_zobrist_96, 96);
define_zobrist_test!(test_zobrist_97, 97);
define_zobrist_test!(test_zobrist_98, 98);
define_zobrist_test!(test_zobrist_99, 99);
define_zobrist_test!(test_zobrist_100, 100);
define_zobrist_test!(test_zobrist_101, 101);
define_zobrist_test!(test_zobrist_102, 102);
define_zobrist_test!(test_zobrist_103, 103);
define_zobrist_test!(test_zobrist_104, 104);
define_zobrist_test!(test_zobrist_105, 105);
define_zobrist_test!(test_zobrist_106, 106);
define_zobrist_test!(test_zobrist_107, 107);
define_zobrist_test!(test_zobrist_108, 108);
define_zobrist_test!(test_zobrist_109, 109);
define_zobrist_test!(test_zobrist_110, 110);
define_zobrist_test!(test_zobrist_111, 111);
define_zobrist_test!(test_zobrist_112, 112);
define_zobrist_test!(test_zobrist_113, 113);
define_zobrist_test!(test_zobrist_114, 114);
define_zobrist_test!(test_zobrist_115, 115);
define_zobrist_test!(test_zobrist_116, 116);
define_zobrist_test!(test_zobrist_117, 117);
define_zobrist_test!(test_zobrist_118, 118);
define_zobrist_test!(test_zobrist_119, 119);
define_zobrist_test!(test_zobrist_120, 120);
define_zobrist_test!(test_zobrist_121, 121);
define_zobrist_test!(test_zobrist_122, 122);
define_zobrist_test!(test_zobrist_123, 123);
define_zobrist_test!(test_zobrist_124, 124);
define_zobrist_test!(test_zobrist_125, 125);
define_zobrist_test!(test_zobrist_126, 126);
define_zobrist_test!(test_zobrist_127, 127);

fn test_eval(pos: &Position, depth: i32) {
    assert_eq!(pos.evaluate(), eval::evaluate(pos));

    if depth > 0 {
        let moves = pos.gen_legal_moves();

        for mv in moves {
            let child = pos.make_move(mv);
            test_eval(&child, depth - 1);
        }
    }
}

fn test_eval_from_position(index: usize) {
    let (fen, _) = FENS[index];
    let pos = Position::from_fen(fen).unwrap();
    test_eval(&pos, 5);
}

macro_rules! define_eval_test {
    ($name:ident, $index: expr) => {
        #[test]
        fn $name() {
            test_eval_from_position($index);
        }
    };
}

define_eval_test!(test_eval_0, 0);
define_eval_test!(test_eval_1, 1);
define_eval_test!(test_eval_2, 2);
define_eval_test!(test_eval_3, 3);
define_eval_test!(test_eval_4, 4);
define_eval_test!(test_eval_5, 5);
define_eval_test!(test_eval_6, 6);
define_eval_test!(test_eval_7, 7);
define_eval_test!(test_eval_8, 8);
define_eval_test!(test_eval_9, 9);
define_eval_test!(test_eval_10, 10);
define_eval_test!(test_eval_11, 11);
define_eval_test!(test_eval_12, 12);
define_eval_test!(test_eval_13, 13);
define_eval_test!(test_eval_14, 14);
define_eval_test!(test_eval_15, 15);
define_eval_test!(test_eval_16, 16);
define_eval_test!(test_eval_17, 17);
define_eval_test!(test_eval_18, 18);
define_eval_test!(test_eval_19, 19);
define_eval_test!(test_eval_20, 20);
define_eval_test!(test_eval_21, 21);
define_eval_test!(test_eval_22, 22);
define_eval_test!(test_eval_23, 23);
define_eval_test!(test_eval_24, 24);
define_eval_test!(test_eval_25, 25);
define_eval_test!(test_eval_26, 26);
define_eval_test!(test_eval_27, 27);
define_eval_test!(test_eval_28, 28);
define_eval_test!(test_eval_29, 29);
define_eval_test!(test_eval_30, 30);
define_eval_test!(test_eval_31, 31);
define_eval_test!(test_eval_32, 32);
define_eval_test!(test_eval_33, 33);
define_eval_test!(test_eval_34, 34);
define_eval_test!(test_eval_35, 35);
define_eval_test!(test_eval_36, 36);
define_eval_test!(test_eval_37, 37);
define_eval_test!(test_eval_38, 38);
define_eval_test!(test_eval_39, 39);
define_eval_test!(test_eval_40, 40);
define_eval_test!(test_eval_41, 41);
define_eval_test!(test_eval_42, 42);
define_eval_test!(test_eval_43, 43);
define_eval_test!(test_eval_44, 44);
define_eval_test!(test_eval_45, 45);
define_eval_test!(test_eval_46, 46);
define_eval_test!(test_eval_47, 47);
define_eval_test!(test_eval_48, 48);
define_eval_test!(test_eval_49, 49);
define_eval_test!(test_eval_50, 50);
define_eval_test!(test_eval_51, 51);
define_eval_test!(test_eval_52, 52);
define_eval_test!(test_eval_53, 53);
define_eval_test!(test_eval_54, 54);
define_eval_test!(test_eval_55, 55);
define_eval_test!(test_eval_56, 56);
define_eval_test!(test_eval_57, 57);
define_eval_test!(test_eval_58, 58);
define_eval_test!(test_eval_59, 59);
define_eval_test!(test_eval_60, 60);
define_eval_test!(test_eval_61, 61);
define_eval_test!(test_eval_62, 62);
define_eval_test!(test_eval_63, 63);
define_eval_test!(test_eval_64, 64);
define_eval_test!(test_eval_65, 65);
define_eval_test!(test_eval_66, 66);
define_eval_test!(test_eval_67, 67);
define_eval_test!(test_eval_68, 68);
define_eval_test!(test_eval_69, 69);
define_eval_test!(test_eval_70, 70);
define_eval_test!(test_eval_71, 71);
define_eval_test!(test_eval_72, 72);
define_eval_test!(test_eval_73, 73);
define_eval_test!(test_eval_74, 74);
define_eval_test!(test_eval_75, 75);
define_eval_test!(test_eval_76, 76);
define_eval_test!(test_eval_77, 77);
define_eval_test!(test_eval_78, 78);
define_eval_test!(test_eval_79, 79);
define_eval_test!(test_eval_80, 80);
define_eval_test!(test_eval_81, 81);
define_eval_test!(test_eval_82, 82);
define_eval_test!(test_eval_83, 83);
define_eval_test!(test_eval_84, 84);
define_eval_test!(test_eval_85, 85);
define_eval_test!(test_eval_86, 86);
define_eval_test!(test_eval_87, 87);
define_eval_test!(test_eval_88, 88);
define_eval_test!(test_eval_89, 89);
define_eval_test!(test_eval_90, 90);
define_eval_test!(test_eval_91, 91);
define_eval_test!(test_eval_92, 92);
define_eval_test!(test_eval_93, 93);
define_eval_test!(test_eval_94, 94);
define_eval_test!(test_eval_95, 95);
define_eval_test!(test_eval_96, 96);
define_eval_test!(test_eval_97, 97);
define_eval_test!(test_eval_98, 98);
define_eval_test!(test_eval_99, 99);
define_eval_test!(test_eval_100, 100);
define_eval_test!(test_eval_101, 101);
define_eval_test!(test_eval_102, 102);
define_eval_test!(test_eval_103, 103);
define_eval_test!(test_eval_104, 104);
define_eval_test!(test_eval_105, 105);
define_eval_test!(test_eval_106, 106);
define_eval_test!(test_eval_107, 107);
define_eval_test!(test_eval_108, 108);
define_eval_test!(test_eval_109, 109);
define_eval_test!(test_eval_110, 110);
define_eval_test!(test_eval_111, 111);
define_eval_test!(test_eval_112, 112);
define_eval_test!(test_eval_113, 113);
define_eval_test!(test_eval_114, 114);
define_eval_test!(test_eval_115, 115);
define_eval_test!(test_eval_116, 116);
define_eval_test!(test_eval_117, 117);
define_eval_test!(test_eval_118, 118);
define_eval_test!(test_eval_119, 119);
define_eval_test!(test_eval_120, 120);
define_eval_test!(test_eval_121, 121);
define_eval_test!(test_eval_122, 122);
define_eval_test!(test_eval_123, 123);
define_eval_test!(test_eval_124, 124);
define_eval_test!(test_eval_125, 125);
define_eval_test!(test_eval_126, 126);
define_eval_test!(test_eval_127, 127);