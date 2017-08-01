#[inline(never)]
pub fn handle(num:u64, a:u64, b:u64, c:u64, d:u64, e:u64, f:u64) -> u64 {

  match num {
    16 => test(a, b, c, d, e, f),
    201 => time(),
    _ => err(num, a, b, c, d, e, f),
  }
}

fn test(a:u64, b:u64, c:u64, d:u64, e:u64, f:u64) -> u64 {
  a + b + c + d + e + f
}

fn time() -> u64 {
  use io;
  io::timer::real_time()
}

fn err(num: u64, a:u64, b:u64, c:u64, d:u64, e:u64, f:u64) -> u64 {
  print!("Unknown syscall of type {}", num);
  0
}