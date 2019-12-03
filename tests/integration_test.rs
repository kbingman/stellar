extern crate stellar;

use stellar::Star;

#[test]
fn o_main_sequence() {
    let star = Star::create_from_bv(-0.31, "V");

    assert_eq!(star.radius, 8.7546612237247);
    assert_eq!(star.luminosity, 16927.256163578622);
    assert_eq!(star.temp, 16991);
    assert_eq!(star.mass, 10.309057895579505);
    assert_eq!(star.spectral_class, "B7");
    assert_eq!(star.color, "Blue-White");
}

#[test]
fn g_main_sequence() {
    let star = Star::create_from_bv(0.749927, "V");

    assert_eq!(star.radius, 0.9470525016124451);
    assert_eq!(star.luminosity, 0.6357974611951837);
    assert_eq!(star.temp, 5436);
    assert_eq!(star.mass, 0.8929552548605576);
    assert_eq!(star.spectral_class, "G8");
    assert_eq!(star.color, "Yellow");
}

#[test]
fn f_main_sequence() {
    let star = Star::create_from_bv(0.32, "V");

    assert_eq!(star.radius, 1.679415144317238);
    assert_eq!(star.luminosity, 8.773298365605115);
    assert_eq!(star.temp, 7337);
    assert_eq!(star.mass, 1.721039051301309);
    assert_eq!(star.spectral_class, "F2");
    assert_eq!(star.color, "Yellow-White");
}

#[test]
fn a_main_sequence() {
    let star = Star::create_from_bv(0.1715, "V");

    assert_eq!(star.radius, 2.1781974741627175);
    assert_eq!(star.luminosity, 28.88098565224987);
    assert_eq!(star.temp, 8390);
    assert_eq!(star.mass, 2.095199296960982);
    assert_eq!(star.spectral_class, "A7");
    assert_eq!(star.color, "White");
}

#[test]
fn m_main_sequence() {
    let star = Star::create_from_bv(1.47, "V");

    assert_eq!(star.radius, 0.4962065645977362);
    assert_eq!(star.luminosity, 0.03289981815369502);
    assert_eq!(star.temp, 3839);
    assert_eq!(star.mass, 0.42947861739265913);
    assert_eq!(star.spectral_class, "K10");
    assert_eq!(star.color, "Orange");
}

#[test]
fn g_giant() {
    let star = Star::create_from_bv(0.86, "III");

    assert_eq!(star.radius, 7.250809222773779, "radius");
    assert_eq!(star.luminosity, 34.20119219547736, "luminosity");
    assert_eq!(star.temp, 5106);
    assert_eq!(star.mass, 2.1856602214208594);
    assert_eq!(star.spectral_class, "K1");
    assert_eq!(star.color, "Orange");
}

#[test]
fn b_main_sequence() {
    let star = Star::create_from_bv(-0.033, "V");

    assert_eq!(star.radius, 3.413270590776482);
    assert_eq!(star.luminosity, 226.1375097189035);
    assert_eq!(star.temp, 10556);
    assert_eq!(star.mass, 3.504818142850233);
    assert_eq!(star.spectral_class, "B10");
    assert_eq!(star.color, "Blue-White");
}
