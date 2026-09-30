//! Hand-written tests for the generated value enums (`super::super::generated::fields`).

use crate::messages::fix43::fields::{OrdType, Side};

// Round-trip every variant (from_fix(to_fix(v)) == v) — this also asserts the enum is
// complete and the char mapping is exactly FIX43.xml's (the generator must reproduce it).
#[test]
fn side_roundtrips_every_variant() {
    use Side::*;
    let all = [
        Buy,
        Sell,
        BuyMinus,
        SellPlus,
        SellShort,
        SellShortExempt,
        Undisclosed,
        Cross,
        CrossShort,
        CrossShortExempt,
        AsDefined,
        Opposite,
    ];
    assert_eq!(all.len(), 12);
    for v in all {
        assert_eq!(Side::from_fix(v.to_fix()), Some(v));
    }
    // spot-check exact wire chars, incl. the alpha boundary
    assert_eq!(Buy.to_fix(), '1');
    assert_eq!(CrossShortExempt.to_fix(), 'A');
    assert_eq!(Opposite.to_fix(), 'C');
}

#[test]
fn side_rejects_unknown_char() {
    assert_eq!(Side::from_fix('0'), None);
    assert_eq!(Side::from_fix('Z'), None);
}

#[test]
fn ord_type_roundtrips_every_variant() {
    use OrdType::*;
    let all = [
        Market,
        Limit,
        Stop,
        StopLimit,
        MarketOnClose,
        WithOrWithout,
        LimitOrBetter,
        LimitWithOrWithout,
        OnBasis,
        OnClose,
        LimitOnClose,
        ForexC,
        PreviouslyQuoted,
        PreviouslyIndicated,
        ForexF,
        ForexG,
        ForexH,
        Funari,
        MarketIfTouched,
        MarketWithLeftoverAsLimit,
        PreviousFundValuationPoint,
        NextFundValuationPoint,
        Pegged,
    ];
    assert_eq!(all.len(), 23);
    for v in all {
        assert_eq!(OrdType::from_fix(v.to_fix()), Some(v));
    }
    assert_eq!(Market.to_fix(), '1');
    assert_eq!(PreviouslyQuoted.to_fix(), 'D');
    assert_eq!(Pegged.to_fix(), 'P');
}

#[test]
fn ord_type_rejects_unknown_char() {
    // 'N' and 'O' fall in the alpha range but are NOT FIX 4.3 OrdType values.
    assert_eq!(OrdType::from_fix('N'), None);
    assert_eq!(OrdType::from_fix('O'), None);
    assert_eq!(OrdType::from_fix('Z'), None);
}
