fn main() {
    if let Err(e) = course_scheduler::run() {
        println!("{}", e);
    }
}
