

use std::{collections::HashMap, hash::BuildHasherDefault};

use ahash::AHasher;
use criterion::{criterion_group, criterion_main, Criterion};


pub fn ahash(c: &mut Criterion) {
    c.bench_function("ahash", |b| {
        let data = [
            "variable", "srt_v", 
            "longer_variable", "a_fucking_test_of_a_variable", "w̵̫͖̄̃h̵̨͕̿͂ḁ̷̏ẗ̷̙́_̸̗̔t̷̗̋́h̵̤͘é̶͔͕_̵̛̟͜f̸̜̟͒ǔ̶͈͆c̶͌̽͜k̸̢͑_̸͙̬̓̊i̷̠̻̒ṡ̶̞̤̚_̴͙̌e̸̦͔͗̈́v̸̪̓͋ͅẽ̷̜̇ṋ̷̾_̴͔̫̏̋t̷͓́͌h̷̞͔̄i̵̳͖͋ŝ̶̡̃",
            "napoli", "java_code_1_de", 
            "de_peffoh", "sessanto!", "s̴̢̧̗̬̞̙̜̤͉̜̉̎̌͋̅̌̇̈́͊͝ǫ̷͔̳͖̟̤̣͙̙͍̳̮́̈̐̈́͂̓̑̄̓͑́̽̉͒m̷̙̙̮̣̥͔̽̽̍̔̎̈́͌̈͆̕͘͝͝e̸̜̹͕͙̮͚͔̩̗̹̯͈̐_̵͍̝̫̺̱̳͗̿̋̐̾͌͜͠ŗ̶̧̡̛̳̪̙̥̟͕̝̤̻̭͎̃̃̈́͗̓̇̇̀͑̅́͑̚͜͝ȧ̷͍̰̮̽̇̊̾̕n̴̢̡͚͖̦̽̅͒̈̍̿̂̾͜ḑ̷͎̮̂̑̈́̈́̐̒̀̓͌̃̈̓̆̔͛͜ǫ̶̧͇͈͖͕̩͇̫̬̠͇̫̻̈́̊̕͠ͅm̶̧̭̩̪̭̔̄̋͑̾̏̏̂̀͗̂̚̕̕_̵͇̭̻͚̖̣̱̈́̀̉͂͘?̶̨̡̭̫͇̺̹͓̔̓͐̆́̃̋̒̔͘ș̴͎͉̤̠̹͉̣̿́͋̕h̸͕͕̭̩̭͇͇̬̱͈̯̠̺̺̲͗͘i̸̡̨̡̨̭̭̘̙̣̟̟̦̺̋̈́͐͂̌̓͠ţ̴̦̲̰͔̥̥̦̦͈̖̥̾̓1̷̧̢̳͈̗̜͔̮̆̉͊",
            "variable", "srt_v", 
            "longer_variable", "a_fucking_test_of_a_variable", "w̵̫͖̄̃h̵̨͕̿͂ḁ̷̏ẗ̷̙́_̸̗̔t̷̗̋́h̵̤͘é̶͔͕_̵̛̟͜f̸̜̟͒ǔ̶͈͆c̶͌̽͜k̸̢͑_̸͙̬̓̊i̷̠̻̒ṡ̶̞̤̚_̴͙̌e̸̦͔͗̈́v̸̪̓͋ͅẽ̷̜̇ṋ̷̾_̴͔̫̏̋t̷͓́͌h̷̞͔̄i̵̳͖͋ŝ̶̡̃",
            "test_variable", "start", "printf", "execute", "namel", "int54",
            "build", "test", "ahash", "fxhash", "benchmark_func", "start", "new", "default",
            "plus", "minus", "clone", "star", "slash", "greater", 
        ];
        b.iter(|| {
            let mut hashmap = HashMap::<&str, u8, BuildHasherDefault<AHasher>>::default(); 
            for var in data.iter() {
                hashmap.insert(var, 1);
            }
        });
    });
}


criterion_group!(benches, ahash);
criterion_main!(benches);

