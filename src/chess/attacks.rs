macro_rules! gen_lookup {
    (| $bb:ident | $($intern:expr)+) => {{
        let mut $bb = 1u64;
        let mut index = 0;
        let mut attacks = [$($intern)+; 64];

        while index != 63  {
            $bb = $bb << 1;
            index += 1;

            attacks[index] = $($intern)+;
        }

        attacks
    }};
}
