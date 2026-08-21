use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 112
#[rustfmt::skip]
pub static CLAUSIFY: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,0),
            Com(1,5),
            Ptr(18, false, false),
        ], 
        vec![ // 1 
            Com(5,132),
            Int(0),
        ], 
        vec![ // 2 
            Com(5,132),
            Int(0),
        ], 
        vec![ // 3 
            Com(2,195),
            Ptr(2, false, false),
            Ptr(1, false, false),
        ], 
        vec![ // 4 
            Com(5,132),
            Int(0),
        ], 
        vec![ // 5 
            Com(2,195),
            Ptr(4, false, false),
        ], 
        vec![ // 6 
            Com(1,0),
            Ptr(5, false, false),
            Ptr(3, false, false),
        ], 
        vec![ // 7 
            Com(5,132),
            Int(0),
        ], 
        vec![ // 8 
            Com(5,132),
            Int(0),
        ], 
        vec![ // 9 
            Com(2,195),
            Ptr(8, false, false),
            Ptr(7, false, false),
        ], 
        vec![ // 10 
            Com(5,132),
            Int(0),
        ], 
        vec![ // 11 
            Com(2,195),
            Ptr(10, false, false),
        ], 
        vec![ // 12 
            Com(1,0),
            Ptr(11, false, false),
            Ptr(9, false, false),
        ], 
        vec![ // 13 
            Com(2,195),
            Ptr(12, false, false),
            Ptr(6, false, false),
        ], 
        vec![ // 14 
            Com(2,189),
            Int(2),
            Ptr(13, false, false),
        ], 
        vec![ // 15 
            Com(5,132),
            Int(0),
        ], 
        vec![ // 16 
            Com(2,13),
            Com(6,99),
            Ptr(15, false, false),
        ], 
        vec![ // 17 
            Com(1,0),
            Ptr(16, false, false),
            Ptr(14, false, false),
        ], 
        vec![ // 18 
            Com(1,0),
            Com(1,23),
            Ptr(17, false, false),
        ], 
        // AExp1
        vec![ // 19 
            Com(1,25),
            Ptr(21, false, false),
        ], 
        vec![ // 20 
            Com(2,32),
            Com(2,66),
        ], 
        vec![ // 21 
            Com(3,26),
            Ptr(20, false, false),
            Com(1,82),
        ], 
        // AExp2
        vec![ // 22 
            Com(1,48),
            Com(1,85),
        ], 
        // AExp3
        vec![ // 23 
            Com(1,96),
            Ptr(24, false, false),
        ], 
        vec![ // 24 
            Com(2,110),
            Com(1,90),
        ], 
        // AExp4
        vec![ // 25 
            Com(1,129),
            Com(2,0),
        ], 
        // AExp5
        vec![ // 26 
            Com(2,13),
            Prm(Add,false),
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
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(1,5),
            Arg(1, true),
        ], 
        vec![ // 4 
            Com(1,9),
            Arg(0, true),
        ], 
        // AExp3
        vec![ // 5 
            Arg(0, true),
            Int(0),
            Com(2,2),
        ], 
        // AExp4
        vec![ // 6 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 7 
            Ptr(26, false, false),
            Arg(1, true),
        ], 
        vec![ // 8 
            Ptr(26, false, false),
            Arg(0, true),
        ], 
        // AExp5
        vec![ // 9 
            Arg(0, true),
            Com(2,6),
        ], 
        // AExp6
        vec![ // 10 
            Arg(3, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Arg(4, true),
            Arg(1, true),
        ], 
        // AExp7
        vec![ // 12 
            Arg(3, true),
            Com(3,0),
            Com(5,10),
            Arg(1, true),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp8
        vec![ // 13 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Com(4,12),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp9
        vec![ // 15 
            Com(1,0),
            Com(1,139),
            Ptr(0, true, true),
        ], 
        vec![ // 16 
            Com(1,183),
            Arg(0, true),
        ], 
        // AExp10
        vec![ // 17 
            Com(1,0),
            Ptr(25, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 18 
            Com(1,15),
            Arg(0, true),
        ], 
        // AExp11
        vec![ // 19 
            Com(1,0),
            Ptr(23, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Com(1,17),
            Arg(0, true),
        ], 
        // AExp12
        vec![ // 21 
            Com(1,0),
            Ptr(22, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Com(1,19),
            Arg(0, true),
        ], 
        // AExp13
        vec![ // 23 
            Com(1,0),
            Ptr(19, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Com(1,21),
            Arg(0, true),
        ], 
        // AExp14
        vec![ // 25 
            Com(2,13),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp15
        vec![ // 26 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp16
        vec![ // 28 
            Com(2,37),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Com(1,48),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp17
        vec![ // 30 
            Com(3,26),
            Com(1,50),
            Ptr(0, true, true),
        ], 
        vec![ // 31 
            Com(1,57),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp18
        vec![ // 32 
            Com(3,28),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 33 
            Com(2,30),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp19
        vec![ // 34 
            Com(4,39),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp20
        vec![ // 36 
            Arg(2, true),
            Com(2,0),
            Com(4,34),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp21
        vec![ // 37 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 38 
            Com(3,36),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 39 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp23
        vec![ // 40 
            Com(2,0),
        ], 
        // AExp24
        vec![ // 41 
            Arg(3, true),
        ], 
        // AExp25
        vec![ // 42 
            Com(4,39),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 43 
            Com(1,48),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp26
        vec![ // 44 
            Arg(3, false),
            Arg(1, false),
            Com(4,41),
            Com(4,42),
            Arg(1, false),
            Arg(3, false),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 45 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp27
        vec![ // 46 
            Arg(2, true),
            Com(1,40),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 47 
            Com(4,44),
            Arg(1, true),
        ], 
        // AExp28
        vec![ // 48 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Com(3,46),
            Arg(0, true),
        ], 
        // AExp29
        vec![ // 50 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp30
        vec![ // 51 
            Com(2,0),
        ], 
        // AExp31
        vec![ // 52 
            Com(1,60),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 53 
            Arg(0, true),
            Arg(2, true),
            Arg(4, false),
        ], 
        vec![ // 54 
            Arg(3, true),
            Arg(1, true),
            Arg(4, false),
        ], 
        // AExp32
        vec![ // 55 
            Arg(2, true),
            Com(2,51),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 56 
            Com(5,52),
            Arg(1, true),
        ], 
        // AExp33
        vec![ // 57 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 58 
            Com(3,55),
            Arg(0, true),
        ], 
        // AExp34
        vec![ // 59 
            Com(2,1),
        ], 
        // AExp35
        vec![ // 60 
            Arg(0, true),
            Com(1,0),
            Com(1,59),
        ], 
        // AExp36
        vec![ // 61 
            Com(1,69),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 62 
            Com(1,80),
            Prm(EQ,false),
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 63 
            Com(1,80),
            Prm(EQ,false),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp37
        vec![ // 64 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 65 
            Com(4,61),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp38
        vec![ // 66 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 67 
            Com(3,64),
            Arg(1, true),
        ], 
        // AExp39
        vec![ // 68 
            Com(2,0),
        ], 
        // AExp40
        vec![ // 69 
            Arg(0, true),
            Com(1,68),
            Com(1,0),
        ], 
        // AExp41
        vec![ // 70 
            Com(2,0),
        ], 
        // AExp42
        vec![ // 71 
            Arg(0, true),
            Com(2,1),
            Com(2,70),
        ], 
        // AExp43
        vec![ // 72 
            Com(2,0),
        ], 
        // AExp44
        vec![ // 73 
            Com(1,69),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 74 
            Arg(0, true),
            Arg(5, true),
            Arg(2, true),
        ], 
        vec![ // 75 
            Arg(3, true),
            Arg(4, true),
            Arg(1, true),
        ], 
        // AExp45
        vec![ // 76 
            Arg(3, true),
            Com(3,72),
            Ptr(0, true, true),
            Arg(4, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 77 
            Com(6,73),
            Arg(0, true),
        ], 
        // AExp46
        vec![ // 78 
            Arg(2, true),
            Com(2,71),
            Ptr(0, true, true),
            Arg(3, true),
            Arg(0, true),
        ], 
        vec![ // 79 
            Com(5,76),
            Arg(1, true),
        ], 
        // AExp47
        vec![ // 80 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 81 
            Com(4,78),
            Arg(0, true),
        ], 
        // AExp48
        vec![ // 82 
            Com(4,39),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp49
        vec![ // 83 
            Com(1,87),
            Ptr(0, true, true),
        ], 
        vec![ // 84 
            Com(2,88),
            Prm(EQ,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp50
        vec![ // 85 
            Arg(0, true),
            Com(2,83),
        ], 
        // AExp51
        vec![ // 86 
            Com(2,0),
        ], 
        // AExp52
        vec![ // 87 
            Arg(0, true),
            Com(2,1),
            Com(2,86),
        ], 
        // AExp53
        vec![ // 88 
            Com(1,48),
            Ptr(0, true, true),
        ], 
        vec![ // 89 
            Com(1,57),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp54
        vec![ // 90 
            Arg(0, true),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp55
        vec![ // 91 
            Com(2,0),
        ], 
        // AExp56
        vec![ // 92 
            Com(4,39),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 93 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 94 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp57
        vec![ // 95 
            Arg(2, true),
            Com(2,91),
            Com(4,92),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp58
        vec![ // 96 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 97 
            Com(3,95),
            Arg(0, true),
        ], 
        // AExp59
        vec![ // 98 
            Arg(2, true),
        ], 
        // AExp60
        vec![ // 99 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp61
        vec![ // 100 
            Com(2,110),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 101 
            Com(3,99),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp62
        vec![ // 102 
            Com(2,110),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 103 
            Com(3,100),
            Arg(0, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp63
        vec![ // 104 
            Arg(4, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 105 
            Com(2,118),
            Arg(0, true),
            Arg(3, true),
        ], 
        // AExp64
        vec![ // 106 
            Arg(0, true),
            Com(5,98),
            Com(5,98),
            Com(4,1),
            Com(5,104),
        ], 
        // AExp65
        vec![ // 107 
            Arg(4, true),
            Ptr(0, true, true),
            Arg(3, true),
        ], 
        vec![ // 108 
            Com(2,118),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp66
        vec![ // 109 
            Arg(0, true),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp67
        vec![ // 110 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 111 
            Arg(1, true),
            Com(5,98),
            Com(5,102),
            Com(1,106),
            Com(5,107),
            Com(1,109),
        ], 
        // AExp68
        vec![ // 112 
            Com(4,39),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp69
        vec![ // 113 
            Com(4,39),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 114 
            Com(2,118),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp70
        vec![ // 115 
            Com(4,39),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 116 
            Com(4,39),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp71
        vec![ // 117 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,113),
            Com(3,115),
            Arg(2, false),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp72
        vec![ // 118 
            Arg(1, true),
            Com(1,112),
            Com(3,117),
            Arg(0, true),
        ], 
        // AExp73
        vec![ // 119 
            Arg(6, true),
            Ptr(0, true, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        vec![ // 120 
            Arg(0, true),
            Arg(5, true),
        ], 
        // AExp74
        vec![ // 121 
            Com(1,129),
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 122 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp75
        vec![ // 123 
            Com(4,39),
            Ptr(0, true, true),
        ], 
        vec![ // 124 
            Com(6,39),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp76
        vec![ // 125 
            Com(4,39),
            Ptr(0, true, true),
        ], 
        vec![ // 126 
            Com(5,131),
            Arg(0, true),
        ], 
        // AExp77
        vec![ // 127 
            Com(4,39),
            Ptr(0, true, true),
        ], 
        vec![ // 128 
            Com(5,132),
            Arg(0, true),
        ], 
        // AExp78
        vec![ // 129 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 130 
            Com(7,119),
            Com(4,121),
            Com(2,123),
            Com(1,125),
            Com(1,127),
            Arg(0, true),
        ], 
        // AExp79
        vec![ // 131 
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp80
        vec![ // 132 
            Arg(4, true),
            Arg(0, true),
        ], 
        // AExp81
        vec![ // 133 
            Com(6,99),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 134 
            Com(1,139),
            Arg(1, true),
        ], 
        vec![ // 135 
            Com(1,139),
            Arg(0, true),
        ], 
        // AExp82
        vec![ // 136 
            Com(1,149),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 137 
            Com(1,139),
            Arg(1, true),
        ], 
        vec![ // 138 
            Com(1,139),
            Arg(0, true),
        ], 
        // AExp83
        vec![ // 139 
            Arg(0, true),
            Com(2,133),
            Com(2,136),
            Com(5,131),
            Com(5,132),
        ], 
        // AExp84
        vec![ // 140 
            Com(6,99),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 141 
            Com(1,149),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 142 
            Com(1,149),
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp85
        vec![ // 143 
            Com(2,159),
            Ptr(0, true, true),
        ], 
        vec![ // 144 
            Com(6,39),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp86
        vec![ // 145 
            Com(2,159),
            Ptr(0, true, true),
        ], 
        vec![ // 146 
            Com(5,131),
            Arg(0, true),
        ], 
        // AExp87
        vec![ // 147 
            Com(2,159),
            Ptr(0, true, true),
        ], 
        vec![ // 148 
            Com(5,132),
            Arg(0, true),
        ], 
        // AExp88
        vec![ // 149 
            Arg(0, true),
            Com(3,140),
            Com(2,143),
            Com(1,145),
            Com(1,147),
        ], 
        // AExp89
        vec![ // 150 
            Com(6,99),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 151 
            Com(1,149),
            Arg(2, false),
            Arg(1, true),
        ], 
        vec![ // 152 
            Com(1,149),
            Arg(2, false),
            Arg(0, true),
        ], 
        // AExp90
        vec![ // 153 
            Com(6,39),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 154 
            Com(6,39),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp91
        vec![ // 155 
            Com(6,39),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 156 
            Com(5,131),
            Arg(0, true),
        ], 
        // AExp92
        vec![ // 157 
            Com(6,39),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 158 
            Com(5,132),
            Arg(0, true),
        ], 
        // AExp93
        vec![ // 159 
            Arg(1, true),
            Com(3,150),
            Com(3,153),
            Com(2,155),
            Com(2,157),
            Arg(0, true),
        ], 
        // AExp94
        vec![ // 160 
            Com(6,99),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 161 
            Com(1,183),
            Arg(1, true),
        ], 
        vec![ // 162 
            Com(1,183),
            Arg(0, true),
        ], 
        // AExp95
        vec![ // 163 
            Com(6,39),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 164 
            Com(1,183),
            Arg(1, true),
        ], 
        vec![ // 165 
            Com(1,183),
            Arg(0, true),
        ], 
        // AExp96
        vec![ // 166 
            Com(1,183),
            Ptr(0, true, true),
        ], 
        vec![ // 167 
            Com(5,131),
            Arg(0, true),
        ], 
        // AExp97
        vec![ // 168 
            Com(1,183),
            Ptr(0, true, true),
        ], 
        vec![ // 169 
            Com(5,131),
            Arg(0, true),
        ], 
        // AExp98
        vec![ // 170 
            Com(6,39),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 171 
            Com(1,168),
            Arg(1, true),
        ], 
        vec![ // 172 
            Com(1,166),
            Arg(0, true),
        ], 
        // AExp99
        vec![ // 173 
            Com(1,183),
            Ptr(0, true, true),
        ], 
        vec![ // 174 
            Com(5,131),
            Arg(0, true),
        ], 
        // AExp100
        vec![ // 175 
            Com(1,183),
            Ptr(0, true, true),
        ], 
        vec![ // 176 
            Com(5,131),
            Arg(0, true),
        ], 
        // AExp101
        vec![ // 177 
            Com(6,99),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 178 
            Com(1,175),
            Arg(1, true),
        ], 
        vec![ // 179 
            Com(1,173),
            Arg(0, true),
        ], 
        // AExp102
        vec![ // 180 
            Com(5,131),
            Ptr(0, true, true),
        ], 
        vec![ // 181 
            Com(5,132),
            Arg(0, true),
        ], 
        // AExp103
        vec![ // 182 
            Arg(0, true),
            Com(2,170),
            Com(2,177),
            Com(1,183),
            Com(1,180),
        ], 
        // AExp104
        vec![ // 183 
            Arg(0, true),
            Com(2,160),
            Com(2,163),
            Com(1,182),
            Com(5,132),
        ], 
        // AExp105
        vec![ // 184 
            Com(2,0),
        ], 
        // AExp106
        vec![ // 185 
            Com(2,189),
            Ptr(0, true, true),
        ], 
        vec![ // 186 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp107
        vec![ // 187 
            Com(4,39),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 188 
            Com(1,185),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp108
        vec![ // 189 
            Prm(LE,false),
            Arg(0, false),
            Int(0),
            Com(1,0),
            Com(1,184),
            Ptr(0, true, true),
        ], 
        vec![ // 190 
            Com(2,187),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp109
        vec![ // 191 
            Com(6,39),
            Ptr(0, true, true),
        ], 
        vec![ // 192 
            Com(5,131),
            Arg(0, true),
        ], 
        // AExp110
        vec![ // 193 
            Com(6,39),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 194 
            Com(5,131),
            Arg(1, true),
        ], 
        // AExp111
        vec![ // 195 
            Com(6,99),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 196 
            Com(2,193),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 197 
            Com(1,191),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});