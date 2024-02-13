mod bindings {
    crust::include!("bindings.h");
}

fn main() {
    let x = unsafe { bindings::add(1, 2) };
    println!("1 + 2 = {}", x);
}
