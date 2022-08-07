use async_nats::{subj, subject::Error, ToSubject};

#[derive(Debug, ToSubject)]
#[subject("a.simple.subject")]
struct Simple;

#[test]
fn simple_to_subject() -> Result<(), Error> {
    let expected = subj!("a.simple.subject")?;
    let from_derive = Simple.to_subject()?;
    assert_eq!(expected, from_derive);
    Ok(())
}

#[derive(Debug, ToSubject)]
#[subject("hi.{ name }.age.{ number }")]
struct WithFields {
    name: String,
    number: u32,
}

#[test]
fn fields_to_subject() -> Result<(), Error> {
    let with_fields = WithFields {
        name: "peter".to_string(),
        number: 42,
    };
    let expected = subj!("hi.{}.age.{}", "peter", 42)?;
    let from_derive = with_fields.to_subject()?;
    assert_eq!(expected, from_derive);
    Ok(())
}
