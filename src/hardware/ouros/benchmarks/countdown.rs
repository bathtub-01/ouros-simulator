use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 109
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
            Com(3,63),
            Ptr(11, false, false),
        ], 
        vec![ // 7 
            Com(4,2),
            Com(1,163),
            Com(2,0),
        ], 
        vec![ // 8 
            Com(4,2),
            Com(1,162),
            Ptr(7, false, false),
        ], 
        vec![ // 9 
            Com(1,69),
            Ptr(8, false, false),
        ], 
        vec![ // 10 
            Com(5,67),
            Ptr(9, false, false),
        ], 
        vec![ // 11 
            Com(4,65),
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
            Com(1,196),
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
            Com(1,34),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Com(1,47),
            Arg(1, true),
        ], 
        // AExp13
        vec![ // 25 
            Com(4,2),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 26 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp14
        vec![ // 27 
            Prm(EQ,false),
            Arg(4, true),
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 28 
            Com(3,25),
            Arg(1, false),
            Arg(2, false),
            Arg(3, true),
        ], 
        vec![ // 29 
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp15
        vec![ // 30 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 31 
            Com(5,27),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp16
        vec![ // 32 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 33 
            Com(4,30),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp17
        vec![ // 34 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Com(3,32),
            Arg(0, true),
        ], 
        // AExp18
        vec![ // 36 
            Com(1,178),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp19
        vec![ // 38 
            Com(1,15),
            Com(1,53),
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Com(2,36),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp20
        vec![ // 40 
            Com(1,49),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 41 
            Com(4,2),
            Arg(1, true),
            Com(2,0),
        ], 
        vec![ // 42 
            Com(2,38),
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp21
        vec![ // 43 
            Arg(1, true),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 44 
            Com(2,194),
            Arg(0, false),
        ], 
        // AExp22
        vec![ // 45 
            Com(3,40),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 46 
            Com(2,43),
            Arg(0, false),
        ], 
        // AExp23
        vec![ // 47 
            Arg(0, true),
            Com(2,0),
            Com(1,45),
        ], 
        // AExp24
        vec![ // 48 
            Com(2,0),
        ], 
        // AExp25
        vec![ // 49 
            Arg(0, true),
            Com(2,1),
            Com(2,48),
        ], 
        // AExp26
        vec![ // 50 
            Com(1,61),
            Ptr(6, false, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 51 
            Com(1,47),
            Arg(1, true),
        ], 
        vec![ // 52 
            Com(1,47),
            Arg(0, true),
        ], 
        // AExp27
        vec![ // 53 
            Arg(0, true),
            Com(2,50),
        ], 
        // AExp28
        vec![ // 54 
            Com(1,15),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 55 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp29
        vec![ // 56 
            Com(2,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Arg(1, true),
            Arg(4, true),
            Arg(2, false),
        ], 
        vec![ // 58 
            Com(3,54),
            Arg(0, true),
            Arg(2, false),
            Arg(3, true),
        ], 
        // AExp30
        vec![ // 59 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 60 
            Com(5,56),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp31
        vec![ // 61 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 62 
            Com(4,59),
            Arg(0, true),
        ], 
        // AExp32
        vec![ // 63 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp33
        vec![ // 65 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 66 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp34
        vec![ // 67 
            Com(1,15),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 68 
            Com(5,76),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp35
        vec![ // 69 
            Com(4,2),
            Com(1,160),
            Ptr(0, true, true),
        ], 
        vec![ // 70 
            Com(4,2),
            Com(1,161),
            Arg(0, true),
        ], 
        // AExp36
        vec![ // 71 
            Arg(5, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 72 
            Com(3,148),
            Arg(4, false),
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 73 
            Com(4,127),
            Arg(4, false),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp37
        vec![ // 74 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 75 
            Com(6,71),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp38
        vec![ // 76 
            Com(3,97),
            Arg(4, false),
            Arg(1, false),
            Arg(3, false),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 77 
            Com(5,74),
            Arg(0, true),
            Arg(1, false),
            Arg(2, true),
            Arg(3, false),
            Arg(4, false),
        ], 
        // AExp39
        vec![ // 78 
            Prm(EQ,false),
            Arg(1, true),
            Int(3),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 79 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp40
        vec![ // 80 
            Com(1,99),
            Ptr(0, true, true),
        ], 
        vec![ // 81 
            Prm(LE,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp41
        vec![ // 82 
            Com(3,78),
            Ptr(0, true, true),
        ], 
        vec![ // 83 
            Com(2,80),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp42
        vec![ // 84 
            Prm(LT,false),
            Arg(2, false),
            Int(3),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 85 
            Arg(3, false),
            Com(2,1),
        ], 
        vec![ // 86 
            Com(2,82),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp43
        vec![ // 87 
            Prm(LT,false),
            Arg(1, true),
            Int(1),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 88 
            Arg(2, false),
            Com(2,1),
        ], 
        vec![ // 89 
            Arg(2, false),
            Arg(0, true),
        ], 
        // AExp44
        vec![ // 90 
            Prm(EQ,false),
            Ptr(0, true, true),
            Int(0),
        ], 
        vec![ // 91 
            Com(2,100),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp45
        vec![ // 92 
            Com(3,87),
            Ptr(0, true, true),
        ], 
        vec![ // 93 
            Com(2,90),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp46
        vec![ // 94 
            Prm(LT,false),
            Arg(2, false),
            Int(2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 95 
            Com(2,92),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 96 
            Com(4,84),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp47
        vec![ // 97 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 98 
            Com(4,94),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp48
        vec![ // 99 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp49
        vec![ // 100 
            Com(1,124),
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
        ], 
        // AExp50
        vec![ // 101 
            Prm(LE,false),
            Arg(5, false),
            Arg(1, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 102 
            Arg(3, true),
            Arg(5, false),
        ], 
        vec![ // 103 
            Prm(LE,false),
            Arg(4, true),
            Arg(1, false),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp51
        vec![ // 104 
            Arg(1, true),
            Int(0),
            Arg(0, true),
        ], 
        // AExp52
        vec![ // 105 
            Arg(2, true),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 106 
            Prm(Sub,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp53
        vec![ // 107 
            Arg(2, true),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp54
        vec![ // 108 
            Arg(3, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 109 
            Prm(Sub,false),
            Arg(1, true),
            Arg(0, true),
        ], 
        vec![ // 110 
            Prm(Add,false),
            Arg(2, true),
            Int(1),
        ], 
        // AExp55
        vec![ // 111 
            Prm(LE,false),
            Arg(0, false),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 112 
            Com(4,108),
            Arg(0, false),
            Arg(2, false),
            Arg(1, false),
        ], 
        vec![ // 113 
            Com(3,107),
            Arg(2, false),
            Arg(1, false),
        ], 
        // AExp56
        vec![ // 114 
            Prm(Add,false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp57
        vec![ // 115 
            Com(3,111),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 116 
            Com(1,114),
            Arg(1, true),
        ], 
        // AExp58
        vec![ // 117 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 118 
            Com(2,115),
            Arg(1, true),
        ], 
        // AExp59
        vec![ // 119 
            Com(6,101),
            Ptr(3, true, true),
            Arg(0, false),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 120 
            Prm(Add,false),
            Arg(2, false),
            Arg(2, false),
        ], 
        vec![ // 121 
            Com(3,117),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 122 
            Com(3,105),
            Arg(0, false),
            Arg(2, false),
        ], 
        vec![ // 123 
            Com(2,104),
            Arg(0, false),
        ], 
        // AExp60
        vec![ // 124 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 125 
            Com(3,119),
            Arg(0, true),
        ], 
        // AExp61
        vec![ // 126 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp62
        vec![ // 127 
            Arg(3, true),
            Int(5),
            Ptr(0, true, true),
        ], 
        vec![ // 128 
            Com(4,126),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp63
        vec![ // 129 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 130 
            Prm(Sub,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp64
        vec![ // 131 
            Prm(EQ,false),
            Arg(2, true),
            Int(3),
            Int(0),
            Ptr(0, true, true),
        ], 
        vec![ // 132 
            Com(3,129),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp65
        vec![ // 133 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 134 
            Com(1,157),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp66
        vec![ // 135 
            Prm(LT,false),
            Arg(2, false),
            Int(3),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 136 
            Com(3,133),
            Arg(0, false),
            Arg(1, false),
            Arg(3, false),
        ], 
        vec![ // 137 
            Com(4,131),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp67
        vec![ // 138 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 139 
            Com(2,159),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp68
        vec![ // 140 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 141 
            Prm(Add,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp69
        vec![ // 142 
            Prm(LT,false),
            Arg(2, true),
            Int(1),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 143 
            Com(3,140),
            Arg(0, false),
            Arg(1, false),
            Arg(3, false),
        ], 
        vec![ // 144 
            Com(3,138),
            Arg(0, false),
            Arg(1, false),
            Arg(3, false),
        ], 
        // AExp70
        vec![ // 145 
            Prm(LT,false),
            Arg(2, false),
            Int(2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 146 
            Com(4,142),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 147 
            Com(4,135),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp71
        vec![ // 148 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 149 
            Com(4,145),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp72
        vec![ // 150 
            Prm(EQ,false),
            Arg(2, false),
            Int(1),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 151 
            Com(1,124),
            Arg(2, false),
            Int(2),
            Arg(0, true),
        ], 
        // AExp73
        vec![ // 152 
            Com(1,157),
            Ptr(0, true, true),
        ], 
        vec![ // 153 
            Prm(Add,false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp74
        vec![ // 154 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 155 
            Prm(EQ,false),
            Arg(2, true),
            Int(0),
            Arg(0, false),
            Int(0),
        ], 
        vec![ // 156 
            Com(1,152),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp75
        vec![ // 157 
            Com(3,150),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 158 
            Com(3,154),
            Arg(0, false),
        ], 
        // AExp76
        vec![ // 159 
            Com(1,124),
            Arg(0, true),
            Arg(1, true),
            Com(2,0),
        ], 
        // AExp77
        vec![ // 160 
            Arg(0, true),
            Int(0),
            Com(1,0),
        ], 
        // AExp78
        vec![ // 161 
            Arg(0, true),
            Int(3),
            Com(1,0),
        ], 
        // AExp79
        vec![ // 162 
            Arg(0, true),
            Int(2),
            Com(1,0),
        ], 
        // AExp80
        vec![ // 163 
            Arg(0, true),
            Int(1),
            Com(1,0),
        ], 
        // AExp81
        vec![ // 164 
            Arg(2, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 165 
            Com(4,2),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp82
        vec![ // 166 
            Com(1,184),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 167 
            Com(1,178),
            Arg(1, true),
        ], 
        vec![ // 168 
            Com(2,191),
            Arg(0, true),
        ], 
        // AExp83
        vec![ // 169 
            Arg(1, true),
            Ptr(0, true, true),
            Com(1,0),
        ], 
        vec![ // 170 
            Com(4,2),
            Arg(0, true),
        ], 
        // AExp84
        vec![ // 171 
            Com(2,166),
            Ptr(0, true, true),
        ], 
        vec![ // 172 
            Com(2,169),
            Arg(0, true),
        ], 
        // AExp85
        vec![ // 173 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 174 
            Com(1,171),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 175 
            Com(3,164),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp86
        vec![ // 176 
            Com(1,49),
            Arg(1, false),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 177 
            Com(2,173),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp87
        vec![ // 178 
            Arg(0, true),
            Com(2,0),
            Com(2,176),
        ], 
        // AExp88
        vec![ // 179 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 180 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 181 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp89
        vec![ // 182 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 183 
            Com(4,179),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp90
        vec![ // 184 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 185 
            Com(3,182),
            Arg(0, true),
        ], 
        // AExp91
        vec![ // 186 
            Arg(4, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 187 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 188 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp92
        vec![ // 189 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 190 
            Com(5,186),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp93
        vec![ // 191 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 192 
            Com(3,189),
            Arg(1, true),
        ], 
        // AExp94
        vec![ // 193 
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp95
        vec![ // 194 
            Arg(1, true),
            Int(4),
            Ptr(0, true, true),
        ], 
        vec![ // 195 
            Com(2,193),
            Arg(0, true),
        ], 
        // AExp96
        vec![ // 196 
            Com(1,15),
            Com(1,201),
            Ptr(0, true, true),
        ], 
        vec![ // 197 
            Com(1,224),
            Arg(0, true),
        ], 
        // AExp97
        vec![ // 198 
            Com(1,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 199 
            Com(1,201),
            Arg(1, true),
        ], 
        vec![ // 200 
            Com(1,216),
            Arg(0, true),
        ], 
        // AExp98
        vec![ // 201 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,198),
        ], 
        vec![ // 202 
            Com(4,2),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp99
        vec![ // 203 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 204 
            Com(4,2),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp100
        vec![ // 205 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 206 
            Com(4,2),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp101
        vec![ // 207 
            Com(1,184),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 208 
            Arg(0, true),
            Arg(2, true),
        ], 
        vec![ // 209 
            Com(4,2),
            Arg(1, true),
        ], 
        // AExp102
        vec![ // 210 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 211 
            Com(3,207),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 212 
            Com(3,205),
            Arg(0, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp103
        vec![ // 213 
            Arg(2, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 214 
            Com(4,210),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 215 
            Com(1,203),
            Arg(0, false),
        ], 
        // AExp104
        vec![ // 216 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 217 
            Com(3,213),
            Arg(0, true),
        ], 
        // AExp105
        vec![ // 218 
            Com(1,184),
            Ptr(0, true, true),
        ], 
        vec![ // 219 
            Com(4,2),
            Arg(0, true),
        ], 
        // AExp106
        vec![ // 220 
            Com(2,21),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 221 
            Com(1,218),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp107
        vec![ // 222 
            Com(2,220),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 223 
            Com(1,224),
            Arg(1, true),
        ], 
        // AExp108
        vec![ // 224 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,222),
        ], 
        vec![ // 225 
            Com(4,2),
            Com(2,0),
            Com(2,0),
        ], 
    ],

}});
