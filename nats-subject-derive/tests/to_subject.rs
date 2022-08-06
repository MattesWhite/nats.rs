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
