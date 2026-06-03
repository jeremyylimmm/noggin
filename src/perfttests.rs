use crate::*;

#[test]
fn test_make_and_unmake_move() {
    let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
    let baseline = pos.clone();

    let moves = movegen::gen_pseudolegal_moves(&pos);

    for i in 0..moves.len() {
        let mv = moves[i];

        pos.make_move(mv);
        pos.unmake_move();

        assert_eq!(pos, baseline);
    }
}

#[test]
fn test_kiwipete_perft_depth_1() {
    let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
    assert_eq!(pos.perft(1), 48);
}

#[test]
fn test_kiwipete_perft_depth_2() {
    let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
    assert_eq!(pos.perft(2), 2039);
}

#[test]
fn test_kiwipete_perft_depth_3() {
    let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
    assert_eq!(pos.perft(3), 97862);
}

#[test]
fn test_kiwipete_perft_depth_4() {
    let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
    assert_eq!(pos.perft(4), 4085603);
}

#[test]
fn test_kiwipete_perft_depth_5() {
    let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
    assert_eq!(pos.perft(5), 193690690);
}

fn get_perft_suite() -> [(&'static str, Vec<Option<usize>>); 128] {
    [
        ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![Some(20), Some(400), Some(8902), Some(197281), Some(4865609), Some(119060324)]),
        ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", vec![Some(48), Some(2039), Some(97862), Some(4085603), Some(193690690)]),
        ("4k3/8/8/8/8/8/8/4K2R w K - 0 1", vec![Some(15), Some(66), Some(1197), Some(7059), Some(133987), Some(764643)]),
        ("4k3/8/8/8/8/8/8/R3K3 w Q - 0 1", vec![Some(16), Some(71), Some(1287), Some(7626), Some(145232), Some(846648)]),
        ("4k2r/8/8/8/8/8/8/4K3 w k - 0 1", vec![Some(5), Some(75), Some(459), Some(8290), Some(47635), Some(899442)]),
        ("r3k3/8/8/8/8/8/8/4K3 w q - 0 1", vec![Some(5), Some(80), Some(493), Some(8897), Some(52710), Some(1001523)]),
        ("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1", vec![Some(26), Some(112), Some(3189), Some(17945), Some(532933), Some(2788982)]),
        ("r3k2r/8/8/8/8/8/8/4K3 w kq - 0 1", vec![Some(5), Some(130), Some(782), Some(22180), Some(118882), Some(3517770)]),
        ("8/8/8/8/8/8/6k1/4K2R w K - 0 1", vec![Some(12), Some(38), Some(564), Some(2219), Some(37735), Some(185867)]),
        ("8/8/8/8/8/8/1k6/R3K3 w Q - 0 1", vec![Some(15), Some(65), Some(1018), Some(4573), Some(80619), Some(413018)]),
        ("4k2r/6K1/8/8/8/8/8/8 w k - 0 1", vec![Some(3), Some(32), Some(134), Some(2073), Some(10485), Some(179869)]),
        ("r3k3/1K6/8/8/8/8/8/8 w q - 0 1", vec![Some(4), Some(49), Some(243), Some(3991), Some(20780), Some(367724)]),
        ("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", vec![Some(26), Some(568), Some(13744), Some(314346), Some(7594526), Some(179862938)]),
        ("r3k2r/8/8/8/8/8/8/1R2K2R w Kkq - 0 1", vec![Some(25), Some(567), Some(14095), Some(328965), Some(8153719), Some(195629489)]),
        ("r3k2r/8/8/8/8/8/8/2R1K2R w Kkq - 0 1", vec![Some(25), Some(548), Some(13502), Some(312835), Some(7736373), Some(184411439)]),
        ("r3k2r/8/8/8/8/8/8/R3K1R1 w Qkq - 0 1", vec![Some(25), Some(547), Some(13579), Some(316214), Some(7878456), Some(189224276)]),
        ("1r2k2r/8/8/8/8/8/8/R3K2R w KQk - 0 1", vec![Some(26), Some(583), Some(14252), Some(334705), Some(8198901), Some(198328929)]),
        ("2r1k2r/8/8/8/8/8/8/R3K2R w KQk - 0 1", vec![Some(25), Some(560), Some(13592), Some(317324), Some(7710115), Some(185959088)]),
        ("r3k1r1/8/8/8/8/8/8/R3K2R w KQq - 0 1", vec![Some(25), Some(560), Some(13607), Some(320792), Some(7848606), Some(190755813)]),
        ("4k3/8/8/8/8/8/8/4K2R b K - 0 1", vec![Some(5), Some(75), Some(459), Some(8290), Some(47635), Some(899442)]),
        ("4k3/8/8/8/8/8/8/R3K3 b Q - 0 1", vec![Some(5), Some(80), Some(493), Some(8897), Some(52710), Some(1001523)]),
        ("4k2r/8/8/8/8/8/8/4K3 b k - 0 1", vec![Some(15), Some(66), Some(1197), Some(7059), Some(133987), Some(764643)]),
        ("r3k3/8/8/8/8/8/8/4K3 b q - 0 1", vec![Some(16), Some(71), Some(1287), Some(7626), Some(145232), Some(846648)]),
        ("4k3/8/8/8/8/8/8/R3K2R b KQ - 0 1", vec![Some(5), Some(130), Some(782), Some(22180), Some(118882), Some(3517770)]),
        ("r3k2r/8/8/8/8/8/8/4K3 b kq - 0 1", vec![Some(26), Some(112), Some(3189), Some(17945), Some(532933), Some(2788982)]),
        ("8/8/8/8/8/8/6k1/4K2R b K - 0 1", vec![Some(3), Some(32), Some(134), Some(2073), Some(10485), Some(179869)]),
        ("8/8/8/8/8/8/1k6/R3K3 b Q - 0 1", vec![Some(4), Some(49), Some(243), Some(3991), Some(20780), Some(367724)]),
        ("4k2r/6K1/8/8/8/8/8/8 b k - 0 1", vec![Some(12), Some(38), Some(564), Some(2219), Some(37735), Some(185867)]),
        ("r3k3/1K6/8/8/8/8/8/8 b q - 0 1", vec![Some(15), Some(65), Some(1018), Some(4573), Some(80619), Some(413018)]),
        ("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1", vec![Some(26), Some(568), Some(13744), Some(314346), Some(7594526), Some(179862938)]),
        ("r3k2r/8/8/8/8/8/8/1R2K2R b Kkq - 0 1", vec![Some(26), Some(583), Some(14252), Some(334705), Some(8198901), Some(198328929)]),
        ("r3k2r/8/8/8/8/8/8/2R1K2R b Kkq - 0 1", vec![Some(25), Some(560), Some(13592), Some(317324), Some(7710115), Some(185959088)]),
        ("r3k2r/8/8/8/8/8/8/R3K1R1 b Qkq - 0 1", vec![Some(25), Some(560), Some(13607), Some(320792), Some(7848606), Some(190755813)]),
        ("1r2k2r/8/8/8/8/8/8/R3K2R b KQk - 0 1", vec![Some(25), Some(567), Some(14095), Some(328965), Some(8153719), Some(195629489)]),
        ("2r1k2r/8/8/8/8/8/8/R3K2R b KQk - 0 1", vec![Some(25), Some(548), Some(13502), Some(312835), Some(7736373), Some(184411439)]),
        ("r3k1r1/8/8/8/8/8/8/R3K2R b KQq - 0 1", vec![Some(25), Some(547), Some(13579), Some(316214), Some(7878456), Some(189224276)]),
        ("8/1n4N1/2k5/8/8/5K2/1N4n1/8 w - - 0 1", vec![Some(14), Some(195), Some(2760), Some(38675), Some(570726), Some(8107539)]),
        ("8/1k6/8/5N2/8/4n3/8/2K5 w - - 0 1", vec![Some(11), Some(156), Some(1636), Some(20534), Some(223507), Some(2594412)]),
        ("8/8/4k3/3Nn3/3nN3/4K3/8/8 w - - 0 1", vec![Some(19), Some(289), Some(4442), Some(73584), Some(1198299), Some(19870403)]),
        ("K7/8/2n5/1n6/8/8/8/k6N w - - 0 1", vec![Some(3), Some(51), Some(345), Some(5301), Some(38348), Some(588695)]),
        ("k7/8/2N5/1N6/8/8/8/K6n w - - 0 1", vec![Some(17), Some(54), Some(835), Some(5910), Some(92250), Some(688780)]),
        ("8/1n4N1/2k5/8/8/5K2/1N4n1/8 b - - 0 1", vec![Some(15), Some(193), Some(2816), Some(40039), Some(582642), Some(8503277)]),
        ("8/1k6/8/5N2/8/4n3/8/2K5 b - - 0 1", vec![Some(16), Some(180), Some(2290), Some(24640), Some(288141), Some(3147566)]),
        ("8/8/3K4/3Nn3/3nN3/4k3/8/8 b - - 0 1", vec![Some(4), Some(68), Some(1118), Some(16199), Some(281190), Some(4405103)]),
        ("K7/8/2n5/1n6/8/8/8/k6N b - - 0 1", vec![Some(17), Some(54), Some(835), Some(5910), Some(92250), Some(688780)]),
        ("k7/8/2N5/1N6/8/8/8/K6n b - - 0 1", vec![Some(3), Some(51), Some(345), Some(5301), Some(38348), Some(588695)]),
        ("B6b/8/8/8/2K5/4k3/8/b6B w - - 0 1", vec![Some(17), Some(278), Some(4607), Some(76778), Some(1320507), Some(22823890)]),
        ("8/8/1B6/7b/7k/8/2B1b3/7K w - - 0 1", vec![Some(21), Some(316), Some(5744), Some(93338), Some(1713368), Some(28861171)]),
        ("k7/B7/1B6/1B6/8/8/8/K6b w - - 0 1", vec![Some(21), Some(144), Some(3242), Some(32955), Some(787524), Some(7881673)]),
        ("K7/b7/1b6/1b6/8/8/8/k6B w - - 0 1", vec![Some(7), Some(143), Some(1416), Some(31787), Some(310862), Some(7382896)]),
        ("B6b/8/8/8/2K5/5k2/8/b6B b - - 0 1", vec![Some(6), Some(106), Some(1829), Some(31151), Some(530585), Some(9250746)]),
        ("8/8/1B6/7b/7k/8/2B1b3/7K b - - 0 1", vec![Some(17), Some(309), Some(5133), Some(93603), Some(1591064), Some(29027891)]),
        ("k7/B7/1B6/1B6/8/8/8/K6b b - - 0 1", vec![Some(7), Some(143), Some(1416), Some(31787), Some(310862), Some(7382896)]),
        ("K7/b7/1b6/1b6/8/8/8/k6B b - - 0 1", vec![Some(21), Some(144), Some(3242), Some(32955), Some(787524), Some(7881673)]),
        ("7k/RR6/8/8/8/8/rr6/7K w - - 0 1", vec![Some(19), Some(275), Some(5300), Some(104342), Some(2161211), Some(44956585)]),
        ("R6r/8/8/2K5/5k2/8/8/r6R w - - 0 1", vec![Some(36), Some(1027), Some(29215), Some(771461), Some(20506480), Some(525169084)]),
        ("7k/RR6/8/8/8/8/rr6/7K b - - 0 1", vec![Some(19), Some(275), Some(5300), Some(104342), Some(2161211), Some(44956585)]),
        ("R6r/8/8/2K5/5k2/8/8/r6R b - - 0 1", vec![Some(36), Some(1027), Some(29227), Some(771368), Some(20521342), Some(524966748)]),
        ("6kq/8/8/8/8/8/8/7K w - - 0 1", vec![Some(2), Some(36), Some(143), Some(3637), Some(14893), Some(391507)]),
        ("6KQ/8/8/8/8/8/8/7k b - - 0 1", vec![Some(2), Some(36), Some(143), Some(3637), Some(14893), Some(391507)]),
        ("K7/8/8/3Q4/4q3/8/8/7k w - - 0 1", vec![Some(6), Some(35), Some(495), Some(8349), Some(166741), Some(3370175)]),
        ("6qk/8/8/8/8/8/8/7K b - - 0 1", vec![Some(22), Some(43), Some(1015), Some(4167), Some(105749), Some(419369)]),
        ("6KQ/8/8/8/8/8/8/7k b - - 0 1", vec![Some(2), Some(36), Some(143), Some(3637), Some(14893), Some(391507)]),
        ("K7/8/8/3Q4/4q3/8/8/7k b - - 0 1", vec![Some(6), Some(35), Some(495), Some(8349), Some(166741), Some(3370175)]),
        ("8/8/8/8/8/K7/P7/k7 w - - 0 1", vec![Some(3), Some(7), Some(43), Some(199), Some(1347), Some(6249)]),
        ("8/8/8/8/8/7K/7P/7k w - - 0 1", vec![Some(3), Some(7), Some(43), Some(199), Some(1347), Some(6249)]),
        ("K7/p7/k7/8/8/8/8/8 w - - 0 1", vec![Some(1), Some(3), Some(12), Some(80), Some(342), Some(2343)]),
        ("7K/7p/7k/8/8/8/8/8 w - - 0 1", vec![Some(1), Some(3), Some(12), Some(80), Some(342), Some(2343)]),
        ("8/2k1p3/3pP3/3P2K1/8/8/8/8 w - - 0 1", vec![Some(7), Some(35), Some(210), Some(1091), Some(7028), Some(34834)]),
        ("8/8/8/8/8/K7/P7/k7 b - - 0 1", vec![Some(1), Some(3), Some(12), Some(80), Some(342), Some(2343)]),
        ("8/8/8/8/8/7K/7P/7k b - - 0 1", vec![Some(1), Some(3), Some(12), Some(80), Some(342), Some(2343)]),
        ("K7/p7/k7/8/8/8/8/8 b - - 0 1", vec![Some(3), Some(7), Some(43), Some(199), Some(1347), Some(6249)]),
        ("7K/7p/7k/8/8/8/8/8 b - - 0 1", vec![Some(3), Some(7), Some(43), Some(199), Some(1347), Some(6249)]),
        ("8/2k1p3/3pP3/3P2K1/8/8/8/8 b - - 0 1", vec![Some(5), Some(35), Some(182), Some(1091), Some(5408), Some(34822)]),
        ("8/8/8/8/8/4k3/4P3/4K3 w - - 0 1", vec![Some(2), Some(8), Some(44), Some(282), Some(1814), Some(11848)]),
        ("4k3/4p3/4K3/8/8/8/8/8 b - - 0 1", vec![Some(2), Some(8), Some(44), Some(282), Some(1814), Some(11848)]),
        ("8/8/7k/7p/7P/7K/8/8 w - - 0 1", vec![Some(3), Some(9), Some(57), Some(360), Some(1969), Some(10724)]),
        ("8/8/k7/p7/P7/K7/8/8 w - - 0 1", vec![Some(3), Some(9), Some(57), Some(360), Some(1969), Some(10724)]),
        ("8/8/3k4/3p4/3P4/3K4/8/8 w - - 0 1", vec![Some(5), Some(25), Some(180), Some(1294), Some(8296), Some(53138)]),
        ("8/3k4/3p4/8/3P4/3K4/8/8 w - - 0 1", vec![Some(8), Some(61), Some(483), Some(3213), Some(23599), Some(157093)]),
        ("8/8/3k4/3p4/8/3P4/3K4/8 w - - 0 1", vec![Some(8), Some(61), Some(411), Some(3213), Some(21637), Some(158065)]),
        ("k7/8/3p4/8/3P4/8/8/7K w - - 0 1", vec![Some(4), Some(15), Some(90), Some(534), Some(3450), Some(20960)]),
        ("8/8/7k/7p/7P/7K/8/8 b - - 0 1", vec![Some(3), Some(9), Some(57), Some(360), Some(1969), Some(10724)]),
        ("8/8/k7/p7/P7/K7/8/8 b - - 0 1", vec![Some(3), Some(9), Some(57), Some(360), Some(1969), Some(10724)]),
        ("8/8/3k4/3p4/3P4/3K4/8/8 b - - 0 1", vec![Some(5), Some(25), Some(180), Some(1294), Some(8296), Some(53138)]),
        ("8/3k4/3p4/8/3P4/3K4/8/8 b - - 0 1", vec![Some(8), Some(61), Some(411), Some(3213), Some(21637), Some(158065)]),
        ("8/8/3k4/3p4/8/3P4/3K4/8 b - - 0 1", vec![Some(8), Some(61), Some(483), Some(3213), Some(23599), Some(157093)]),
        ("k7/8/3p4/8/3P4/8/8/7K b - - 0 1", vec![Some(4), Some(15), Some(89), Some(537), Some(3309), Some(21104)]),
        ("7k/3p4/8/8/3P4/8/8/K7 w - - 0 1", vec![Some(4), Some(19), Some(117), Some(720), Some(4661), Some(32191)]),
        ("7k/8/8/3p4/8/8/3P4/K7 w - - 0 1", vec![Some(5), Some(19), Some(116), Some(716), Some(4786), Some(30980)]),
        ("k7/8/8/7p/6P1/8/8/K7 w - - 0 1", vec![Some(5), Some(22), Some(139), Some(877), Some(6112), Some(41874)]),
        ("k7/8/7p/8/8/6P1/8/K7 w - - 0 1", vec![Some(4), Some(16), Some(101), Some(637), Some(4354), Some(29679)]),
        ("k7/8/8/6p1/7P/8/8/K7 w - - 0 1", vec![Some(5), Some(22), Some(139), Some(877), Some(6112), Some(41874)]),
        ("k7/8/6p1/8/8/7P/8/K7 w - - 0 1", vec![Some(4), Some(16), Some(101), Some(637), Some(4354), Some(29679)]),
        ("k7/8/8/3p4/4p3/8/8/7K w - - 0 1", vec![Some(3), Some(15), Some(84), Some(573), Some(3013), Some(22886)]),
        ("k7/8/3p4/8/8/4P3/8/7K w - - 0 1", vec![Some(4), Some(16), Some(101), Some(637), Some(4271), Some(28662)]),
        ("7k/3p4/8/8/3P4/8/8/K7 b - - 0 1", vec![Some(5), Some(19), Some(117), Some(720), Some(5014), Some(32167)]),
        ("7k/8/8/3p4/8/8/3P4/K7 b - - 0 1", vec![Some(4), Some(19), Some(117), Some(712), Some(4658), Some(30749)]),
        ("k7/8/8/7p/6P1/8/8/K7 b - - 0 1", vec![Some(5), Some(22), Some(139), Some(877), Some(6112), Some(41874)]),
        ("k7/8/7p/8/8/6P1/8/K7 b - - 0 1", vec![Some(4), Some(16), Some(101), Some(637), Some(4354), Some(29679)]),
        ("k7/8/8/6p1/7P/8/8/K7 b - - 0 1", vec![Some(5), Some(22), Some(139), Some(877), Some(6112), Some(41874)]),
        ("k7/8/6p1/8/8/7P/8/K7 b - - 0 1", vec![Some(4), Some(16), Some(101), Some(637), Some(4354), Some(29679)]),
        ("k7/8/8/3p4/4p3/8/8/7K b - - 0 1", vec![Some(5), Some(15), Some(102), Some(569), Some(4337), Some(22579)]),
        ("k7/8/3p4/8/8/4P3/8/7K b - - 0 1", vec![Some(4), Some(16), Some(101), Some(637), Some(4271), Some(28662)]),
        ("7k/8/8/p7/1P6/8/8/7K w - - 0 1", vec![Some(5), Some(22), Some(139), Some(877), Some(6112), Some(41874)]),
        ("7k/8/p7/8/8/1P6/8/7K w - - 0 1", vec![Some(4), Some(16), Some(101), Some(637), Some(4354), Some(29679)]),
        ("7k/8/8/1p6/P7/8/8/7K w - - 0 1", vec![Some(5), Some(22), Some(139), Some(877), Some(6112), Some(41874)]),
        ("7k/8/1p6/8/8/P7/8/7K w - - 0 1", vec![Some(4), Some(16), Some(101), Some(637), Some(4354), Some(29679)]),
        ("k7/7p/8/8/8/8/6P1/K7 w - - 0 1", vec![Some(5), Some(25), Some(161), Some(1035), Some(7574), Some(55338)]),
        ("k7/6p1/8/8/8/8/7P/K7 w - - 0 1", vec![Some(5), Some(25), Some(161), Some(1035), Some(7574), Some(55338)]),
        ("3k4/3pp3/8/8/8/8/3PP3/3K4 w - - 0 1", vec![Some(7), Some(49), Some(378), Some(2902), Some(24122), Some(199002)]),
        ("7k/8/8/p7/1P6/8/8/7K b - - 0 1", vec![Some(5), Some(22), Some(139), Some(877), Some(6112), Some(41874)]),
        ("7k/8/p7/8/8/1P6/8/7K b - - 0 1", vec![Some(4), Some(16), Some(101), Some(637), Some(4354), Some(29679)]),
        ("7k/8/8/1p6/P7/8/8/7K b - - 0 1", vec![Some(5), Some(22), Some(139), Some(877), Some(6112), Some(41874)]),
        ("7k/8/1p6/8/8/P7/8/7K b - - 0 1", vec![Some(4), Some(16), Some(101), Some(637), Some(4354), Some(29679)]),
        ("k7/7p/8/8/8/8/6P1/K7 b - - 0 1", vec![Some(5), Some(25), Some(161), Some(1035), Some(7574), Some(55338)]),
        ("k7/6p1/8/8/8/8/7P/K7 b - - 0 1", vec![Some(5), Some(25), Some(161), Some(1035), Some(7574), Some(55338)]),
        ("3k4/3pp3/8/8/8/8/3PP3/3K4 b - - 0 1", vec![Some(7), Some(49), Some(378), Some(2902), Some(24122), Some(199002)]),
        ("8/Pk6/8/8/8/8/6Kp/8 w - - 0 1", vec![Some(11), Some(97), Some(887), Some(8048), Some(90606), Some(1030499)]),
        ("n1n5/1Pk5/8/8/8/8/5Kp1/5N1N w - - 0 1", vec![Some(24), Some(421), Some(7421), Some(124608), Some(2193768), Some(37665329)]),
        ("8/PPPk4/8/8/8/8/4Kppp/8 w - - 0 1", vec![Some(18), Some(270), Some(4699), Some(79355), Some(1533145), Some(28859283)]),
        ("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N w - - 0 1", vec![Some(24), Some(496), Some(9483), Some(182838), Some(3605103), Some(71179139)]),
        ("8/Pk6/8/8/8/8/6Kp/8 b - - 0 1", vec![Some(11), Some(97), Some(887), Some(8048), Some(90606), Some(1030499)]),
        ("n1n5/1Pk5/8/8/8/8/5Kp1/5N1N b - - 0 1", vec![Some(24), Some(421), Some(7421), Some(124608), Some(2193768), Some(37665329)]),
        ("8/PPPk4/8/8/8/8/4Kppp/8 b - - 0 1", vec![Some(18), Some(270), Some(4699), Some(79355), Some(1533145), Some(28859283)]),
        ("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1", vec![Some(24), Some(496), Some(9483), Some(182838), Some(3605103), Some(71179139)]),
        ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", vec![None, None, None, Some(43238), Some(674624), Some(11030083)]),
        ("rnbqkb1r/ppppp1pp/7n/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3", vec![None, None, None, None, Some(11139762)]),
    ]
}

macro_rules! make_test {
    ($name:ident, $index:expr) => {
        #[test]
        fn $name() {
            let (fen, counts) = &get_perft_suite()[$index];

            let mut pos = Position::from_fen(fen).unwrap();

            for (d, &nopt) in counts.iter().enumerate() {

                if let Some(n) = nopt {
                    assert_eq!(pos.perft((d+1) as isize), n);
                }
            }
        }
    };
}

make_test!(test_perft_suite_0, 0);
make_test!(test_perft_suite_1, 1);
make_test!(test_perft_suite_2, 2);
make_test!(test_perft_suite_3, 3);
make_test!(test_perft_suite_4, 4);
make_test!(test_perft_suite_5, 5);
make_test!(test_perft_suite_6, 6);
make_test!(test_perft_suite_7, 7);
make_test!(test_perft_suite_8, 8);
make_test!(test_perft_suite_9, 9);
make_test!(test_perft_suite_10, 10);
make_test!(test_perft_suite_11, 11);
make_test!(test_perft_suite_12, 12);
make_test!(test_perft_suite_13, 13);
make_test!(test_perft_suite_14, 14);
make_test!(test_perft_suite_15, 15);
make_test!(test_perft_suite_16, 16);
make_test!(test_perft_suite_17, 17);
make_test!(test_perft_suite_18, 18);
make_test!(test_perft_suite_19, 19);
make_test!(test_perft_suite_20, 20);
make_test!(test_perft_suite_21, 21);
make_test!(test_perft_suite_22, 22);
make_test!(test_perft_suite_23, 23);
make_test!(test_perft_suite_24, 24);
make_test!(test_perft_suite_25, 25);
make_test!(test_perft_suite_26, 26);
make_test!(test_perft_suite_27, 27);
make_test!(test_perft_suite_28, 28);
make_test!(test_perft_suite_29, 29);
make_test!(test_perft_suite_30, 30);
make_test!(test_perft_suite_31, 31);
make_test!(test_perft_suite_32, 32);
make_test!(test_perft_suite_33, 33);
make_test!(test_perft_suite_34, 34);
make_test!(test_perft_suite_35, 35);
make_test!(test_perft_suite_36, 36);
make_test!(test_perft_suite_37, 37);
make_test!(test_perft_suite_38, 38);
make_test!(test_perft_suite_39, 39);
make_test!(test_perft_suite_40, 40);
make_test!(test_perft_suite_41, 41);
make_test!(test_perft_suite_42, 42);
make_test!(test_perft_suite_43, 43);
make_test!(test_perft_suite_44, 44);
make_test!(test_perft_suite_45, 45);
make_test!(test_perft_suite_46, 46);
make_test!(test_perft_suite_47, 47);
make_test!(test_perft_suite_48, 48);
make_test!(test_perft_suite_49, 49);
make_test!(test_perft_suite_50, 50);
make_test!(test_perft_suite_51, 51);
make_test!(test_perft_suite_52, 52);
make_test!(test_perft_suite_53, 53);
make_test!(test_perft_suite_54, 54);
make_test!(test_perft_suite_55, 55);
make_test!(test_perft_suite_56, 56);
make_test!(test_perft_suite_57, 57);
make_test!(test_perft_suite_58, 58);
make_test!(test_perft_suite_59, 59);
make_test!(test_perft_suite_60, 60);
make_test!(test_perft_suite_61, 61);
make_test!(test_perft_suite_62, 62);
make_test!(test_perft_suite_63, 63);
make_test!(test_perft_suite_64, 64);
make_test!(test_perft_suite_65, 65);
make_test!(test_perft_suite_66, 66);
make_test!(test_perft_suite_67, 67);
make_test!(test_perft_suite_68, 68);
make_test!(test_perft_suite_69, 69);
make_test!(test_perft_suite_70, 70);
make_test!(test_perft_suite_71, 71);
make_test!(test_perft_suite_72, 72);
make_test!(test_perft_suite_73, 73);
make_test!(test_perft_suite_74, 74);
make_test!(test_perft_suite_75, 75);
make_test!(test_perft_suite_76, 76);
make_test!(test_perft_suite_77, 77);
make_test!(test_perft_suite_78, 78);
make_test!(test_perft_suite_79, 79);
make_test!(test_perft_suite_80, 80);
make_test!(test_perft_suite_81, 81);
make_test!(test_perft_suite_82, 82);
make_test!(test_perft_suite_83, 83);
make_test!(test_perft_suite_84, 84);
make_test!(test_perft_suite_85, 85);
make_test!(test_perft_suite_86, 86);
make_test!(test_perft_suite_87, 87);
make_test!(test_perft_suite_88, 88);
make_test!(test_perft_suite_89, 89);
make_test!(test_perft_suite_90, 90);
make_test!(test_perft_suite_91, 91);
make_test!(test_perft_suite_92, 92);
make_test!(test_perft_suite_93, 93);
make_test!(test_perft_suite_94, 94);
make_test!(test_perft_suite_95, 95);
make_test!(test_perft_suite_96, 96);
make_test!(test_perft_suite_97, 97);
make_test!(test_perft_suite_98, 98);
make_test!(test_perft_suite_99, 99);
make_test!(test_perft_suite_100, 100);
make_test!(test_perft_suite_101, 101);
make_test!(test_perft_suite_102, 102);
make_test!(test_perft_suite_103, 103);
make_test!(test_perft_suite_104, 104);
make_test!(test_perft_suite_105, 105);
make_test!(test_perft_suite_106, 106);
make_test!(test_perft_suite_107, 107);
make_test!(test_perft_suite_108, 108);
make_test!(test_perft_suite_109, 109);
make_test!(test_perft_suite_110, 110);
make_test!(test_perft_suite_111, 111);
make_test!(test_perft_suite_112, 112);
make_test!(test_perft_suite_113, 113);
make_test!(test_perft_suite_114, 114);
make_test!(test_perft_suite_115, 115);
make_test!(test_perft_suite_116, 116);
make_test!(test_perft_suite_117, 117);
make_test!(test_perft_suite_118, 118);
make_test!(test_perft_suite_119, 119);
make_test!(test_perft_suite_120, 120);
make_test!(test_perft_suite_121, 121);
make_test!(test_perft_suite_122, 122);
make_test!(test_perft_suite_123, 123);
make_test!(test_perft_suite_124, 124);
make_test!(test_perft_suite_125, 125);
make_test!(test_perft_suite_126, 126);
make_test!(test_perft_suite_127, 127);