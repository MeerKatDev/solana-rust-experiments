use solana_program_test::*;

#[tokio::test]
async fn test_addition_instruction() {
    let _ = ProgramTest::new(
        "pinocchio_favorites_spt",
        pinocchio_favorites_spt::ID.into(),
        None,
    )
    .start_with_context();
}
