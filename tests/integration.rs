use sigmaos_pytest_pacman::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_pytest_pacman::PytestPacman::new("sigmaos-pytest-pacman").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-pytest-pacman"));
}
