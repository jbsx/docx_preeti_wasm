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

    #[test]
    fn temp_test00() {
        let test_string = "स्वस्थ".to_owned();
        let control = ":j:y".to_owned();
        let converted = unicode_to_preeti(test_string);
        assert_eq!(converted, control);
    }

    #[test]
    fn temp_test01() {
        let test_string = "सिकिस्त".to_owned();
        let control = "l;ls:t".to_owned();
        let converted = unicode_to_preeti(test_string);
        assert_eq!(converted, control);
    }

    #[test]
    fn temp_test02() {
        let test_string = "प्रिय".to_owned();
        let control = "k|lo".to_owned();
        let converted = unicode_to_preeti(test_string);
        assert_eq!(converted, control);
    }

    #[test]
    fn temp_test03() {
        let test_string = "प्रिय मानव, मलाइ थाहा छ उता तिमी स्वस्थ छैनौं एता म पनि सिकिस्त बिरामी परेकि छु जताततै गाडी र कलकारखानाको बिषालु धुंवामा रूमलिएका मेरा धड्कनहरु बिस्तारैबिस्तारै अस्पतालका विस्तारामा पुगे उता तिमीहरुको जीवनशैली र मिसावटयुत्त आहारले पटकपटक अस्वस्थ हुनु आजभोलि म निकै चिंतित रहेकी छु आशा विहिन पनि भएकी छु" .to_owned();
        let control = "k|lo dfgj, dnfO yfxf 5 ptf ltdL :j:y 5}gf}+ Ptf d klg l;ls:t la/fdL k/]ls 5' htftt} uf8L / snsf/vfgfsf] laiffn' w'+jfdf ¿dlnPsf d]/f w8\\sgx? la:tf/}la:tf/} c:ktfnsf lj:tf/fdf k'u] ptf ltdLx?sf] hLjgz}nL / ld;fj6o'Q cfxf/n] k6sk6s c:j:y x'g' cfhef]ln d lgs} lr+ltt /x]sL 5' cfzf ljlxg klg ePsL 5'" .to_owned();
        let converted = unicode_to_preeti(test_string);
        assert_eq!(converted, control);
    }

    #[test]
    fn temp_test04() {
        let test_string = "गयौ?".to_owned();
        let control = "uof}<".to_owned();
        let converted = unicode_to_preeti(test_string);
        assert_eq!(converted, control);
    }

    #[test]
    fn temp_test06() {
        let test_string = "प्रभुजीले".to_owned();
        let control = "k|e'hLn]".to_owned();
        let converted = unicode_to_preeti(test_string);
        assert_eq!(converted, control);
    }

    #[test]
    fn temp_test05() {
        let test_string = "मैले तिमीहरुलाइ श्वास पेनं लाइ प्राणवायु, पिउन लाइ पानी, र बस्न लाइ खुला जमीन प्रदान गरें, आकाश बनेर संधैं संरक्षण गरिरहें तिमीहरुको तिखा लाइ तृप्त गदा गदा म आपै सुकेर चट्टान भएं".to_owned();
        let control = "d}n] ltdLx?nfO Zjf; k]g+ nfO k|f0fjfo', lkpg nfO kfgL, / a:g nfO v'nf hdLg k|bfg u/]+, cfsfz ag]/ ;+w}+ ;+/If0f ul//x]+ ltdLx?sf] ltvf nfO t[Kt ubf ubf d cfk} ;'s]/ r6\\6fg eP+".to_owned();
        let converted = unicode_to_preeti(test_string);
        assert_eq!(converted, control);
    }

    #[test]
    fn test_new_line() {
        let test_string = "\n".to_owned();
        let control = "\n".to_owned();
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
