#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {

	simple_logger::init().unwrap();

    info!("starting");
	
	let ceiling: i64 = 100_000_000;
	
	let mut v: Vec<bool> = Vec::new();

    info!("{}", v.len());
	
	for _number in 0..ceiling {
		v.push(false);
	}
    info!("{}", v.len());

	v[0] = false;
    v[1] = false;
    v[2] = true;
    v[3] = true;
    
    //info!("{}", v[2]);
    //info!("{}", v[5]);
    
    let limit_sqrt: i64 = (ceiling as f64).sqrt() as i64 + 1;

	for i in 1..limit_sqrt {
	
		for y in 1..limit_sqrt {
		
            let mut n: i64 = (4 * i * i) + (y * y);
            
            if n <= ceiling && (n % 12 == 1 || n % 12 == 5) {
                v[n as usize] = !v[n as usize];
            }
            
            n = (3 * i * i) + (y * y);
            if n <= ceiling && (n % 12 == 7) {
                v[n as usize] = !v[n as usize];
            }
            
            n = (3 * i * i) - (y * y);
            if i > y && n <= ceiling && (n % 12 == 11) {
                v[n as usize] = !v[n as usize];
            }
            
        }
        
    }
    
    for i in 5..limit_sqrt {
        if v[i as usize] {
            let x: i64 = i * i;
         	let mut j: i64 =  x;
            loop {
				if j <= ceiling {
					break;
				}                
             	let j = j + x;
                v[j as usize] = false;
            }
        }
    }
    
	let mut primes: Vec<i64> = Vec::new();

    for i in 0..ceiling {
        if v[i as usize] {
            primes.push(i);
        }
    }
    
    //for i in &primes {
    	//info!("{}", i);
	//}
    
    info!("done");
    
}
