use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 145
#[rustfmt::skip]
pub static SKIABSEVAL: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,7),
            PTR(5, false, false),
        ], 
        vec![ // 1 
            COM(2,183),
            INT(5),
        ], 
        vec![ // 2 
            COM(2,183),
            INT(1),
        ], 
        vec![ // 3 
            COM(1,214),
            PTR(6, false, false),
        ], 
        vec![ // 4 
            COM(3,21),
            PTR(3, false, false),
            PTR(2, false, false),
        ], 
        vec![ // 5 
            COM(3,21),
            PTR(4, false, false),
            PTR(1, false, false),
        ], 
        // AExp1
        vec![ // 6 
            COM(3,21),
            COM(1,201),
            PTR(27, false, false),
        ], 
        vec![ // 7 
            COM(2,295),
            INT(2),
        ], 
        vec![ // 8 
            COM(2,183),
            INT(1),
        ], 
        vec![ // 9 
            COM(2,295),
            INT(1),
        ], 
        vec![ // 10 
            COM(3,21),
            COM(1,297),
            PTR(9, false, false),
        ], 
        vec![ // 11 
            COM(3,21),
            PTR(10, false, false),
            PTR(8, false, false),
        ], 
        vec![ // 12 
            COM(2,295),
            INT(0),
        ], 
        vec![ // 13 
            COM(3,21),
            PTR(12, false, false),
            PTR(11, false, false),
        ], 
        vec![ // 14 
            COM(3,21),
            PTR(13, false, false),
            PTR(7, false, false),
        ], 
        vec![ // 15 
            COM(2,295),
            INT(1),
        ], 
        vec![ // 16 
            COM(3,21),
            COM(1,297),
            PTR(15, false, false),
        ], 
        vec![ // 17 
            COM(3,21),
            PTR(16, false, false),
            PTR(14, false, false),
        ], 
        vec![ // 18 
            COM(2,295),
            INT(1),
        ], 
        vec![ // 19 
            COM(2,295),
            INT(2),
        ], 
        vec![ // 20 
            COM(2,295),
            INT(1),
        ], 
        vec![ // 21 
            COM(3,21),
            COM(1,294),
            PTR(20, false, false),
        ], 
        vec![ // 22 
            COM(3,21),
            PTR(21, false, false),
            PTR(19, false, false),
        ], 
        vec![ // 23 
            COM(3,21),
            PTR(22, false, false),
            PTR(18, false, false),
        ], 
        vec![ // 24 
            COM(3,21),
            PTR(23, false, false),
            PTR(17, false, false),
        ], 
        vec![ // 25 
            COM(3,292),
            INT(2),
            PTR(24, false, false),
        ], 
        vec![ // 26 
            COM(3,292),
            INT(1),
            PTR(25, false, false),
        ], 
        vec![ // 27 
            COM(3,292),
            INT(0),
            PTR(26, false, false),
        ], 
        // AExp2
        vec![ // 28 
            COM(3,9),
            COM(1,11),
            PTR(29, false, false),
        ], 
        vec![ // 29 
            COM(3,9),
            COM(1,174),
            COM(1,200),
        ], 
    ],
    comb_img: vec![
        // AExp0
        vec![ // 0 
            ARG(0, true),
        ], 
        // AExp1
        vec![ // 1 
            ARG(1, true),
        ], 
        // AExp2
        vec![ // 2 
            COM(1,7),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp3
        vec![ // 4 
            PRM(EQ,false),
            ARG(1, true),
            INT(10),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            ARG(2, true),
            COM(1,0),
        ], 
        vec![ // 6 
            COM(1,2),
            ARG(0, true),
        ], 
        // AExp4
        vec![ // 7 
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            COM(3,4),
            ARG(0, false),
        ], 
        // AExp5
        vec![ // 9 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp6
        vec![ // 11 
            ARG(0, true),
            COM(1,13),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            COM(1,18),
            COM(3,21),
        ], 
        // AExp7
        vec![ // 13 
            ARG(0, true),
            INT(5),
            COM(1,0),
        ], 
        // AExp8
        vec![ // 14 
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp9
        vec![ // 16 
            ARG(3, true),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            COM(4,14),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
        ], 
        // AExp10
        vec![ // 18 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 19 
            COM(4,16),
            ARG(0, true),
        ], 
        // AExp11
        vec![ // 20 
            ARG(2, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp12
        vec![ // 21 
            ARG(2, true),
            INT(2),
            PTR(0, true, true),
        ], 
        vec![ // 22 
            COM(3,20),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp13
        vec![ // 23 
            ARG(3, true),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            ARG(2, true),
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp14
        vec![ // 25 
            COM(4,176),
            PTR(0, true, true),
        ], 
        vec![ // 26 
            PRM(EQ,false),
            ARG(0, true),
            ARG(1, true),
            COM(1,177),
            COM(1,178),
        ], 
        // AExp15
        vec![ // 27 
            COM(1,181),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp16
        vec![ // 29 
            COM(1,181),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp17
        vec![ // 31 
            COM(2,25),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 32 
            COM(1,29),
            ARG(1, true),
        ], 
        vec![ // 33 
            COM(1,27),
            ARG(0, true),
        ], 
        // AExp18
        vec![ // 34 
            COM(1,174),
            PTR(0, true, true),
        ], 
        vec![ // 35 
            COM(2,31),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp19
        vec![ // 36 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            COM(3,34),
            ARG(1, true),
        ], 
        // AExp20
        vec![ // 38 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 39 
            COM(3,36),
            ARG(0, false),
        ], 
        // AExp21
        vec![ // 40 
            COM(1,174),
            PTR(0, true, true),
        ], 
        vec![ // 41 
            COM(4,176),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp22
        vec![ // 42 
            COM(2,183),
            PTR(0, true, true),
        ], 
        vec![ // 43 
            PRM(ADD,false),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp23
        vec![ // 44 
            COM(1,181),
            PTR(0, true, true),
        ], 
        vec![ // 45 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp24
        vec![ // 46 
            COM(1,181),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp25
        vec![ // 48 
            COM(2,42),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            COM(1,46),
            ARG(1, true),
        ], 
        vec![ // 50 
            COM(1,44),
            ARG(0, true),
        ], 
        // AExp26
        vec![ // 51 
            COM(2,40),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            COM(2,48),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp27
        vec![ // 53 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            COM(2,51),
            ARG(1, true),
        ], 
        // AExp28
        vec![ // 55 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 56 
            COM(3,53),
            ARG(0, false),
        ], 
        // AExp29
        vec![ // 57 
            PRM(LT,false),
            ARG(2, true),
            INT(12),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 58 
            COM(1,55),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        vec![ // 59 
            COM(1,38),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp30
        vec![ // 60 
            COM(4,176),
            PTR(0, true, true),
        ], 
        vec![ // 61 
            COM(3,21),
            COM(1,201),
            ARG(0, true),
        ], 
        // AExp31
        vec![ // 62 
            COM(2,189),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 63 
            COM(1,60),
            ARG(0, false),
            ARG(1, true),
        ], 
        vec![ // 64 
            COM(1,200),
            ARG(0, false),
        ], 
        // AExp32
        vec![ // 65 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 66 
            ARG(1, true),
            ARG(0, true),
            COM(2,62),
        ], 
        // AExp33
        vec![ // 67 
            PRM(EQ,false),
            ARG(2, true),
            INT(9),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 68 
            COM(3,65),
            ARG(0, false),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp34
        vec![ // 69 
            PRM(LT,false),
            ARG(2, false),
            INT(11),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            COM(4,67),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 71 
            COM(4,57),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp35
        vec![ // 72 
            COM(2,189),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            COM(1,200),
            ARG(0, true),
        ], 
        // AExp36
        vec![ // 74 
            COM(1,174),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            COM(1,72),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp37
        vec![ // 76 
            ARG(2, true),
            ARG(0, true),
            COM(2,74),
        ], 
        // AExp38
        vec![ // 77 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 78 
            COM(3,76),
            ARG(0, false),
        ], 
        // AExp39
        vec![ // 79 
            COM(4,176),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 80 
            COM(4,176),
            ARG(2, true),
            ARG(0, true),
        ], 
        // AExp40
        vec![ // 81 
            COM(2,189),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 82 
            COM(3,79),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        vec![ // 83 
            COM(1,200),
            ARG(0, true),
        ], 
        // AExp41
        vec![ // 84 
            COM(1,174),
            PTR(0, true, true),
        ], 
        vec![ // 85 
            COM(4,81),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp42
        vec![ // 86 
            SEQ(false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 87 
            COM(4,84),
            ARG(0, true),
            ARG(3, true),
            ARG(2, false),
            ARG(1, false),
        ], 
        vec![ // 88 
            SEQ(false),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp43
        vec![ // 89 
            COM(4,86),
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 90 
            PTR(28, false, false),
            ARG(2, true),
        ], 
        vec![ // 91 
            PTR(28, false, false),
            ARG(1, true),
        ], 
        // AExp44
        vec![ // 92 
            ARG(3, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 93 
            COM(3,89),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp45
        vec![ // 94 
            ARG(2, true),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 95 
            COM(4,92),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp46
        vec![ // 96 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 97 
            COM(3,94),
            ARG(0, false),
        ], 
        // AExp47
        vec![ // 98 
            PRM(LT,false),
            ARG(2, true),
            INT(8),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 99 
            COM(1,96),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        vec![ // 100 
            COM(1,77),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp48
        vec![ // 101 
            PRM(LT,false),
            ARG(2, false),
            INT(9),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 102 
            COM(4,98),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 103 
            COM(4,69),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp49
        vec![ // 104 
            COM(4,176),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 105 
            COM(3,21),
            ARG(2, true),
            ARG(1, true),
        ], 
        // AExp50
        vec![ // 106 
            COM(2,189),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 107 
            COM(3,104),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        vec![ // 108 
            COM(1,200),
            ARG(0, true),
        ], 
        // AExp51
        vec![ // 109 
            COM(1,174),
            PTR(0, true, true),
        ], 
        vec![ // 110 
            COM(4,106),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp52
        vec![ // 111 
            SEQ(false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 112 
            COM(4,109),
            ARG(0, true),
            ARG(3, true),
            ARG(2, false),
            ARG(1, false),
        ], 
        vec![ // 113 
            SEQ(false),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp53
        vec![ // 114 
            COM(4,111),
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 115 
            PTR(28, false, false),
            ARG(2, true),
        ], 
        vec![ // 116 
            PTR(28, false, false),
            ARG(1, true),
        ], 
        // AExp54
        vec![ // 117 
            ARG(3, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 118 
            COM(3,114),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp55
        vec![ // 119 
            ARG(2, true),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 120 
            COM(4,117),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp56
        vec![ // 121 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 122 
            COM(3,119),
            ARG(0, false),
        ], 
        // AExp57
        vec![ // 123 
            COM(2,189),
            PTR(0, true, true),
        ], 
        vec![ // 124 
            COM(1,200),
            ARG(0, true),
        ], 
        // AExp58
        vec![ // 125 
            COM(1,174),
            PTR(0, true, true),
        ], 
        vec![ // 126 
            COM(1,123),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp59
        vec![ // 127 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 128 
            ARG(1, true),
            ARG(0, true),
            COM(2,125),
        ], 
        // AExp60
        vec![ // 129 
            PRM(LT,false),
            ARG(2, true),
            INT(6),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 130 
            COM(3,127),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        vec![ // 131 
            COM(1,121),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp61
        vec![ // 132 
            COM(2,189),
            PTR(0, true, true),
        ], 
        vec![ // 133 
            COM(1,200),
            ARG(0, true),
        ], 
        // AExp62
        vec![ // 134 
            COM(1,174),
            PTR(0, true, true),
        ], 
        vec![ // 135 
            COM(1,132),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp63
        vec![ // 136 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 137 
            COM(3,134),
            ARG(1, true),
        ], 
        // AExp64
        vec![ // 138 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 139 
            COM(3,136),
            ARG(0, false),
        ], 
        // AExp65
        vec![ // 140 
            COM(4,176),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 141 
            COM(3,21),
            ARG(2, true),
            ARG(1, true),
        ], 
        // AExp66
        vec![ // 142 
            COM(4,176),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 143 
            COM(3,140),
            ARG(0, true),
            ARG(1, false),
            ARG(2, true),
        ], 
        // AExp67
        vec![ // 144 
            COM(2,189),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 145 
            COM(3,142),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        vec![ // 146 
            COM(1,200),
            ARG(0, true),
        ], 
        // AExp68
        vec![ // 147 
            COM(1,174),
            PTR(0, true, true),
        ], 
        vec![ // 148 
            COM(4,144),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp69
        vec![ // 149 
            SEQ(false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 150 
            COM(4,147),
            ARG(0, true),
            ARG(3, true),
            ARG(2, false),
            ARG(1, false),
        ], 
        vec![ // 151 
            SEQ(false),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp70
        vec![ // 152 
            COM(4,149),
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 153 
            PTR(28, false, false),
            ARG(2, true),
        ], 
        vec![ // 154 
            PTR(28, false, false),
            ARG(1, true),
        ], 
        // AExp71
        vec![ // 155 
            ARG(3, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 156 
            COM(3,152),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp72
        vec![ // 157 
            ARG(2, true),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 158 
            COM(4,155),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp73
        vec![ // 159 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 160 
            COM(3,157),
            ARG(0, false),
        ], 
        // AExp74
        vec![ // 161 
            PRM(EQ,false),
            ARG(2, true),
            INT(3),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 162 
            COM(1,159),
            ARG(0, false),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp75
        vec![ // 163 
            PRM(LT,false),
            ARG(2, false),
            INT(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 164 
            COM(4,161),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 165 
            COM(1,138),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp76
        vec![ // 166 
            PRM(LT,false),
            ARG(2, false),
            INT(5),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 167 
            COM(4,163),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 168 
            COM(4,129),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp77
        vec![ // 169 
            PRM(LT,false),
            ARG(2, false),
            INT(7),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 170 
            COM(4,166),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 171 
            COM(4,101),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp78
        vec![ // 172 
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 173 
            COM(4,169),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp79
        vec![ // 174 
            ARG(0, false),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 175 
            COM(3,172),
            ARG(0, false),
        ], 
        // AExp80
        vec![ // 176 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp81
        vec![ // 177 
            ARG(0, true),
            INT(8),
            COM(1,0),
        ], 
        // AExp82
        vec![ // 178 
            ARG(0, true),
            INT(4),
            COM(1,0),
        ], 
        // AExp83
        vec![ // 179 
            PRM(EQ,false),
            ARG(0, true),
            INT(10),
            INT(0),
            PTR(0, true, true),
        ], 
        vec![ // 180 
            ARG(1, true),
            COM(1,0),
        ], 
        // AExp84
        vec![ // 181 
            ARG(0, true),
            COM(2,179),
        ], 
        // AExp85
        vec![ // 182 
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp86
        vec![ // 183 
            ARG(1, true),
            INT(10),
            PTR(0, true, true),
        ], 
        vec![ // 184 
            COM(2,182),
            ARG(0, true),
        ], 
        // AExp87
        vec![ // 185 
            COM(4,176),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 186 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp88
        vec![ // 187 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 188 
            COM(3,185),
            ARG(1, true),
        ], 
        // AExp89
        vec![ // 189 
            Y,
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 190 
            COM(3,187),
            ARG(1, true),
        ], 
        // AExp90
        vec![ // 191 
            PRM(EQ,false),
            ARG(3, true),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 192 
            ARG(4, true),
            ARG(1, true),
        ], 
        vec![ // 193 
            COM(4,176),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp91
        vec![ // 194 
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 195 
            COM(4,176),
            ARG(3, true),
            ARG(1, true),
        ], 
        // AExp92
        vec![ // 196 
            COM(5,191),
            ARG(1, true),
            PTR(0, true, true),
            ARG(2, false),
        ], 
        vec![ // 197 
            COM(4,194),
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp93
        vec![ // 198 
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 199 
            COM(3,196),
            ARG(0, true),
            ARG(1, false),
            ARG(2, true),
        ], 
        // AExp94
        vec![ // 200 
            Y,
            COM(3,198),
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp95
        vec![ // 201 
            ARG(0, true),
            INT(9),
            COM(1,0),
        ], 
        // AExp96
        vec![ // 202 
            COM(3,21),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 203 
            COM(1,214),
            ARG(1, true),
        ], 
        vec![ // 204 
            COM(1,214),
            ARG(0, true),
        ], 
        // AExp97
        vec![ // 205 
            PRM(EQ,false),
            ARG(1, true),
            INT(2),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 206 
            ARG(2, true),
            COM(2,202),
        ], 
        // AExp98
        vec![ // 207 
            COM(1,235),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 208 
            COM(1,214),
            ARG(1, true),
        ], 
        // AExp99
        vec![ // 209 
            PRM(EQ,false),
            ARG(1, true),
            INT(1),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 210 
            ARG(2, true),
            COM(2,207),
        ], 
        // AExp100
        vec![ // 211 
            PRM(LT,false),
            ARG(1, false),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 212 
            COM(3,209),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
        ], 
        vec![ // 213 
            COM(3,205),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp101
        vec![ // 214 
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 215 
            COM(3,211),
            ARG(0, false),
        ], 
        // AExp102
        vec![ // 216 
            PRM(EQ,false),
            ARG(2, true),
            INT(2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 217 
            ARG(3, true),
            ARG(0, true),
        ], 
        // AExp103
        vec![ // 218 
            COM(2,287),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 219 
            ARG(0, false),
            ARG(2, true),
        ], 
        vec![ // 220 
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp104
        vec![ // 221 
            COM(4,216),
            PTR(0, true, true),
        ], 
        vec![ // 222 
            COM(3,218),
            ARG(0, true),
        ], 
        // AExp105
        vec![ // 223 
            PRM(EQ,false),
            ARG(2, true),
            INT(0),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 224 
            ARG(3, true),
            ARG(0, true),
        ], 
        // AExp106
        vec![ // 225 
            PRM(EQ,false),
            ARG(0, true),
            ARG(2, true),
            ARG(1, true),
            COM(1,13),
        ], 
        // AExp107
        vec![ // 226 
            COM(4,223),
            PTR(0, true, true),
            ARG(1, false),
        ], 
        vec![ // 227 
            COM(3,225),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp108
        vec![ // 228 
            PRM(LT,false),
            ARG(3, false),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 229 
            COM(2,226),
            ARG(0, true),
            ARG(2, false),
            ARG(3, false),
            ARG(4, false),
        ], 
        vec![ // 230 
            COM(1,221),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
            ARG(4, false),
        ], 
        // AExp109
        vec![ // 231 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 232 
            COM(5,228),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp110
        vec![ // 233 
            COM(4,231),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 234 
            COM(3,21),
            COM(1,178),
            ARG(2, false),
        ], 
        // AExp111
        vec![ // 235 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 236 
            COM(3,233),
            ARG(0, true),
        ], 
        // AExp112
        vec![ // 237 
            PRM(EQ,false),
            ARG(2, true),
            INT(2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 238 
            ARG(3, true),
            ARG(0, true),
        ], 
        // AExp113
        vec![ // 239 
            PRM(EQ,false),
            ARG(2, true),
            INT(4),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 240 
            ARG(3, true),
            ARG(1, true),
        ], 
        // AExp114
        vec![ // 241 
            PRM(EQ,false),
            ARG(2, true),
            INT(2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 242 
            ARG(3, true),
            ARG(0, true),
        ], 
        // AExp115
        vec![ // 243 
            PRM(EQ,false),
            ARG(2, true),
            INT(4),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 244 
            ARG(3, true),
            ARG(1, true),
        ], 
        // AExp116
        vec![ // 245 
            COM(3,21),
            COM(1,178),
            PTR(0, true, true),
        ], 
        vec![ // 246 
            COM(3,21),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp117
        vec![ // 247 
            COM(4,243),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 248 
            COM(2,245),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp118
        vec![ // 249 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 250 
            COM(3,247),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp119
        vec![ // 251 
            COM(4,241),
            PTR(0, true, true),
            ARG(1, false),
        ], 
        vec![ // 252 
            COM(4,249),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp120
        vec![ // 253 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 254 
            COM(2,251),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp121
        vec![ // 255 
            COM(3,21),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 256 
            COM(3,21),
            COM(1,289),
            ARG(1, true),
        ], 
        // AExp122
        vec![ // 257 
            COM(3,253),
            ARG(0, false),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 258 
            COM(2,255),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp123
        vec![ // 259 
            COM(4,239),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 260 
            COM(2,257),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp124
        vec![ // 261 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 262 
            COM(3,259),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp125
        vec![ // 263 
            COM(4,237),
            PTR(0, true, true),
            ARG(1, false),
        ], 
        vec![ // 264 
            COM(4,261),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp126
        vec![ // 265 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 266 
            COM(2,263),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp127
        vec![ // 267 
            PRM(EQ,false),
            ARG(2, true),
            INT(2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 268 
            ARG(3, true),
            ARG(0, true),
        ], 
        // AExp128
        vec![ // 269 
            PRM(EQ,false),
            ARG(2, true),
            INT(4),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 270 
            ARG(3, true),
            ARG(1, true),
        ], 
        // AExp129
        vec![ // 271 
            COM(3,21),
            PTR(0, true, true),
        ], 
        vec![ // 272 
            COM(3,21),
            COM(1,290),
            ARG(0, true),
        ], 
        // AExp130
        vec![ // 273 
            COM(4,269),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 274 
            COM(1,271),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp131
        vec![ // 275 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 276 
            COM(3,273),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp132
        vec![ // 277 
            COM(4,267),
            PTR(0, true, true),
            ARG(1, false),
        ], 
        vec![ // 278 
            COM(4,275),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp133
        vec![ // 279 
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 280 
            COM(2,277),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp134
        vec![ // 281 
            COM(3,21),
            PTR(0, true, true),
        ], 
        vec![ // 282 
            COM(3,21),
            COM(1,291),
            ARG(0, true),
        ], 
        // AExp135
        vec![ // 283 
            COM(3,279),
            ARG(0, false),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 284 
            COM(1,281),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp136
        vec![ // 285 
            COM(3,265),
            ARG(0, false),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 286 
            COM(2,283),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp137
        vec![ // 287 
            SEQ(false),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 288 
            COM(2,285),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp138
        vec![ // 289 
            ARG(0, true),
            INT(6),
            COM(1,0),
        ], 
        // AExp139
        vec![ // 290 
            ARG(0, true),
            INT(7),
            COM(1,0),
        ], 
        // AExp140
        vec![ // 291 
            ARG(0, true),
            INT(3),
            COM(1,0),
        ], 
        // AExp141
        vec![ // 292 
            ARG(2, true),
            INT(1),
            PTR(0, true, true),
        ], 
        vec![ // 293 
            COM(3,20),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp142
        vec![ // 294 
            ARG(0, true),
            INT(12),
            COM(1,0),
        ], 
        // AExp143
        vec![ // 295 
            ARG(1, true),
            INT(0),
            PTR(0, true, true),
        ], 
        vec![ // 296 
            COM(2,182),
            ARG(0, true),
        ], 
        // AExp144
        vec![ // 297 
            ARG(0, true),
            INT(11),
            COM(1,0),
        ], 
    ],

}});