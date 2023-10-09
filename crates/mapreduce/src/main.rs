use dataflow::prelude::{MapReduce, Node};

fn main() {
    let map_function = |input: &str| {
        let mut result = Vec::new();
        for word in input.split_whitespace() {
            result.push((word.to_string(), 1));
        }
        result
    };

    let reduce_function = |pair: (String, Vec<i32>)| {
        let (word, counts) = pair;
        let sum: i32 = counts.iter().sum();
        vec![(word, sum)]
    };

    let mut map_reduce = MapReduce::new(map_function, reduce_function);

    let input_data = vec!["hello world", "world world hello", "hello hello"];

    let output = map_reduce.process(input_data);

    let expected_output: Vec<(String, i32)> =
        vec![("hello".to_string(), 4), ("world".to_string(), 3)];

    assert_eq!(output, expected_output);
}
