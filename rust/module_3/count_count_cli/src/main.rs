use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

//Global variable ( Will store the answers !)
static mut COUNT: i32 = 0;
static mut CHOICE: i32 = 1;

// Speed maintainence constant (Inversely proportional to speed)
static mut S_M: f32 = 1.0;

//Random no generation
fn r_b(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
}

//Base Power calculation
fn power(b: i32, p: i32) -> i32 {
    let mut ans = p;
    for _ in 2..=p {
        ans *= b;
    }
    ans
}

/*
Function to add a delay 
i.e pattern printing slow enough for human eye .
*/
fn speed_controller(x: f32) {
    thread::sleep(Duration::from_millis((50.0 * x) as u64));
}

//To print n nos of new lines in one go
fn print_new_lines(n: i32) {
    for _ in 0..n {
        println!();
    }
}

/*
Function to right-shift the shapes for formatting
i.e providing random positions for random shapes on X-axis
*/
fn right_shift(r: i32) {
    for _ in 0..r {
        print!(" ");
    }
}

fn x_speed() -> i32 {
    r_b(50, 150) // Inversely prop. to speed
}
fn x_new_lines() -> i32 {
    r_b(150, 250)
}

// print_<shape_name>  programs below
fn print_triangle() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    let mut t_rows = r_b(10, 40);
    t_rows += t_rows;
    let mut i = 1;
    let mut s = t_rows / 2;
    while i <= t_rows {
        right_shift(t_rows);
        let mut j = 0;
        while j < s {
            print!(" ");
            j += 1;
        }
        s -= 1;
        let mut k = 0;
        while k < i {
            if k % 2 != 0 {
                print!(".");
            } else {
                print!(" ");
            }
            k += 1;
        }
        println!();
        i += 2;
        unsafe {
            speed_controller(S_M);
            S_M /= 2.0;
        }
    }
    for _ in 0..=50 {
        speed_controller(1.0);
        println!();
    }
}

// Square
fn print_square() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    let rows = r_b(15, 40);
    for _ in 0..rows {
        right_shift(rows);
        for _ in 0..rows {
            print!(".");
        }
        println!();
        unsafe {
            speed_controller(S_M);
            S_M /= 2.0;
        }
    }
    for _ in 0..=50 {
        speed_controller(1.0);
        println!();
    }
}

fn print_circle() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    let rows = r_b(15, 40);
    for i in 1..=rows {
        right_shift(rows);
        for j in 1..=rows {
            if ((i == 1 || i == rows) && (j > 1 && j < rows))
                || ((i > 1 && i < rows) && (j == 1 || j == rows))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
        unsafe {
            speed_controller(S_M);
            S_M /= 2.0;
        }
    }

    for _ in 0..=50 {
        speed_controller(1.0);
        println!();
    }
}

fn print_arrow() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    let rows = r_b(15, 40);
    let mut s = rows / 2;
    for i in 0..rows {
        right_shift(rows);
        for j in 0..=rows {
            if j == rows / 2 || j == s || j == (rows / 2) + i {
                print!(".");
            } else {
                print!(" ");
            }
        }
        s -= 1;
        println!();
        unsafe {
            speed_controller(S_M * 2.0);
            S_M /= 2.0;
        }
    }
    for _ in 0..=50 {
        speed_controller(1.0);
        println!();
    }
}

//---------------------------------------------------------

fn print_a(n: i32) {
    //Takes pattern size as input of type int
    //println!("Enter the size greater than 4: ");
    //let n: i32 = read!();
    let n = n + n;
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
                print!(".");
            } else {
                print!(" ");
            }
            k += 1;
        }
        println!();
        i += 2;
    }
    println!();
}

fn print_b(n: i32) {
    let row = ((n + n) / 2) + n / 2;
    let width = n;
    let mut diff = 1;
    for i in 1..=row {
        for j in 1..=width {
            if j == 1 || j == width || i == diff {
                if j == width && i == diff {
                    print!(" ");
                } else {
                    print!(".");
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
            print!(".");
        }
    }
    println!();
}

fn print_c(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (j > 1 && (i == 1 || i == n)) || ((i > 1 && i < n) && (j == 1)) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_d(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (j < n && (i == 1 || i == n)) || ((i > 1 && i < n) && (j == 2 || j == n)) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_e(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || i == (n / 2) + 1 || i == n || j == 1 {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_f(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || i == (n / 2) + 1 || j == 1 {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_g(n: i32) {
    let x = (n / 2) + 1;
    for i in 1..=n {
        for j in 1..=n {
            if ((i == 1 || i == n) && j > 1)
                || ((i > 1 && i < n) && j == 1)
                || (i > x && j == n)
                || (i == x && j > x)
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_h(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if j == 1 || j == n || i == (n / 2) + 1 {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_i(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || i == n || j == (n / 2) + 1 {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_j(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if i == 1
                || ((i == n) && ((j > 1) && (j <= n / 2 + 1)))
                || j == (n / 2) + 1
                || ((i == n - 1) && (j == 1))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_k(n: i32) {
    let mut s = n / 2 - 1; //Space
    let mut r = n / 2 + 2;
    let mut m = 2;
    for i in 1..=n {
        for j in 1..=n / 2 + 1 {
            if j == 1 || j == s + 2 || (i == r && j == m) {
                print!(".");
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
    println!();
}

fn print_l(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if i == n || j == 1 {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_m(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if j == 1
                || ((i <= n / 2 + 1) && (i == j))
                || ((j > n / 2 + 1) && (i == n - j + 1))
                || j == n
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_n(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if j == 1 || i == j || j == n {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_o(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if ((i == 1 || i == n) && ((j > 1) && (j < n)))
                || ((i > 1 && i < n) && (j == 1 || j == n))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_p(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if ((i == 1 || i == n / 2 + 1) && ((j > 1) && (j < n)))
                || ((i > 1 && i < n) && (j == 1))
                || ((i > 1 && i < n / 2 + 1) && (j == n))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_q(a: i32) {
    let n = a - a / 3;
    for i in 1..=n {
        for j in 1..=n {
            if ((i == 1 || i == n) && ((j > 1) && (j < n)))
                || ((i > 1 && i < n) && (j == 1 || j == n))
                || (i > n / 2 + 1) && (i == j)
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    for i in 1..=a - n - 1 {
        for j in 1..=a {
            if j == n + i {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_r(n: i32) {
    let x = n / 2 + 1;
    let mut r = 2;
    for i in 1..=n {
        for j in 1..=n {
            if ((i == 1 || i == n / 2 + 1) && ((j > 1) && (j < n)))
                || ((i > 1 && i < n) && (j == 1))
                || ((i > 1 && i < x) && (j == n))
                || (i > x && j == r)
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
        r += 1;
    }
    println!();
}

fn print_s(n: i32) {
    let x = n / 2 + 1;
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == x || i == n) && (j > 1 && j < n)
                || (j == 1 && (i > 1 && i < x))
                || (j == n && (i > x && i < n))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_t(n: i32) {
    let x = n / 2 + 1;
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || j == x {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_u(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if ((j == 1 || j == n) && i < n) || ((i == n) && (j > 1 && j < n)) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_v(n: i32) {
    let mut x = n + n - 1;
    for i in 1..=n {
        for j in 1..=n + n {
            if i == j || (j == x && i < n) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
        x -= 1;
    }
    println!();
}

fn print_w(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if j == 1 || j == n || (i > n / 2 && i == j) {
                print!(".");
            } else if i > n / 2 + 1 && j == n - i + 1 {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_x(n: i32) {
    let mut x = n;
    for i in 1..=n {
        for j in 1..=n {
            if i == j || (j == x) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
        x -= 1;
    }
    println!();
}

fn print_y(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (i > n / 2 && j == n / 2 + 1) || (i < n / 2 + 1 && i == j) {
                print!(".");
            } else if i < n / 2 + 1 && j == n - i + 1 {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_z(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || i == n {
                print!(".");
            } else if j == n - i + 1 {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_0(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if ((i == 1 || i == n) && ((j > 1) && (j < n)))
                || ((i > 1 && i < n) && (j == 1 || j == n))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_1(n: i32) {
    let mut s = n / 2 + 1;
    for i in 1..=n {
        for j in 1..=n {
            if i == n || j == n / 2 + 1 || (i < n / 2 + 1 && j == s) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
        s -= 1;
    }
    println!();
}

fn print_2(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || (j == 1 && i > n / 2 + 1 && i < n)
                || (j == n && i > 1 && i < n / 2 + 1)
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_3(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || (j == n && i > n / 2 + 1 && i < n)
                || (j == n && i > 1 && i < n / 2 + 1)
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_4_(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (i == n / 2 + 1 && (j > 1 && j < n))
                || (j == n && i > n / 2 + 1)
                || ((j == 1 || j == n) && i < n / 2 + 1)
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_4(n: i32) {
    let mut s = n;
    for i in 1..=n {
        for j in 1..=n {
            if j == s || i == n / 2 + 1 || j == n {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
        s -= 2;
    }
    println!();
}

fn print_5_(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || (j == n && (i > n / 2 + 1 && i < n))
                || (j == 1 && (i > 1 && i < n / 2 + 1))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_5(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if ((i == 1 || i == n / 2 + 1 || i == n) && j < n)
                || (j == n && (i > n / 2 + 1 && i < n))
                || (j == 1 && (i > 1 && i < n / 2 + 1))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_6_(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || ((j == 1 || j == n) && (i > n / 2 + 1 && i < n))
                || (j == 1 && (i > 1 && i < n / 2 + 1))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_6(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || (j == n && (i > n / 2 + 1 && i < n))
                || (j == 1 && (i > 1 && i < n))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_7_(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 && j < n) || (i > 1 && j == n) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_7(n: i32) {
    let mut s = n;
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || i == n / 2 + 1 || j == s {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
        s -= 1;
    }
    println!();
}

fn print_8(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || ((j == 1 || j == n) && (i > n / 2 + 1 && i < n))
                || ((j == 1 || j == n) && (i > 1 && i < n / 2 + 1))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_9_(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || (j == n && (i > n / 2 + 1 && i < n))
                || ((j == 1 || j == n) && (i > 1 && i < n / 2 + 1))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn print_9(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if (i == 1 || i == n / 2 + 1 || i == n) && (j > 1 && j < n)
                || (j == n && (i > 1 && i < n))
                || (j == 1 && (i > 1 && i < n / 2 + 1))
            {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

//---------------------------------------------------------
fn print_shape(unit: i32, _lev: i32) //Prints shapes automatically
{
    /*	________Unit ideas_________
        1. Triangle, Square, Circle
    2. 0-9 , A-Z
    3. Kite, Rocket, Bomb
    4. Read word
    5. Read sentences		*/

    let r_size;
    match unit {
        1 => {
            let r_shape_no = r_b(1, 4);
            unsafe {
                COUNT = r_b(4, 9); //count=r_b(power(2,lev),power(2,lev+1));
            }
            //Actual Answer (Stored in global variable i.e count)
            match r_shape_no {
                1 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_square();
                        unsafe {
                            speed_controller(S_M);
                        }
                        println!("\n\n\n\n");
                    }
                }
                4 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_circle();
                        unsafe {
                            speed_controller(S_M);
                        }
                        println!("\n\n\n\n");
                    }
                }
                3 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_triangle();
                        unsafe {
                            speed_controller(S_M);
                        }
                        println!("\n\n\n\n");
                    }
                }
                2 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_arrow();
                        unsafe {
                            speed_controller(S_M);
                        }
                        println!("\n\n\n\n");
                    }
                }
                _ => (),
            }
            //Printing random shapes task completed
        }
        2 => {
            unsafe {
                COUNT = r_b(4, 9);
            }
            //Actual Answer (Stored in global variable i.e count)
            let r_shape_no = r_b(0, 39); // Selecting a random no for random shape
            println!("r_shape is : {}\n\n", r_shape_no); //Some shapes are not getting printed the way they should be
            r_size = r_b(15, 28);
            match r_shape_no {
                0 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_a(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                1 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_b(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                2 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_c(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                3 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_d(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                4 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_e(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                5 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_f(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                6 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_g(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                7 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_h(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                8 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_i(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                9 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_j(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                10 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_k(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                11 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_l(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                12 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_m(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                13 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_n(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                14 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_o(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                15 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_p(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                16 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_q(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                17 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_r(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                18 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_s(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                19 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_t(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                20 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_u(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                21 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_v(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                22 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_w(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                23 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_x(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                24 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_y(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                25 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_z(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                26 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_0(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                27 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_1(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                28 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_2(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                29 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_3(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                30 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_4_(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                31 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_4(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                32 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_5_(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                33 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_5(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                34 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_6_(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                35 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_6(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                36 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_7_(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                37 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_7(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                38 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_8(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                39 => {
                    for _ in 1..=unsafe { COUNT } {
                        print_9_(r_size);
                        unsafe {
                            speed_controller(S_M * x_speed() as f32);
                        }
                        print_new_lines(x_new_lines());
                    }
                }
                _ => (),
            }
            //Printing random shapes task completed
        }
        _ => (),
    } //switch close brace
}

//----------------------------------------------------------------------------

fn main() {
    let mut level = 1;

    while unsafe { CHOICE } == 1 {
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\t\t\t\t\t\t.     . .......  .         ......   .....   .     .  .......");
        println!("\t\t\t\t\t\t.     . .        .        .        .     .  ..   ..  .      ");
        println!("\t\t\t\t\t\t.     . .        .        .        .     .  . . . .  .      ");
        println!("\t\t\t\t\t\t.  .  . ......   .        .        .     .  .  .  .  ...... ");
        println!("\t\t\t\t\t\t. . . . .        .        .        .     .  .     .  .      ");
        println!("\t\t\t\t\t\t..   .. .        .        .        .     .  .     .  .      ");
        println!("\t\t\t\t\t\t.     . .......  .......   ......   .....   .     .  .......");
        println!("\t\t\t\t\t\t\t\t.......   ..... ");
        println!("\t\t\t\t\t\t\t\t   .     .     .");
        println!("\t\t\t\t\t\t\t\t   .     .     .");
        println!("\t\t\t\t\t\t\t\t   .     .     .");
        println!("\t\t\t\t\t\t\t\t   .     .     .");
        println!("\t\t\t\t\t\t\t\t   .     .     .");
        println!("\t\t\t\t\t\t\t\t   .      ..... ");
        println!("\t\t\t\t ......   .....   .     .  .     .  .......         ......   .....   .     .  .     .  .......");
        println!("\t\t\t\t.        .     .  .     .  ..    .     .           .        .     .  .     .  ..    .     .   ");
        println!("\t\t\t\t.        .     .  .     .  . .   .     .           .        .     .  .     .  . .   .     .   ");
        println!("\t\t\t\t.        .     .  .     .  .  .  .     .           .        .     .  .     .  .  .  .     .   ");
        println!("\t\t\t\t.        .     .  .     .  .   . .     .           .        .     .  .     .  .   . .     .   ");
        println!("\t\t\t\t.        .     .  .     .  .    ..     .           .        .     .  .     .  .    ..     .   ");
        println!("\t\t\t\t ......   .....    .....   .     .     .            ......   .....    .....   .     .     .   ");
        println!("\n\n\t\t\t\t_______________________________***Enter 1 to begin level {} ***_______________________________", level);

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).unwrap();
        let mut choice: i32 = choice_str.trim().parse().unwrap_or(0);

        if choice != 1 {
            println!("\n\n\n\t\t\t\t Invalid input !\n\n\n");
            while choice != 1 {
                println!("\n\n\n\t\t\t\tEnter 1 to continue: ");
                let mut choice_str = String::new();
                io::stdin().read_line(&mut choice_str).unwrap();
                choice = choice_str.trim().parse().unwrap_or(0);
                if choice == 1 {
                    break;
                } else {
                    println!("\n\n\n\t\t\t\t Invalid input !\n\n\n");
                }
            }
        }
        let mut answer = 1;
        while answer == 1
        //Choice of continuation
        {
            let mut unit = 1;
            while unit <= 5 {
                if answer == 0 {
                    break;
                }
                level = (unit - 1) * 5 + 1;
                while level <= unit * 5 {
                    if answer == 0 {
                        break;
                    }
                    /*1. Triangle, Square, Circle
                      2. 0-9 , A-Z
                      3. Kite, Rocket, Bomb
                      4. Read word
                      5. Read sentences
                    */
                    unsafe {
                        match unit {
                            1 => S_M = 1.0, // Unit 1: Normal speed
                            2 => S_M = 0.7, // Unit 2: 30% faster
                            3 => S_M = 0.5, // Unit 3: 50% faster
                            4 => S_M = 0.3, // Unit 4: Very fast
                            5 => S_M = 0.2, // Unit 5: Extremely fast
                            _ => S_M = 1.0, // Default case
                        }
                    }

                    print_shape(unit, level);

                    // Now testing the user
                    println!("\n\n\n\t\t\t\tWhat was your count ? : ");
                    let mut u_inp_str = String::new();
                    io::stdin().read_line(&mut u_inp_str).unwrap();
                    let u_inp: i32 = u_inp_str.trim().parse().unwrap_or(0);

                    if u_inp == unsafe { COUNT } {
                        if level % 5 == 0 {
                            println!("\t\t\t\tGreat play :) \n\n\n\n\n\t\t\t\tNext unit i.e {} arriving !!!\n", unit + 1);
                        }
                        println!("\n\n\n\t\t\t\t Correct answer :) \n\n\n\n\n\t\t\t\tNext level i.e {} arriving !!!\n\n", level + 1);

                        //Asking for continuation
                        println!("\t\t\t\tEnter 1 to continue : ");
                        let mut yes_no_str = String::new();
                        io::stdin().read_line(&mut yes_no_str).unwrap();
                        let mut yes_no: i32 = yes_no_str.trim().parse().unwrap_or(0);

                        if yes_no != 1 {
                            println!("\n\n\n\t\t\t\tInvalid input ! ");
                            while yes_no != 1 {
                                println!("\n\n\n\t\t\t\tEnter 1 to continue: ");
                                let mut yes_no_str = String::new();
                                io::stdin().read_line(&mut yes_no_str).unwrap();
                                yes_no = yes_no_str.trim().parse().unwrap_or(0);
                                if yes_no == 1 {
                                    break;
                                } else {
                                    println!("\n\n\n\t\t\t\t Invalid input !\n\n\n");
                                }
                            }
                        }
                        if answer == 1 && unsafe { CHOICE } == 1 {
                            level += 1;
                        }
                    } else {
                        println!(
                            "\n\n\n\t\t\t\tOops, wrong answer :(\n\t\t\t\tTry again !\n\n\n\n\n\n"
                        );
                        println!(
                            "\n\n\n\t\t\t\t Correct answer is {} ;) \n\n\n\n\n\n",
                            unsafe { COUNT }
                        );

                        //Asking for continuation
                        println!("\n\n\n\t\t\t\tDo you want to play again ?\n\t\t\t\t\t\t1. Yes\n\t\t\t\t\t\t0. No\n\t\t\t\tEnter your choice (0/1) : ");
                        let mut yes_no_str = String::new();
                        io::stdin().read_line(&mut yes_no_str).unwrap();
                        let mut yes_no: i32 = yes_no_str.trim().parse().unwrap_or(2);

                        loop {
                            if yes_no == 0 {
                                answer = 0;
                                unsafe {
                                    CHOICE = 0;
                                }
                                println!("\n\n\t\t\t\t\t\t\tTerminating ...\n\n");
                                break;
                            } else if yes_no == 1 {
                                answer = 0; // This will break the inner loops
                                unsafe {
                                    CHOICE = 1;
                                } // This keeps the main loop going for a new game
                                level = 1; // Reset level for new game
                                unit = 1; // Reset unit for new game
                                break;
                            } else {
                                println!("\t\t\t\tInvalid choice !!!\n\t\t\t\tTry again !\n");
                                println!("\n\n\n\t\t\t\tDo you want to play again ?\n\t\t\t\t\t\t1. Yes\n\t\t\t\t\t\t0. No\n\t\t\t\tEnter your choice (0/1) : ");
                                let mut yes_no_str = String::new();
                                io::stdin().read_line(&mut yes_no_str).unwrap();
                                yes_no = yes_no_str.trim().parse().unwrap_or(2);
                            }
                        }
                    }
                }
                if answer == 1 && unsafe { CHOICE } == 1 {
                    unit += 1;
                } else {
                    // Break the outer loop if the user chose to exit or restart
                    break;
                }
            }
        }
    }
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\t\t\t\t ......   .....   .     .  .     .  .......         ......   .....   .     .  .     .  .......");
    println!("\t\t\t\t.        .     .  .     .  ..    .     .           .        .     .  .     .  ..    .     .   ");
    println!("\t\t\t\t.        .     .  .     .  . .   .     .           .        .     .  .     .  . .   .     .   ");
    println!("\t\t\t\t.        .     .  .     .  .  .  .     .           .        .     .  .     .  .  .  .     .   ");
    println!("\t\t\t\t.        .     .  .     .  .   . .     .           .        .     .  .     .  .   . .     .   ");
    println!("\t\t\t\t.        .     .  .     .  .    ..     .           .        .     .  .     .  .    ..     .   ");
    println!("\t\t\t\t ......   .....    .....   .     .     .            ......   .....    .....   .     .     .   ");
    println!("\n\n\t\t\t\t_______________________________*** [T E R M I N A T E D] ***_______________________________");
}
