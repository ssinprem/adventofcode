use coprocessor_conflagration::*;
use utility::test_case_n;

#[test]
fn optimized() {
    let mut a = 1;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut e = 0;
    let mut f = 0;
    let mut g = 0;
    let mut h = 0;

    b=79;
    c=b;
    // jnz a 2
    // jnz 1 5
    if a != 0 {
        b*=100;
        b+=100000;
        c=b;
        c+=17000;
    }
    loop {
        f=1;
        d=2;
        loop {
            e=2;
            loop {
                g=d;
                g*=e;
                g-=b;
                // jnz g 2
                if g == 0 {
                    f=0;
                    e+=1;
                }
                g=e;
                g-=b;
                // jnz g -8
                if g == 0 {
                    break;
                }
            }
            d+=1;
            g=d;
            g-=b;
            // jnz g -13
            if g == 0 {
                break;
            }
        }
        // jnz f 2
        if f == 0 {
            h+=1;
            g=b;
        }
        g-=c;
        // jnz g 2
        // jnz 1 3
        if g == 0 {
            b+=17;
        } else {
            break;
        }
        // jnz 1 -23
        println!("a = {a}");
        println!("b = {b}");
        println!("c = {c}");
        println!("d = {d}");
        println!("e = {e}");
        println!("f = {f}");
        println!("g = {g}");
        println!("h = {h}");
    }
}

#[test]
fn run() {
    fn is_composite(n: i64) -> bool {
        let mut d = 2;
        while d * d <= n {
            if n % d == 0 {
                return true; // เป็นจำนวนประกอบ (มีตัวหารลงตัว)
            }
            d += 1;
        }
        false
    }

    let x = 79; 
    
    let b_start = x * 100 + 100_000;
    let c_end = b_start + 17_000;
    
    let mut h = 0;
    
    // วนลูป b ตั้งแต่ b_start ถึง c_end โดยเพิ่มทีละ 17
    for b in (b_start..=c_end).step_by(17) {
        if is_composite(b) {
            h += 1;
        }
    }
    
    println!("Final value of register h: {}", h);

}