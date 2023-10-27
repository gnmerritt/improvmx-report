use mailgun_v3::email::{EmailAddress, Message, MessageBody};
use mailgun_v3::Credentials;
use std::error::Error;

mod improvmx;

struct DomainReport {
    domain: String,
    undelivered: Vec<improvmx::UndeliveredMessage>,
    error: Option<String>,
}

fn email_report(report: String) -> Result<(), Box<dyn Error>> {
    let report_dest = std::env::var("REPORT_DEST")?;
    let mailgun_key = std::env::var("MG_KEY")?;
    let mailgun_domain = std::env::var("MG_DOMAIN")?;
    let report_from = std::env::var("REPORT_FROM")?;

    let sender = EmailAddress::address(report_from);
    let creds = Credentials::new(mailgun_key, mailgun_domain);
    let msg = Message {
        to: vec![EmailAddress::address(report_dest)],
        body: MessageBody::Text(report),
        subject: String::from("Undelivered email report"),
        ..Default::default()
    };
    let res = mailgun_v3::email::send_email(&creds, &sender, msg);
    println!("\nEmail send status: {:?}", res);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let key = std::env::var("IMPROVMX_KEY")?;
    let client = improvmx::ImprovMx::new(&key);
    let domains = client.domains()?;
    let reports = domains
        .into_iter()
        .map(|domain| match client.undelivered_messages(&domain) {
            Ok(undelivered) => DomainReport {
                domain: domain.domain,
                undelivered,
                error: None,
            },
            Err(e) => DomainReport {
                domain: domain.domain,
                undelivered: vec![],
                error: Some(e.to_string()),
            },
        });
    let mut report_body = "-- Email forwarding report --\n".to_owned();
    for r in reports {
        match r.error {
            Some(error_message) => {
                report_body.push_str(&format!(
                    "\n\n{} error fetching logs:\n{}\n",
                    r.domain, error_message
                ));
            }
            _ => {
                report_body.push_str(&format!(
                    "\n\n{} had {} undelivered messages\n\n",
                    r.domain,
                    r.undelivered.len()
                ));
                for msg in r.undelivered {
                    report_body.push_str(&format!(
                        "to: {} ({}) from: {},\n",
                        msg.to, msg.forwarded_to, msg.from
                    ));
                    report_body.push_str(&format!(
                        "    subj: {}\n    {}: {}",
                        msg.subject, msg.last_status, msg.last_message
                    ));
                }
            }
        }
    }
    print!("{}", report_body);
    email_report(report_body)?;
    Ok(())
}
