#[macro_use]
extern crate bencher;

use task_06::grepz_cache as GrepzCache;
use task_06::normal_cache as NormalCache;
use task_06::webchemist_cache as WebchemistCache;
use bencher::Bencher;
use rand::Rng;
use std::time::Duration;

static BENCH_LARGE_SZ: usize = 100000;
static BENCH_SMALL_SZ: usize = 256;

fn grepz_expire_large(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut numbers_vec: Vec<(String, Box<usize>, Duration)> =
        Vec::with_capacity(BENCH_LARGE_SZ);
    for n in 0..BENCH_LARGE_SZ {
        numbers_vec.push(
            (n.to_string(),
             Box::new(n),
             Duration::from_secs(rng.gen_range(1..5))))
    }

    // std::thread::sleep(Duration::from_millis(2500));

    bench.iter(|| {
        let mut cache = GrepzCache::Cache::new();
        numbers_vec.iter().for_each(|item| {
            cache.insert(&item.0, &item.1, item.2);
        });
        cache.expire();
    })
}

fn grepz_expire_small(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut numbers_vec: Vec<(String, Box<usize>, Duration)> =
        Vec::with_capacity(BENCH_SMALL_SZ);
    for n in 0..BENCH_SMALL_SZ {
        numbers_vec.push(
            (n.to_string(),
             Box::new(n),
             Duration::from_millis(rng.gen_range(1..1000))))
    }

    // std::thread::sleep(Duration::from_millis(2500));

    bench.iter(|| {
        let mut cache = GrepzCache::Cache::new();
        numbers_vec.iter().for_each(|item| {
            cache.insert(&item.0, &item.1, item.2);
        });
        cache.expire();
    })
}

fn normal_expire_replace_large(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut numbers_vec: Vec<(String, Box<usize>, Duration)> =
        Vec::with_capacity(BENCH_LARGE_SZ);
    for n in 0..BENCH_LARGE_SZ {
        numbers_vec.push(
            (n.to_string(),
             Box::new(n),
             Duration::from_secs(rng.gen_range(1..5))))
    }

    // std::thread::sleep(Duration::from_millis(2500));

    bench.iter(|| {
        let mut cache = NormalCache::Cache::new();
        numbers_vec.iter().for_each(|item| {
            cache.insert(&item.0, &item.1, item.2);
        });
        cache.expire_replace();
    })
}

fn normal_expire_replace_small(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut numbers_vec: Vec<(String, Box<usize>, Duration)> =
        Vec::with_capacity(BENCH_SMALL_SZ);
    for n in 0..BENCH_SMALL_SZ {
        numbers_vec.push(
            (n.to_string(),
             Box::new(n),
             Duration::from_millis(rng.gen_range(1..1000))))
    }

    // std::thread::sleep(Duration::from_millis(2500));

    bench.iter(|| {
        let mut cache = NormalCache::Cache::new();
        numbers_vec.iter().for_each(|item| {
            cache.insert(&item.0, &item.1, item.2);
        });
        cache.expire_replace();
    })
}


fn normal_expire_retain_large(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut numbers_vec: Vec<(String, Box<usize>, Duration)> =
        Vec::with_capacity(BENCH_LARGE_SZ);
    for n in 0..BENCH_LARGE_SZ {
        numbers_vec.push(
            (n.to_string(),
             Box::new(n),
             Duration::from_secs(rng.gen_range(1..5))))
    }

    // std::thread::sleep(Duration::from_millis(2500));

    bench.iter(|| {
        let mut cache = NormalCache::Cache::new();
        numbers_vec.iter().for_each(|item| {
            cache.insert(&item.0, &item.1, item.2);
        });
        cache.expire_retain();
    })
}

fn normal_expire_retain_small(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut numbers_vec: Vec<(String, Box<usize>, Duration)> =
        Vec::with_capacity(BENCH_SMALL_SZ);
    for n in 0..BENCH_SMALL_SZ {
        numbers_vec.push(
            (n.to_string(),
             Box::new(n),
             Duration::from_millis(rng.gen_range(1..1000))))
    }

    // std::thread::sleep(Duration::from_millis(2500));

    bench.iter(|| {
        let mut cache = NormalCache::Cache::new();
        numbers_vec.iter().for_each(|item| {
            cache.insert(&item.0, &item.1, item.2);
        });
        cache.expire_retain();
    })
}

fn webchemist_expire_large(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut numbers_vec: Vec<(String, Box<usize>, Duration)> =
        Vec::with_capacity(BENCH_LARGE_SZ);
    for n in 0..BENCH_LARGE_SZ {
        numbers_vec.push(
            (n.to_string(),
             Box::new(n),
             Duration::from_secs(rng.gen_range(1..5))))
    }

    // std::thread::sleep(Duration::from_millis(2500));

    bench.iter(|| {
        let mut cache = WebchemistCache::Cache::new();
        numbers_vec.iter().for_each(|item| {
            cache.insert(&item.0, &item.1, item.2);
        });
        cache.expire();
    })
}

fn webchemist_expire_small(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut numbers_vec: Vec<(String, Box<usize>, Duration)> =
        Vec::with_capacity(BENCH_SMALL_SZ);
    for n in 0..BENCH_SMALL_SZ {
        numbers_vec.push(
            (n.to_string(),
             Box::new(n),
             Duration::from_millis(rng.gen_range(1..1000))))
    }

    // std::thread::sleep(Duration::from_millis(2500));

    bench.iter(|| {
        let mut cache = WebchemistCache::Cache::new();
        numbers_vec.iter().for_each(|item| {
            cache.insert(&item.0, &item.1, item.2);
        });
        cache.expire();
    })
}

benchmark_group!(benches_normal_retain, normal_expire_retain_large, normal_expire_retain_small);
benchmark_group!(benches_normal_expire, normal_expire_replace_large, normal_expire_replace_small);
benchmark_group!(benches_webchemist, webchemist_expire_large, webchemist_expire_small);
benchmark_group!(benches_grepz, grepz_expire_large, grepz_expire_small);
benchmark_main!(benches_grepz, benches_webchemist, benches_normal_retain, benches_normal_expire);
