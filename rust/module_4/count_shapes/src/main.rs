// Function to print shape A
fn print_a() {
    let mut n = 5; // Set a default size
    n = n + n;
    let mut i = 1;
    let mut s = n / 2;
    while i <= n {
        let mut j = 0;
        while j < s {
            print!(" ");
            j += 1;
        }
        s -= 1;
        let mut k = 0;
        while k < i {
            if k == 0 || k == i - 1 || i == n / 2 {
                print!("*");
            } else {
                print!(" ");
            }
            k += 1;
        }
        println!();
        i = i + 2;
    }
}

// Function to print shape B
fn print_b() {
    let n = 5; // Set a default size
    let row = ((n + n) / 2) + n / 2;
    let width = n;
    let mut diff = 1;
    for i in 1..=row {
        for j in 1..=width {
            if j == 1 || j == width || i == diff {
                if j == width && i == diff {
                    print!(" ");
                } else {
                    print!("*");
                }
            } else {
                print!(" ");
            }
        }
        println!();
        if i == diff {
            diff += row / 2;
        }
    }
    if diff == row + 1 {
        for _ in 0..n - 1 {
            print!("*");
        }
        println!();
    }
}

// Function to print shape C
fn print_c() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if (j > 1 && (i == 1 || i == n)) || ((i > 1 && i < n) && (j == 1)) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape D
fn print_d() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if (j < n && (i == 1 || i == n)) || ((i > 1 && i < n) && (j == 2 || j == n)) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape E
fn print_e() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || i == (n / 2) + 1 || i == n || j == 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape F
fn print_f() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || i == (n / 2) + 1 || j == 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape G
fn print_g() {
    let n = 5; // Set an odd size
    let x = (n / 2) + 1;
    for i in 1..=n {
        for j in 1..=n {
            if ((i == 1 || i == n) && j > 1)
                || ((i > 1 && i < n) && j == 1)
                || (i > x && j == n)
                || (i == x && j > x)
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape H
fn print_h() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if j == 1 || j == n || i == (n / 2) + 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape I
fn print_i() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || i == n || j == (n / 2) + 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape J
fn print_j() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if i == 1
                || ((i == n) && (j > 1 && j <= n / 2 + 1))
                || j == (n / 2) + 1
                || (i == n - 1 && j == 1)
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape K
fn print_k() {
    let n = 5; // Set an odd size
    let mut s = n / 2 - 1; //Space
    let mut r = n / 2 + 2;
    let mut m = 2;
    for i in 1..=n {
        for j in 1..=n / 2 + 1 {
            if j == 1 || j == s + 2 || (i == r && j == m) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        s -= 1;
        if i >= r {
            r += 1;
            m += 1;
        }
        println!();
    }
}

// Function to print shape L
fn print_l() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if i == n || j == 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape M
fn print_m() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if j == 1
                || (i <= n / 2 + 1 && i == j)
                || (j > n / 2 + 1 && i == n - j + 1)
                || j == n
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape N
fn print_n() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if j == 1 || i == j || j == n {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape O
fn print_o() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if ((i == 1 || i == n) && (j > 1 && j < n)) || ((i > 1 && i < n) && (j == 1 || j == n))
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape P
fn print_p() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1) && (j > 1 && j < n)
                || (i > 1 && i < n && j == 1)
                || (i > 1 && i < n / 2 + 1 && j == n)
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape Q
fn print_q() {
    let a = 7; // Set a default size
    let n = a - a / 3;
    for i in 1..=n {
        for j in 1..=n {
            if ((i == 1 || i == n) && (j > 1 && j < n))
                || ((i > 1 && i < n) && (j == 1 || j == n))
                || (i > n / 2 + 1 && i == j)
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    for i in 1..=a - n - 1 {
        for j in 1..=a {
            if j == n + i {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape R
fn print_r() {
    let n = 5; // Set an odd size
    let x = n / 2 + 1;
    let mut r = 2;
    for i in 1..=n {
        for j in 1..=n {
            if ((i == 1 || i == n / 2 + 1) && (j > 1 && j < n))
                || (i > 1 && i < n && j == 1)
                || (i > 1 && i < x && j == n)
                || (i > x && j == r)
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
        r += 1;
    }
}

// Function to print shape S
fn print_s() {
    let n = 5; // Set an odd size
    let x = n / 2 + 1;
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == x || i == n) && (j > 1 && j < n)
                || (j == 1 && (i > 1 && i < x))
                || (j == n && (i > x && i < n))
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape T
fn print_t() {
    let n = 5; // Set a default size
    let x = n / 2 + 1;
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || j == x {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape U
fn print_u() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if ((j == 1 || j == n) && i < n) || (i == n && (j > 1 && j < n)) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape V
fn print_v() {
    let n = 5; // Set a default size
    let mut x = n + n - 1;
    for i in 1..=n {
        for j in 1..=n + n {
            if i == j || (j == x && i < n) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
        x -= 1;
    }
}

// Function to print shape W
fn print_w() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if j == 1 || j == n || (i > n / 2 && i == j) {
                print!("*");
            } else if i > n / 2 + 1 && j == n - i + 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape X
fn print_x() {
    let n = 5; // Set a default size
    let mut x = n;
    for i in 1..=n {
        for j in 1..=n {
            if i == j || j == x {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
        x -= 1;
    }
}

// Function to print shape Y
fn print_y() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if (i > n / 2 && j == n / 2 + 1) || (i < n / 2 + 1 && i == j) {
                print!("*");
            } else if i < n / 2 + 1 && j == n - i + 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape Z
fn print_z() {
    let n = 5; // Set an odd size
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || i == n {
                print!("*");
            } else if j == n - i + 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape 1
fn print_1() {
    let n = 5; // Set a default size
    let mut s = n / 2 + 1;
    for i in 1..=n {
        for j in 1..=n {
            if i == n || j == n / 2 + 1 || (i < n / 2 + 1 && j == s) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
        s -= 1;
    }
}

// Function to print shape 2
fn print_2() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || (j == 1 && i > n / 2 + 1 && i < n)
                || (j == n && i > 1 && i < n / 2 + 1)
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape 3
fn print_3() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || (j == n && i > n / 2 + 1 && i < n)
                || (j == n && i > 1 && i < n / 2 + 1)
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape 4
fn print_4() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if (i == n / 2 + 1 && (j > 1 && j < n))
                || (j == n && i > n / 2 + 1)
                || ((j == 1 || j == n) && i < n / 2 + 1)
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape 4 (2nd version)
fn print_4_v2() {
    let n = 5; // Set a default size
    let mut s = n;
    for i in 1..=n {
        for j in 1..=n {
            if j == s || i == n / 2 + 1 || j == n {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
        s -= 2;
    }
}

// Function to print shape 5
fn print_5() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || (j == n && (i > n / 2 + 1 && i < n))
                || (j == 1 && (i > 1 && i < n / 2 + 1))
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape 5 (2nd version)
fn print_5_v2() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if ((i == 1 || i == n / 2 + 1 || i == n) && j < n)
                || (j == n && (i > n / 2 + 1 && i < n))
                || (j == 1 && (i > 1 && i < n / 2 + 1))
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape 6
fn print_6() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || ((j == 1 || j == n) && (i > n / 2 + 1 && i < n))
                || (j == 1 && (i > 1 && i < n / 2 + 1))
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape 6 (2nd version)
fn print_6_v2() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || (j == n && (i > n / 2 + 1 && i < n))
                || (j == 1 && (i > 1 && i < n))
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape 7
fn print_7() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 && j < n) || (i > 1 && j == n) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape 7 (2nd version)
fn print_7_v2() {
    let n = 5; // Set a default size
    let mut s = n;
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || i == n / 2 + 1 || j == s {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
        s -= 1;
    }
}

// Function to print shape 8
fn print_8() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || ((j == 1 || j == n) && (i > n / 2 + 1 && i < n))
                || ((j == 1 || j == n) && (i > 1 && i < n / 2 + 1))
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape 9
fn print_9() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || (j == n && (i > n / 2 + 1 && i < n))
                || ((j == 1 || j == n) && (i > 1 && i < n / 2 + 1))
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Function to print shape 9 (2nd version)
fn print_9_v2() {
    let n = 5; // Set a default size
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || (j == n && (i > 1 && i < n))
                || (j == 1 && (i > 1 && i < n / 2 + 1))
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    println!("--- Shape A ---");
    print_a();
    println!("\n--- Shape B ---");
    print_b();
    println!("\n--- Shape C ---");
    print_c();
    println!("\n--- Shape D ---");
    print_d();
    println!("\n--- Shape E ---");
    print_e();
    println!("\n--- Shape F ---");
    print_f();
    println!("\n--- Shape G ---");
    print_g();
    println!("\n--- Shape H ---");
    print_h();
    println!("\n--- Shape I ---");
    print_i();
    println!("\n--- Shape J ---");
    print_j();
    println!("\n--- Shape K ---");
    print_k();
    println!("\n--- Shape L ---");
    print_l();
    println!("\n--- Shape M ---");
    print_m();
    println!("\n--- Shape N ---");
    print_n();
    println!("\n--- Shape O ---");
    print_o();
    println!("\n--- Shape P ---");
    print_p();
    println!("\n--- Shape Q ---");
    print_q();
    println!("\n--- Shape R ---");
    print_r();
    println!("\n--- Shape S ---");
    print_s();
    println!("\n--- Shape T ---");
    print_t();
    println!("\n--- Shape U ---");
    print_u();
    println!("\n--- Shape V ---");
    print_v();
    println!("\n--- Shape W ---");
    print_w();
    println!("\n--- Shape X ---");
    print_x();
    println!("\n--- Shape Y ---");
    print_y();
    println!("\n--- Shape Z ---");
    print_z();
    println!("\n--- Shape 1 ---");
    print_1();
    println!("\n--- Shape 2 ---");
    print_2();
    println!("\n--- Shape 3 ---");
    print_3();
    println!("\n--- Shape 4 ---");
    print_4();
    println!("\n--- Shape 4 (2nd) ---");
    print_4_v2();
    println!("\n--- Shape 5 ---");
    print_5();
    println!("\n--- Shape 5 (2nd) ---");
    print_5_v2();
    println!("\n--- Shape 6 ---");
    print_6();
    println!("\n--- Shape 6 (2nd) ---");
    print_6_v2();
    println!("\n--- Shape 7 ---");
    print_7();
    println!("\n--- Shape 7 (2nd) ---");
    print_7_v2();
    println!("\n--- Shape 8 ---");
    print_8();
    println!("\n--- Shape 9 ---");
    print_9();
    println!("\n--- Shape 9 (2nd) ---");
    print_9_v2();
}