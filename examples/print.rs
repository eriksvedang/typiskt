use tyr::ty::*;
use tyr::unifier::*;

fn main() {
    env_logger::init();

    let mut unifier = Unifier::new();

    // Unify (Type_0) with (a -> b), this should fail.
    unifier.add_constraint(Constraint::new(
        Ty::Data(TypeId(0), vec![]),
        Ty::Func(
            Box::new(Ty::Var(String::from("a"))),
            Box::new(Ty::Var(String::from("b"))),
        ),
    ));

    let result = unifier.solve();

    match result {
        Ok(_) => println!("OK."),
        Err(err) => println!("{}", err),
    }
}
