/// <https://leetcode.cn/problems/can-make-palindrome-from-substring/>
struct Solution;

impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // 超时
        // queries
        //     .iter()
        //     .map(|query: &Vec<i32>| {
        //         let ss = &s.as_bytes()[query[0] as usize..(query[1] + 1) as usize];
        //         let mut hm = HashMap::new();
        //         ss.iter().for_each(|s| {
        //             *hm.entry(s).or_insert(0) += 1;
        //         });
        //         let diff = hm
        //             .values()
        //             .filter(|&&v| (v & 1) == 1)
        //             .count() as i32;
        //         if (ss.len() & 1) == 1 {
        //             diff <= query[2] * 2 + 1
        //         } else {
        //             diff <= query[2] * 2
        //         }
        //     })
        //     .collect()

        let n = s.len();
        let ss = s.as_bytes();
        let mut count = Vec::with_capacity(n);
        count.resize(n + 1, 0);
        for i in 0..n {
            count[i + 1] = count[i] ^ (1 << (ss[i] - b'a'));
        }
        queries
            .iter()
            .map(|query| {
                let l = query[0] as usize;
                let r = query[1] as usize;
                let mut bits = 0;
                let mut x = count[r + 1] ^ count[l];
                while x > 0 {
                    x &= x - 1;
                    bits += 1;
                }
                bits <= query[2] * 2 + 1
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::can_make_palindrome_from_substring::Solution;

    #[test]
    fn test_can_make_pali_queries() {
        let queries = vec![
            vec![3, 3, 0],
            vec![1, 2, 0],
            vec![0, 3, 1],
            vec![0, 3, 2],
            vec![0, 4, 1]
        ];
        let ans = Solution::can_make_pali_queries("abcda".to_string(), queries);
        assert_eq!(ans, vec![true, false, false, true, true]);

        let queries = vec![vec![0, 3, 1]];
        let ans = Solution::can_make_pali_queries("hunu".to_string(), queries);
        assert_eq!(ans, vec![true]);

        let queries = vec![vec![3193, 3223, 3]];
        let ans = Solution::can_make_pali_queries(
            "lybypsnwvozshezupkrubmrapgbsbininmjmjkbkjkvoxcpqrsvwfshmtulqrypyhofubmnylkrapqhgxgdofcvmrylqpejqbalahwryrkzavgdmdgtqpgpmjghexybyrgzczyhafcdqbgncrcbihkdmhunuzqrkzsnidwbunszulspmhwpazoxijwbqpapmretkborsrurgtinansnupotstmnkfcfavaxglyzebsbuxmtcfmtodclszghejevmhcvshclydqrulwbyhajgtargjctqvijshexyjcjcrepyzazexujqtsjebcnadahobwfuvirivwbkdijstyjgdahmtutavapazcdspcnolsvmlorqxazglyjqfmtclsfaxchgjavqrifqbkzspazwnczivetoxqjclwbwtibqvelwxsdazixefcvarevabmfabqfivodqfaluxqpcxwfkzyxabytijcnohgzgbchwpshwnufcvqfcnglshwpgxujwrylqzejmdubkxsbctsfwdelkdqzshupmrufyxklsjurevipyfobudkbgpqtadspgvinafefktctinyvgfkpurgrihwbkjsrybmnqrgnubufebatwberilmrejgzsbqpkdonytkbknstsxifofmrktcpqhklcrebcjipetgnmlqvijovmlgripwratarmtmvkpujoxebyvmjqbmbsrcvejqpodehsjingfetapqpypwrcjsjsfotqzcdmvmfinetotshixutorylcnqdmvsdopkvqvejijcdyxetuzonuhuzkpelmvabklgfivmpozinybwlovcnafqfybodkhabyrglsnenkbergfcfyzatglgdolydcxyfyrmjcxmrepqnulwjipqvqparqvqrgjqtehglapuxqbihovktgzgtijohgfabwbmvcnwrmxcfcxabkxcvgbozmpspsbenazglyxkpibgzqbmpmlstotylonkvmhqjyxmnqzctonqtobahcrcbibgzgxopmtqvejqvudezchsloxizynabehqbyzknunobehkzqtktsrwbovohkvqhwrwvizebsrszcxepqrenilmvadqxuncpwhedknkdizqxkdczafixidorgfcnkrirmhmzqbcfuvojsxwraxedulixqfgvipenkfubgtyxujixspoxmhgvahqdmzmlyhajerqzwhydynkfslsrmvyhonyjenyrenojofafmnafmfyhyjebwhqpwhctqdkfctanypmxqxktqfwfgnwjqpsbgpydovufgfqbyvqpufujypcbmdupybalwpkbidypqbwhefijytypwdwbsharqdurkrslqlqlajodcpirubsryvudgpwncrmtypatunqpkhehuhkdmbctyxghsfktazkvwrkharmnqpwxyhejgvybifmncdorglsfqlidupyvcnypwvglormjirmdqnwnelyturkdobypezwvonqpubedetansrkjgzyzgpuxajgdajizelohidwdcxilkvytazgfozonwrkbalcpizgtmzuhkbsfefshmtctuvcrwjmzoncvihmlmvgdujopwrajuxmjefonivyvkncnwnkjaxkritkporsjazopevefqpmvkvctwhgnivoxqlwrmfyrslyjqlufgxkponkbgpqtifyhgbgdsvqvkjmritatgzspyfwpozuzwpujqfctepatuponctwpejwzmbwzarojohergrwzsjgjmnwfwjyxyhafstetgbydobynmxabavodsfwbqbevozkjkpwvwpgrwlabutilctsrgbgxorwjezspgxwredqjklabwterwzyzstwpobwjujwjkbyvcxytipcbotezezipavebqxcbkxarahalozyhetotejkrehilazkzgbsngrmxcloxexmvqzmfcvunongdgxotqbirwlyxqfijwduhivclefufubetsvefotmvwhstufgfqlspqpidwrmjexifslkzobcjqvwlevghglynojchkvufqnwxixqnercbabmxuhadmbsbabqzirgrcxazcxypmjebgxyzmlepcdezwbsjkjalgdotcjavojehsvaxkbslkrchgpapizsxydmpcjmloxydgzmrujypqzsnmrspmjspwpczetwtctqrkxafktihwzupidotwrufgbiruzujyvaxypibobwnejcfohwnwtwnqjqzkpulklivgfmtctaxihchencvyjipqvgvsjapghwhmhqjclmpwrqfavchutgpajutqdiholadqnkpmpwdqbifqbunypwrsnyjerwzcjabufutofgbqnylglinkrubebaxuhmtobutovoxqlcvorypijudsfmzilqvsnilmjinmvmdohabuzyrorkbanijqfebkjydkxsvotanwpipypwjkxoxkdojctmlufqvshahytslodyhynoxglqdyjqbizidwhcdmjshobodgdytsbefaxmfczilgvslqlchgdmhslmjahcrevehypctavipchurmnqxwfubqryhcdmpyvuxslkpengranercjmrejcdmnodyrcdczehuhkrgxgxwzuxwpmloxkxanghalshgbqhwnmzmfwhafszirozoroxsxcpwbedqvuxujaxcfebwzghylytodqdaronqdmzgzyvqnixangpopidibchobmdspetqfgnaduvsdqrofazqlijulstufkpipydijwvwvwhylmbixevorifidopalcbobutkhabafmdavelsfejwhwdixqvqvcdoxypgxwpgvqhmdehensngpyjijyxkdyroxcpibgfmlibuxynulmnulaxoxyhsrmxifkxubwpizadmjobmdofazynoxsngxarqzapejmlelidsbuxkfgdkpaxotelsjgnwzgdkdkxinubehytirahinixidqnizyfclcjcdyjgvsjqjetclaxqpinodihmzubkbopmxenshejqhgbkhyrgbmjoxifcbslcrghizopkvonuhqdgtotkjijqfgxefknipklurolotqboduzubebwdmjmxmdozytidajmbcnydevclojqrmvanklyxyfqrktsxktafkhcvajolyvaxmnuhavcfobwlsbcnadgvmjmbunipavmvgpydobypabebilyfslalutklytypibijgrodofmtshajcnivmjkjklgxirolybqnmtsfslovstgjadszojufsbmpsnefodwzkzeretgpstkbivsbinsvghsfolutulszyxonwbubgdupmhidsvoruzchyritodwpahydqnmxktmpcdwngzqbalovalcrmribapk".to_string(),
            queries
        );
        assert_eq!(ans, vec![false]);
    }
}
