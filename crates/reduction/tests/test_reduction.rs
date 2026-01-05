use ltsinfo_reduction::preprocess_branching;
use test_case::test_case;

use ltsinfo_io::io_aut::read_aut;
use ltsinfo_reduction::strong_bisim_sigref_naive;
use ltsinfo_reduction::strong_bisim_sigref;
use ltsinfo_utilities::Timing;

#[test_case(include_str!("../../../examples/lts/abp.aut") ; "abp.aut")]
#[test_case(include_str!("../../../examples/lts/selfloops.aut") ; "selfloops.aut")]
fn test_strong_bisimilation_reduction(input: &str) {
    let _ = env_logger::builder().is_test(true).try_init();

    let lts = read_aut(input.as_bytes(), vec![]).unwrap();
    let mut timing = Timing::new();

    let reduced = strong_bisim_sigref(&lts, &mut timing);
    let naive_reduced = strong_bisim_sigref_naive(&lts, &mut timing);

    assert_eq!(reduced, naive_reduced, "The partitions are not equal");
}

#[test_case(include_str!("../../../examples/lts/abp.aut") ; "abp.aut")]
#[test_case(include_str!("../../../examples/lts/selfloops.aut") ; "selfloops.aut")]
fn test_branching_bisimilation_reduction(input: &str) {
    let _ = env_logger::builder().is_test(true).try_init();

    let lts = read_aut(input.as_bytes(), vec!["tau".into(), "i".into()]).unwrap();
    let mut timing = Timing::new();
    let preprocessed = preprocess_branching(&lts);
    // let reduced = branching_bisim_sigref(&preprocessed, &mut timing);
    // let naive_reduced = branching_bisim_sigref_naive(&lts, &mut timing);

    // assert_eq!(reduced, naive_reduced, "The partitions are not equal");
}
