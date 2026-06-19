use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 104
#[rustfmt::skip]
pub static SUMPUZ: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,2),
            Ptr(9, false, false),
        ], 
        vec![ // 1 
            Com(4,3),
            Int(2),
            Com(2,0),
        ], 
        vec![ // 2 
            Com(4,3),
            Int(1),
            Ptr(1, false, false),
        ], 
        vec![ // 3 
            Com(4,3),
            Int(2),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(4,3),
            Ptr(3, false, false),
            Com(2,0),
        ], 
        vec![ // 5 
            Com(4,3),
            Int(1),
            Com(2,0),
        ], 
        vec![ // 6 
            Com(4,3),
            Int(2),
            Ptr(5, false, false),
        ], 
        vec![ // 7 
            Com(4,3),
            Int(1),
            Ptr(6, false, false),
        ], 
        vec![ // 8 
            Com(4,3),
            Int(0),
            Ptr(7, false, false),
        ], 
        vec![ // 9 
            Com(4,3),
            Ptr(8, false, false),
            Ptr(4, false, false),
        ], 
        // AExp1
        vec![ // 10 
            Com(1,141),
            Ptr(11, false, false),
        ], 
        vec![ // 11 
            Com(3,126),
            Com(1,149),
        ], 
        // AExp2
        vec![ // 12 
            Com(1,124),
            Com(1,95),
        ], 
        // AExp3
        vec![ // 13 
            Y,
            Com(3,38),
            Int(0),
        ], 
    ],
    comb_img: vec![
        // AExp0
        vec![ // 0 
            Arg(0, true),
        ], 
        // AExp1
        vec![ // 1 
            Arg(1, true),
        ], 
        // AExp2
        vec![ // 2 
            Com(3,4),
            Arg(0, false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp3
        vec![ // 3 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp4
        vec![ // 4 
            Com(2,6),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 5 
            Com(3,15),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp5
        vec![ // 6 
            Com(1,13),
            Arg(0, true),
            Arg(1, true),
            Int(0),
        ], 
        // AExp6
        vec![ // 7 
            Prm(Add,false),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 8 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp7
        vec![ // 9 
            Arg(1, true),
            Arg(4, true),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Com(3,7),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp8
        vec![ // 11 
            Arg(2, true),
            Arg(3, false),
            Ptr(0, true, true),
        ], 
        vec![ // 12 
            Com(5,9),
            Arg(0, true),
            Arg(1, true),
            Arg(3, false),
        ], 
        // AExp9
        vec![ // 13 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Com(4,11),
            Arg(0, true),
        ], 
        // AExp10
        vec![ // 15 
            Com(2,6),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 16 
            Com(3,17),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 17 
            Com(2,6),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 18 
            Com(3,19),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp12
        vec![ // 19 
            Com(3,32),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Int(0),
            Int(1),
        ], 
        // AExp13
        vec![ // 20 
            Prm(EQ,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 21 
            Ptr(13, false, false),
            Arg(1, true),
        ], 
        vec![ // 22 
            Ptr(13, false, false),
            Arg(0, true),
        ], 
        // AExp14
        vec![ // 23 
            Prm(EQ,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Ptr(13, false, false),
            Arg(1, true),
        ], 
        vec![ // 25 
            Ptr(13, false, false),
            Arg(0, true),
        ], 
        // AExp15
        vec![ // 26 
            Arg(0, true),
            Int(0),
            Com(2,0),
        ], 
        // AExp16
        vec![ // 27 
            Com(1,0),
            Com(1,42),
            Ptr(0, true, true),
        ], 
        vec![ // 28 
            Com(4,91),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Com(1,26),
        ], 
        // AExp17
        vec![ // 29 
            Com(1,35),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Com(3,27),
            Arg(0, false),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 31 
            Com(2,23),
            Arg(0, false),
            Arg(2, false),
        ], 
        // AExp18
        vec![ // 32 
            Com(1,35),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 33 
            Com(3,29),
            Arg(0, false),
            Arg(1, false),
            Arg(2, true),
        ], 
        vec![ // 34 
            Com(2,20),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp19
        vec![ // 35 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp20
        vec![ // 36 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp21
        vec![ // 38 
            Arg(2, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Com(3,36),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp22
        vec![ // 40 
            Com(2,0),
        ], 
        // AExp23
        vec![ // 41 
            Arg(1, true),
            Com(2,1),
            Com(2,40),
        ], 
        // AExp24
        vec![ // 42 
            Arg(0, true),
            Com(2,0),
            Com(2,41),
        ], 
        // AExp25
        vec![ // 43 
            Com(4,3),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 44 
            Com(1,95),
            Arg(0, true),
        ], 
        // AExp26
        vec![ // 45 
            Prm(EQ,false),
            Ptr(1, true, true),
            Int(0),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 46 
            Com(1,43),
            Arg(0, false),
        ], 
        vec![ // 47 
            Com(1,94),
            Arg(0, false),
        ], 
        // AExp27
        vec![ // 48 
            Com(3,106),
            Arg(1, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Com(1,95),
            Arg(0, true),
        ], 
        vec![ // 50 
            Com(4,3),
            Int(1),
            Com(2,0),
        ], 
        // AExp28
        vec![ // 51 
            Prm(EQ,false),
            Ptr(1, true, true),
            Int(1),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(2,48),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 53 
            Com(1,94),
            Arg(0, false),
        ], 
        // AExp29
        vec![ // 54 
            Com(2,0),
        ], 
        // AExp30
        vec![ // 55 
            Arg(2, true),
            Ptr(0, true, true),
            Com(2,54),
        ], 
        vec![ // 56 
            Com(2,51),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp31
        vec![ // 57 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 58 
            Com(3,55),
            Arg(1, false),
        ], 
        vec![ // 59 
            Com(1,45),
            Arg(1, false),
        ], 
        // AExp32
        vec![ // 60 
            Com(1,0),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 61 
            Com(1,162),
            Arg(1, true),
            Arg(4, true),
        ], 
        vec![ // 62 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp33
        vec![ // 63 
            Com(1,188),
            Ptr(2, true, true),
            Arg(3, true),
            Ptr(1, true, true),
            Arg(5, true),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Arg(2, true),
            Arg(4, true),
            Arg(6, true),
        ], 
        vec![ // 65 
            Com(1,203),
            Arg(1, true),
        ], 
        vec![ // 66 
            Com(1,94),
            Arg(0, true),
        ], 
        // AExp34
        vec![ // 67 
            Com(1,162),
            Ptr(0, true, true),
        ], 
        vec![ // 68 
            Com(7,63),
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp35
        vec![ // 69 
            Com(7,60),
            Ptr(1, true, true),
            Arg(1, true),
            Arg(5, false),
            Arg(6, false),
            Ptr(0, true, true),
        ], 
        vec![ // 70 
            Arg(2, true),
            Arg(4, false),
            Arg(5, false),
            Arg(6, false),
        ], 
        vec![ // 71 
            Com(7,67),
            Arg(3, true),
            Arg(0, true),
            Arg(4, false),
        ], 
        // AExp36
        vec![ // 72 
            Com(4,91),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 73 
            Com(1,204),
            Arg(0, true),
        ], 
        // AExp37
        vec![ // 74 
            Com(3,106),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 75 
            Com(2,209),
            Arg(0, true),
            Int(9),
        ], 
        vec![ // 76 
            Com(1,203),
            Arg(1, true),
        ], 
        // AExp38
        vec![ // 77 
            Com(3,211),
            Ptr(0, true, true),
            Int(1),
            Int(0),
        ], 
        vec![ // 78 
            Com(1,204),
            Arg(0, true),
        ], 
        // AExp39
        vec![ // 79 
            Com(2,74),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 80 
            Com(1,77),
            Arg(0, false),
        ], 
        // AExp40
        vec![ // 81 
            Com(2,209),
            Ptr(0, true, true),
            Int(9),
        ], 
        vec![ // 82 
            Com(3,211),
            Arg(0, true),
            Int(1),
            Int(0),
        ], 
        // AExp41
        vec![ // 83 
            Com(3,106),
            Arg(1, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 84 
            Com(1,95),
            Arg(0, true),
        ], 
        vec![ // 85 
            Com(1,81),
            Arg(2, true),
        ], 
        // AExp42
        vec![ // 86 
            Com(7,69),
            Ptr(1, true, true),
            Ptr(0, true, true),
            Com(3,83),
            Arg(0, false),
        ], 
        vec![ // 87 
            Com(1,79),
            Arg(0, false),
        ], 
        vec![ // 88 
            Com(2,72),
            Arg(0, false),
        ], 
        // AExp43
        vec![ // 89 
            Arg(1, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 90 
            Com(1,86),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp44
        vec![ // 91 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 92 
            Com(5,89),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 93 
            Com(2,57),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp45
        vec![ // 94 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp46
        vec![ // 95 
            Arg(0, true),
            Com(2,1),
        ], 
        // AExp47
        vec![ // 96 
            Ptr(10, false, false),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 97 
            Ptr(12, false, false),
            Arg(1, true),
        ], 
        // AExp48
        vec![ // 98 
            Com(2,133),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 99 
            Com(2,96),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 100 
            Com(1,135),
            Arg(0, true),
        ], 
        // AExp49
        vec![ // 101 
            Com(1,124),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 102 
            Com(3,98),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 103 
            Com(3,126),
            Com(4,3),
            Arg(2, false),
        ], 
        // AExp50
        vec![ // 104 
            Com(1,155),
            Arg(2, true),
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 105 
            Com(4,3),
            Arg(1, true),
            Com(2,0),
        ], 
        // AExp51
        vec![ // 106 
            Com(1,116),
            Arg(0, false),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 107 
            Com(3,104),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 108 
            Com(3,101),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp52
        vec![ // 109 
            Prm(EQ,false),
            Arg(0, true),
            Arg(3, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 110 
            Com(3,118),
            Arg(4, true),
        ], 
        vec![ // 111 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp53
        vec![ // 112 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 113 
            Com(5,109),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp54
        vec![ // 114 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 115 
            Com(4,112),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp55
        vec![ // 116 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 117 
            Com(3,114),
            Arg(0, true),
        ], 
        // AExp56
        vec![ // 118 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp57
        vec![ // 119 
            Com(4,3),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 120 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 121 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp58
        vec![ // 122 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 123 
            Com(4,119),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp59
        vec![ // 124 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 125 
            Com(3,122),
            Arg(0, true),
        ], 
        // AExp60
        vec![ // 126 
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp61
        vec![ // 127 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp62
        vec![ // 128 
            Com(4,3),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 129 
            Com(2,133),
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 130 
            Com(3,127),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp63
        vec![ // 131 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 132 
            Com(4,128),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp64
        vec![ // 133 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 134 
            Com(3,131),
            Arg(1, true),
        ], 
        // AExp65
        vec![ // 135 
            Com(4,3),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 136 
            Com(1,135),
            Arg(0, false),
        ], 
        // AExp66
        vec![ // 137 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 138 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp67
        vec![ // 139 
            Arg(3, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 140 
            Com(4,137),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp68
        vec![ // 141 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 142 
            Com(4,139),
            Arg(0, true),
        ], 
        // AExp69
        vec![ // 143 
            Com(4,3),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 144 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp70
        vec![ // 145 
            Prm(EQ,false),
            Arg(0, true),
            Arg(2, false),
            Ptr(0, true, true),
            Arg(3, false),
        ], 
        vec![ // 146 
            Com(3,143),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp71
        vec![ // 147 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 148 
            Com(4,145),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp72
        vec![ // 149 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 150 
            Com(3,147),
            Arg(0, true),
        ], 
        // AExp73
        vec![ // 151 
            Prm(EQ,false),
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
            Com(2,1),
        ], 
        vec![ // 152 
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp74
        vec![ // 153 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 154 
            Com(4,151),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp75
        vec![ // 155 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 156 
            Com(3,153),
            Arg(0, true),
        ], 
        // AExp76
        vec![ // 157 
            Com(2,168),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 158 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 159 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp77
        vec![ // 160 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 161 
            Com(4,157),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp78
        vec![ // 162 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 163 
            Com(3,160),
            Arg(0, true),
        ], 
        // AExp79
        vec![ // 164 
            Com(4,3),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 165 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp80
        vec![ // 166 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 167 
            Com(3,164),
            Arg(1, true),
        ], 
        // AExp81
        vec![ // 168 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 169 
            Com(3,166),
            Arg(1, true),
        ], 
        // AExp82
        vec![ // 170 
            Arg(0, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, false),
            Ptr(0, true, true),
        ], 
        vec![ // 171 
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(6, false),
        ], 
        // AExp83
        vec![ // 172 
            Com(3,190),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 173 
            Com(1,94),
            Arg(1, true),
        ], 
        // AExp84
        vec![ // 174 
            Com(4,3),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 175 
            Com(1,95),
            Arg(0, true),
        ], 
        // AExp85
        vec![ // 176 
            Com(3,106),
            Arg(0, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 177 
            Com(1,174),
            Arg(2, true),
        ], 
        // AExp86
        vec![ // 178 
            Com(1,162),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 179 
            Com(3,176),
            Arg(0, true),
            Arg(2, true),
            Arg(3, false),
        ], 
        vec![ // 180 
            Com(2,172),
            Arg(1, true),
            Arg(3, false),
        ], 
        // AExp87
        vec![ // 181 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 182 
            Com(2,200),
            Arg(2, false),
            Arg(1, true),
        ], 
        vec![ // 183 
            Com(2,200),
            Arg(2, false),
            Arg(0, true),
        ], 
        // AExp88
        vec![ // 184 
            Prm(Add,false),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 185 
            Com(3,181),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp89
        vec![ // 186 
            Com(1,197),
            Ptr(0, true, true),
        ], 
        vec![ // 187 
            Com(4,184),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp90
        vec![ // 188 
            Com(7,170),
            Com(4,178),
            Ptr(0, true, true),
        ], 
        vec![ // 189 
            Com(4,186),
            Arg(0, true),
        ], 
        // AExp91
        vec![ // 190 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 191 
            Com(3,127),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp92
        vec![ // 192 
            Arg(2, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 193 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp93
        vec![ // 194 
            Com(1,197),
            Ptr(0, true, true),
            Com(3,192),
        ], 
        vec![ // 195 
            Prm(Sub,false),
            Arg(0, true),
            Int(10),
        ], 
        // AExp94
        vec![ // 196 
            Arg(1, true),
            Int(0),
            Arg(0, true),
        ], 
        // AExp95
        vec![ // 197 
            Prm(LE,false),
            Arg(0, false),
            Int(9),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 198 
            Com(2,196),
            Arg(0, false),
        ], 
        vec![ // 199 
            Com(1,194),
            Arg(0, false),
        ], 
        // AExp96
        vec![ // 200 
            Com(1,202),
            Ptr(0, true, true),
        ], 
        vec![ // 201 
            Com(1,116),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp97
        vec![ // 202 
            Arg(0, true),
            Err(4),
            Com(1,0),
        ], 
        // AExp98
        vec![ // 203 
            Arg(0, true),
            Err(3),
            Com(2,0),
        ], 
        // AExp99
        vec![ // 204 
            Arg(0, true),
            Com(2,0),
            Com(2,1),
        ], 
        // AExp100
        vec![ // 205 
            Com(2,209),
            Ptr(0, true, true),
        ], 
        vec![ // 206 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp101
        vec![ // 207 
            Com(4,3),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 208 
            Com(1,205),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp102
        vec![ // 209 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 210 
            Com(2,207),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp103
        vec![ // 211 
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 212 
            Com(3,0),
            Arg(2, true),
        ], 
    ],

}});
