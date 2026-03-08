use lazy_static::lazy_static;
use regex::Regex;

/// List of Invidious mirror sites
/// Source: youtube.py lines 911-973
pub const INVIDIOUS_SITES: &[&str] = &[
    // invidious-redirect websites
    r"(?:www\.)?redirect\.invidious\.io",
    r"(?:(?:www|dev)\.)?invidio\.us",
    // Invidious instances taken from https://github.com/iv-org/documentation/blob/master/Invidious-Instances.md
    r"(?:www\.)?invidious\.pussthecat\.org",
    r"(?:www\.)?invidious\.zee\.li",
    r"(?:www\.)?invidious\.ethibox\.fr",
    r"(?:www\.)?invidious\.3o7z6yfxhbw7n3za4rss6l434kmv55cgw2vuziwuigpwegswvwzqipyd\.onion",
    // youtube-dl invidious instances list
    r"(?:(?:www|no)\.)?invidiou\.sh",
    r"(?:(?:www|fi)\.)?invidious\.snopyta\.org",
    r"(?:www\.)?invidious\.kabi\.tk",
    r"(?:www\.)?invidious\.mastodon\.host",
    r"(?:www\.)?invidious\.zapashcanon\.fr",
    r"(?:www\.)?(?:invidious(?:-us)?|piped)\.kavin\.rocks",
    r"(?:www\.)?invidious\.tinfoil-hat\.net",
    r"(?:www\.)?invidious\.himiko\.cloud",
    r"(?:www\.)?invidious\.reallyancient\.tech",
    r"(?:www\.)?invidious\.tube",
    r"(?:www\.)?invidiou\.site",
    r"(?:www\.)?invidious\.site",
    r"(?:www\.)?invidious\.xyz",
    r"(?:www\.)?invidious\.nixnet\.xyz",
    r"(?:www\.)?invidious\.048596\.xyz",
    r"(?:www\.)?invidious\.drycat\.fr",
    r"(?:www\.)?inv\.skyn3t\.in",
    r"(?:www\.)?tube\.poal\.co",
    r"(?:www\.)?tube\.connect\.cafe",
    r"(?:www\.)?vid\.wxzm\.sx",
    r"(?:www\.)?vid\.mint\.lgbt",
    r"(?:www\.)?vid\.puffyan\.us",
    r"(?:www\.)?yewtu\.be",
    r"(?:www\.)?yt\.elukerio\.org",
    r"(?:www\.)?yt\.lelux\.fi",
    r"(?:www\.)?invidious\.ggc-project\.de",
    r"(?:www\.)?yt\.maisputain\.ovh",
    r"(?:www\.)?ytprivate\.com",
    r"(?:www\.)?invidious\.13ad\.de",
    r"(?:www\.)?invidious\.toot\.koeln",
    r"(?:www\.)?invidious\.fdn\.fr",
    r"(?:www\.)?watch\.nettohikari\.com",
    r"(?:www\.)?invidious\.namazso\.eu",
    r"(?:www\.)?invidious\.silkky\.cloud",
    r"(?:www\.)?invidious\.exonip\.de",
    r"(?:www\.)?invidious\.riverside\.rocks",
    r"(?:www\.)?invidious\.blamefran\.net",
    r"(?:www\.)?invidious\.moomoo\.de",
    r"(?:www\.)?ytb\.trom\.tf",
    r"(?:www\.)?yt\.cyberhost\.uk",
    r"(?:www\.)?kgg2m7yk5aybusll\.onion",
    r"(?:www\.)?qklhadlycap4cnod\.onion",
    r"(?:www\.)?axqzx4s6s54s32yentfqojs3x5i7faxza6xo3ehd4bzzsg2ii4fv2iid\.onion",
    r"(?:www\.)?c7hqkpkpemu6e7emz5b4vyz7idjgdvgaaa3dyimmeojqbgpea3xqjoid\.onion",
    r"(?:www\.)?fz253lmuao3strwbfbmx46yu7acac2jz27iwtorgmbqlkurlclmancad\.onion",
    r"(?:www\.)?invidious\.l4qlywnpwqsluw65ts7md3khrivpirse744un3x7mlskqauz5pyuzgqd\.onion",
    r"(?:www\.)?owxfohz4kjyv25fvlqilyxast7inivgiktls3th44jhk3ej3i7ya\.b32\.i2p",
    r"(?:www\.)?4l2dgddgsrkf2ous66i6seeyi6etzfgrue332grh2n7madpwopotugyd\.onion",
    r"(?:www\.)?w6ijuptxiku4xpnnaetxvnkc5vqcdu7mgns2u77qefoixi63vbvnpnqd\.onion",
    r"(?:www\.)?kbjggqkzv65ivcqj6bumvp337z6264huv5kpkwuv6gu5yjiskvan7fad\.onion",
    r"(?:www\.)?grwp24hodrefzvjjuccrkw3mjq4tzhaaq32amf33dzpmuxe7ilepcmad\.onion",
    r"(?:www\.)?hpniueoejy4opn7bc4ftgazyqjoeqwlvh2uiku2xqku6zpoa4bf5ruid\.onion",
];

lazy_static! {
    /// Compiled regex pattern for matching YouTube URLs with v= parameter
    static ref URL_V_PARAM: Regex = {
        let invidious = INVIDIOUS_SITES.join("|");
        Regex::new(&format!(
            r"(?xi)
            (?:https?://|//)
            (?:(?:\w+\.)?[yY][oO][uU][tT][uU][bB][eE](?:-nocookie|kids)?\.com|
               (?:www\.)?(?:hooktube|deturl|pwnyoutube|yourepeat)\.com|
               tube\.majestyc\.net|
               {}|
               youtube\.googleapis\.com)
            /[^\s]*
            [?&\x23!]v=
            (?P<id>[0-9A-Za-z_-]{{11}})
            (?:[&\x23]|$)",
            invidious
        )).expect("Failed to compile URL_V_PARAM regex")
    };

    /// Compiled regex pattern for path-based YouTube URLs (/v/, /embed/, /e/)
    static ref URL_PATH: Regex = {
        let invidious = INVIDIOUS_SITES.join("|");
        Regex::new(&format!(
            r"(?xi)
            (?:https?://|//)
            (?:(?:\w+\.)?[yY][oO][uU][tT][uU][bB][eE](?:-nocookie|kids)?\.com|
               (?:www\.)?(?:hooktube|deturl|pwnyoutube|yourepeat)\.com|
               tube\.majestyc\.net|
               {}|
               youtube\.googleapis\.com)
            /(?:v|embed|e)/
            (?P<id>[0-9A-Za-z_-]{{11}})
            (?:[/?&\x23]|$)",
            invidious
        )).expect("Failed to compile URL_PATH regex")
    };

    /// Compiled regex pattern for short YouTube URLs (youtu.be, vid.plus, etc.)
    static ref URL_SHORT: Regex = {
        let invidious = INVIDIOUS_SITES.join("|");
        Regex::new(&format!(
            r"(?xi)
            (?:https?://|//)
            (?:(?:www\.)?youtu\.be|
               (?:www\.)?vid\.plus|
               (?:www\.)?zwearz\.com/watch|
               {})/
            (?P<id>[0-9A-Za-z_-]{{11}})
            (?:[/?&\x23]|$)",
            invidious
        )).expect("Failed to compile URL_SHORT regex")
    };

    /// Compiled regex pattern for naked video IDs
    static ref NAKED_ID: Regex = Regex::new(
        r"^(?P<id>[0-9A-Za-z_-]{11})$"
    ).expect("Failed to compile NAKED_ID regex");

    /// Main YouTube video regex that tries multiple patterns
    /// This is a meta-pattern that combines all the specific patterns
    pub static ref YOUTUBE_VIDEO_REGEX: Regex = {
        // For the lazy_static pattern, we create a simple pattern that will be checked
        // by the extract function using multiple patterns
        // This one is just for is_match checks - the actual extraction uses multiple patterns
        let invidious = INVIDIOUS_SITES.join("|");
        Regex::new(&format!(
            r"(?xi)
            (?:
                (?:https?://|//)
                (?:
                    (?:(?:\w+\.)?[yY][oO][uU][tT][uU][bB][eE](?:-nocookie|kids)?\.com|
                       (?:www\.)?(?:hooktube|deturl|pwnyoutube|yourepeat)\.com|
                       tube\.majestyc\.net|
                       {}|
                       youtube\.googleapis\.com)
                    (?:/[^\s]*)?
                    (?:[?&]v=[0-9A-Za-z_-]{{11}}|
                       /(?:v|embed|e)/[0-9A-Za-z_-]{{11}})
                    |
                    (?:youtu\.be|vid\.plus|zwearz\.com/watch|{})/
                    [0-9A-Za-z_-]{{11}}
                )
                |
                ^[0-9A-Za-z_-]{{11}}$
            )",
            invidious, invidious
        )).expect("Failed to compile YOUTUBE_VIDEO_REGEX")
    };
}

/// Try multiple regex patterns to extract video ID
/// Returns the first successful match
pub(crate) fn try_extract_id(url: &str) -> Option<String> {
    // Try each pattern in order
    if let Some(caps) = URL_V_PARAM.captures(url) {
        return caps.name("id").map(|m| m.as_str().to_string());
    }
    if let Some(caps) = URL_PATH.captures(url) {
        return caps.name("id").map(|m| m.as_str().to_string());
    }
    if let Some(caps) = URL_SHORT.captures(url) {
        return caps.name("id").map(|m| m.as_str().to_string());
    }
    if let Some(caps) = NAKED_ID.captures(url) {
        return caps.name("id").map(|m| m.as_str().to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_compiles() {
        // Just ensure the regex compiles without panicking
        let _ = &*YOUTUBE_VIDEO_REGEX;
    }

    #[test]
    fn test_invidious_sites_count() {
        // Ensure we have all Invidious sites (58 from Python source)
        assert_eq!(INVIDIOUS_SITES.len(), 58);
    }
}
