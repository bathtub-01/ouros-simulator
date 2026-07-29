use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 100
#[rustfmt::skip]
pub static COUNTDOWN: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Ptr(5, false, false),
            Ptr(4, false, false),
        ], 
        vec![ // 1 
            Com(4,2),
            Int(10),
            Com(2,0),
        ], 
        vec![ // 2 
            Com(4,2),
            Int(4),
            Ptr(1, false, false),
        ], 
        vec![ // 3 
            Com(4,2),
            Int(3),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(2,7),
            Ptr(3, false, false),
            Int(70),
        ], 
        // AExp1
        vec![ // 5 
            Y,
            Com(3,5),
            Int(0),
        ], 
        // AExp2
        vec![ // 6 
            Com(3,58),
            Ptr(11, false, false),
        ], 
        vec![ // 7 
            Com(4,2),
            Com(6,1),
            Com(2,0),
        ], 
        vec![ // 8 
            Com(4,2),
            Com(6,125),
            Ptr(7, false, false),
        ], 
        vec![ // 9 
            Com(1,64),
            Ptr(8, false, false),
        ], 
        vec![ // 10 
            Com(5,62),
            Ptr(9, false, false),
        ], 
        vec![ // 11 
            Com(4,60),
            Ptr(10, false, false),
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
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp3
        vec![ // 3 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 4 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp4
        vec![ // 5 
            Arg(2, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 6 
            Com(3,3),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp5
        vec![ // 7 
            Com(1,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Com(1,156),
            Arg(0, true),
        ], 
        vec![ // 9 
            Com(2,23),
            Arg(1, true),
        ], 
        // AExp6
        vec![ // 10 
            Com(2,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 12 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp7
        vec![ // 13 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Com(4,10),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp8
        vec![ // 15 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 16 
            Com(3,13),
            Arg(0, true),
        ], 
        // AExp9
        vec![ // 17 
            Com(4,2),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 18 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp10
        vec![ // 19 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Com(3,17),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 21 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 22 
            Com(3,19),
            Arg(1, true),
        ], 
        // AExp12
        vec![ // 23 
            Com(2,31),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Com(1,42),
            Arg(1, true),
        ], 
        // AExp13
        vec![ // 25 
            Com(2,31),
        ], 
        // AExp14
        vec![ // 26 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Com(2,31),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp15
        vec![ // 28 
            Prm(EQ,false),
            Arg(3, true),
            Arg(0, false),
            Com(1,25),
            Com(3,26),
            Arg(2, true),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp16
        vec![ // 29 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Com(4,28),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp17
        vec![ // 31 
            Arg(1, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Com(3,29),
            Arg(0, true),
        ], 
        // AExp18
        vec![ // 33 
            Com(1,140),
            Ptr(0, true, true),
        ], 
        vec![ // 34 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp19
        vec![ // 35 
            Com(1,15),
            Com(1,48),
            Ptr(0, true, true),
        ], 
        vec![ // 36 
            Com(2,33),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp20
        vec![ // 37 
            Arg(1, true),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 38 
            Com(7,155),
            Arg(0, false),
        ], 
        // AExp21
        vec![ // 39 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 40 
            Com(2,37),
            Arg(0, true),
        ], 
        // AExp22
        vec![ // 41 
            Com(1,44),
            Arg(1, false),
            Com(2,35),
            Com(2,39),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp23
        vec![ // 42 
            Arg(0, true),
            Com(2,0),
            Com(2,41),
        ], 
        // AExp24
        vec![ // 43 
            Com(2,0),
        ], 
        // AExp25
        vec![ // 44 
            Arg(0, true),
            Com(2,1),
            Com(2,43),
        ], 
        // AExp26
        vec![ // 45 
            Com(1,56),
            Ptr(6, false, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 46 
            Com(1,42),
            Arg(1, true),
        ], 
        vec![ // 47 
            Com(1,42),
            Arg(0, true),
        ], 
        // AExp27
        vec![ // 48 
            Arg(0, true),
            Com(2,45),
        ], 
        // AExp28
        vec![ // 49 
            Com(1,15),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 50 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp29
        vec![ // 51 
            Com(2,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Arg(1, true),
            Arg(4, true),
            Arg(2, false),
        ], 
        vec![ // 53 
            Com(3,49),
            Arg(0, true),
            Arg(2, false),
            Arg(3, true),
        ], 
        // AExp30
        vec![ // 54 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Com(5,51),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp31
        vec![ // 56 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Com(4,54),
            Arg(0, true),
        ], 
        // AExp32
        vec![ // 58 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 59 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp33
        vec![ // 60 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 61 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp34
        vec![ // 62 
            Com(1,15),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 63 
            Com(2,76),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp35
        vec![ // 64 
            Com(4,2),
            Com(6,0),
            Ptr(0, true, true),
        ], 
        vec![ // 65 
            Com(4,2),
            Com(6,124),
            Arg(0, true),
        ], 
        // AExp36
        vec![ // 66 
            Arg(0, true),
            Arg(4, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp37
        vec![ // 67 
            Com(5,66),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(1, true),
            Arg(3, false),
        ], 
        vec![ // 68 
            Arg(0, true),
            Arg(3, false),
        ], 
        // AExp38
        vec![ // 69 
            Com(2,0),
        ], 
        // AExp39
        vec![ // 70 
            Arg(5, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 71 
            Com(2,109),
            Arg(0, false),
            Arg(3, true),
            Arg(4, true),
        ], 
        vec![ // 72 
            Com(5,107),
            Arg(0, false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp40
        vec![ // 73 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 74 
            Com(6,70),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp41
        vec![ // 75 
            Com(3,84),
            Arg(3, false),
            Arg(1, true),
            Arg(2, true),
            Com(5,69),
            Com(5,73),
            Arg(3, false),
            Arg(0, true),
        ], 
        // AExp42
        vec![ // 76 
            Com(4,67),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 77 
            Com(4,75),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp43
        vec![ // 78 
            Prm(EQ,false),
            Ptr(0, true, true),
            Int(0),
        ], 
        vec![ // 79 
            Com(2,87),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp44
        vec![ // 80 
            Com(1,105),
            Ptr(0, true, true),
        ], 
        vec![ // 81 
            Prm(LE,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp45
        vec![ // 82 
            Com(2,0),
        ], 
        // AExp46
        vec![ // 83 
            Com(2,0),
        ], 
        // AExp47
        vec![ // 84 
            Arg(0, true),
            Com(2,1),
            Ptr(1, true, true),
            Com(2,1),
            Ptr(0, true, true),
            Com(1,82),
            Com(3,83),
        ], 
        vec![ // 85 
            Com(2,80),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 86 
            Com(2,78),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp48
        vec![ // 87 
            Com(1,103),
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
        ], 
        // AExp49
        vec![ // 88 
            Arg(0, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 89 
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp50
        vec![ // 90 
            Arg(2, true),
            Int(0),
            Arg(0, true),
        ], 
        // AExp51
        vec![ // 91 
            Arg(2, true),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 92 
            Prm(Sub,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp52
        vec![ // 93 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,90),
            Com(3,91),
            Arg(0, false),
            Arg(2, false),
        ], 
        // AExp53
        vec![ // 94 
            Arg(3, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 95 
            Prm(Sub,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 96 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp54
        vec![ // 97 
            Prm(LE,false),
            Arg(0, false),
            Arg(2, false),
            Com(4,2),
            Com(4,94),
            Ptr(0, true, true),
            Arg(2, false),
            Arg(0, false),
        ], 
        vec![ // 98 
            Prm(Add,false),
            Arg(1, false),
            Arg(1, false),
        ], 
        // AExp55
        vec![ // 99 
            Com(1,103),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 100 
            Com(3,97),
            Arg(2, true),
        ], 
        // AExp56
        vec![ // 101 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,93),
            Com(3,99),
            Arg(0, false),
            Arg(2, false),
            Arg(1, true),
        ], 
        // AExp57
        vec![ // 102 
            Prm(Add,false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp58
        vec![ // 103 
            Com(3,88),
            Ptr(0, true, true),
            Com(1,102),
        ], 
        vec![ // 104 
            Com(3,101),
            Arg(0, true),
        ], 
        // AExp59
        vec![ // 105 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp60
        vec![ // 106 
            Arg(6, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp61
        vec![ // 107 
            Com(7,106),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp62
        vec![ // 108 
            Int(0),
        ], 
        // AExp63
        vec![ // 109 
            Arg(0, true),
            Ptr(3, true, true),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
            Com(1,108),
        ], 
        vec![ // 110 
            Prm(Sub,false),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 111 
            Com(2,123),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 112 
            Com(2,114),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 113 
            Prm(Add,false),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp64
        vec![ // 114 
            Com(1,103),
            Arg(0, true),
            Arg(1, true),
            Com(2,0),
        ], 
        // AExp65
        vec![ // 115 
            Com(2,123),
            Ptr(0, true, true),
        ], 
        vec![ // 116 
            Prm(Add,false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp66
        vec![ // 117 
            Int(0),
        ], 
        // AExp67
        vec![ // 118 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 119 
            Prm(EQ,false),
            Arg(2, true),
            Int(0),
            Com(1,0),
            Com(1,117),
            Arg(0, false),
        ], 
        vec![ // 120 
            Com(1,115),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp68
        vec![ // 121 
            Com(1,103),
            Arg(1, true),
            Int(2),
            Ptr(0, true, true),
        ], 
        vec![ // 122 
            Com(3,118),
            Arg(0, true),
        ], 
        // AExp69
        vec![ // 123 
            Prm(EQ,false),
            Arg(1, false),
            Int(1),
            Com(2,121),
            Com(2,0),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp70
        vec![ // 124 
            Arg(3, true),
        ], 
        // AExp71
        vec![ // 125 
            Arg(2, true),
        ], 
        // AExp72
        vec![ // 126 
            Arg(2, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 127 
            Com(4,2),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp73
        vec![ // 128 
            Com(1,146),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 129 
            Com(1,140),
            Arg(1, true),
        ], 
        vec![ // 130 
            Com(2,153),
            Arg(0, true),
        ], 
        // AExp74
        vec![ // 131 
            Arg(1, true),
            Ptr(0, true, true),
            Com(1,0),
        ], 
        vec![ // 132 
            Com(4,2),
            Arg(0, true),
        ], 
        // AExp75
        vec![ // 133 
            Com(2,128),
            Ptr(0, true, true),
        ], 
        vec![ // 134 
            Com(2,131),
            Arg(0, true),
        ], 
        // AExp76
        vec![ // 135 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 136 
            Com(1,133),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 137 
            Com(3,126),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp77
        vec![ // 138 
            Com(2,0),
        ], 
        // AExp78
        vec![ // 139 
            Com(1,44),
            Arg(1, false),
            Com(2,135),
            Com(2,138),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp79
        vec![ // 140 
            Arg(0, true),
            Com(2,0),
            Com(2,139),
        ], 
        // AExp80
        vec![ // 141 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 142 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 143 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp81
        vec![ // 144 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 145 
            Com(4,141),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp82
        vec![ // 146 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 147 
            Com(3,144),
            Arg(0, true),
        ], 
        // AExp83
        vec![ // 148 
            Arg(4, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 149 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 150 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp84
        vec![ // 151 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 152 
            Com(5,148),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp85
        vec![ // 153 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 154 
            Com(3,151),
            Arg(1, true),
        ], 
        // AExp86
        vec![ // 155 
            Arg(5, true),
            Arg(0, true),
        ], 
        // AExp87
        vec![ // 156 
            Com(1,15),
            Com(1,161),
            Ptr(0, true, true),
        ], 
        vec![ // 157 
            Com(1,184),
            Arg(0, true),
        ], 
        // AExp88
        vec![ // 158 
            Com(1,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 159 
            Com(1,161),
            Arg(1, true),
        ], 
        vec![ // 160 
            Com(1,176),
            Arg(0, true),
        ], 
        // AExp89
        vec![ // 161 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,158),
        ], 
        vec![ // 162 
            Com(4,2),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp90
        vec![ // 163 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 164 
            Com(4,2),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp91
        vec![ // 165 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 166 
            Com(4,2),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp92
        vec![ // 167 
            Com(1,146),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 168 
            Arg(0, true),
            Arg(2, true),
        ], 
        vec![ // 169 
            Com(4,2),
            Arg(1, true),
        ], 
        // AExp93
        vec![ // 170 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 171 
            Com(3,167),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 172 
            Com(3,165),
            Arg(0, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp94
        vec![ // 173 
            Arg(2, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 174 
            Com(4,170),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 175 
            Com(1,163),
            Arg(0, false),
        ], 
        // AExp95
        vec![ // 176 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 177 
            Com(3,173),
            Arg(0, true),
        ], 
        // AExp96
        vec![ // 178 
            Com(1,146),
            Ptr(0, true, true),
        ], 
        vec![ // 179 
            Com(4,2),
            Arg(0, true),
        ], 
        // AExp97
        vec![ // 180 
            Com(2,21),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 181 
            Com(1,178),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp98
        vec![ // 182 
            Com(2,180),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 183 
            Com(1,184),
            Arg(1, true),
        ], 
        // AExp99
        vec![ // 184 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,182),
        ], 
        vec![ // 185 
            Com(4,2),
            Com(2,0),
            Com(2,0),
        ], 
    ],

}});