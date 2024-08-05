#[cfg(test)]
mod test_preeti_converter {
    use crate::preeti_to_unicode;

    #[test]
    fn post_rules() {
        let test_string = "klxrfg".to_owned();
        let control: String = "पहिचान".to_owned();

        let converted = preeti_to_unicode(test_string);

        assert_eq!(control, converted);
    }

    #[test]
    fn sign_nukta() {
        let test_string = "gfhÞ".to_owned();
        let control: String = "नाज़".to_owned();

        let converted = preeti_to_unicode(test_string);

        assert_eq!(control, converted);
    }

    #[test]
    fn half_ra() {
        let test_string = "ubf{ub}{".to_owned();
        let control: String = "गर्दागर्दै".to_owned();

        let converted = preeti_to_unicode(test_string);

        assert_eq!(control, converted);
    }

    #[test]
    fn half_ra_p2() {
        let test_string = "jfld{+usf]".to_owned();
        let control: String = "वार्मिंगको".to_owned();

        let converted = preeti_to_unicode(test_string);

        assert_eq!(control, converted);
    }

    #[test]
    fn test_html_entity() {
        let test_string = "ckg ;dfhs] sf]gf 3/d] a]6Ls] hgd x]nf;] ck;u'0f cf/ 3[0ff s/n hfo x}o . PsfO{;f} ztfAbLd] rn /xn ;dfh Pvlgof] 3/d] klxnsf af/] a]6f hGd e]nfk/ ef]h et]/ dgfjn hfo x} . jx] hux j]6Ls] hGd e]nf;] …3/d] nIdLÚ s] cfudg xf]on x}o sxn hfo x} n]lsg dgd] j]6f xj}s] OR5f bjfs] 5f]6df]6 ef]het]/ cf/ ljwL ljwfg s/n hfo x} . ha ls O ;dfhs] yfxf gxo gf/L lx O &gt;[i6Ls] gf/L xL rfnj x}o . ".to_string();
        let control = "अपन समाजके कोना घरमे बेटीके जनम हेलासे अपसगुण आर घृणा करल जाय हैय । एकाईसौ शताब्दीमे चल रहल समाज एखनियो घरमे पहिलका बारे बेटा जन्म भेलापर भोज भतेर मनावल जाय है । वहे जगह वेटीके जन्म भेलासे ‘घरमे लक्ष्मी’ के आगमन होयल हैय कहल जाय है लेकिन मनमे वेटा हवैके इच्छा दवाके छोटमोट भोजभतेर आर विधी विधान करल जाय है । जब कि इ समाजके थाहा नहय नारी हि इ श्रृष्टीके नारी ही चालव हैय । ".to_string();

        let converted = preeti_to_unicode(test_string);

        assert_eq!(control, converted);
    }
}

#[cfg(test)]
mod test_unicode_converter {
    use crate::unicode_to_preeti;

    //#[test]
    //fn post_rules() {
    //    let test_string = "नेपाल".to_owned();
    //    let control = "g]kfn".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}

    //#[test]
    //fn post_rules2() {
    //    let test_string = "नेपाली".to_owned();
    //    let control = "g]kfnL".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}

    //#[test]
    //fn temp_test() {
    //    let test_string = "ट्रक".to_owned();
    //    let control = "6«s".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //
    //#[test]
    //fn temp_test2() {
    //    let test_string = "क्रम".to_owned();
    //    let control = "qmd".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}

    //#[test]
    //fn temp_test() {
    //    let test_string = "खुट्टा".to_owned();
    //    let control = "v'§f".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}

    //#[test]
    //fn post_rules3() {
    //    let test_string = "चन्द".to_owned();
    //    let control = "rGb".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules4() {
    //    let test_string = "राष्ट्रिय".to_owned();
    //    let control = "/fli6«o".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules5() {
    //    let test_string = "विशेषण".to_owned();
    //    let control = "ljz]if0f".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules6() {
    //    let test_string = "गणेश".to_owned();
    //    let control = "u0f]z".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules7() {
    //    let test_string = "पाणि".to_owned();
    //    let control = "kfl0f".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules8() {
    //    let test_string = "गर्न".to_owned();
    //    let control = "ug{".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules9() {
    //    let test_string = "गर्ने".to_owned();
    //    let control = "ug{]".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules10() {
    //    let test_string = "गर्ने".to_owned();
    //    let control = "ug]{".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules11() {
    //    let test_string = "निमार्चोक".to_owned();
    //    let control = "lgdfrf]{s".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules12() {
    //    let test_string = "चन्द्रोदय".to_owned();
    //    let control = "rGb|f]bo".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules13() {
    //    let test_string = "ईश्वर".to_owned();
    //    let control = "O{Zj/".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules14() {
    //    let test_string = "फ्राई".to_owned();
    //    let control = "k|mfO{".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules15() {
    //    let test_string = "स्याफ्रुबेशी".to_owned();
    //    let control = ":ofk|m'a]zL".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules16() {
    //    let test_string = "आफ्नो".to_owned();
    //    let control = "cfkm\\gf]".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules17() {
    //    let test_string = "आफ्नौ".to_owned();
    //    let control = "cfk\\mgf}".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules18() {
    //    let test_string = "भक्त".to_owned();
    //    let control = "eQm".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules19() {
    //    let test_string = "झरना".to_owned();
    //    let control = "em/gf".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules20() {
    //    let test_string = "क्रुर".to_owned();
    //    let control = "s|'/".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules21() {
    //    let test_string = "क्रुर".to_owned();
    //    let control = "qm'/".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules22() {
    //    let test_string = "क्रुर".to_owned();
    //    let control = "q'm/".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules23() {
    //    let test_string = "प्रतिक्रिया".to_owned();
    //    let control = "k|ltlqmof".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules24() {
    //    let test_string = "जाऊ".to_owned();
    //    let control = "hfpm".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules25() {
    //    let test_string = "जाऊँ".to_owned();
    //    let control = "hfpFm".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules26() {
    //    let test_string = "झेल्नु".to_owned();
    //    let control = "e]mNg'".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules27() {
    //    let test_string = "झेल्नु".to_owned();
    //    let control = "em]Ng'".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules28() {
    //    let test_string = "झाक्री".to_owned();
    //    let control = "emfs|L".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules29() {
    //    let test_string = "पौवा".to_owned();
    //    let control = "kf}jf".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules30() {
    //    let test_string = "क्षेत्रपाटी".to_owned();
    //    let control = "If]qkf6L".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules31() {
    //    let test_string = "षोडशी".to_owned();
    //    let control = "iff]8zL".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules32() {
    //    let test_string = "ओखती".to_owned();
    //    let control = "cf]vtL".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules33() {
    //    let test_string = "ऐना".to_owned();
    //    let control = "P]gf".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules34() {
    //    let test_string = "जाऔं".to_owned();
    //    let control = "hfcf}+".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules35() {
    //    let test_string = "जाऔं".to_owned();
    //    let control = "hfcf+}".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules36() {
    //    let test_string = "घरमैं".to_owned();
    //    let control = "3/d+}".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules37() {
    //    let test_string = "(क)".to_owned();
    //    let control = "-s_".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules38() {
    //    let test_string = "१२३४५६७८९०".to_owned();
    //    let control = "!@#$%^&*()".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules39() {
    //    let test_string = "पर्‍यो".to_owned();
    //    let control = "k¥of]".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules40() {
    //    let test_string = "हेर्थ्यो".to_owned();
    //    let control = "x]Yof{]".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules41() {
    //    let test_string = "हेर्थ्यो".to_owned();
    //    let control = "x]Yof]{".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules42() {
    //    let test_string = "मद्दत".to_owned();
    //    let control = "d2t".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules43() {
    //    let test_string = "ऋषि".to_owned();
    //    let control = "Clif".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules44() {
    //    let test_string = "ट्रेक्स".to_owned();
    //    let control = "6]«S;".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules45() {
    //    let test_string = "प्रस्तुत".to_owned();
    //    let control = "k|:'tt".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules46() {
    //    let test_string = "बमोजिम".to_owned();
    //    let control = "ad]flhd".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules47() {
    //    let test_string = "बेमौसमी".to_owned();
    //    let control = "a]d}f;dL".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}

    //#[test]
    //fn post_rules48() {
    //    let test_string = "klxrfg".to_owned();
    //    let control = "पहिचान".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
    //#[test]
    //fn post_rules49() {
    //    let test_string = "हिमाल".to_owned();
    //    let control = "lxdfn".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}
}
