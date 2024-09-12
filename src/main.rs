fn main() {
    // power of 2 not yet supported...
    let n = 6561;
    println!("{:?}", quadratic_residues_of_prime_power(n).unwrap());
    println!("{:?}", quadratic_residues_of_prime_power(n).unwrap().len());

    println!("{}", fast_number_of_quadratic_residues_of_prime_power(n).unwrap());
}

fn fast_number_of_quadratic_residues_of_prime_power(pp: u128) -> Result<u128, String> {
    if prime_factorize(pp).len() != 1 {
        return Err("not a prime power...".to_string());
    }
    let (p, e) = *prime_factorize(pp).last().unwrap();
    let prime_qr = quadratic_residues_of_prime(p).unwrap();
    let number_of_dangerouses = prime_qr.iter().filter(|qr| !is_square(**qr)).count();
    let number_of_squares = floor_sqrt(p - 1);
    let mut res = (p - 1) / 2;
    for i in 2..e+1 {
        res *= p;
        if i & 1 == 1 {
            res += number_of_dangerouses as u128;
            res += number_of_squares;
        }
    }
    Ok(res)
}

fn floor_sqrt(n: u128) -> u128 {
    let mut i = 0;
    while i * i <= n {
        if i * i == n {
            return i;
        }
        i += 1;
    }
    i - 1
}

fn quadratic_residues_of_prime_power(pp: u128) -> Result<Vec<u128>, String> {
    if prime_factorize(pp).len() != 1 {
        return Err("not a prime power...".to_string());
    }
    let (p, e) = *prime_factorize(pp).last().unwrap();
    let prime_qr = quadratic_residues_of_prime(p).unwrap();
    let dangerous = prime_qr.iter().filter(|qr| !is_square(**qr)).collect::<Vec<_>>();
    let mut qrs = prime_qr.clone();
    let mut res = prime_qr.clone();
    for sisuu in 1..e {
        for j in 1..p {
            if sisuu & 1 == 0 && is_square(j) {
                res.push(j * p.pow(sisuu as _));
            }
            if sisuu & 1 == 0 && !is_square(j) && dangerous.contains(&&j) {
                res.push(j * p.pow(sisuu as _));
            }
            res.extend(qrs.iter().map(|qr| qr + j * p.pow(sisuu as _)).collect::<Vec<_>>().clone());
        }
        qrs = res.clone();
    }
    Ok(res)
}

fn is_square(n: u128) -> bool {
    let mut sqrt = 0;
    while sqrt * sqrt <= n {
        if sqrt * sqrt == n {
            return true;
        }
        sqrt += 1;
    }
    false
}

fn quadratic_residues_of_prime(n: u128) -> Result<Vec<u128>, String> {
    if !is_prime(n) {
        return Err("not a prime...".to_string());
    }
    let mut qrs = vec![1]; // excepted zero
    for i in 2..n {
        if mod_pow(i, (n - 1) / 2, n)? == 1 {
            qrs.push(i);
        }
    }
    Ok(qrs)
}

fn mod_pow(n: u128, exp: u128, modulo: u128) -> Result<u128, String> {
    if modulo < 1 {
        return Err("not a positive...".to_string());
    }
    Ok(match (n, exp, modulo) {
        (_, _, 1) => 0,
        (_, 0, _) => 1,
        (0, _, _) => 0,
        _ => {
            let mut res = n % modulo;
            for _ in 1..exp {
                res *= n;
                res %= modulo;
            }
            res
        }
    })
}

fn is_prime(n: u128) -> bool {
    if n < 3 {
        if n == 2 {
            return true;
        }
        return false;
    }
    if n & 1 == 0 {
        return false;
    }
    let mut odd = 3;
    while odd * odd <= n {
        if n % odd == 0 {
            return false;
        }
        odd += 2;
    }
    true
}

fn prime_factorize(n: u128) -> Vec<(u128, u128)> {
    if is_prime(n) {
        return vec![(n, 1)];
    }
    if n == 1 {
        return vec![];
    }
    let mut res = Vec::new();
    let mut n = n;
    let mut p = 2;
    while p * p <= n {
        let mut e = 0;
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            res.push((p, e))
        }
        p += 1;
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}
