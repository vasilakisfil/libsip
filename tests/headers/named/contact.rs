use libsip::{headers::parse::parse_contact_header, *};

use nom::error::VerboseError;

#[test]
fn write() {
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(named_header!(uri, "Guy"));
    assert_eq!(
        "Contact: Guy <sip:guy@example.com>".to_string(),
        format!("{}", header)
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(named_header!(uri, "Guy With Face"));
    assert_eq!(
        "Contact: \"Guy With Face\" <sip:guy@example.com>".to_string(),
        format!("{}", header)
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(named_header!(uri));
    assert_eq!(
        "Contact: sip:guy@example.com".to_string(),
        format!("{}", header)
    );
/*
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let mut named_header = named_header!(uri);
    named_header.set_param(
        "+sip.instance",
        Some(GenValue::QuotedString("<urn:uuid:1e020c2b-46f6-4867-9d11-65547b8967fa>".into())),
    );
    named_header.set_param("expires", Some("600"));
    let header = Header::Contact(named_header);
    assert_eq!(
        "Contact: <sip:guy@example.com>;+sip.instance=\"<urn:uuid:1e020c2b-46f6-4867-9d11-65547b8967fa>\";expires=600\r\n",
        format!("{}", header)
    );

*/
}

#[test]
fn read() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(named_header!(uri, "Guy"));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_contact_header::<VerboseError<&[u8]>>(b"Contact: Guy <sip:guy@example.com>\r\n")
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(named_header!(uri, "Guy with face"));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_contact_header::<VerboseError<&[u8]>>(
            b"Contact: \"Guy with face\" <sip:guy@example.com>\r\n"
        )
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(named_header!(uri));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_contact_header::<VerboseError<&[u8]>>(b"Contact: <sip:guy@example.com>\r\n")
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let mut named_header = named_header!(uri);
    named_header.set_param(
        "+sip.instance",
        Some("<urn:uuid:1e020c2b-46f6-4867-9d11-65547b8967fa>"),
    );
    named_header.set_param("expires", Some("600"));
    assert_eq!(
        Some(&Some(String::from("<urn:uuid:1e020c2b-46f6-4867-9d11-65547b8967fa>").into())),
        named_header.parameters.get("+sip.instance")
    );
    let header = Header::Contact(named_header);
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_contact_header::<VerboseError<&[u8]>>(b"Contact: <sip:guy@example.com>;+sip.instance=\"<urn:uuid:1e020c2b-46f6-4867-9d11-65547b8967fa>\";expires=600\r\n")
    );
}
