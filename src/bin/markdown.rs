use serde_json::{Deserializer, Value};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("json.txt").unwrap();
    let runs = BenchRun::parse_all(&contents);
    // runs.sort_by(|a, b| a.id.cmp(&b.id));

    let mut results = HashMap::new();
    for run in runs.iter() {
        let key = (run.bench.as_str(), run.backend.as_str(), run.bit_size);
        results.insert(key, run.estimate);
    }

    let mut benches: Vec<&str> = runs.iter().map(|run| run.bench.as_str()).collect();
    benches.dedup();

    let mut bit_sizes: Vec<u64> = runs.iter().map(|run| run.bit_size).collect();
    bit_sizes.sort();
    bit_sizes.dedup();

    const BACKEND_WIDTH: usize = 8;

    for bench in benches {
        let backends: Vec<&str> = {
            let mut set = HashSet::new();
            runs.iter()
                .filter(|run| run.bench == bench && set.insert(run.backend.as_str()))
                .map(|run| run.backend.as_str())
                .collect()
        };

        print!("| {:^10} |", bench);
        for backend in backends.iter() {
            print!(" {:^width$} |", backend, width = BACKEND_WIDTH);
        }
        println!("");
        print!("|{:-<12}|", '-');
        for backend in backends.iter() {
            print!(
                " {:->width$} |",
                format!("-:"),
                width = BACKEND_WIDTH.max(backend.len())
            );
        }
        println!("");

        for &bit_size in bit_sizes.iter() {
            print!("| {:>10} |", format!("{} bits", bit_size));

            for &backend in backends.iter() {
                let key = (bench, backend, bit_size);
                if let Some(result) = results.get(&key) {
                    print!(
                        " {:>width$} |",
                        pp_duration(*result),
                        width = BACKEND_WIDTH.max(backend.len())
                    );
                } else {
                    print!(
                        " {:>width$} |",
                        ' ',
                        width = BACKEND_WIDTH.max(backend.len())
                    );
                }
            }
            println!("");
        }
        println!("");
    }
}

fn pp_duration(ns: f64) -> String {
    if ns > 1_000_000_000.0 {
        let s = ns / 1_000_000_000.0;
        format!("{:>.1}  s", s)
    } else if ns > 1_000_000.0 {
        let ms = ns / 1_000_000.0;
        format!("{:>.1} ms", ms)
    } else if ns > 1_000.0 {
        let us = ns / 1_000.0;
        format!("{:>.1} μs", us)
    } else {
        format!("{:>.1} ns", ns)
    }
}

#[derive(Debug, Clone)]
struct BenchRun {
    bench: String,
    backend: String,
    bit_size: u64,
    estimate: f64,
}

impl BenchRun {
    fn parse_all(input: &str) -> Vec<BenchRun> {
        let stream = Deserializer::from_str(input).into_iter::<Value>();
        stream
            .map(|v| v.unwrap())
            .filter_map(BenchRun::parse)
            .collect()
    }
    fn parse(v: Value) -> Option<BenchRun> {
        let mut id = v["id"].as_str()?.split('/');
        let bench = id.next()?.to_string();
        let bits = id
            .next()?
            .chars()
            .take_while(|ch| ch.is_digit(10))
            .collect::<String>();
        let backend = id.next()?.to_string();
        Some(BenchRun {
            bench,
            backend,
            bit_size: bits.parse().ok()?,
            estimate: v["typical"]["estimate"].as_f64()?,
        })
    }
}

/*
{"reason":"benchmark-complete"
,"id":"clone/64bits/uint"
,"report_directory":"/home/lemmih/coding/num-criterion/target/criterion/reports/clone_64bits/uint"\
,"iteration_count":[3474,6948,10422,13896,17370,20844,24318,27792,31266,34740,38214,41688,45162,48636,52110,55584,59058,62532,66006,69480,72954,76428,79902,83376,86850,90324,93798,97272,100746,104220,107694,111168,114642,118116,121590,125064,128538,132012,135486,138960,142434,145908,149382,152856,156330,159804,163278,166752,170226,173700,177174,180648,184122,187596,191070,194544,198018,201492,204966,208440,211914,215388,218862,222336,225810,229284,232758,236232,239706,243180,246654,250128,253602,257076,260550,264024,267498,270972,274446,277920,281394,284868,288342,291816,295290,298764,302238,305712,309186,312660,316134,319608,323082,326556,330030,333504,336978,340452,343926,347400]
,"measured_values":[5450.0,10180.0,15188.0,20160.0,25006.0,30425.0,35078.0,40537.0,45295.0,50014.0,55114.0,59806.0,64855.0,69783.0,75112.0,79761.0,100100.0,112910.0,92324.0,96523.0,101482.0,106293.0,110820.0,118433.0,120510.0,125427.0,129975.0,135177.0,140404.0,144597.0,151576.0,153701.0,158219.0,165173.0,169984.0,187948.0,187265.0,189779.0,196643.0,199640.0,205299.0,209997.0,218295.0,220346.0,225366.0,230817.0,234836.0,240276.0,245204.0,250174.0,255315.0,260825.0,270071.0,272396.0,285040.0,286722.0,285694.0,294207.0,295090.0,299717.0,304585.0,308935.0,313634.0,319365.0,323845.0,328943.0,343722.0,338573.0,343612.0,352307.0,358739.0,359382.0,363699.0,369520.0,380343.0,378187.0,383474.0,389128.0,393376.0,406792.0,423684.0,412403.0,415067.0,422855.0,425148.0,432458.0,436686.0,439475.0,434845.0,435296.0,441507.0,448160.0,448580.0,453940.0,464793.0,498716.0,476825.0,473188.0,484630.0,483856.0]
,"unit":"ns"
,"throughput":[]
,"typical": {"estimate":1.4325670625990756,"lower_bound":1.4248770384169342,"upper_bound":1.4407761974403446,"unit":"ns"}
,"mean":{"estimate":1.4401793550510666,"lower_bound":1.4305135699657614,"upper_bound":1.4519647918950247,"unit":"ns"}
,"median":{"estimate":1.4377471997495177,"lower_bound":1.434963298791019,"upper_bound":1.4405915371329878,"unit":"ns"}
,"median_abs_dev":{"estimate":0.019772408456793678,"lower_bound":0.008500320584654652,"upper_bound":0.03432105417096886,"unit":"ns"}
,"slope":{"estimate":1.4325670625990756,"lower_bound":1.4248770384169342,"upper_bound":1.4407761974403446,"unit":"ns"}
,"change":{"mean":{"estimate":0.025495114115220874,"lower_bound":0.017565565851336444,"upper_bound":0.034184515488389355,"unit":"%"}
,"median":{"estimate":0.028349824533216106,"lower_bound":0.02631669692906846,"upper_bound":0.030412469151133603,"unit":"%"}
,"change":"Regressed"}}

 */
