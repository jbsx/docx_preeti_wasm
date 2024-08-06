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

    //TODO: test coverage

    #[test]
    fn temp_test() {
        let test_string = "ट्रक".to_owned();
        let control = "6«s".to_owned();

        let converted = unicode_to_preeti(test_string);

        assert_eq!(converted, control);
    }

    //#[test]
    //fn temp_test2() {
    //    let test_string = "क्रम".to_owned();
    //    let control = "qmd".to_owned();

    //    let converted = unicode_to_preeti(test_string);

    //    assert_eq!(converted, control);
    //}

    #[test]
    fn temp_test3() {
        let test_string = "खुट्टा".to_owned();
        let control = "v'6\\6f".to_owned();

        let converted = unicode_to_preeti(test_string);

        assert_eq!(converted, control);
    }
    #[test]
    fn post_rules24() {
        let test_string = "जाऊ".to_owned();
        let control = "hfpm".to_owned();

        let converted = unicode_to_preeti(test_string);

        assert_eq!(converted, control);
    }
    #[test]
    fn post_rules25() {
        let test_string = "जाऊँ".to_owned();
        let control = "hfpmF".to_owned();

        let converted = unicode_to_preeti(test_string);

        assert_eq!(converted, control);
    }
    #[test]
    fn post_rules34() {
        let test_string = "जाऔं".to_owned();
        let control = "hfcf}+".to_owned();

        let converted = unicode_to_preeti(test_string);

        assert_eq!(converted, control);
    }
}
