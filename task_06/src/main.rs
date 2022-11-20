use task_06::grepz_cache;

use std::time::Duration;

// #[bench]
// fn bench_large_ivankin_expire(b: &mut Bencher) {
//     let mut rng = rand::thread_rng();
//     let mut numbers_vec: Vec<(String, Box<usize>, Duration)> =
//         Vec::with_capacity(BENCH_LARGE_SZ);
//     for n in 0..BENCH_LARGE_SZ {
//         numbers_vec.push(
//             (n.to_string(),
//              Box::new(n),
//              Duration::from_secs(rng.gen_range(1..10))))
//     }

//     b.iter(|| {
//         let mut cache = Cache::new();
//         numbers_vec.iter().for_each(|item| {
//             cache.insert(&item.0, &item.1, item.2);
//         });
//         cache.expire();
//     })
// }

fn main() {
    let data = vec![
        (String::from("one"), Box::new(1), Duration::from_secs(5)),
        (String::from("two"), Box::new(2), Duration::from_millis(1000)),
        (String::from("two"), Box::new(2), Duration::from_millis(1000)),
        (String::from("two"), Box::new(2), Duration::from_millis(999)),
        (String::from("two"), Box::new(2), Duration::from_millis(18446744073709551615)),
        (String::from("two"), Box::new(999), Duration::from_millis(1)),
        (String::from("three"), Box::new(3), Duration::from_secs(5)),
        (String::from("four"), Box::new(4), Duration::from_secs(1)),
        (String::from("fifteen"), Box::new(15), Duration::from_secs(2)),
        (String::from("fifteen"), Box::new(15), Duration::from_secs(3)),
        (String::from("twenty"), Box::new(20), Duration::from_millis(1510))
    ];

    let mut cache = grepz_cache::Cache::new();
    data.iter().for_each(|item| {
        cache.insert(&item.0, &item.1, item.2);
    });

    println!("GET method: {:#?}", cache.get("two"));
    println!("GET method: {:#?}", cache.get("four"));
    cache.remove("two");
    cache.remove("two");
    cache.remove("three");
    cache.remove("one");
    println!("Cache before expire: {:#?}", cache);

    std::thread::sleep(Duration::from_millis(1500));
    cache.expire();

    println!("Cache after expire: {:#?}", cache);

}
