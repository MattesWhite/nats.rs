use async_nats::{
    subj,
    subject::{Error, FromSubjectError},
    FromSubject, ToSubject,
};

#[derive(Debug, PartialEq, Eq, ToSubject, FromSubject)]
#[subject("a.simple.subject")]
struct Simple;

#[test]
fn simple_to_subject() -> Result<(), Error> {
    let expected = subj!("a.simple.subject")?;
    let from_derive = Simple.to_subject()?;
    assert_eq!(expected, from_derive);
    // let parsed = Simple::from_subject(&from_derive)?;
    // assert_eq!(Simple, parsed);
    Ok(())
}

#[derive(Debug, PartialEq, Eq, ToSubject)]
#[subject("hi.[ > name ].age.[ number ]")]
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
    // let parsed = WithFields::from_subject(&from_derive)?;
    // assert_eq!(with_fields, parsed);
    Ok(())
}

#[derive(Debug, PartialEq, Eq, ToSubject, FromSubject)]
#[subject("[ > prefix ].api.[ number ].[ > rest ]")]
struct MultiField {
    prefix: String,
    number: u32,
    rest: String,
}

#[test]
fn multi_fields_subject() -> Result<(), Error> {
    let multi_fields = MultiField {
        prefix: "$My.prefix".to_string(),
        number: 21,
        rest: "some.trailing.stuff".to_string(),
    };
    let expected = subj!("$My.prefix.api.21.some.trailing.stuff")?;
    let from_derive = multi_fields.to_subject()?;
    assert_eq!(expected, from_derive);
    let parsed = MultiField::from_subject(&from_derive)?;
    assert_eq!(multi_fields, parsed);
    Ok(())
}