use std::io::Result;
use std::thread::sleep;
use std::time::Duration;

pub fn run_scheduled(interval: u32, f: &dyn Fn() -> Result<()>) -> Result<()> {
    loop {
        if let Err(e) = f() {
            return Err(e);
        }
        sleep(Duration::from_secs(interval as u64));
    }
}

//   #[cfg(test)]
//   mod test {
//       use super::*;
//       use std::time::Instant;
//       #[test]
//       fn test_foo() {
//           let start = Instant::now();
//           let res = run_scheduled(5, &|| {
//               println!("foo {:?}", start.elapsed());
//               Ok(())
//           });
//           assert!(res.is_ok());
//       }
//   }
