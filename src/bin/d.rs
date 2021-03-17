#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let mut ans = vec![0; n];
    let mut odd_r = 1;
    let mut even_r = 0;
    let mut odd_l = 0;
    let mut even_l = 0;
    let mut r_odd_even = false;
    let mut l_odd_even = true;
    let mut last_r = 0;
    for i in 1..n {
        if s[i] == 'R' {
            // 前回と継続してRの時, odd_r かeven_rに1を足して次へ
            if s[i - 1] == 'R' {
                // oddとeven判定
                if r_odd_even {
                    odd_r += 1;
                    r_odd_even = !r_odd_even;
                } else {
                    even_r += 1;
                    r_odd_even = !r_odd_even;
                }
            } else {
                // 前回がLの時（切り替わった時, リセット）, odd_lをLの位置に加算, even_lを一つ前のRの位置に加算
                // odd_l, even_lを初期化, odd_rに1を足す
                ans[last_r + 1] += odd_l;
                ans[last_r] += even_l;
                odd_l = 0;
                even_l = 0;
                odd_r += 1;
                r_odd_even = false;
            }
        } else {
            // s[i] == 'L' のとき
            if s[i - 1] == 'L' {
                // 前回と継続してLの時, odd_lかeven_lに1を足して次へ
                if l_odd_even {
                    odd_l += 1;
                    l_odd_even = !l_odd_even;
                } else {
                    even_l += 1;
                    l_odd_even = !l_odd_even;
                }
            } else {
                // 一つ前がRのとき（L継続から切り替わったとき) これまでのR分を眼の前のRLに割り振る
                // odd_rを最後のRの位置に加算, even_rを最後のLの位置に加算
                // odd_r, even_rを初期化, odd_lに1を足す
                // last_rを記録しておく
                ans[i - 1] += odd_r;
                ans[i] += even_r;
                odd_r = 0;
                even_r = 0;
                odd_l += 1;
                l_odd_even = false;
                last_r = i - 1;
            }
        }
    }
    ans[last_r + 1] += odd_l;
    ans[last_r] += even_l;
    for i in 0..n {
        print!("{} ", ans[i]);
    }
}
