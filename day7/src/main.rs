use std::collections::HashMap;

use indoc::indoc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(input: char) -> Self {
        match input {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Unknown card {}", input)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn kind(&self) -> HandKind {
        let mut groups: HashMap<&Card, u8> = HashMap::new();

        for card in &self.cards {
            *groups.entry(card).or_insert(0) += 1;
        }

        if groups.len() > 1 {
            let &best_card = groups
                .keys()
                .filter(|&&card| *card != Card::Joker)
                .max_by_key(|&&card| groups.get(card).copied().unwrap_or(0))
                .unwrap();

            let joker_count = groups
                .get(&Card::Joker)
                .copied()
                .unwrap_or(0);

            groups.entry(best_card).and_modify(|count| *count += joker_count);
            groups.remove(&Card::Joker);
        }

        let mut counts = groups
            .into_iter()
            .map(|(_, count)| count)
            .collect::<Vec<_>>();

        counts.sort();
        counts.reverse();

        match counts.as_slice() {
            [5] => HandKind::FiveOfAKind,
            [4, 1] => HandKind::FourOfAKind,
            [3, 2] => HandKind::FullHouse,
            [3, 1, 1] => HandKind::ThreeOfAKind,
            [2, 2, 1] => HandKind::TwoPair,
            [2, 1, 1, 1] => HandKind::OnePair,
            [1, 1, 1, 1, 1] => HandKind::HighCard,
            _ => unreachable!(),
        }
    }
}

struct One<'a>(&'a str);
struct Two<'a>(&'a str);

impl<'a> From<One<'a>> for Hand {
    fn from(input: One<'a>) -> Self {
        let mut card_chars = input.0.chars();
        let cards = [
            Card::from(card_chars.next().unwrap()),
            Card::from(card_chars.next().unwrap()),
            Card::from(card_chars.next().unwrap()),
            Card::from(card_chars.next().unwrap()),
            Card::from(card_chars.next().unwrap()),
        ];

        Hand { cards }
    }
}

impl<'a> From<Two<'a>> for Hand {
    fn from(input: Two<'a>) -> Self {
        let mut hand = Hand::from(One(input.0));

        for card in &mut hand.cards {
            if *card == Card::Jack {
                *card = Card::Joker;
            }
        }

        hand
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_kind = self.kind();
        let other_kind = other.kind();

        if self_kind != other_kind {
            Some(self_kind.cmp(&other_kind))
        } else {
            Some(self.cards.cmp(&other.cards))
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    hand: Hand,
    bet: usize
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl<'a> From<One<'a>> for Game {
    fn from(value: One<'a>) -> Self {
        let (hand, bet) = value.0.split_at(5);

        Game {
            hand: Hand::from(One(hand.trim())),
            bet: bet
                .trim()
                .parse()
                .expect("Could not parse bet"),
        }
    }
}

impl<'a> From<Two<'a>> for Game {
    fn from(value: Two<'a>) -> Self {
        let (hand, bet) = value.0.split_at(5);

        Game {
            hand: Hand::from(Two(hand.trim())),
            bet: bet
                .trim()
                .parse()
                .expect("Could not parse bet"),
        }
    }
}

#[derive(Debug)]
struct Games(Vec<Game>);

impl Games {
    pub fn total_winnings(self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(rank, game)| (rank + 1) * game.bet)
            .sum()
    }
}

impl<'a> From<One<'a>> for Games {
    fn from(value: One<'a>) -> Self {
        let mut games: Vec<_> = value.0
            .lines()
            .map(|line| Game::from(One(line)))
            .collect();

        games.sort();

        Self(games)
    }
}

impl<'a> From<Two<'a>> for Games {
    fn from(value: Two<'a>) -> Self {
        let mut games: Vec<_> = value.0
            .lines()
            .map(|line| Game::from(Two(line)))
            .collect();

        games.sort();

        Self(games)
    }
}

fn main() {
    let input = indoc!{"
        8444T 864
        6TK4Q 440
        A5555 197
        53353 712
        6K68A 216
        J6A33 975
        J753Q 772
        22776 977
        J4494 215
        A25JJ 101
        AAA3A 637
        TT3KK 1000
        T7T67 895
        8A88J 928
        TTKKT 29
        TJJQ4 719
        333JT 489
        Q9AQQ 492
        TK384 543
        55J33 897
        Q26QQ 549
        TTKK5 254
        T99T9 530
        87888 851
        27828 896
        J9J99 680
        K7KA2 843
        J3749 432
        7TA49 839
        T4466 266
        77QQ7 534
        44AAA 540
        Q4Q44 330
        7TK88 736
        T7479 705
        22JQ2 522
        77227 597
        5555T 296
        33348 203
        A2624 767
        8TK88 41
        4TK7A 869
        6J76K 581
        7T47T 629
        9QA49 302
        KK7K7 588
        8Q93J 872
        A97Q4 235
        A4A45 343
        J3347 755
        4T444 909
        9A999 125
        K88QJ 707
        74474 953
        ATQQT 155
        32242 569
        44888 634
        22429 882
        2QA84 602
        TQTQ4 974
        J9996 392
        5852Q 601
        K5444 833
        85J35 763
        T8282 504
        4KAKK 539
        2233T 68
        AA8AQ 223
        326A3 208
        T955T 319
        A3Q3A 776
        56QKA 230
        T63T6 566
        QA36Q 923
        8J88J 686
        9K5T7 93
        J344J 321
        AA8AA 559
        88777 448
        7667K 487
        T4T7J 476
        8Q4Q5 542
        KK555 699
        7J777 888
        335A3 284
        KQJA9 764
        5684J 956
        288K8 866
        TAJ49 697
        T57T7 727
        47477 980
        78T33 682
        688J9 344
        QKKKK 418
        93KA3 55
        9T58Q 493
        2J22J 39
        J7234 917
        JJ225 100
        4TTTK 854
        93965 676
        A97K3 891
        66TK6 423
        42KTT 457
        79957 983
        3Q3K3 904
        KKK7J 382
        8T8JT 290
        29292 596
        J77JT 219
        27292 25
        3TT36 533
        TTTTA 962
        3QQQ4 38
        T962K 210
        5TK28 571
        555K5 303
        8QKKQ 426
        TJ3TT 865
        5579A 373
        35AA5 954
        444J4 908
        39939 437
        722J2 582
        4442Q 969
        89888 470
        KKTTK 70
        K38AJ 659
        47AA9 314
        A3A3A 307
        TTT72 605
        29JQ7 366
        558J8 580
        684T6 523
        6JTTT 951
        8Q4JT 44
        KKQKJ 903
        K96QQ 374
        4TT5T 165
        AT4J2 300
        93A54 621
        JJ6QQ 214
        79797 349
        QQA88 23
        KKJ8J 679
        AK8K3 479
        6K7Q6 384
        K6KK9 644
        K64J6 297
        36J95 502
        8AJTA 498
        5Q54T 9
        77T37 436
        222KK 363
        KTAT5 989
        2AAJJ 496
        55TT4 170
        2T9JQ 113
        37333 972
        5A8J3 40
        89T98 428
        6KA2J 748
        23733 445
        27QAJ 616
        4TT44 280
        54K4K 814
        59335 472
        86888 354
        558T4 407
        35683 347
        AA4A5 431
        943Q5 671
        JTTJ2 808
        6J6J6 76
        AKKKK 557
        JQJ3Q 610
        TK392 435
        5455Q 973
        4JKKK 714
        2T284 561
        A6A24 332
        4332J 514
        69873 859
        45488 441
        39285 648
        AQ333 753
        AA3Q5 433
        2AJT2 355
        84K4J 890
        A8T26 312
        55955 704
        33233 813
        9J959 984
        52KK5 998
        J88KK 906
        9TAK5 352
        T8TTA 464
        A47AJ 880
        2Q22Q 205
        J228J 817
        AA77A 790
        QQ99Q 701
        44244 477
        J6T2K 841
        K492J 408
        82288 873
        97623 996
        6633A 613
        6AAAA 668
        535K3 110
        6KK8T 279
        54KKA 721
        66268 528
        8J3J5 829
        88983 239
        J5775 78
        KKK3K 402
        8Q656 74
        66T68 612
        8J572 121
        23T85 83
        76877 757
        JT88J 661
        83383 722
        4J4T4 116
        598K5 791
        33J56 249
        333Q9 603
        5KK2K 177
        66667 565
        88Q88 225
        79T99 1
        592Q4 823
        TQT9T 131
        24874 628
        22732 99
        7T847 328
        A4T96 128
        J6954 2
        6JT66 642
        7JJ44 840
        2379T 372
        88QJQ 509
        39K9J 406
        33363 212
        8A86A 473
        J9QQQ 109
        64K6K 684
        K2Q45 510
        826JT 172
        98968 560
        QQ97Q 905
        K93JK 669
        J5AA4 844
        24258 780
        64796 45
        22TK5 647
        23223 825
        7Q77J 822
        4A4AJ 229
        A9699 845
        QTKQ6 796
        5K47T 591
        TKJQ7 48
        49499 282
        T5JTK 17
        4JQ22 54
        8QT88 221
        TAAAT 273
        26Q97 527
        44J84 313
        T38A6 988
        7272Q 734
        AJ22A 512
        T8AJT 959
        KTKKJ 315
        7A787 461
        AA55Q 189
        QAT4A 625
        QJ55T 122
        8K696 463
        T6966 422
        KKK9K 262
        828KT 778
        2KK73 608
        9KK9K 333
        242K4 242
        QJ7AQ 47
        TT7AA 690
        8688J 94
        QJQQA 305
        35T35 795
        344A5 186
        Q8444 385
        34K24 270
        QK888 227
        2J757 286
        TA28Q 218
        2TTQQ 104
        64J4A 209
        39933 877
        265Q5 449
        9K939 142
        KKJKJ 86
        A77A9 304
        JKAAK 876
        8989Q 857
        T5556 234
        7K7JK 886
        J8828 507
        436Q3 192
        9J4KK 72
        32954 563
        22AAA 12
        9TT96 703
        62928 233
        J4A4J 98
        4244A 164
        88J88 84
        JQQJ5 511
        4556J 169
        QQQJQ 781
        422J2 696
        76K74 815
        AAJ7A 276
        56666 213
        QA22J 585
        JAJAA 746
        TJ562 245
        T556Q 999
        566J5 853
        26KK6 801
        2735J 167
        5QJ55 782
        77473 646
        2T37T 694
        Q4Q45 641
        K66KK 615
        3AQ2K 151
        KKKKT 154
        98788 90
        44669 708
        88A8A 188
        76864 345
        J7767 379
        JJJJJ 739
        75J7A 377
        TJ72J 939
        72KKK 471
        7Q7Q2 394
        9665J 194
        77977 526
        75AA2 92
        535K5 459
        Q8KKT 430
        9TK39 981
        26555 350
        AKK97 417
        44J47 46
        89777 593
        J525T 657
        KKJ23 847
        44492 96
        88A86 922
        T989A 462
        7KKKK 298
        9J9Q9 89
        K7JA7 486
        Q5384 322
        74TQ2 335
        KQQQK 670
        5Q8A6 396
        QQ4QQ 263
        336J3 340
        QQ2QQ 190
        2TT22 285
        5T5QT 294
        9979J 805
        7227T 255
        J49AA 132
        454J4 201
        K29J5 752
        58555 553
        666J6 447
        Q9999 176
        3377J 152
        229A7 334
        2K322 698
        37764 460
        T73JA 633
        2Q9J6 568
        TQJQQ 737
        44555 809
        K42K2 232
        42A47 420
        3A8T9 124
        56556 301
        JQQQJ 243
        6KKKK 537
        98243 832
        6266J 769
        K5J74 339
        7JQJ7 362
        KAK46 666
        K23TK 144
        2T2KQ 645
        69QT4 811
        A9J55 994
        78QT8 424
        QJQQ8 860
        9QTJT 619
        2TTKJ 105
        JAQ2A 91
        4Q334 22
        34444 631
        77477 685
        67777 383
        Q6QQ6 914
        8J699 184
        44484 965
        67667 623
        448T9 598
        35T98 453
        22246 389
        9KA57 723
        5Q2A3 20
        JAA35 740
        56384 728
        3Q4KA 398
        3KK33 756
        8TTJ7 849
        T4J34 59
        22Q27 111
        J7J7J 709
        7772J 742
        J3666 43
        QQ367 28
        653K6 797
        55J65 158
        AA43A 88
        J4QA2 49
        99TK9 554
        4J244 367
        3Q4J8 518
        J5495 624
        6AK88 819
        5T999 609
        85585 415
        57K5K 874
        33Q37 870
        6A6JA 541
        685J6 913
        JKK25 195
        AKAKK 693
        54525 516
        4464K 231
        726K8 244
        J3393 544
        88638 35
        55553 380
        K5K37 875
        5558T 688
        9A975 821
        AAA6T 586
        25AAA 689
        44744 863
        6AKJ9 178
        78868 848
        748J8 816
        A79QQ 900
        K4927 968
        6KKT6 336
        QQ555 525
        24J42 546
        4T982 58
        J367J 114
        62K39 341
        TQ382 120
        K3Q3Q 491
        K33JQ 485
        8323Q 627
        J3353 838
        2K8TA 283
        JQTQ3 198
        3AAJJ 425
        5AJ3J 159
        TQ3JJ 361
        84TA3 444
        88KJ8 108
        772T5 480
        532A2 11
        55T59 331
        8JA4T 481
        TTAAT 427
        6J57Q 157
        JAAA5 21
        58355 730
        8AA67 145
        2TJT2 548
        5522A 238
        A5AAA 468
        Q252K 660
        K6A49 807
        32432 16
        22239 375
        T8J48 30
        Q98K4 325
        8J76Q 308
        55525 324
        Q4KT4 61
        JK6AA 861
        78448 148
        665AT 798
        77999 403
        JJ8JJ 378
        24642 395
        KJ6J8 978
        K6KK8 438
        422J7 717
        A4K86 990
        QJQ9T 715
        K4KKK 318
        87867 777
        42857 80
        AT727 576
        88KK8 376
        33JJ3 31
        K554K 920
        T2Q4A 454
        5J527 620
        784A5 248
        5KKK5 855
        AAAJ6 293
        JATAQ 291
        2J239 885
        555AA 271
        AJAKA 532
        QAQQQ 211
        K473A 649
        82AA9 306
        A6666 639
        K3K66 578
        T5557 931
        A4444 187
        865J5 881
        79669 123
        6AJQQ 173
        83878 277
        KK559 521
        3KKK8 37
        33A88 547
        T678Q 731
        33994 919
        TT33T 185
        J9999 397
        6J58Q 687
        7782J 183
        2Q3KJ 929
        8AQ88 81
        698T4 370
        AAAQQ 337
        46665 478
        2A299 733
        4J44A 945
        AJ725 60
        A6K43 749
        QQAA6 775
        Q8738 515
        KK999 944
        J8899 946
        347AQ 103
        94999 729
        KKKKJ 357
        62JQ8 474
        2422A 834
        66263 741
        QKQKK 937
        9JK3Q 412
        TTJT2 943
        22263 960
        A4372 329
        833QQ 735
        2JA6J 577
        86452 744
        Q8884 204
        AAAQA 656
        742Q4 259
        T6666 53
        A7733 930
        K4K4K 51
        64TT2 827
        J4472 950
        55K56 381
        8QQQQ 799
        979T6 134
        AJATT 52
        8KQJ4 241
        82AK7 338
        23222 268
        QQ77Q 73
        6665A 136
        A9282 587
        862T2 558
        K444J 434
        66565 747
        2K2QA 153
        89889 14
        J9933 309
        523T9 323
        7A87J 106
        7K6K6 614
        38T97 119
        J45J6 787
        9AJ69 199
        78Q5J 818
        3T3KK 8
        TT9TT 710
        77736 771
        A2TQJ 64
        95559 278
        844QQ 246
        666K6 508
        TT6T6 779
        TT7T7 692
        6AAA6 970
        KJ8K7 933
        76J66 966
        7K57K 986
        2T9J6 364
        TT88T 912
        9K4A5 488
        A65A5 206
        A5TKQ 497
        3422Q 971
        38888 770
        25J85 174
        79775 429
        72T38 260
        874TK 257
        477J4 50
        3Q8QK 137
        999QK 146
        6JAQ2 955
        2T333 658
        892Q4 893
        4Q635 726
        7Q685 410
        J9386 852
        KK7KA 247
        77Q57 327
        88K88 898
        QA366 651
        2879J 483
        68886 269
        K7J78 702
        84K5T 831
        8KKKK 964
        4A774 720
        2KK62 400
        69J7Q 57
        J9JQA 264
        2K22T 513
        JK5T4 564
        23T6J 575
        43334 987
        3T596 570
        343J4 4
        63T4K 138
        QT57K 941
        J4792 992
        35339 892
        35KJ3 75
        3A74T 997
        8A8J7 643
        JQ55Q 884
        T3TT9 773
        TQTTT 716
        83AAA 935
        JT38J 957
        258AT 901
        73334 934
        7765J 732
        6KK96 222
        87KTA 942
        KJKT3 251
        A777J 925
        56862 63
        333Q8 590
        AA5A5 272
        Q6AQQ 979
        AK333 626
        84937 675
        J8555 236
        22522 789
        38JQ3 503
        45444 388
        TJK9A 168
        QTTKT 66
        88QQ8 405
        56223 141
        9K6KQ 638
        AA252 758
        77TTJ 995
        542A7 419
        KQ369 33
        87KKK 924
        3JJ88 390
        Q7777 475
        T9AJ6 360
        JTA9T 346
        J5555 126
        279JJ 691
        42TJ8 82
        79Q99 317
        595J8 538
        TAAAJ 802
        5T7J3 482
        563A5 156
        T3933 311
        22Q26 812
        55775 524
        33TJ4 77
        7K555 149
        5Q428 665
        59QQ9 556
        345T8 824
        JA9K5 535
        TK9T8 220
        94J68 79
        8K942 458
        T39QJ 281
        63633 13
        7JJJ5 494
        QJ24Q 115
        QQ5QJ 724
        38JK8 725
        QKJKJ 743
        JQQ6Q 500
        54Q49 800
        JK987 292
        T33JA 224
        833J3 351
        3333J 711
        73T3T 583
        7TQ6T 171
        J665T 766
        3K77K 850
        QT4K7 501
        KK939 915
        99QJ8 754
        3K333 288
        3KT5Q 759
        634TA 804
        86T8T 369
        48777 820
        Q3792 5
        J7425 948
        66J2K 295
        T269T 794
        8948Q 828
        Q452J 450
        46T65 42
        23JTT 695
        KK7Q3 469
        T4433 112
        5JJ76 611
        KK3Q3 520
        32A85 750
        87887 921
        A27A2 760
        Q96A9 358
        TT82T 196
        8A4A8 252
        T8887 365
        66644 862
        7538J 630
        25JJ5 636
        A9889 664
        45T2Q 287
        K8227 368
        9AAAA 26
        82K23 250
        5KJ8A 842
        333Q3 143
        66287 443
        87K94 894
        8KJJ7 536
        88882 253
        33AA3 768
        T8K5T 466
        T9944 256
        43944 150
        66622 745
        3337J 918
        JTT83 936
        3A379 421
        58A55 926
        AJT47 713
        7JT78 738
        3Q58T 677
        T33JJ 275
        66TT8 572
        66636 765
        TJT55 751
        33666 289
        66222 632
        AA5A7 706
        7T2KK 202
        A979J 7
        88KKK 127
        QQQ33 495
        93996 837
        8882Q 856
        93T86 452
        8A8AA 27
        5975J 835
        789KA 947
        J47Q8 413
        AJ758 506
        62966 567
        A664J 409
        J2222 993
        Q5557 455
        99T99 10
        62825 207
        K98JJ 786
        8JAT9 442
        QTQTQ 887
        K7747 889
        57577 439
        KJK44 228
        7T7J7 574
        9TKT4 310
        KQK8K 97
        2J283 24
        QTTQT 465
        22822 604
        TT488 36
        25K59 662
        8Q8QQ 65
        QQQ55 193
        59A28 401
        KA9A9 69
        TATTJ 902
        4Q4QQ 356
        KKK3Q 552
        375KA 490
        6KK6J 161
        Q666J 672
        25A55 683
        AJAAA 180
        A9494 391
        JAAJT 387
        JJ229 62
        22835 595
        JT534 160
        73854 868
        TT7TT 217
        KK587 607
        7KQJ3 451
        97J77 985
        24924 118
        Q6TKA 899
        77877 667
        KA5J5 326
        9939T 640
        3Q522 653
        4429T 846
        Q3AQA 56
        T76JA 761
        57474 237
        QQJ44 878
        J5599 166
        429QK 414
        4J2JT 499
        66669 961
        77TK7 200
        6TQ27 635
        86A68 718
        39TJ2 261
        KTJJK 393
        66JK6 584
        2J7K8 618
        TTTQ7 107
        A59A8 274
        92J49 579
        Q7Q78 938
        33933 320
        Q7TJ4 594
        5QK23 163
        6QQQQ 952
        86666 674
        74554 793
        337JT 911
        3TAT7 19
        KQ22J 267
        2222A 681
        TKTKJ 102
        64622 663
        2J2A7 932
        J8836 545
        TTJJT 617
        232J6 87
        Q244J 967
        AJQ8Q 258
        74799 785
        Q9339 529
        7TJ73 95
        47K3J 139
        Q6767 810
        TTTT6 353
        66TKJ 226
        99995 673
        859T8 655
        23J2A 34
        Q9Q99 600
        9T98J 133
        444A6 573
        A7TA5 803
        7A275 416
        799T7 963
        222J9 879
        K2KAK 467
        AKA66 505
        6ATJJ 916
        23393 140
        22QQ7 927
        QTTJT 179
        954TJ 135
        42698 606
        KT625 342
        7K77J 3
        4A898 788
        6A6A2 147
        44AA4 871
        6K6K6 907
        9J696 991
        4Q5TA 240
        QJQ99 958
        72934 700
        Q696T 949
        77QQ4 806
        TTT22 652
        KK5T5 910
        Q333Q 181
        9J6AJ 883
        63433 858
        TK8Q7 792
        99T2J 867
        QQ7KK 359
        A7A77 130
        JKJ5J 411
        53455 826
        69A2Q 774
        56948 784
        95TA4 650
        299T2 599
        68884 399
        46Q4Q 836
        J828T 622
        J77J7 175
        293A8 762
        8JJ62 940
        4J44J 85
        83333 551
        6A78J 562
        885JJ 456
        83866 592
        TT4T2 191
        KJ66A 117
        TTTTK 162
        7598T 982
        TQT2T 654
        53K82 348
        A9979 386
        K442K 783
        47A4A 517
        J544K 550
        JTAA2 371
        7336A 182
        9QKT8 6
        K5KQQ 299
        JKKJA 15
        555TT 484
        5J5J5 265
        5555Q 519
        22QQQ 555
        J3QQ3 589
        TJTTT 67
        7A777 32
        QQQQ7 446
        64446 830
        7T973 531
        22428 404
        8Q928 678
        AK8A9 316
        99992 129
        48744 976
        9A9KK 18
        TT9T9 71
    "};

    let p1 = Games::from(One(input));
    println!("Part 1: {}", p1.total_winnings());

    let p2 = Games::from(Two(input));
    println!("Part 2: {}", p2.total_winnings());
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::{Card, Hand, HandKind, Games, One, Two};

    #[test]
    fn ord() {
        let stronger = Hand {
            cards: [Card::Three, Card::Three, Card::Three, Card::Three, Card::Two],
        };

        let weaker = Hand {
            cards: [Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
        };

        assert!(stronger > weaker);

        let stronger = Hand {
            cards: [Card::Seven, Card::Seven, Card::Eight, Card::Eight, Card::Eight],
        };

        let weaker = Hand {
            cards: [Card::Seven, Card::Seven, Card::Seven, Card::Eight, Card::Eight],
        };

        assert!(stronger > weaker);
    }

    #[test]
    fn kind() {
        let five_of_a_kind = Hand {
            cards: [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
        };

        assert_eq!(five_of_a_kind.kind(), HandKind::FiveOfAKind);

        let four_of_a_kind = Hand {
            cards: [Card::Ace, Card::Ace, Card::Eight, Card::Ace, Card::Ace],
        };

        assert_eq!(four_of_a_kind.kind(), HandKind::FourOfAKind);

        let full_house = Hand {
            cards: [Card::Two, Card::Three, Card::Three, Card::Three, Card::Two],
        };

        assert_eq!(full_house.kind(), HandKind::FullHouse);

        let two_pair = Hand {
            cards: [Card::Two, Card::Three, Card::Four, Card::Three, Card::Two],
        };

        assert_eq!(two_pair.kind(), HandKind::TwoPair);

        let one_pair = Hand {
            cards: [Card::Ace, Card::Two, Card::Three, Card::Ace, Card::Four],
        };

        assert_eq!(one_pair.kind(), HandKind::OnePair);

        let high_card = Hand {
            cards: [Card::Two, Card::Three, Card::Four, Card::Five, Card::Six],
        };

        assert_eq!(high_card.kind(), HandKind::HighCard);
    }

    #[test]
    fn total_winnings() {
        let input = indoc!{"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        let games = Games::from(One(input));

        assert_eq!(games.total_winnings(), 6440);
        println!("--");

        let games = Games::from(Two(input));

        assert_eq!(games.total_winnings(), 5905)
    }
}
