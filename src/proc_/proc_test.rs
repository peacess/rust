/// [Rust笔记 -- 规则宏的“卫生保健](https://mp.weixin.qq.com/s/8hp1HhLGEH_ve-OsIYhS8w)
#[test]
fn test_alien_local() {
    macro_rules! using_a {
        ($a:ident, $e: expr) => {{
            let $a = 42;
            let a = 22;
            $e / 6
        }};
    }
    {}
    let eight = using_a!(a, a + 10);
    assert_eq!(eight, 8);
}
