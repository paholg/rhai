use rhai::{Engine, EvalAltResult, Scope, INT};

#[test]
fn test_expressions() -> Result<(), EvalAltResult> {
    let mut engine = Engine::new();
    let mut scope = Scope::new();

    scope.push("x", 10 as INT);

    assert_eq!(engine.eval_expression::<INT>("2 + (10 + 10) * 2")?, 42);
    assert_eq!(
        engine.eval_expression_with_scope::<INT>(&mut scope, "2 + (x + 10) * 2")?,
        42
    );

    assert!(engine.eval_expression::<()>("40 + 2;").is_err());
    assert!(engine.eval_expression::<()>("40 + { 2 }").is_err());
    assert!(engine.eval_expression::<()>("x = 42").is_err());
    assert!(engine.compile_expression("let x = 42").is_err());
    assert!(engine
        .eval_expression_with_scope::<INT>(&mut scope, "if x > 0 { 42 } else { 123 }")
        .is_err());

    Ok(())
}

/// This example taken from https://github.com/jonathandturner/rhai/issues/115
#[test]
fn test_expressions_eval() -> Result<(), EvalAltResult> {
    #[derive(Debug, Clone)]
    struct AGENT {
        pub gender: String,
        pub age: INT,
    }

    impl AGENT {
        pub fn get_gender(&mut self) -> String {
            self.gender.clone()
        }
        pub fn get_age(&mut self) -> INT {
            self.age
        }
    }

    // This is your agent
    let my_agent = AGENT {
        gender: "male".into(),
        age: 42,
    };

    // Create the engine
    let mut engine = Engine::new();

    // Register your AGENT type
    engine.register_type_with_name::<AGENT>("AGENT");
    engine.register_get("gender", AGENT::get_gender);
    engine.register_get("age", AGENT::get_age);

    // Create your context, add the agent as a constant
    let mut scope = Scope::new();
    scope.push_constant("agent", my_agent);

    // Evaluate the expression
    let result: bool = engine.eval_expression_with_scope(
        &mut scope,
        r#"
                agent.age > 10 && agent.gender == "male"
        "#,
    )?;

    assert_eq!(result, true);

    Ok(())
}
